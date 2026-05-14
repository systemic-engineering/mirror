//! @git/crystal — compilation cache backed by git refs.
//!
//! Contract:
//! - in: source text (bytes)
//! - out: cached Oid (if content hash matches) or None (cache miss)
//! - bound: O(1) lookup via git refs.
//!
//! Storage: `refs/crystals/<source_hash>` pointing to blobs containing crystal OIDs.
//! No filesystem cache. No `.shatter/`. No `.cache/`. Git refs ONLY.
//!
//! The git CLI is the socket. `git hash-object -w` stores. `git cat-file -p` reads.
//! This IS `@io` — the boundary between the compiler and the world.

use std::process::Command;

use crate::kernel::Oid;

/// Store a crystal OID in git as a ref.
///
/// Creates a blob containing the crystal OID string, then points
/// `refs/crystals/<source_hash>` at that blob.
///
/// Returns the git blob OID on success, None on failure.
pub fn git_store_crystal(source_hash: &str, crystal_oid: &str) -> Option<String> {
    // echo "crystal_oid" | git hash-object -w --stdin
    let blob = Command::new("git")
        .args(["hash-object", "-w", "--stdin"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(crystal_oid.as_bytes());
            }
            child.wait_with_output().ok()
        })
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })?;

    // git update-ref refs/crystals/<source_hash> <blob_oid>
    let status = Command::new("git")
        .args(["update-ref", &format!("refs/crystals/{}", source_hash), &blob])
        .stderr(std::process::Stdio::null())
        .status()
        .ok()?;

    if status.success() {
        Some(blob)
    } else {
        None
    }
}

/// Check if a crystal exists for this source hash.
///
/// Looks up `refs/crystals/<source_hash>`. If it exists, reads the blob
/// content (which is the crystal OID string).
///
/// Returns Some(crystal_oid) on hit, None on miss.
pub fn git_crystal_exists(source_hash: &str) -> Option<String> {
    // git cat-file -p refs/crystals/<source_hash>
    let output = Command::new("git")
        .args(["cat-file", "-p", &format!("refs/crystals/{}", source_hash)])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;

    if output.status.success() {
        let crystal_oid = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if crystal_oid.is_empty() {
            None
        } else {
            Some(crystal_oid)
        }
    } else {
        None
    }
}

/// Delete a crystal ref (for cleanup in tests).
pub fn git_delete_crystal(source_hash: &str) {
    let _ = Command::new("git")
        .args(["update-ref", "-d", &format!("refs/crystals/{}", source_hash)])
        .stderr(std::process::Stdio::null())
        .status();
}

/// Hash source content to produce the cache key.
pub fn source_hash(source: &str) -> String {
    Oid::hash(source.as_bytes()).as_ref().to_string()
}

/// Compile with git crystal cache: hash source, check git refs, tokenize only on miss.
///
/// Returns the content OID of the compiled AST.
pub fn compile_cached(
    source: &str,
    grammar: &crate::tokenize::Grammar,
) -> (Oid, bool) {
    let hash = source_hash(source);

    // Check git crystal cache
    if let Some(cached_oid) = git_crystal_exists(&hash) {
        return (Oid::new(cached_oid), true);
    }

    // Miss: tokenize
    let ast = crate::tokenize::tokenize(source, grammar);
    let oid = ast.content_oid();

    // Store in git (best-effort)
    let _ = git_store_crystal(&hash, oid.as_ref());

    (oid, false)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn git_crystal_roundtrip() {
        let test_hash = "test_roundtrip_abc123";
        let crystal_oid = "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";

        // Store
        let blob = git_store_crystal(test_hash, crystal_oid);
        assert!(blob.is_some(), "git_store_crystal must succeed");

        // Retrieve
        let cached = git_crystal_exists(test_hash);
        assert_eq!(cached, Some(crystal_oid.to_string()));

        // Clean up
        git_delete_crystal(test_hash);

        // Verify cleanup
        let after = git_crystal_exists(test_hash);
        assert_eq!(after, None, "crystal must be gone after delete");
    }

    #[test]
    fn git_crystal_miss_returns_none() {
        let result = git_crystal_exists("nonexistent_hash_xyzzy_99999");
        assert_eq!(result, None);
    }

    #[test]
    fn compile_caches_in_git() {
        let grammar = crate::tokenize::load_grammar("boot/std/code/rust.mirror")
            .expect("grammar");
        let source = "fn cached_test_unique_12345() { }";

        let hash = source_hash(source);

        // Ensure clean state
        git_delete_crystal(&hash);

        // First compile: should be a miss
        let (oid1, hit1) = compile_cached(source, &grammar);
        assert!(!hit1, "first compile must be a cache miss");

        // Second compile: should be a hit
        let (oid2, hit2) = compile_cached(source, &grammar);
        assert!(hit2, "second compile must be a cache hit");

        // Both must produce the same OID
        assert_eq!(oid1, oid2, "cached and uncached must produce same OID");

        // Clean up
        git_delete_crystal(&hash);
    }

    #[test]
    fn cache_invalidates_on_content_change() {
        let grammar = crate::tokenize::load_grammar("boot/std/code/rust.mirror")
            .expect("grammar");
        let source1 = "fn invalidate_test_aaa() { }";
        let source2 = "fn invalidate_test_bbb() { }";

        let hash1 = source_hash(source1);
        let hash2 = source_hash(source2);

        // Ensure clean state
        git_delete_crystal(&hash1);
        git_delete_crystal(&hash2);

        // Different source content -> different source_hash -> different OIDs
        let (oid1, _) = compile_cached(source1, &grammar);
        let (oid2, _) = compile_cached(source2, &grammar);
        assert_ne!(oid1, oid2, "different content must produce different OIDs");

        // Verify they don't cross-contaminate
        assert_ne!(hash1, hash2, "different content must have different hashes");

        // Clean up
        git_delete_crystal(&hash1);
        git_delete_crystal(&hash2);
    }

    #[test]
    fn compile_cached_with_real_mirror_file() {
        let grammar = crate::tokenize::load_grammar("boot/std/mirror/grammar.mirror")
            .expect("grammar");
        let source = std::fs::read_to_string("boot/std/kintsugi.mirror")
            .expect("kintsugi.mirror");

        let hash = source_hash(&source);
        git_delete_crystal(&hash);

        let (oid1, hit1) = compile_cached(&source, &grammar);
        assert!(!hit1, "first compile must miss");

        let (oid2, hit2) = compile_cached(&source, &grammar);
        assert!(hit2, "second compile must hit");
        assert_eq!(oid1, oid2);

        git_delete_crystal(&hash);
    }

    #[test]
    fn no_filesystem_cache_created() {
        // Verify that compile_cached does NOT create .shatter/ or .cache/ directories
        let grammar = crate::tokenize::load_grammar("boot/std/code/rust.mirror")
            .expect("grammar");
        let source = "fn no_fs_cache_test() { }";

        let hash = source_hash(source);
        git_delete_crystal(&hash);

        let _ = compile_cached(source, &grammar);

        assert!(
            !std::path::Path::new(".shatter").exists(),
            ".shatter/ must not exist"
        );
        assert!(
            !std::path::Path::new(".cache").exists(),
            ".cache/ must not exist"
        );

        git_delete_crystal(&hash);
    }
}
