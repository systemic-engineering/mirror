//! Arc 5 TICK 2 RED — `@kintsugi/shift` substrate-decl species.
//!
//! Promotes the already-landed `shift` primitive from five-op algebra keyword
//! to substrate-decl species naming cross-altitude morphism as first-class
//! typed action.
//!
//! Prior art (Taut scout `a596b040214709917`, 54th+ instance of
//! `feedback-substrate-already-had-the-word`):
//! - Recognition #26 `shift(oid, T)` LANDED at `@mirror/spectral/portal`
//!   (Reed 2026-06-08, `docs/insights/2026-06-08-portal-eigenvalue-stream-gen-prism.md`)
//! - `shards/prism.mirror` L38-45 binds shift ≡ lift ≡ basis-transformation
//!   per `[[architecture-operations-as-linear-algebra]]` +
//!   `[[architecture-lift-as-load-bearing]]`
//! - `shards/mirror/mosaic.mirror` has production `shift(resolved, altitude) -> emitter`
//! - Recognition #58 (kintsugi surface) uses shift-language explicitly
//! - 13 #53 bilateral instances = `shift(predicate → operational-altitude)` under the hood
//!
//! Placement: `@kintsugi` process-side per #55 form/process partition
//! (recently promoted via containerd C3 second-witness at Arc 5 TICK 1).
//!
//! Post Arc 5 TICK 1 close (Mara `aaa9a81` @container family-root + Seam Phase D
//! `1787af4` RATIFY). TICK 2 opens shift-primitive promotion; TICK 3 will land
//! `@container/image = shift(@mirror/store/oci → @container altitude)` as first
//! empirical witness.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_shift_shard() -> String {
    let path = repo_root().join("shards/kintsugi/shift.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/kintsugi/shift.mirror at {:?}: {}", path, e))
}

// === T1-T7: canonical species shape ===

#[test]
fn t01_kintsugi_shift_declares_species_prism() {
    let content = read_shift_shard();
    assert!(
        content.contains("prism @kintsugi/shift"),
        "T1: must declare `prism @kintsugi/shift` species per path-namespace pact"
    );
}

#[test]
fn t02_kintsugi_shift_declares_five_op_body() {
    let content = read_shift_shard();
    for op in ["focus", "project", "split", "shift", "settle"] {
        assert!(
            content.contains(op),
            "T2: prism @kintsugi/shift must declare five-op body operation `{}`",
            op
        );
    }
}

#[test]
fn t03_shift_declares_source_altitude_carrier() {
    let content = read_shift_shard();
    assert!(
        content.contains("type source_altitude = ref") || content.contains("source_altitude: ref"),
        "T3: must declare `source_altitude` typed carrier (altitude of origin)"
    );
}

#[test]
fn t04_shift_declares_target_altitude_carrier() {
    let content = read_shift_shard();
    assert!(
        content.contains("type target_altitude = ref") || content.contains("target_altitude: ref"),
        "T4: must declare `target_altitude` typed carrier (altitude of destination)"
    );
}

#[test]
fn t05_shift_declares_shift_witness_or_oid_carrier() {
    let content = read_shift_shard();
    assert!(
        content.contains("shift_witness") || content.contains("witness: ref") || content.contains("oid_preserved"),
        "T5: must declare persistence-across-altitudes carrier (shift_witness / witness / oid_preserved) — the OID that persists (hardlink semantic)"
    );
}

#[test]
fn t06_shift_declares_typed_shift_action() {
    let content = read_shift_shard();
    // The typed action signature: shift(witness, source, target) -> ref (or similar shape)
    assert!(
        content.contains("shift(") && (content.contains("source_altitude") || content.contains("target_altitude")),
        "T6: must declare typed `shift(...)` action taking source_altitude + target_altitude carriers"
    );
}

#[test]
fn t07_shift_declares_composed_bilateral_preserves_content() {
    let content = read_shift_shard();
    assert!(
        content.contains("shift_preserves_content"),
        "T7: must declare composed-bilateral `shift_preserves_content(...) -> verdict` — the substrate-decl guarantee that shift preserves content-address (hardlink semantic)"
    );
}

// === T8-T13: Interpretation B + inheritance discipline ===

fn first_nonempty_line(content: &str) -> Option<&str> {
    content.lines().find(|l| !l.trim().is_empty())
}

fn seam_line_indices(content: &str) -> Vec<usize> {
    content
        .lines()
        .enumerate()
        .filter_map(|(i, l)| if l == "---" { Some(i) } else { None })
        .collect()
}

#[test]
fn t08_first_nonempty_line_is_narrative_docblock() {
    let content = read_shift_shard();
    let first = first_nonempty_line(&content).expect("T8: must have non-empty content");
    assert!(
        first.trim_start().starts_with('#'),
        "T8: first non-empty line must be `#`-narrative per Interpretation B; got `{}`",
        first
    );
    assert_ne!(first.trim(), "---", "T8: line-1 `---` is drift");
    assert!(
        !first.trim_start().starts_with("in "),
        "T8: `in @...` clauses live BELOW the seam"
    );
}

#[test]
fn t09_exactly_one_seam_at_column_zero() {
    let content = read_shift_shard();
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T9: exactly one `---` at column 0 required; found {}",
        seams.len()
    );
}

#[test]
fn t10_in_clauses_below_seam() {
    let content = read_shift_shard();
    let seams = seam_line_indices(&content);
    let seam_idx = *seams.first().expect("T10 depends on T9 (one seam present)");
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T10: `in @...` at line {} appears ABOVE seam at line {}. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}

#[test]
fn t11_shift_inherits_prism() {
    let content = read_shift_shard();
    assert!(
        content.contains("in @prism"),
        "T11: must inherit `in @prism` (universal)"
    );
}

#[test]
fn t12_shift_inherits_meta() {
    let content = read_shift_shard();
    assert!(
        content.contains("in @meta"),
        "T12: must inherit `in @meta`"
    );
}

#[test]
fn t13_shift_inherits_kintsugi() {
    let content = read_shift_shard();
    assert!(
        content.contains("in @kintsugi"),
        "T13: must inherit `in @kintsugi` (parent family per path-namespace pact)"
    );
}

// === T14-T16: Prior-art grounding (substrate-already-had-the-word discipline) ===

#[test]
fn t14_narrative_cites_prior_shift_landing_at_portal() {
    let content = read_shift_shard();
    assert!(
        content.contains("@mirror/spectral/portal") || content.contains("#26") || content.contains("portal"),
        "T14: narrative must cite prior `shift(oid, T)` landing at @mirror/spectral/portal (Recognition #26, Reed 2026-06-08) — substrate-already-had-the-word discipline"
    );
}

#[test]
fn t15_narrative_cites_shift_equals_lift_binding() {
    let content = read_shift_shard();
    assert!(
        content.contains("lift") && (content.contains("basis") || content.contains("prism.mirror") || content.contains("[[architecture-lift")),
        "T15: narrative must cite prism.mirror binding `shift ≡ lift ≡ basis-transformation` per [[architecture-lift-as-load-bearing]] + [[architecture-operations-as-linear-algebra]]"
    );
}

#[test]
fn t16_narrative_grounds_form_process_partition_placement() {
    let content = read_shift_shard();
    assert!(
        content.contains("#55") || content.contains("form/process") || content.contains("process-side"),
        "T16: narrative must ground @kintsugi (process-side per #55) placement — shift IS the process-side substrate-decl of cross-altitude morphism"
    );
}
