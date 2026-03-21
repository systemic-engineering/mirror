//! C-FFI surface for the conversation crate.
//!
//! Exposes `conv_parse` and `conv_compile_grammar` as `extern "C"` functions
//! callable from C NIF wrappers.
//! Uses write-to-buffer pattern — no heap allocation crosses the FFI boundary.

use crate::compile;
use crate::matrix;
use crate::parse::Parse;
use crate::resolve::TypeRegistry;
use crate::ContentAddressed;
use crate::Vector;

/// Parse .conv source → content OID string.
///
/// Pipeline: source → parse → Prism → extract vocabulary → encode matrix.
/// In production with the `git` feature, commits the matrix to
/// `refs/conversation/<branch>` and returns the commit OID.
/// Without `git` (or in tests), returns the content-addressed OID.
///
/// The git commit path is gated on `not(test)` — tests that need git
/// integration exercise `commit_matrix_to_repo` directly with isolated
/// temp repos. `Repository::discover(".")` from a test runner would
/// find the real repo and race on shared refs.
pub fn parse_to_oid(source: &str) -> Result<String, String> {
    let tree = Parse
        .trace(source.to_string())
        .into_result()
        .map_err(|e| e.to_string())?;

    let vocabulary = matrix::extract_vocabulary(&tree);
    let _matrix = matrix::encode(&tree, &vocabulary);

    #[cfg(all(feature = "git", not(test)))]
    let oid = match commit_matrix(&_matrix) {
        Ok(commit_oid) => commit_oid,
        Err(_) => tree.content_oid().as_ref().to_string(),
    };

    #[cfg(any(not(feature = "git"), test))]
    let oid = tree.content_oid().as_ref().to_string();

    Ok(oid)
}

/// Compile .conv grammar source → ETF bytes for actor dispatch module.
///
/// Collects `in @x` declarations as Lenses — structural dependencies that
/// tell the compiled module where it lives in the domain graph.
fn compile_grammar_to_etf(source: &str) -> Result<Vec<u8>, String> {
    let ast = Parse
        .trace(source.to_string())
        .into_result()
        .map_err(|e| e.to_string())?;

    let grammar_node = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .ok_or_else(|| "no grammar block found".to_string())?;

    // Collect `in @x` declarations as Lens dependencies.
    // Self-references (in @domain where domain == grammar domain) are filtered out.
    let grammar_domain = grammar_node
        .data()
        .value
        .strip_prefix('@')
        .unwrap_or(&grammar_node.data().value);
    let lenses: Vec<String> = ast
        .children()
        .iter()
        .filter(|c| c.data().is_decl("in"))
        .map(|c| {
            c.data()
                .value
                .strip_prefix('@')
                .unwrap_or(&c.data().value)
                .to_string()
        })
        .filter(|d| d != grammar_domain)
        .collect();

    // Collect `extends @domain` children from the grammar node.
    let extends: Vec<String> = grammar_node
        .children()
        .iter()
        .filter(|c| c.data().is_ref("extends"))
        .map(|c| {
            c.data()
                .value
                .strip_prefix('@')
                .unwrap_or(&c.data().value)
                .to_string()
        })
        .collect();

    let registry = TypeRegistry::compile(grammar_node).map_err(|e| e.to_string())?;
    Ok(compile::emit_actor_module(&registry, &lenses, &extends))
}

/// Write FFI result to output buffer. Returns 0 on success, -1 on error.
unsafe fn write_ffi_result(
    result: Result<&[u8], &[u8]>,
    out_ptr: *mut u8,
    out_cap: usize,
    out_len: *mut usize,
) -> i32 {
    let (bytes, code) = match result {
        Ok(b) => (b, 0),
        Err(b) => (b, -1),
    };
    let n = bytes.len().min(out_cap);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_ptr, n);
    *out_len = n;
    code
}

/// Parse a .conv source string and return its content OID.
///
/// On success: returns 0, writes OID hex to `out_ptr` (up to `out_cap` bytes),
///             sets `*out_len` to the number of bytes written.
/// On error:   returns -1, writes error message to `out_ptr`, sets `*out_len`.
///
/// # Safety
///
/// - `src_ptr` must point to `src_len` valid UTF-8 bytes.
/// - `out_ptr` must point to a buffer of at least `out_cap` bytes.
/// - `out_len` must be a valid pointer.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn conv_parse(
    src_ptr: *const u8,
    src_len: usize,
    out_ptr: *mut u8,
    out_cap: usize,
    out_len: *mut usize,
) -> i32 {
    let source = match std::str::from_utf8(std::slice::from_raw_parts(src_ptr, src_len)) {
        Ok(s) => s,
        Err(_) => return write_ffi_result(Err(b"invalid UTF-8 input"), out_ptr, out_cap, out_len),
    };
    match parse_to_oid(source) {
        Ok(ref oid) => write_ffi_result(Ok(oid.as_bytes()), out_ptr, out_cap, out_len),
        Err(ref msg) => write_ffi_result(Err(msg.as_bytes()), out_ptr, out_cap, out_len),
    }
}

/// Compile a grammar block from .conv source into an actor dispatch module.
///
/// Parses the source, finds the first grammar block, compiles it via
/// TypeRegistry, then emits ETF-encoded EAF bytes for the actor module.
///
/// On success: returns 0, writes ETF bytes to `out_ptr` (up to `out_cap`),
///             sets `*out_len` to the number of bytes written.
/// On error:   returns -1, writes error message to `out_ptr`, sets `*out_len`.
///
/// # Safety
///
/// - `src_ptr` must point to `src_len` valid UTF-8 bytes.
/// - `out_ptr` must point to a buffer of at least `out_cap` bytes.
/// - `out_len` must be a valid pointer.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn conv_compile_grammar(
    src_ptr: *const u8,
    src_len: usize,
    out_ptr: *mut u8,
    out_cap: usize,
    out_len: *mut usize,
) -> i32 {
    let source = match std::str::from_utf8(std::slice::from_raw_parts(src_ptr, src_len)) {
        Ok(s) => s,
        Err(_) => return write_ffi_result(Err(b"invalid UTF-8 input"), out_ptr, out_cap, out_len),
    };
    match compile_grammar_to_etf(source) {
        Ok(ref etf) => write_ffi_result(Ok(etf), out_ptr, out_cap, out_len),
        Err(ref msg) => write_ffi_result(Err(msg.as_bytes()), out_ptr, out_cap, out_len),
    }
}

/// Write a Prism tree to git objects. Returns the root tree OID.
///
/// Maps Prism variants to git objects following fragmentation conventions:
/// - Shard → blob (data bytes)
/// - Fractal → tree with `.data` blob + numbered children
/// - Lens → tree with `.data` blob + `.lens` blob (target OIDs)
/// - Optics → tree with `.data` blob + `.lens` blob + numbered children
#[cfg(all(feature = "git", test))]
fn write_prism_tree(
    repo: &git2::Repository,
    tree: &crate::prism::Prism<crate::ast::AstNode>,
) -> Result<git2::Oid, git2::Error> {
    use fragmentation::encoding::Encode;

    let data_bytes = tree.data().encode();

    if tree.is_shard() {
        return repo.blob(&data_bytes);
    }

    let mut builder = repo.treebuilder(None)?;
    let data_oid = repo.blob(&data_bytes)?;
    builder.insert(".data", data_oid, 0o100644)?;

    // Lens targets
    if tree.is_lens() || !tree.targets().is_empty() {
        let lens_content: String = tree
            .targets()
            .iter()
            .map(|sha| sha.0.as_str())
            .collect::<Vec<&str>>()
            .join("\n");
        let lens_oid = repo.blob(lens_content.as_bytes())?;
        builder.insert(".lens", lens_oid, 0o100644)?;
    }

    // Children
    for (i, child) in tree.children().iter().enumerate() {
        let child_oid = write_prism_tree(repo, child)?;
        let mode = if child.is_shard() { 0o100644 } else { 0o040000 };
        builder.insert(format!("{:04}", i), child_oid, mode)?;
    }

    builder.write()
}

/// Deterministic Ed25519 signing key for conversation commits.
/// sha256("conversation") → 32-byte seed → Ed25519 keypair.
/// Same pattern as @compiler actor in Gleam (sha256("compiler") → keypair).
#[cfg(feature = "git")]
fn conversation_key() -> Result<ssh_key::PrivateKey, String> {
    use sha2::{Digest, Sha256};
    use ssh_key::private::{Ed25519Keypair, KeypairData};

    let seed: [u8; 32] = Sha256::digest(b"conversation").into();
    let keypair = Ed25519Keypair::from_seed(&seed);
    let key_data = KeypairData::Ed25519(keypair);
    ssh_key::PrivateKey::new(key_data, "conversation@systemic.engineering")
        .map_err(|e| format!("key: {}", e))
}

/// Sign a commit buffer with the conversation SSH key.
/// Returns PEM-encoded SSH signature suitable for git.
#[cfg(feature = "git")]
fn sign_commit(commit_content: &[u8]) -> Result<String, String> {
    let key = conversation_key()?;
    let sig = key
        .sign("git", ssh_key::HashAlg::Sha512, commit_content)
        .map_err(|e| format!("sign: {}", e))?;
    let pem = sig
        .to_pem(ssh_key::LineEnding::LF)
        .map_err(|e| format!("pem: {}", e))?;
    Ok(pem)
}

/// Commit a parsed Prism to a git repository.
///
/// Writes the Prism tree to git objects, creates an SSH-signed commit
/// authored by `conversation@systemic.engineering`, and updates
/// `refs/conversation/<branch>`. Returns the commit OID hex string.
#[cfg(all(feature = "git", test))]
fn commit_prism_to_repo(
    repo: &git2::Repository,
    tree: &crate::prism::Prism<crate::ast::AstNode>,
) -> Result<String, String> {
    let head = repo.head().map_err(|e| format!("HEAD: {}", e))?;
    let branch = head
        .shorthand()
        .ok_or_else(|| "HEAD: not a branch".to_string())?
        .to_string();

    let tree_oid = write_prism_tree(repo, tree).map_err(|e| format!("write tree: {}", e))?;
    let git_tree = repo
        .find_tree(tree_oid)
        .map_err(|e| format!("find tree: {}", e))?;

    let sig = git2::Signature::now("conversation", "conversation@systemic.engineering")
        .map_err(|e| format!("signature: {}", e))?;

    // Find parent commit on refs/conversation/<branch> if it exists
    let ref_name = format!("refs/conversation/{}", branch);
    let parent_commit;
    let parents: Vec<&git2::Commit> = match repo.find_reference(&ref_name) {
        Ok(r) => {
            let oid = r.target().ok_or_else(|| "ref: no target".to_string())?;
            parent_commit = repo
                .find_commit(oid)
                .map_err(|e| format!("parent commit: {}", e))?;
            vec![&parent_commit]
        }
        Err(_) => vec![],
    };

    // Build commit content, sign it, write signed commit
    let commit_buf = repo
        .commit_create_buffer(&sig, &sig, "prism", &git_tree, &parents)
        .map_err(|e| format!("commit buffer: {}", e))?;
    let commit_content =
        std::str::from_utf8(&commit_buf).map_err(|e| format!("commit content: {}", e))?;

    let signature = sign_commit(commit_buf.as_ref())?;

    let commit_oid = repo
        .commit_signed(commit_content, &signature, Some("gpgsig"))
        .map_err(|e| format!("signed commit: {}", e))?;

    // Update the ref to point to the new commit
    repo.reference(&ref_name, commit_oid, true, "conversation: prism commit")
        .map_err(|e| format!("update ref: {}", e))?;

    Ok(commit_oid.to_string())
}

/// Discover the git repo from cwd and commit the Prism tree.
#[cfg(all(feature = "git", test))]
fn commit_prism(tree: &crate::prism::Prism<crate::ast::AstNode>) -> Result<String, String> {
    let repo = git2::Repository::discover(".").map_err(|e| format!("git repo: {}", e))?;
    commit_prism_to_repo(&repo, tree)
}

/// Commit a matrix to a git repository.
///
/// Writes the matrix bytes as a blob, wraps it in a tree with a `.matrix` entry,
/// creates an SSH-signed commit authored by `conversation@systemic.engineering`,
/// and updates `refs/conversation/<branch>`. Returns the commit OID hex string.
#[cfg(feature = "git")]
fn commit_matrix_to_repo(repo: &git2::Repository, mat: &matrix::Matrix) -> Result<String, String> {
    let head = repo.head().map_err(|e| format!("HEAD: {}", e))?;
    let branch = head
        .shorthand()
        .ok_or_else(|| "HEAD: not a branch".to_string())?
        .to_string();

    // Write matrix bytes as a blob
    let matrix_bytes = mat.to_bytes();
    let blob_oid = repo
        .blob(&matrix_bytes)
        .map_err(|e| format!("write blob: {}", e))?;

    // Existence check: if the latest commit already has an identical .matrix
    // blob, return the existing commit OID. Git is content-addressed — same
    // bytes → same blob OID — so this is deterministic.
    let ref_name = format!("refs/conversation/{}", branch);
    if let Ok(r) = repo.find_reference(&ref_name) {
        if let Some(target) = r.target() {
            if let Ok(commit) = repo.find_commit(target) {
                if let Ok(tree) = commit.tree() {
                    if let Some(entry) = tree.get_name(".matrix") {
                        if entry.id() == blob_oid {
                            return Ok(target.to_string());
                        }
                    }
                }
            }
        }
    }

    // Wrap in a tree with a `.matrix` entry
    let mut builder = repo
        .treebuilder(None)
        .map_err(|e| format!("treebuilder: {}", e))?;
    builder
        .insert(".matrix", blob_oid, 0o100644)
        .map_err(|e| format!("insert .matrix: {}", e))?;
    let tree_oid = builder.write().map_err(|e| format!("write tree: {}", e))?;
    let git_tree = repo
        .find_tree(tree_oid)
        .map_err(|e| format!("find tree: {}", e))?;

    let sig = git2::Signature::now("conversation", "conversation@systemic.engineering")
        .map_err(|e| format!("signature: {}", e))?;

    // Find parent commit on refs/conversation/<branch> if it exists
    let parent_commit;
    let parents: Vec<&git2::Commit> = match repo.find_reference(&ref_name) {
        Ok(r) => {
            let oid = r.target().ok_or_else(|| "ref: no target".to_string())?;
            parent_commit = repo
                .find_commit(oid)
                .map_err(|e| format!("parent commit: {}", e))?;
            vec![&parent_commit]
        }
        Err(_) => vec![],
    };

    // Build commit content, sign it, write signed commit
    let commit_buf = repo
        .commit_create_buffer(&sig, &sig, "matrix", &git_tree, &parents)
        .map_err(|e| format!("commit buffer: {}", e))?;
    let commit_content =
        std::str::from_utf8(&commit_buf).map_err(|e| format!("commit content: {}", e))?;

    let signature = sign_commit(commit_buf.as_ref())?;

    let commit_oid = repo
        .commit_signed(commit_content, &signature, Some("gpgsig"))
        .map_err(|e| format!("signed commit: {}", e))?;

    // Update the ref to point to the new commit
    repo.reference(&ref_name, commit_oid, true, "conversation: matrix commit")
        .map_err(|e| format!("update ref: {}", e))?;

    Ok(commit_oid.to_string())
}

/// Discover the git repo from cwd and commit matrix.
#[cfg(feature = "git")]
fn commit_matrix(mat: &matrix::Matrix) -> Result<String, String> {
    let repo = git2::Repository::discover(".").map_err(|e| format!("git repo: {}", e))?;
    commit_matrix_to_repo(&repo, mat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_success() {
        let oid = parse_to_oid("grammar @test {\n  type = a | b\n}\n").unwrap();
        assert!(!oid.is_empty());
    }

    #[test]
    fn parse_error() {
        let err = parse_to_oid("@@@invalid").unwrap_err();
        assert!(!err.is_empty());
    }

    #[test]
    fn parse_deterministic() {
        let a = parse_to_oid("grammar @test {\n  type = a | b\n}\n").unwrap();
        let b = parse_to_oid("grammar @test {\n  type = a | b\n}\n").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn compile_grammar_success() {
        let etf = compile_grammar_to_etf(
            "grammar @compiler {\n  type = target\n  type target = eaf | beam\n  action compile {\n    source: target\n  }\n}\n",
        )
        .unwrap();
        assert!(!etf.is_empty());
        assert_eq!(etf[0], 131); // ETF version byte
    }

    #[test]
    fn compile_grammar_parse_error() {
        let err = compile_grammar_to_etf("!!! not valid conv syntax").unwrap_err();
        assert!(!err.is_empty());
    }

    #[test]
    fn compile_grammar_type_ref_error() {
        let err =
            compile_grammar_to_etf("grammar @test {\n  type = when(nonexistent)\n}\n").unwrap_err();
        assert!(err.contains("unknown type"));
    }

    #[test]
    fn compile_grammar_no_grammar_block() {
        let err = compile_grammar_to_etf("in @filesystem\ntemplate $t {\n\tslug\n}\n").unwrap_err();
        assert!(err.contains("grammar"));
    }

    #[test]
    fn compile_grammar_deterministic() {
        let a = compile_grammar_to_etf(
            "grammar @test {\n  type = a | b\n  action ping {\n    target: a\n  }\n}\n",
        )
        .unwrap();
        let b = compile_grammar_to_etf(
            "grammar @test {\n  type = a | b\n  action ping {\n    target: a\n  }\n}\n",
        )
        .unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn compile_grammar_includes_lenses() {
        // Source with `in @reality` sibling — compile should include the Lens
        let etf = compile_grammar_to_etf(
            "grammar @filesystem {\n  type = file | folder\n}\n\nin @filesystem\nin @reality\n",
        )
        .unwrap();

        // Decode and verify the lenses/0 function is present
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let forms_str = format!("{:?}", term);
        assert!(
            forms_str.contains("lenses"),
            "expected 'lenses' export in EAF: {}",
            forms_str,
        );
        // "reality" encoded as ByteList bytes
        let reality_bytes: Vec<u8> = "reality".bytes().collect();
        assert!(
            forms_str.contains(&format!("{:?}", reality_bytes)),
            "expected 'reality' Lens bytes in EAF: {}",
            forms_str,
        );
    }

    #[test]
    fn compile_grammar_no_lenses_when_no_in() {
        // Source with grammar only — no `in` declarations
        let etf = compile_grammar_to_etf("grammar @test {\n  type = a | b\n}\n").unwrap();
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let forms_str = format!("{:?}", term);
        // lenses/0 should still exist but return empty list
        assert!(
            forms_str.contains("lenses"),
            "expected 'lenses' export even when empty: {}",
            forms_str,
        );
    }

    #[test]
    fn compile_grammar_includes_extends() {
        let etf = compile_grammar_to_etf(
            "grammar @fox extends @smash, @controller {\n  type = move | attack\n}\n",
        )
        .unwrap();

        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let forms_str = format!("{:?}", term);
        assert!(
            forms_str.contains("extends"),
            "expected 'extends' export in EAF: {}",
            forms_str,
        );
        let smash_bytes: Vec<u8> = "smash".bytes().collect();
        assert!(
            forms_str.contains(&format!("{:?}", smash_bytes)),
            "expected 'smash' extends bytes in EAF: {}",
            forms_str,
        );
        let controller_bytes: Vec<u8> = "controller".bytes().collect();
        assert!(
            forms_str.contains(&format!("{:?}", controller_bytes)),
            "expected 'controller' extends bytes in EAF: {}",
            forms_str,
        );
    }

    #[test]
    fn compile_grammar_no_extends_when_absent() {
        let etf = compile_grammar_to_etf("grammar @test {\n  type = a | b\n}\n").unwrap();
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let forms_str = format!("{:?}", term);
        // extends/0 should still exist but return empty list
        assert!(
            forms_str.contains("extends"),
            "expected 'extends' export even when empty: {}",
            forms_str,
        );
    }

    // -- FFI wrappers: exercise unsafe boundary + UTF-8 rejection --

    #[test]
    fn ffi_conv_parse_roundtrip() {
        let source = b"grammar @test {\n  type = a | b\n}\n";
        let mut buf = [0u8; 256];
        let mut len: usize = 0;
        let rc = unsafe {
            conv_parse(
                source.as_ptr(),
                source.len(),
                buf.as_mut_ptr(),
                buf.len(),
                &mut len,
            )
        };
        assert_eq!(rc, 0);
        assert!(len > 0);
    }

    #[test]
    fn ffi_conv_parse_invalid_utf8() {
        let source: &[u8] = &[0xFF, 0xFE, 0x00];
        let mut buf = [0u8; 256];
        let mut len: usize = 0;
        let rc = unsafe {
            conv_parse(
                source.as_ptr(),
                source.len(),
                buf.as_mut_ptr(),
                buf.len(),
                &mut len,
            )
        };
        assert_eq!(rc, -1);
        let msg = std::str::from_utf8(&buf[..len]).unwrap();
        assert!(msg.contains("UTF-8"));
    }

    #[test]
    fn ffi_conv_compile_grammar_roundtrip() {
        let source = b"grammar @test {\n  type = a | b\n}\n";
        let mut buf = [0u8; 4096];
        let mut len: usize = 0;
        let rc = unsafe {
            conv_compile_grammar(
                source.as_ptr(),
                source.len(),
                buf.as_mut_ptr(),
                buf.len(),
                &mut len,
            )
        };
        assert_eq!(rc, 0);
        assert!(len > 0);
    }

    #[test]
    fn ffi_conv_parse_error() {
        let source = b"@@@invalid";
        let mut buf = [0u8; 256];
        let mut len: usize = 0;
        let rc = unsafe {
            conv_parse(
                source.as_ptr(),
                source.len(),
                buf.as_mut_ptr(),
                buf.len(),
                &mut len,
            )
        };
        assert_eq!(rc, -1);
        assert!(len > 0);
    }

    #[test]
    fn ffi_conv_compile_grammar_error() {
        let source = b"in @filesystem\ntemplate $t {\n\tslug\n}\n";
        let mut buf = [0u8; 4096];
        let mut len: usize = 0;
        let rc = unsafe {
            conv_compile_grammar(
                source.as_ptr(),
                source.len(),
                buf.as_mut_ptr(),
                buf.len(),
                &mut len,
            )
        };
        assert_eq!(rc, -1);
        assert!(len > 0);
    }

    #[test]
    fn ffi_conv_compile_grammar_invalid_utf8() {
        let source: &[u8] = &[0xFF, 0xFE, 0x00];
        let mut buf = [0u8; 4096];
        let mut len: usize = 0;
        let rc = unsafe {
            conv_compile_grammar(
                source.as_ptr(),
                source.len(),
                buf.as_mut_ptr(),
                buf.len(),
                &mut len,
            )
        };
        assert_eq!(rc, -1);
        let msg = std::str::from_utf8(&buf[..len]).unwrap();
        assert!(msg.contains("UTF-8"));
    }

    // -- git commit tests --

    #[cfg(feature = "git")]
    mod git_tests {
        use super::*;

        fn init_repo_with_branch() -> (tempfile::TempDir, git2::Repository) {
            let dir = tempfile::tempdir().unwrap();
            let repo = git2::Repository::init(dir.path()).unwrap();

            // Create an initial commit on an explicit "main" branch so HEAD
            // has a deterministic shorthand regardless of git config.
            {
                let sig = git2::Signature::now("test", "test@test").unwrap();
                let tree_oid = repo.treebuilder(None).unwrap().write().unwrap();
                let tree = repo.find_tree(tree_oid).unwrap();
                repo.commit(Some("refs/heads/main"), &sig, &sig, "init", &tree, &[])
                    .unwrap();
                repo.set_head("refs/heads/main").unwrap();
            }

            (dir, repo)
        }

        #[test]
        fn commit_prism_creates_signed_ref() {
            let (_dir, repo) = init_repo_with_branch();

            let source = "grammar @test {\n  type = a | b\n}\n";
            let tree = Parse.trace(source.to_string()).into_result().unwrap();
            let oid_str = commit_prism_to_repo(&repo, &tree).unwrap();
            assert!(!oid_str.is_empty());

            // Verify the ref exists
            let reference = repo.find_reference("refs/conversation/main").unwrap();
            let commit_oid = reference.target().unwrap();
            assert_eq!(commit_oid.to_string(), oid_str);

            // Verify commit metadata
            let commit = repo.find_commit(commit_oid).unwrap();
            assert_eq!(commit.author().name(), Some("conversation"));
            assert_eq!(
                commit.author().email(),
                Some("conversation@systemic.engineering")
            );
            assert_eq!(commit.message(), Some("prism"));

            // Verify SSH signature exists
            let (sig_bytes, _) = repo.extract_signature(&commit_oid, None).unwrap();
            let sig_str = std::str::from_utf8(&sig_bytes).unwrap();
            assert!(sig_str.contains("BEGIN SSH SIGNATURE"));
        }

        #[test]
        fn commit_prism_chains_parents() {
            let (_dir, repo) = init_repo_with_branch();

            let source1 = "grammar @a {\n  type = x\n}\n";
            let tree1 = Parse.trace(source1.to_string()).into_result().unwrap();
            let oid1 = commit_prism_to_repo(&repo, &tree1).unwrap();

            let source2 = "grammar @b {\n  type = y\n}\n";
            let tree2 = Parse.trace(source2.to_string()).into_result().unwrap();
            let oid2 = commit_prism_to_repo(&repo, &tree2).unwrap();

            assert_ne!(oid1, oid2);

            // Second commit should have first as parent
            let commit2 = repo
                .find_commit(git2::Oid::from_str(&oid2).unwrap())
                .unwrap();
            assert_eq!(commit2.parent_id(0).unwrap().to_string(), oid1);
        }

        #[test]
        fn write_prism_tree_roundtrip() {
            let (_dir, repo) = init_repo_with_branch();

            let source = "grammar @test {\n  type = a | b\n}\n";
            let tree = Parse.trace(source.to_string()).into_result().unwrap();
            let oid = write_prism_tree(&repo, &tree).unwrap();

            // The OID should be a valid git object
            let obj = repo.find_object(oid, None).unwrap();
            assert!(obj.kind() == Some(git2::ObjectType::Tree));
        }

        #[test]
        fn commit_matrix_creates_signed_ref() {
            let (_dir, repo) = init_repo_with_branch();

            let mat = matrix::Matrix::identity(2);
            let oid_str = commit_matrix_to_repo(&repo, &mat).unwrap();
            assert!(!oid_str.is_empty());

            // Verify the ref exists
            let reference = repo.find_reference("refs/conversation/main").unwrap();
            let commit_oid = reference.target().unwrap();
            assert_eq!(commit_oid.to_string(), oid_str);

            // Verify commit metadata
            let commit = repo.find_commit(commit_oid).unwrap();
            assert_eq!(commit.author().name(), Some("conversation"));
            assert_eq!(
                commit.author().email(),
                Some("conversation@systemic.engineering")
            );
            assert_eq!(commit.message(), Some("matrix"));

            // Verify SSH signature exists
            let (sig_bytes, _) = repo.extract_signature(&commit_oid, None).unwrap();
            let sig_str = std::str::from_utf8(&sig_bytes).unwrap();
            assert!(sig_str.contains("BEGIN SSH SIGNATURE"));

            // Verify the tree contains a .matrix blob
            let tree = commit.tree().unwrap();
            let entry = tree.get_name(".matrix").unwrap();
            assert_eq!(entry.kind(), Some(git2::ObjectType::Blob));

            // Verify the blob contents are valid matrix bytes
            let blob = repo.find_blob(entry.id()).unwrap();
            let roundtrip = matrix::Matrix::from_bytes(blob.content()).unwrap();
            assert_eq!(roundtrip.n, 2);
            assert!((roundtrip.get(0, 0) - 1.0).abs() < 1e-12);
        }

        #[test]
        fn commit_matrix_chains_parents() {
            let (_dir, repo) = init_repo_with_branch();

            let m1 = matrix::Matrix::identity(2);
            let oid1 = commit_matrix_to_repo(&repo, &m1).unwrap();

            let mut m2 = matrix::Matrix::zeros(2);
            m2.set(0, 0, 1.0);
            let oid2 = commit_matrix_to_repo(&repo, &m2).unwrap();

            assert_ne!(oid1, oid2);

            // Second commit should have first as parent
            let commit2 = repo
                .find_commit(git2::Oid::from_str(&oid2).unwrap())
                .unwrap();
            assert_eq!(commit2.parent_id(0).unwrap().to_string(), oid1);
        }

        #[test]
        fn commit_matrix_idempotent() {
            let (_dir, repo) = init_repo_with_branch();

            let mat = matrix::Matrix::identity(3);
            let oid1 = commit_matrix_to_repo(&repo, &mat).unwrap();
            let oid2 = commit_matrix_to_repo(&repo, &mat).unwrap();

            // Same matrix → same commit OID (no new commit created)
            assert_eq!(oid1, oid2);

            // Only one commit on refs/conversation/main (not two)
            let reference = repo.find_reference("refs/conversation/main").unwrap();
            let commit = repo.find_commit(reference.target().unwrap()).unwrap();
            assert_eq!(commit.parent_count(), 0);
        }

        #[test]
        fn commit_matrix_different_creates_new() {
            let (_dir, repo) = init_repo_with_branch();

            let m1 = matrix::Matrix::identity(3);
            let oid1 = commit_matrix_to_repo(&repo, &m1).unwrap();

            let mut m2 = matrix::Matrix::zeros(3);
            m2.set(1, 1, 42.0);
            let oid2 = commit_matrix_to_repo(&repo, &m2).unwrap();

            // Different matrix → different commit
            assert_ne!(oid1, oid2);

            // Parent chain intact
            let commit2 = repo
                .find_commit(git2::Oid::from_str(&oid2).unwrap())
                .unwrap();
            assert_eq!(commit2.parent_id(0).unwrap().to_string(), oid1);
        }
    }
}
