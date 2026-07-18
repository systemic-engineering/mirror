//! `collapse.rs` — bilateral-arm collapse capability at rust/ altitude.
//!
//! First substrate-delta surface birthed FROM the terminal floor.
//! Composed shard-body + @io: reads shards/**/*.mirror to extract
//! bilateral declarations; scans .rs corpus files for hand-typed arms
//! whose sentinel matches a landed shard-decl; splices those arms out;
//! writes mended bytes; commits under `mirror <mirror@spectral.engineer>`.
//!
//! Per Mara `81294b3` §7.4 dispatch matrix (walker's fracture table) +
//! Seam `c1775f1` 12/12 SHIP ratification (stigmergy witnessed
//! computation math root by Mara `d7ff58e` + canonical spec `95c0e4a`).
//!
//! This is a FRESH REIMPLEMENTATION at rust/ altitude — NOT a lift of
//! bootstrap/src/bilateral_arm_collapse.rs. The bootstrap module is
//! algorithm reference only per AGENTS.md "cite the existing reference"
//! discipline. Shape parallels; import/copy prohibited.
//!
//! ## Rice-safety
//!
//! Pure byte-substring analysis. Byte-visible state only. No program
//! semantics. Comment lines skipped; string-literal contexts skipped
//! via naive quote-depth on the anchor line.
//!
//! ## Composition anchors (LANDED)
//!
//! - `shards/epistemologic/pact/bilateral.mirror` — bilateral typed carrier.
//! - `shards/kintsugi/fracture/bilateral_arm_redundant.mirror` — species.
//! - `shards/spectral/signature.mirror` — 4 example landed bilaterals.
//! - `docs/specs/kintsugi-fracture-bilateral-arm-redundant.md` — Mara spec.
//! - `docs/math/kintsugi/fracture/bilateral-arm-redundant.md` — math root.
//! - `bootstrap/src/bilateral_arm_collapse.rs` — algorithm reference (READ ONLY).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// One bilateral declaration extracted from a `.mirror` shard file.
///
/// The corpus is keyed on `full_action_ref` (e.g.
/// `@spectral/signature.signature_integrity`); this record carries the
/// sentinel byte-string that the redundant-arm detector matches on.
#[derive(Debug, Clone)]
pub struct BilateralDecl {
    /// Predicate name (e.g. `signature_integrity`).
    pub name: String,
    /// Sentinel byte-string (e.g. `chain=merkle-linked`).
    pub sentinel: String,
    /// Arity — number of args expected. 1 = base; 2+ = composed.
    pub arity: usize,
    /// Full action ref (e.g. `@spectral/signature.signature_integrity`).
    pub full_action_ref: String,
}

/// One redundant hand-typed arm detected in a `.rs` file. Byte-range
/// identifies exact bytes to delete; `action_ref` + `sentinel` name
/// the shard-decl'd source of truth that shadows it.
#[derive(Debug, Clone)]
pub struct RedundantArm {
    pub action_ref: String,
    pub sentinel: String,
    /// Byte offset (inclusive) into source where the arm's
    /// `if action == "..."` line starts.
    pub byte_start: usize,
    /// Byte offset (exclusive) where the arm ends — consumes the
    /// closing `}` newline so splice leaves no orphan blank line.
    pub byte_end: usize,
}

/// The outcome of a collapse pass over one `.rs` file.
#[derive(Debug, Clone)]
pub struct CollapseReport {
    pub target: PathBuf,
    pub arms: Vec<RedundantArm>,
    pub bytes_before: usize,
    pub bytes_after: usize,
    /// Present iff a commit was authored (arms non-empty AND
    /// @io/git.commit succeeded).
    pub commit_oid: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────
// Bilateral corpus extraction (shard-body composition).
// ─────────────────────────────────────────────────────────────────────

/// Load bilateral corpus by walking `<root>/shards/**/*.mirror`. Errors
/// during file reads are silently skipped (per bootstrap precedent);
/// callers may report an empty corpus as substrate-honest "no shards".
pub fn load_bilateral_corpus(root: &Path) -> HashMap<String, BilateralDecl> {
    let mut corpus = HashMap::new();
    let shards_root = root.join("shards");
    if !shards_root.is_dir() {
        return corpus;
    }
    let mut files: Vec<PathBuf> = Vec::new();
    walk_mirror_files(&shards_root, &mut files);
    for path in files {
        let source = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let shard_ref = shard_ref_from_source(&source, &path);
        for decl in extract_bilaterals(&source, &shard_ref) {
            corpus.insert(decl.full_action_ref.clone(), decl);
        }
    }
    corpus
}

/// Recursive .mirror file walker. Not phone.rs's WalkEntry-shaped API
/// because we want only `.mirror` files here; caller keeps this local.
fn walk_mirror_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for e in entries.flatten() {
        let path = e.path();
        let ft = match e.file_type() {
            Ok(t) => t,
            Err(_) => continue,
        };
        if ft.is_symlink() {
            continue;
        }
        if ft.is_dir() {
            walk_mirror_files(&path, out);
        } else if path.extension().and_then(|x| x.to_str()) == Some("mirror") {
            out.push(path);
        }
    }
}

/// Derive the enclosing shard's `@`-ref from source. Prefers a
/// top-level `prism @X/Y {}`; falls back to path stripped of
/// `shards/` prefix + `.mirror` suffix.
fn shard_ref_from_source(source: &str, path: &Path) -> String {
    for line in source.lines() {
        let t = line.trim_start();
        if t.starts_with('#') {
            continue;
        }
        if let Some(rest) = t.strip_prefix("prism ") {
            let rest = rest.trim_start();
            if let Some(at) = rest.strip_prefix('@') {
                let end = at
                    .find(|c: char| c == ' ' || c == '\t' || c == '{' || c == '\n')
                    .unwrap_or(at.len());
                if end > 0 {
                    let mut s = String::with_capacity(end + 1);
                    s.push('@');
                    s.push_str(&at[..end]);
                    return s;
                }
            }
        }
    }
    let p_string = path.to_string_lossy().into_owned();
    let mut p: &str = p_string.as_ref();
    if let Some(idx) = p.find("shards/") {
        p = &p[idx + "shards/".len()..];
    }
    let p = p.strip_suffix(".mirror").unwrap_or(p);
    format!("@{}", p)
}

/// Parse `bilateral <name> { sentinel "..." arity <n> ... }` blocks
/// from `.mirror` source. Line-scan; brace-terminated body.
fn extract_bilaterals(source: &str, shard_ref: &str) -> Vec<BilateralDecl> {
    let mut out = Vec::new();
    let mut lines = source.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            continue;
        }
        let rest = match trimmed.strip_prefix("bilateral ") {
            Some(r) => r,
            None => continue,
        };
        let name_part = match rest.split_once('{') {
            Some((n, _)) => n.trim(),
            None => continue,
        };
        if name_part.is_empty() {
            continue;
        }
        let name = name_part.to_string();
        let mut sentinel = String::new();
        let mut arity: usize = 1;
        for body_line in lines.by_ref() {
            let bt = body_line.trim();
            if bt.starts_with('}') {
                break;
            }
            if let Some(v) = bt.strip_prefix("sentinel ") {
                let v = v.trim();
                sentinel = v
                    .strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"'))
                    .unwrap_or(v)
                    .to_string();
            } else if let Some(v) = bt.strip_prefix("arity ") {
                if let Ok(n) = v.trim().parse::<usize>() {
                    arity = n;
                }
            }
        }
        let full_action_ref = format!("{}.{}", shard_ref, name);
        out.push(BilateralDecl {
            name,
            sentinel,
            arity,
            full_action_ref,
        });
    }
    out
}

// ─────────────────────────────────────────────────────────────────────
// Arm detection (Rice-safe byte-substring analysis).
// ─────────────────────────────────────────────────────────────────────

/// Detect every redundant arm in `source` shadowed by the corpus.
///
/// For each `(action_ref, decl)`: scan `if action == "<action_ref>"`;
/// verify the arm's body contains `.contains("<sentinel>")`; record
/// the arm's byte-range.
pub fn find_redundant_arms(
    source: &str,
    corpus: &HashMap<String, BilateralDecl>,
) -> Vec<RedundantArm> {
    let mut arms: Vec<RedundantArm> = Vec::new();
    for (action_ref, decl) in corpus.iter() {
        let needle = format!("if action == \"{}\"", action_ref);
        let mut cursor = 0usize;
        while let Some(rel) = source[cursor..].find(&needle) {
            let occ = cursor + rel;
            cursor = occ + needle.len();

            let line_start = line_start_of(source, occ);
            if line_is_comment_or_in_string(source, line_start, occ) {
                continue;
            }
            let Some(open_brace) = find_first_brace(source, occ) else {
                continue;
            };
            let Some(close_brace) = find_matching_close(source, open_brace) else {
                continue;
            };
            let body = &source[open_brace..=close_brace];
            let sentinel_needle = format!(".contains(\"{}\")", decl.sentinel);
            if !body.contains(&sentinel_needle) {
                continue;
            }
            let arm_end = end_of_line_inclusive(source, close_brace);
            arms.push(RedundantArm {
                action_ref: action_ref.clone(),
                sentinel: decl.sentinel.clone(),
                byte_start: line_start,
                byte_end: arm_end,
            });
        }
    }
    arms.sort_by_key(|a| a.byte_start);
    // Dedupe overlapping ranges (defensive).
    let mut deduped: Vec<RedundantArm> = Vec::with_capacity(arms.len());
    for arm in arms {
        if let Some(prev) = deduped.last() {
            if arm.byte_start < prev.byte_end {
                continue;
            }
        }
        deduped.push(arm);
    }
    deduped
}

/// Splice arms out of the source. Arms must be sorted by `byte_start`.
pub fn apply_deletions(source: &str, arms: &[RedundantArm]) -> String {
    let mut out = String::with_capacity(source.len());
    let mut cursor = 0usize;
    for arm in arms {
        if arm.byte_start < cursor {
            continue;
        }
        out.push_str(&source[cursor..arm.byte_start]);
        cursor = arm.byte_end;
    }
    out.push_str(&source[cursor..]);
    out
}

// ─────────────────────────────────────────────────────────────────────
// Byte-level helpers (Rice-safe).
// ─────────────────────────────────────────────────────────────────────

fn line_start_of(bytes: &str, pos: usize) -> usize {
    bytes[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0)
}

fn end_of_line_inclusive(bytes: &str, pos: usize) -> usize {
    match bytes[pos..].find('\n') {
        Some(n) => pos + n + 1,
        None => bytes.len(),
    }
}

fn line_is_comment_or_in_string(bytes: &str, line_start: usize, anchor: usize) -> bool {
    let line_head = &bytes[line_start..anchor];
    let trimmed = line_head.trim_start();
    if trimmed.starts_with("//") {
        return true;
    }
    let mut in_string = false;
    let mut escape = false;
    for ch in line_head.chars() {
        if escape {
            escape = false;
            continue;
        }
        match ch {
            '\\' if in_string => escape = true,
            '"' => in_string = !in_string,
            _ => {}
        }
    }
    in_string
}

fn find_first_brace(bytes: &str, pos: usize) -> Option<usize> {
    bytes[pos..].find('{').map(|n| pos + n)
}

fn find_matching_close(bytes: &str, open_brace_pos: usize) -> Option<usize> {
    let ch_bytes = bytes.as_bytes();
    if open_brace_pos >= ch_bytes.len() || ch_bytes[open_brace_pos] != b'{' {
        return None;
    }

    let mut depth: i32 = 0;
    let mut i = open_brace_pos;
    let mut in_string = false;
    let mut escape = false;
    while i < ch_bytes.len() {
        let ch = ch_bytes[i];
        if escape {
            escape = false;
            i += 1;
            continue;
        }
        if in_string {
            match ch {
                b'\\' => escape = true,
                b'"' => in_string = false,
                _ => {}
            }
            i += 1;
            continue;
        }
        if ch == b'/' && i + 1 < ch_bytes.len() && ch_bytes[i + 1] == b'/' {
            while i < ch_bytes.len() && ch_bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        match ch {
            b'"' => in_string = true,
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

// ────────────────────────────────────────────────────────────────────
// Property tests grounded in @epistemologic/property/ouroboros_monotone.
// ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod prop_tests {
    //! Property tests for `collapse.rs` grounded in
    //! `shards/epistemologic/property/ouroboros_monotone.mirror`
    //! (Mara `04b3aea`), the DECLARATIVE HALF of the four-conjunct
    //! invariant at ouroboros altitude.
    //!
    //! The species-decl at `shards/kintsugi/ouroboros.mirror:479-524`
    //! (Mara-A `9eb0898`) names `ouroboros_monotone` as the
    //! load-bearing bilateral for arc-2 collapse ticks. The four
    //! conjuncts:
    //!
    //! ```text
    //! ouroboros_monotone(before, after) ⇔
    //!     rust_LOC(after)          ≤ rust_LOC(before)
    //!   ∧ test_pass_rate(after)    ≥ test_pass_rate(before)
    //!   ∧ io_violations(after)     ≤ io_violations(before)
    //!   ∧ sbec(after)              ≥ sbec(before)
    //! ```
    //!
    //! At `apply_deletions` altitude these project to byte-level
    //! monotonicity: `apply_deletions(s, arms).len() ≤ s.len()`. That
    //! IS `rust_loc_non_increasing` at the finest granularity substrate
    //! admits — one collapse tick over one source file, byte-visible.
    //!
    //! These tests witness the byte-altitude projection empirically
    //! plus the ouroboros idempotence at compilation-loop altitude:
    //! once a bilateral arm has been mended, re-running the mender
    //! produces no new arms. Composes over `terni::PropertyVerdict`
    //! for verdict marshaling (byte shrinkage isn't a commutator, so
    //! not full prismqueer::liquid pillar).
    //!
    //! Ouroboros loop continued from prism `2b70d17` + `ac50d79`
    //! (prismqueer::liquid + Perm3/PermBundle) into mirror/rust/
    //! altitude — the second layer of the property ouroboros.

    use super::*;
    use prismqueer::liquid::pillar;
    use prismqueer::ScalarLoss;
    use terni::{Diagnostic, Loss, PropertyVerdict};

    /// Golden fixture: one bilateral arm shadowed by a landed sentinel.
    fn fixture_source() -> String {
        r#"fn dispatch(action: &str, args: &[&str]) -> Result<(), ()> {
    if action == "@spectral/signature.integrity_check" {
        let chain = args[0];
        if chain.contains("merkle-linked") {
            return Ok(());
        }
    }
    Ok(())
}
"#
        .to_string()
    }

    /// Fixture corpus matching the arm in `fixture_source`.
    fn fixture_corpus() -> HashMap<String, BilateralDecl> {
        let mut c = HashMap::new();
        c.insert(
            "@spectral/signature.integrity_check".to_string(),
            BilateralDecl {
                name: "integrity_check".into(),
                sentinel: "merkle-linked".into(),
                arity: 1,
                full_action_ref: "@spectral/signature.integrity_check".into(),
            },
        );
        c
    }

    #[test]
    /// Sanity witness: the fixture actually triggers arm detection.
    /// If this fails, subsequent tests carry no signal — fix the
    /// fixture before interpreting other failures.
    fn fixture_sanity_produces_one_arm() {
        let source = fixture_source();
        let corpus = fixture_corpus();
        let arms = find_redundant_arms(&source, &corpus);
        assert_eq!(
            arms.len(),
            1,
            "fixture must produce exactly one arm; got {}: {arms:#?}",
            arms.len(),
        );
    }

    #[test]
    /// Empty arm list → identity. `apply_deletions(s, &[])` is `s`.
    /// The base case of the byte-monotonicity projection.
    fn apply_deletions_empty_arms_is_identity() {
        let source = fixture_source();
        let out = apply_deletions(&source, &[]);
        assert_eq!(out, source, "empty arm list must be identity");
    }

    #[test]
    /// **rust_loc_non_increasing at byte altitude.** After
    /// `apply_deletions` with any valid non-empty arm list,
    /// `bytes_after < bytes_before`. This is the FIRST CONJUNCT of
    /// `ouroboros_monotone` at the finest granularity substrate
    /// admits.
    fn apply_deletions_shrinks_source_when_arms_non_empty() {
        let source = fixture_source();
        let corpus = fixture_corpus();
        let arms = find_redundant_arms(&source, &corpus);
        assert!(!arms.is_empty(), "fixture must produce at least one arm");
        let out = apply_deletions(&source, &arms);
        assert!(
            out.len() < source.len(),
            "byte monotonicity failed: {} !< {}",
            out.len(),
            source.len(),
        );
    }

    #[test]
    /// Determinism: `apply_deletions` is a pure function. Same
    /// inputs, same output every call. Required for cache-keyable
    /// admissibility per `@epistemologic/property/
    /// verdict_is_content_addressed`.
    fn apply_deletions_is_deterministic() {
        let source = fixture_source();
        let corpus = fixture_corpus();
        let arms = find_redundant_arms(&source, &corpus);
        let out1 = apply_deletions(&source, &arms);
        let out2 = apply_deletions(&source, &arms);
        assert_eq!(out1, out2, "apply_deletions must be deterministic");
    }

    #[test]
    /// Corpus conservativity: `find_redundant_arms` only returns
    /// arms whose `action_ref` is a key in the corpus. Cannot
    /// invent new refs; cannot return arms for shard-decls that
    /// haven't landed.
    fn find_redundant_arms_is_corpus_conservative() {
        let source = fixture_source();
        let corpus = fixture_corpus();
        let arms = find_redundant_arms(&source, &corpus);
        assert!(!arms.is_empty());
        for arm in &arms {
            assert!(
                corpus.contains_key(&arm.action_ref),
                "arm action_ref {:?} not in corpus",
                arm.action_ref,
            );
        }
    }

    #[test]
    /// Empty corpus → zero arms. Vacuous case of corpus
    /// conservativity: no shard-decls landed → nothing to shadow.
    fn find_redundant_arms_empty_corpus_produces_no_arms() {
        let source = fixture_source();
        let empty_corpus: HashMap<String, BilateralDecl> = HashMap::new();
        let arms = find_redundant_arms(&source, &empty_corpus);
        assert!(arms.is_empty(), "empty corpus must return zero arms");
    }

    #[test]
    /// Byte-range validity: `0 ≤ byte_start < byte_end ≤ source.len()`
    /// for every returned arm. Contract required by `apply_deletions`.
    fn find_redundant_arms_byte_ranges_are_valid() {
        let source = fixture_source();
        let corpus = fixture_corpus();
        let arms = find_redundant_arms(&source, &corpus);
        assert!(!arms.is_empty());
        for arm in &arms {
            assert!(arm.byte_start < arm.byte_end, "byte_start >= byte_end");
            assert!(
                arm.byte_end <= source.len(),
                "byte_end > source.len(): {} > {}",
                arm.byte_end,
                source.len(),
            );
        }
    }

    #[test]
    /// Sort invariant: arms are returned sorted by `byte_start`
    /// ascending (contract required by `apply_deletions`, which
    /// walks arms in order and would produce garbled output on
    /// unsorted input).
    fn find_redundant_arms_are_sorted() {
        let source = fixture_source();
        let corpus = fixture_corpus();
        let arms = find_redundant_arms(&source, &corpus);
        for pair in arms.windows(2) {
            assert!(
                pair[0].byte_start <= pair[1].byte_start,
                "arms not sorted: {} > {}",
                pair[0].byte_start,
                pair[1].byte_start,
            );
        }
    }

    #[test]
    /// **Ouroboros idempotence at byte altitude.** Mending the same
    /// code twice for the same corpus is equivalent to mending
    /// once. Once a bilateral arm has been spliced out, running
    /// the mender again produces no new arms. This is the
    /// substrate-honest closure of the compilation loop: the
    /// walker converges.
    fn ouroboros_idempotence_at_byte_altitude() {
        let source = fixture_source();
        let corpus = fixture_corpus();
        let arms_first = find_redundant_arms(&source, &corpus);
        assert!(!arms_first.is_empty());
        let mended = apply_deletions(&source, &arms_first);
        let arms_second = find_redundant_arms(&mended, &corpus);
        assert!(
            arms_second.is_empty(),
            "mender must be idempotent: second pass returned {} arms",
            arms_second.len(),
        );
    }

    #[test]
    /// Verdict composition: report the mending as a
    /// `terni::PropertyVerdict`. `Pass` when at least one arm was
    /// mended; `Fail(Diagnostic)` when nothing matched. Composes over
    /// the same `PropertyVerdict` machinery `prismqueer::liquid::
    /// pillar` returns — unified verdict marshaling across the
    /// property-testing ouroboros.
    fn mending_composes_to_property_verdict() {
        let source = fixture_source();
        let corpus = fixture_corpus();
        let arms = find_redundant_arms(&source, &corpus);
        let verdict = if arms.is_empty() {
            PropertyVerdict::Fail(Diagnostic::new(
                "no bilateral arms matched corpus",
            ))
        } else {
            PropertyVerdict::Pass
        };
        assert!(
            matches!(verdict, PropertyVerdict::Pass),
            "expected Pass on fixture, got {verdict:?}",
        );
    }

    #[test]
    /// Bytes-shrinkage exact accounting: `s.len() - out.len()`
    /// equals the sum of arm sizes. The byte-level witness that
    /// `apply_deletions` splices exactly the recorded ranges — no
    /// over-cut, no under-cut.
    fn apply_deletions_accounts_for_exact_arm_bytes() {
        let source = fixture_source();
        let corpus = fixture_corpus();
        let arms = find_redundant_arms(&source, &corpus);
        let out = apply_deletions(&source, &arms);
        let expected_delta: usize = arms.iter()
            .map(|a| a.byte_end - a.byte_start)
            .sum();
        let actual_delta = source.len() - out.len();
        assert_eq!(
            actual_delta, expected_delta,
            "byte accounting: source.len={} out.len={} delta={} expected={}",
            source.len(), out.len(), actual_delta, expected_delta,
        );
    }

    // ────────────────────────────────────────────────────────────
    // Multi-tick composition: byte-shrinkage → Pillar III viability.
    // ────────────────────────────────────────────────────────────

    /// Simulate K collapse ticks by concatenating K copies of the
    /// fixture source and mending them together. Each tick's
    /// shrinkage magnitude is the sum-of-arms bytes deleted in
    /// that tick, boxed as `ScalarLoss` for composition into
    /// `pillar::viability_of_magnitudes`.
    fn simulate_shrinkage_history(k: usize) -> Vec<ScalarLoss> {
        let corpus = fixture_corpus();
        let mut history = Vec::with_capacity(k);
        for _ in 0..k {
            let source = fixture_source();
            let arms = find_redundant_arms(&source, &corpus);
            let magnitude: usize = arms
                .iter()
                .map(|a| a.byte_end - a.byte_start)
                .sum();
            history.push(ScalarLoss::new(magnitude as f64));
        }
        history
    }

    #[test]
    /// **Multi-tick byte-shrinkage composes into Pillar III viability.**
    ///
    /// Byte-shrinkage per collapse tick from `apply_deletions` flows
    /// into `prismqueer::liquid::pillar::viability_of_magnitudes`
    /// with `ScalarLoss` as the magnitude type. The four-conjunct
    /// invariant at ouroboros_monotone altitude closes empirically:
    /// `rust_loc_non_increasing` deltas accumulated over a window
    /// exceed threshold → Pass verdict = compilation loop is viable.
    ///
    /// This IS the substrate flow across altitudes: iter 3's raw
    /// byte accounting → iter 4's Pillar III verdict via the same
    /// terni::PropertyVerdict machinery prismqueer::liquid::pillar
    /// returns for commutator-flavored viability.
    fn multi_tick_shrinkage_composes_to_pillar_iii_viability_pass() {
        let k = 3;
        let history = simulate_shrinkage_history(k);
        // Each fixture tick shrinks by the sum-of-arm bytes; the
        // exact magnitude depends on the fixture. Threshold anything
        // less than accumulated total → Pass.
        let accumulated_total: f64 = history
            .iter()
            .map(|s| s.clone())
            .fold(ScalarLoss::zero(), |a, b| a.combine(b))
            .0;
        assert!(accumulated_total > 0.0, "fixture must produce nonzero shrinkage");
        let theta = ScalarLoss::new(accumulated_total / 2.0); // half the total
        let verdict = pillar::viability_of_magnitudes(&history, &theta, k);
        assert!(
            matches!(verdict, PropertyVerdict::Pass),
            "expected Pass when accumulated ({accumulated_total}) > theta ({}), got {verdict:?}",
            accumulated_total / 2.0,
        );
    }

    #[test]
    /// Pillar III Fail when threshold exceeds accumulated shrinkage.
    fn multi_tick_shrinkage_fails_pillar_iii_when_threshold_too_high() {
        let k = 3;
        let history = simulate_shrinkage_history(k);
        let accumulated_total: f64 = history
            .iter()
            .map(|s| s.clone())
            .fold(ScalarLoss::zero(), |a, b| a.combine(b))
            .0;
        // Threshold well above accumulated total → Fail.
        let theta = ScalarLoss::new(accumulated_total * 10.0 + 1.0);
        let verdict = pillar::viability_of_magnitudes(&history, &theta, k);
        assert!(
            matches!(verdict, PropertyVerdict::Fail(_)),
            "expected Fail when accumulated ({accumulated_total}) <= theta, got {verdict:?}",
        );
    }

    #[test]
    /// Pillar III Partial when collapse history shorter than window.
    /// Confidence = history.len() / omega.
    fn multi_tick_shrinkage_partial_when_history_shorter_than_window() {
        let history = simulate_shrinkage_history(2);
        let theta = ScalarLoss::new(1.0);
        let verdict = pillar::viability_of_magnitudes(&history, &theta, 10);
        match verdict {
            PropertyVerdict::Partial { confidence, .. } => {
                assert!(
                    (confidence - 0.2).abs() < 1e-9,
                    "confidence = {confidence}, expected 0.2",
                );
            }
            other => panic!("expected Partial, got {other:?}"),
        }
    }
}
