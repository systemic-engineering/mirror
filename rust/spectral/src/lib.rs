//! `spectral` crate — math substrate at rust/ altitude.
//!
//! Migrated 2026-07-28 from `rust/src/{liquid.rs, spectral.rs}` per
//! Alex 2026-07-25 four-crate decomposition + Mara `9bb1f57` naming
//! discipline. Second cell in the peer-foam mycelial-autopoietic
//! geometry to materialize as its own crate boundary.
//!
//! ## What lives here
//!
//! - `shard_paths()` — runtime enumeration of `shards/**/*.mirror`
//!   (H-fibre count over the substrate manifold).
//! - `liquid` — property/pillar surface for spectral-triple discharge.
//!
//! ## Ancestry
//!
//! - Foerster 1974 (Understanding Understanding) — eigen-behaviors on
//!   the observer torus; the compilation primitive at nervous-system
//!   substrate.
//! - Fiedler 1973 — second-smallest Laplacian eigenvalue as spectral
//!   graph conductance; the H-fibre coordinate at pillar altitude.
//! - Connes 1994 (Noncommutative Geometry) — the (A, H, D) spectral
//!   triple; realized as prismqueer::bundle::{Fiber, Connection,
//!   Transport, Bundle} + prismqueer::apply_h operator action.
//! - Kauffman 2003/2005 (Eigenforms) — the recursive fixed-point
//!   ontology; foundational for `SpectralCoordinate<5>`.
//! - Grothendieck 1957 (Tôhoku) — sheaf theory; the section/fiber/
//!   stalk vocabulary the shard manifest realizes.
//! - Spärck Jones, K. (1972). *A Statistical Interpretation of Term
//!   Specificity and Its Application in Retrieval.* Journal of
//!   Documentation 28(1): 11-21. IDF is the ancestor of every
//!   vector-space embedding this crate composes over via prismqueer's
//!   spectral machinery. Cited at introduction site per Mara `9bb1f57`
//!   §3.2 convention (Void-Revenge anti-theft discipline).

use std::path::{Path, PathBuf};

pub mod liquid;

/// Walk upward from `start` looking for a directory containing
/// `shards/`. Returns the substrate-repo root if found, else `start`.
///
/// std::fs direct at spectral-crate altitude per Mara `9bb1f57`
/// (spectral cannot depend on rust/ bin crate; walking is a bounded
/// finite operation Rice-safe by construction).
fn find_substrate_root(start: &Path) -> PathBuf {
    let mut cur = start.to_path_buf();
    loop {
        if cur.join("shards").is_dir() {
            return cur;
        }
        if !cur.pop() {
            return start.to_path_buf();
        }
    }
}

/// Recursively collect `.mirror` file paths under `dir`. Skips `.git/`
/// + `target/` + symlinks per phone.rs walker discipline (preserved
/// verbatim at concrete-floor altitude; the walker discipline IS
/// substrate-honest at any crate).
fn walk_mirror_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(t) => t,
            Err(_) => continue,
        };
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name == ".git" || name == "target" {
                    continue;
                }
            }
            walk_mirror_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("mirror") {
            out.push(path);
        }
    }
}

/// Runtime manifest of every `shards/**/*.mirror` file present in the
/// workspace when called. Paths are `shards/<relative>` strings,
/// sorted lexicographically for byte-stable output.
///
/// Walks upward from CWD to find substrate root (looking for the
/// `shards/` directory), then recursively enumerates `.mirror` files.
/// All @io via std::fs direct — no cross-crate dep on rust/ bin.
pub fn shard_paths() -> Vec<String> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let root = find_substrate_root(&cwd);
    let shards_root = root.join("shards");

    let mut raw: Vec<PathBuf> = Vec::new();
    walk_mirror_files(&shards_root, &mut raw);

    let mut paths: Vec<String> = raw
        .into_iter()
        .filter_map(|p| {
            p.strip_prefix(&root)
                .ok()
                .map(|rel| rel.to_string_lossy().replace('\\', "/"))
        })
        .collect();
    paths.sort();
    paths
}

#[cfg(test)]
mod tests {
    //! Migration 4a pipeline-proof: crate-external runtime walk yields
    //! the same manifest shape as the previous mirror-bin-internal one.

    use super::shard_paths;

    #[test]
    fn manifest_non_empty() {
        let paths = shard_paths();
        assert!(
            !paths.is_empty(),
            "shard_paths() returned empty — std::fs walk did not find \
             shards/. Expected 300+ entries; got 0. Verify cwd is inside \
             the substrate tree or a subdir of it (find_substrate_root \
             walks upward).",
        );
    }

    #[test]
    fn manifest_contains_known_shards() {
        let paths = shard_paths();
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
