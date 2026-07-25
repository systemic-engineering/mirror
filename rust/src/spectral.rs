//! `spectral.rs` — runtime shard manifest via `phone::list_dir_recursive`.
//!
//! ## Tick 2 (2026-07-25): build.rs shortcut RETIRED
//!
//! Alex 2026-07-25 verbatim: "build.rs in spectral, that's a smell.
//! What's the math underneath? And how do we collapse? We might need
//! a rust/roomba/ for the first order sub-Turing execution machinery."
//!
//! The math underneath IS the colimit computation over the shard-
//! manifold: `H = colim(shard_decl_fibres)`. The substrate reads
//! itself (@mirror/reflection) to construct its own H-space by folding
//! over its declarations. That is Foerster COORD applied to the
//! shard-tree. **@roomba's walk IS the colimit computation.**
//!
//! build.rs was a compile-time @roomba shortcut that bypassed the
//! substrate's own walker at a substrate-invisible altitude (cargo
//! build script). This tick retires it: the walk happens at runtime
//! via `phone::list_dir_recursive` (existing `@io/fs` primitive),
//! `shard_paths()` returns `Vec<String>` computed on invocation.
//!
//! Substrate delta: `rust/build.rs` deleted (-103 LOC); this file
//! restructured (net minor). Rust FLOOR shrinks; substrate-honesty
//! grows. Sub-Turing preserved: walk is bounded by finite shard-tree
//! and per-step decidable dispatch.
//!
//! ## Forward-promise: migration to rust/roomba/
//!
//! The runtime walk currently lives inline in this module. Per Alex
//! 2026-07-25 four-crate decomposition, it migrates to a new crate
//! `rust/roomba/` (first-order sub-Turing execution machinery). At
//! that migration, this function becomes a thin call into
//! `roomba::walk_shards(root)`; signature (Vec<String> return)
//! preserved for source-compat.
//!
//! ## Composition anchors (LANDED)
//!
//! - `rust/src/phone.rs` — `list_dir_recursive` @io/fs primitive +
//!   `find_substrate_root` (walks upward for `shards/` directory).
//! - `prism/prismqueer/src/bundle.rs:71-193` — Connes (A, H, D) tower.
//! - `prism/prismqueer/src/lib.rs:216-238` — `apply_h` action.
//! - `mirror/docs/specs/spectral-triple-grammar.md` (22.1KB canonical).
//! - `mirror/shards/epistemologic/spectral_triple.mirror` (substrate).
//!
//! ## History
//!
//! - Tick 1 (2026-07-23, commit `4185255`): rust/build.rs walked at
//!   cargo build time; emitted `$OUT_DIR/shard_manifest.rs`; this
//!   module `include!()`d it. First empirical pipeline proof.
//! - Tick 2 (2026-07-25, this landing): build.rs retired per Alex
//!   substrate-smell callout; runtime walk via phone::list_dir_recursive.

use std::path::PathBuf;

use crate::phone;

/// Runtime manifest of every `shards/**/*.mirror` file present in the
/// workspace when called. Paths are `shards/<relative>` strings,
/// sorted lexicographically for byte-stable output.
///
/// Composes over `phone::find_substrate_root` (walks upward from CWD
/// looking for `shards/` directory) + `phone::list_dir_recursive`
/// (recursive walker that already skips `.git/` + `target/` +
/// symlinks per phone.rs walker discipline).
///
/// Migration forward-promise: this function migrates to
/// `roomba::walk_shards(root)` per Alex 2026-07-25 four-crate
/// decomposition. Return signature (`Vec<String>`) preserved for
/// source-compat.
pub fn shard_paths() -> Vec<String> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let root = phone::find_substrate_root(&cwd);
    let shards_root = root.join("shards");

    let mut paths: Vec<String> = Vec::new();
    if let Ok(entries) = phone::list_dir_recursive(&shards_root) {
        for entry in entries {
            if entry.is_dir {
                continue;
            }
            if entry.path.extension().and_then(|e| e.to_str()) != Some("mirror") {
                continue;
            }
            if let Ok(rel) = entry.path.strip_prefix(&root) {
                // Force POSIX-style separators for byte-stable output
                // across macOS/Linux (both use `/`; this is defense-in-depth).
                let rel_str = rel.to_string_lossy().replace('\\', "/");
                paths.push(rel_str);
            }
        }
    }
    paths.sort();
    paths
}

#[cfg(test)]
mod tests {
    //! Tick 2 pipeline-proof: runtime walk yields the same manifest
    //! shape that Tick 1's compile-time embed did. Non-empty; contains
    //! known shards; sorted.

    use super::shard_paths;

    #[test]
    fn manifest_non_empty() {
        let paths = shard_paths();
        assert!(
            !paths.is_empty(),
            "shard_paths() returned empty — phone::list_dir_recursive did \
             not find shards/. Expected 300+ entries (per Taut 2026-07-23 \
             shard census); got 0. Verify cwd is inside the substrate tree \
             or a subdir of it (phone::find_substrate_root walks upward).",
        );
    }

    #[test]
    fn manifest_contains_known_shards() {
        let paths = shard_paths();
        // Spot-check three shards from three different family-roots
        // to catch phantom-successful-walk (empty subtree yielding
        // empty vec) that manifest_non_empty alone would not detect.
        for expected in [
            "shards/reality/subject.mirror",
            "shards/mirror/spec/system.mirror",
            "shards/magic/trick.mirror",
        ] {
            assert!(
                paths.iter().any(|p| p == expected),
                "expected shard `{}` in manifest; got {} entries. First 5: {:?}",
                expected,
                paths.len(),
                &paths.iter().take(5).collect::<Vec<_>>(),
            );
        }
    }

    #[test]
    fn manifest_is_sorted() {
        let paths = shard_paths();
        let mut sorted = paths.clone();
        sorted.sort();
        assert_eq!(
            paths, sorted,
            "shard_paths() is not sorted; deterministic manifest broken.",
        );
    }
}
