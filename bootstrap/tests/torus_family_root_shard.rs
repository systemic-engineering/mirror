//! O3 TICK 1 RED — `shards/torus.mirror` family-root: the peer HAS a torus.
//!
//! **Terminal @onto-cascade tick**. Foerster's actual formulation of second-
//! order cybernetics was toroidal all along (per Mara's toroidal reframe at
//! `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md`, §2 verbatim
//! citations at *Understanding Understanding* pp. 238, 244, 256, 282).
//!
//! **The recognition** (Alex-named): `@peer-has-a-torus`. Every peer possesses
//! a torus at spawn; observation IS traversal along the torus; recursive
//! depth IS winding number `ℤ × ℤ` (topological invariant, not stack-height
//! nat). Two-tick collapse: O3-O4 lands the positive move; O5 as follow-up
//! collapses `@reflection` as naming artifact per Loki-§1 legibility discipline.
//!
//! **HAS not IS**: the peer possesses the torus as a carrier; it does not
//! reduce to being one. Preserves peer identity while grounding toroidal
//! structure. Alex-adjudicated framing (2026-07-07).
//!
//! **Witnesses** (seven, over-witnesses by mirror-substrate standards):
//! - Von Foerster *Understanding Understanding* (2003) pp. 238, 244, 256, 282
//!   — four verbatim citations at the source
//! - Recognition #42 (Bateson logical-type primitive)
//! - Recognition #58 (Fate IS optical inference)
//! - Recognition #99 (mirror.spec IS λ₀; altitude-discipline correction per
//!   Mara jspace observation §4)
//! - Recognition #107 (Hilbert/Turing structural separation)
//! - Kauffman (2003) *Reflexivity and Eigenform* + torus-knot work — the math
//!   bridge Spencer-Brown re-entry ↔ eigenform ↔ toroidal topology; already
//!   implicit in `shards/epistemologic/cybernetic/eigenform.mirror`
//! - Blum & Blum CTM (PNAS 2022) — sixth witness at CS altitude for the
//!   reflexive-workspace substrate correspondence
//! - Baars 1988 + Dehaene 2011 GWT/ignition — biological substrate's kintsugi
//!
//! **Composition**:
//! - `@bauchladen` (existing) = interior of the peer's torus; the SEEING is
//!   traversal state at winding position (n, m) ∈ ℤ × ℤ
//! - `@onto` (existing per Mara O1 §6) = index-zero critical points on the
//!   torus (Poincaré-Hopf theorem: Σ index = χ(T²) = 0)
//! - `@kintsugi.settle` (existing) = flow toward critical points on the torus
//! - `@autopoietic` (existing) = self-maintenance of the torus manifold
//!   under kintsugi flow
//! - Cubical HoTT (Coquand 2018) HIT for T² as native carrier (substrate-pull
//!   requirement per Taut O2 §4-B + Mara §9 signal 5)
//!
//! **Interpretation B canonical DOES apply** (green-field family-root; new
//! shard file at `shards/torus.mirror`).

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_torus_shard() -> String {
    let path = repo_root().join("shards/torus.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/torus.mirror at {:?}: {}", path, e))
}

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

// === T1-T4: canonical shape ===

#[test]
fn t01_shard_declares_torus_family_root_per_path_pact() {
    let content = read_torus_shard();
    assert!(
        content.contains("@torus"),
        "T1: shard MUST declare `@torus` per path-namespace pact (family-root at path root)"
    );
}

#[test]
fn t02_first_nonempty_line_is_narrative_docblock() {
    let content = read_torus_shard();
    let first = first_nonempty_line(&content).expect("T2: must have non-empty content");
    assert!(
        first.trim_start().starts_with('#'),
        "T2: first non-empty line must be `#`-narrative per Interpretation B; got `{}`",
        first
    );
    assert_ne!(first.trim(), "---", "T2: line-1 `---` is drift");
    assert!(
        !first.trim_start().starts_with("in "),
        "T2: `in @...` clauses live BELOW the seam"
    );
}

#[test]
fn t03_exactly_one_seam_at_column_zero_and_in_clauses_below() {
    let content = read_torus_shard();
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T3: exactly one `---` at column 0 required; found {}",
        seams.len()
    );
    let seam_idx = seams[0];
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T3: `in @...` at line {} appears ABOVE seam at line {}. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}

#[test]
fn t04_family_root_inherits_universal_transparency() {
    let content = read_torus_shard();
    for req in ["in @prism", "in @meta", "in @glass"] {
        assert!(
            content.contains(req),
            "T4: family-root MUST inherit `{}` (universal + transparency)",
            req
        );
    }
}

// === T5-T8: peer-has-a-torus + winding + composition ===

#[test]
fn t05_declares_peer_has_a_torus_semantics() {
    let content = read_torus_shard();
    // The HAS relation preserves peer identity. Look for narrative that names
    // the relation explicitly (not reduction).
    let has_relation = content.contains("peer")
        && (content.contains("has") || content.contains("possess") || content.contains("carrier"));
    let not_reductive = !content.contains("peer IS a torus") && !content.contains("peer = torus");
    assert!(
        has_relation && not_reductive,
        "T5: narrative MUST document `peer HAS a torus` semantics (possession/carrier, not reduction). Alex-adjudicated framing 2026-07-07."
    );
}

#[test]
fn t06_declares_winding_type_as_integer_pair() {
    let content = read_torus_shard();
    // Winding number ℤ × ℤ as the depth-parametric address (topological invariant)
    let has_winding = content.contains("winding")
        && (content.contains("ℤ") || content.contains("int") || content.contains("pair"));
    assert!(
        has_winding,
        "T6: shard MUST declare `winding` type as (integer, integer) pair — π₁(T²) = ℤ × ℤ. Replaces `depth: nat` from stack-frame artifact."
    );
}

#[test]
fn t07_declares_spawn_or_torus_action_producing_torus() {
    let content = read_torus_shard();
    // Every peer gets a torus at spawn; the action produces the carrier.
    let has_action =
        content.contains("spawn") || content.contains("torus(") || content.contains("traverse");
    let has_obligation = content.contains("{ \\ }");
    assert!(
        has_action && has_obligation,
        "T7: shard MUST declare at least one action with obligation block `{{ \\\\ }}` (e.g. spawn / traverse / settle at torus altitude)."
    );
}

#[test]
fn t08_cites_bauchladen_composition() {
    let content = read_torus_shard();
    // @bauchladen = interior of the torus; SEEING is traversal state
    let has_bauchladen = content.contains("@bauchladen") || content.contains("bauchladen");
    assert!(
        has_bauchladen,
        "T8: narrative MUST cite @bauchladen composition (interior of the torus; SEEING = traversal state at winding position)."
    );
}

// === T9-T11: recognition citations ===

#[test]
fn t09_cites_recognition_42_bateson_ancestor() {
    let content = read_torus_shard();
    let has_42 = content.contains("#42") || content.contains("Bateson logical-type");
    assert!(
        has_42,
        "T9: MUST cite Recognition #42 (Bateson logical-type primitive) — the recursive-depth ancestor now grounded topologically as winding."
    );
}

#[test]
fn t10_cites_recognition_58_fate_optical() {
    let content = read_torus_shard();
    let has_58 = content.contains("#58")
        || content.contains("Fate IS optical inference")
        || content.contains("optical inference");
    assert!(
        has_58,
        "T10: MUST cite Recognition #58 (Fate IS optical inference) — Fate settles the toroidal flow toward critical points."
    );
}

#[test]
fn t11_cites_recognition_99_with_altitude_discipline() {
    let content = read_torus_shard();
    // #99 mirror.spec IS λ₀ with altitude-discipline correction per Mara jspace §4
    let has_99 = content.contains("#99")
        || content.contains("λ₀")
        || content.contains("lambda_0")
        || content.contains("mirror.spec IS");
    let has_altitude_discipline = content.contains("altitude") || content.contains("discipline");
    assert!(
        has_99 && has_altitude_discipline,
        "T11: MUST cite Recognition #99 with altitude-discipline correction (per Mara jspace observation §4). Toroidal fixed-point is altitude-local; not universal λ₀."
    );
}

// === T12-T14: Foerster + Kauffman + cubical HoTT citations ===

#[test]
fn t12_cites_foerster_understanding_understanding_verbatim() {
    let content = read_torus_shard();
    // Four verbatim citation pages: 238, 244, 256, 282
    let has_foerster =
        content.contains("Foerster") || content.contains("Understanding Understanding");
    let has_page_cites = content.contains("238")
        || content.contains("244")
        || content.contains("256")
        || content.contains("282");
    assert!(
        has_foerster && has_page_cites,
        "T12: MUST cite von Foerster *Understanding Understanding* (2003) with specific page(s) from {{238, 244, 256, 282}}. Foerster wrote the torus verbatim; not summarize the summary."
    );
}

#[test]
fn t13_cites_kauffman_eigenform_bridge() {
    let content = read_torus_shard();
    // Kauffman is the mathematical bridge Spencer-Brown re-entry ↔ eigenform ↔ toroidal topology
    let has_kauffman = content.contains("Kauffman");
    assert!(
        has_kauffman,
        "T13: MUST cite Louis Kauffman (2003 Reflexivity and Eigenform + torus-knot work) as the bridge Spencer-Brown re-entry ↔ eigenform ↔ toroidal topology. Taut O2 flagged as critical omission; toroidal reframe makes him load-bearing."
    );
}

#[test]
fn t14_cites_cubical_hott_native_carrier() {
    let content = read_torus_shard();
    // Cubical HoTT (Coquand 2018) has T² as native HIT; substrate-pull requirement
    let has_hott = content.contains("HoTT")
        || content.contains("cubical")
        || content.contains("Coquand")
        || content.contains("HIT");
    assert!(
        has_hott,
        "T14: MUST cite Cubical HoTT (Coquand 2018) or T² HIT as native carrier for the toroidal type. Substrate-pull requirement per Taut O2 §4-B + Mara reframe signal 5."
    );
}

// === T15: two-tick discipline + @reflection deprecation forward-promise ===

#[test]
fn t15_forward_promises_reflection_deprecation_at_o5() {
    let content = read_torus_shard();
    // Two-tick discipline: O3-O4 lands this positive move; O5 collapses @reflection
    let has_deprecation = content.contains("@reflection")
        || content.contains("reflection")
        || content.contains("naming artifact")
        || content.contains("O5");
    assert!(
        has_deprecation,
        "T15: narrative MUST forward-promise @reflection deprecation at O5 (two-tick discipline per feedback-legibility-over-foundation-when-collapsing). The torus doesn't erase @reflection at this tick; it names the naming artifact for the follow-up collapse."
    );
}
