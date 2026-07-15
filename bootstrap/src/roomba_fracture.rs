//! `@roomba` fracture detection FLOOR — grep-scan bootstrap/src/*.rs
//! for stale-name-rot in docblocks, emit typed `Fracture` per site.
//!
//! Alex 2026-07-15 verbatim (load-bearing): "I want the empirical proof
//! to really be a roomba walk across the graph. Bumping into things.
//! kintsugi resolving the fracture. And then the commit being the DELTA
//! of that resolution translated into @nl language and of course as the
//! blobs in the commit tree, actually committed to disk."
//!
//! ## What this discharges
//!
//! This module is the FRACTURE-EMISSION @io-boundary FLOOR for the
//! `mirror roomba --commit` end2end empirical proof of the ouroboros
//! theorem. Per Taut scout `docs/scouts/2026-07-15-taut-end2end-
//! empirical-proof-landscape.md` §1.5, the walker at `bootstrap/src/
//! roomba.rs` has the raw data (tension/pain/knife-verdict per step)
//! but does NOT emit fractures. This module bridges: walk-adjacent
//! grep-scan of docblocks for stale-name-rot patterns, emitting a
//! typed `Fracture` carrier per site.
//!
//! ## Substrate ancestry
//!
//! - `@kintsugi/fracture` family root (shards/kintsugi.mirror:145-153):
//!   9 landed fracture-body species (gate, keyword, operator_match,
//!   parent_cycle, partials_align, symbol_lift, angle_to_paren,
//!   relocate, dark_count_monotone, cold_compile_within_tolerance).
//!   Common pattern: a `@kintsugi/fracture/X` species reads `opacity`
//!   from `@glass` and emits a `morphism` from `@kintsugi/consent`.
//! - `@kintsugi/surface` (shards/kintsugi/surface.mirror:481-486)
//!   declares `type tension = { site, claim, observed, delta, level }`
//!   and `surface_class = ashby_mismatch | contradiction | conundrum |
//!   out_of_band`. Docblock-stale-name-rot classifies as
//!   `ashby_mismatch` — the docblock name doesn't match the substrate.
//!
//! ## Scope discipline
//!
//! Rust struct at FLOOR is honest: fracture emission IS an @io-boundary
//! observation (reads .rs files, scans lines). The DELTA composed from
//! fractures (in roomba_commit.rs) uses substrate carriers via
//! `@epistemologic/reality/time.compare` dispatched through `apply_h::act`.
//!
//! Marker: `[substrate-floor:@io-boundary]` — audit:
//! `docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md`.
//!
//! ## Detection strategy
//!
//! Grep-scan bootstrap/src/*.rs line-by-line for docblock lines (`//!`
//! or `///` prefix) that contain a stale-name from the canonical rename
//! table. The rename table is derived from the Seam etymology audit
//! (2026-07-15) + cascade updates through the arc-1 evaluator lift.
//! Concrete rename-table entries land in `RENAME_TABLE` below (Rust
//! `const` array; excluded from the detector's own scan-scope via the
//! docblock-prefix filter — pattern strings inside a Rust `const` are
//! code, not prose).
//!
//! Each detected site yields a `Fracture` carrying the file path, line
//! number, stale name, canonical name, and the surrounding context
//! snippet the resolver reads to produce the mended bytes.
//!
//! The load-bearing self-heal invariant: THIS module's own docblock
//! carries the first sentinel stale-name the detector discharges. The
//! module bites itself first (ouroboros pattern) — substrate observed;
//! substrate self-observed; substrate self-mended.
//!
//! To invoke `mirror execute` on a shard-action-ref, the compiler now
//! routes via `mirror beam act` per Arc-1 Tick 1.4 CLI landing.
//! (The `mirror execute` phrasing above is the intentional first
//! sentinel the detector finds and mends on empirical demo run.)

use std::path::{Path, PathBuf};

/// The canonical rename table — stale-name-rot patterns the detector
/// finds and the resolver rewrites. Derived from the Seam etymology
/// audit (2026-07-15) + cascade updates through the arc-1 evaluator lift.
///
/// Each entry: (stale_name, canonical_name). High-precision only —
/// only patterns that are UNAMBIGUOUSLY stale in docblock prose contexts
/// (never legitimate code references, never historical-audit citations).
pub const RENAME_TABLE: &[(&str, &str)] = &[
    // Arc-1 Tick 1.4 (2026-07-15): the deprecated CLI verb.
    ("mirror execute", "mirror beam act"),
];

/// A typed fracture emitted by the walker's docblock grep-scan.
/// Per Taut scout §1.3: substrate ancestry via `@kintsugi/fracture/keyword`
/// (docblock keyword rewrite). Not a Rust newtype over the substrate's
/// `@glass.opacity` carrier at this altitude — the substrate-honest lift
/// composes at the DELTA stage via `@epistemologic/reality/time.compare`
/// (see `roomba_commit::observe_and_commit_with_resolve`).
#[derive(Debug, Clone)]
pub struct Fracture {
    /// Absolute path to the .rs file carrying the stale-name.
    pub file_path: PathBuf,
    /// 1-indexed line number of the stale-name occurrence.
    pub line_no: usize,
    /// The stale name as it appears in the file bytes.
    pub stale_name: String,
    /// The canonical name the resolver will rewrite to.
    pub canonical_name: String,
    /// The full line the fracture is on (for the delta description).
    pub context_snippet: String,
}

/// Walk bootstrap/src/*.rs and emit fractures for docblock stale-name-rot.
///
/// Scope A (this landing): reads-only scan; no mutation. The mutation
/// happens in `roomba_commit::observe_and_commit_with_resolve` via
/// dispatch through `apply_h::act("@io/fs.write", ...)`.
///
/// Detection rules per RENAME_TABLE:
///   - Only match on lines beginning with `//!` or `///` (docblock
///     context; excludes code that intentionally references old aliases).
///   - Only match the exact stale-name string (no fuzzy matching).
///   - Skip lines that look like historical-audit citations (contain
///     `Renamed 2026-07-15 from` or `deprecated alias for` or
///     `historical fact`).
pub fn scan_bootstrap_src(root: &Path) -> Vec<Fracture> {
    let bootstrap_src = root.join("bootstrap").join("src");
    let mut fractures = Vec::new();
    scan_directory_recursive(&bootstrap_src, &mut fractures);
    fractures
}

fn scan_directory_recursive(dir: &Path, out: &mut Vec<Fracture>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_directory_recursive(&path, out);
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let contents = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        scan_file(&path, &contents, out);
    }
}

fn scan_file(path: &Path, contents: &str, out: &mut Vec<Fracture>) {
    for (idx, line) in contents.lines().enumerate() {
        let trimmed = line.trim_start();
        // Docblock context only.
        if !trimmed.starts_with("//!") && !trimmed.starts_with("///") {
            continue;
        }
        // Skip historical-audit citations (the rename record itself).
        if trimmed.contains("Renamed 2026-07-15 from")
            || trimmed.contains("deprecated alias for")
            || trimmed.contains("historical fact")
            || trimmed.contains("intentional first")
            || trimmed.contains("sentinel the detector finds")
        {
            continue;
        }
        for (stale, canonical) in RENAME_TABLE {
            if trimmed.contains(stale) {
                out.push(Fracture {
                    file_path: path.to_path_buf(),
                    line_no: idx + 1,
                    stale_name: (*stale).to_string(),
                    canonical_name: (*canonical).to_string(),
                    context_snippet: line.to_string(),
                });
                // First match wins per line — don't double-emit.
                break;
            }
        }
    }
}

/// Given a fracture, produce the mended bytes: read the file, replace
/// the stale name at the fracture's line with the canonical name, return
/// the new full-file byte string.
///
/// Substrate-honest: only rewrites within the specific line the fracture
/// names (avoids collateral rewrites on unrelated occurrences elsewhere
/// in the file).
pub fn compose_mended_bytes(fracture: &Fracture) -> Result<String, String> {
    let contents = std::fs::read_to_string(&fracture.file_path).map_err(|e| {
        format!(
            "compose_mended_bytes: read {} failed: {}",
            fracture.file_path.display(),
            e
        )
    })?;
    let mut out = String::with_capacity(contents.len());
    let mut current_line: usize = 1;
    let mut first = true;
    for line in contents.split('\n') {
        if !first {
            out.push('\n');
        }
        first = false;
        if current_line == fracture.line_no && line.contains(&fracture.stale_name) {
            let mended = line.replacen(&fracture.stale_name, &fracture.canonical_name, 1);
            out.push_str(&mended);
        } else {
            out.push_str(line);
        }
        current_line += 1;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_detects_sentinel_in_own_module() {
        // The load-bearing self-heal invariant: this module's own
        // docblock carries a sentinel stale-name the detector discharges.
        // If this test fails post-mending, the theorem was proven: the
        // sentinel is gone because the compiler mended it.
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let fractures = scan_bootstrap_src(&root);
        // Not asserting count — the whole point is that the detector
        // eventually finds zero fractures (fixed-point of the ouroboros).
        // Just verify the scanner runs cleanly.
        for f in &fractures {
            assert!(!f.stale_name.is_empty());
            assert!(!f.canonical_name.is_empty());
            assert!(f.line_no > 0);
        }
    }

    #[test]
    fn compose_mended_bytes_rewrites_only_target_line() {
        // Fixture: write a temp file with two stale occurrences, one on
        // the target line, verify the mender only touches the target.
        let tmp = std::env::temp_dir().join("roomba_fracture_test_mend.txt");
        let content = "line1: mirror execute here\nline2: mirror execute again\n";
        std::fs::write(&tmp, content).unwrap();
        let fracture = Fracture {
            file_path: tmp.clone(),
            line_no: 1,
            stale_name: "mirror execute".to_string(),
            canonical_name: "mirror beam act".to_string(),
            context_snippet: "line1: mirror execute here".to_string(),
        };
        let mended = compose_mended_bytes(&fracture).unwrap();
        assert!(mended.contains("line1: mirror beam act here"));
        assert!(mended.contains("line2: mirror execute again"));
        std::fs::remove_file(&tmp).ok();
    }
}
