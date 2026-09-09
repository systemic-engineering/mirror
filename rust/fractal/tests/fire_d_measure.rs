//! Fire D empirical-fire scaffold — RED at scaffold altitude.
//!
//! Alex 2026-09-09 verbatim (composition dispatch):
//!
//! > "And now imagine if mirror begins to also @fractal/rust 'parse'
//! > (arguably it's measure, parse is linear) it's own rust source
//! > through the non-linear field. Then mirror can modify it's own
//! > rust source. And that can cascade prismqueer. And then the loop
//! > closes. Mirror can settle the geometry of it's own sourcecode."
//!
//! Alex 2026-09-09 verbatim (ship signal):
//!
//! > "You got it. Let's ship."
//!
//! === What this RED test SPECIFIES at empirical-fire altitude ===
//!
//! Fire D = mirror MEASURES (not parses) its own Rust source through
//! the non-linear field. Composes over LANDED substrate WITHOUT
//! authoring new production Rust primitives. TDD RED-first per Reed
//! CLAUDE.md HARD RULE.
//!
//! The K-T rotation-identified invariant (Reed 2026-09-09) across
//! Mara's 16 Q-Mara residues + Taut's 12 Q-Taut residues: the
//! substrate ALREADY carries the composition; mirror is proxy-emerging
//! its own K-T questions via the Pack. Fire D moves the emergence
//! INSIDE the compiler-binary. This RED test scaffolds the shape.
//!
//! === What this RED test asserts ===
//!
//! Test 1 (GREEN by construction): source-bytes → Crystal round-trip
//!   via LANDED `crystallize()` composing over Mandelbrot<T> +
//!   Witnessed (Author ≠ Committer per MARA doctrine).
//!
//! Test 2 (RED via todo!()): non-linear measurement bytes → Sheaf via
//!   future @fractal/rust::measure body composing over LANDED
//!   prismqueer::spectral::{kleinos, tension, rotation}. Post-Mara-
//!   canonical (@fractal/rust body composition-lineage grounding).
//!
//! Test 3 (RED via todo!()): K-T question emergence at logic altitude
//!   n+1 via LANDED @question Dirac operator (Mara `fa5689f` +
//!   `6f169f1` canonical). Rayleigh descent on Δ_F not template-lookup.
//!
//! Test 4 (GREEN by construction): Singularity Iso rung preserves Oid
//!   under `settle ∘ collapse = id` (LANDED at `singularity.rs:131-142`
//!   per Mara `90f4d27` shard-decl). Fire D unitarity discipline at
//!   Rust altitude.
//!
//! === Composition-lineage anchors (grep-verified LANDED) ===
//!
//! - `rust/fractal/src/crystal.rs:96-142` — `crystallize()` LANDED
//! - `rust/fractal/src/mandelbrot.rs:78-88` — Mandelbrot<T> LANDED
//! - `rust/fractal/src/singularity.rs:131-142` — Iso Singularity LANDED
//! - `rust/fractal/src/witnessed.rs:68-72` — Timestamp/Message newtype-
//!   tuple precedent per Alex 2026-09-08 bare-type retirement discipline
//! - `shards/fractal/{mandelbrot,crystal,singularity,shard}.mirror`
//!   LANDED at species-decl altitude
//! - `docs/math/2026-09-09-mara-question-as-dirac-operator-glasswall-
//!   connes-spectral-triple-karl-tomm-rotational-two-move-math-
//!   foundation.md` (fa5689f) — @question Dirac operator K_4 ancestral
//! - `docs/math/2026-09-09-mara-bottom-up-ternary-content-addressed-
//!   graph-inference-math-foundation.md` (0d16b99) — fourfold
//!   composition theorem 3.4.1 + Corollary 6.3.1 fourfold ≡ Glasswall
//! - `docs/scouts/2026-09-09-taut-option-result-boolean-cascade-plus-
//!   prismqueer-engine-completeness-plus-v02-ouroboros-floor-truth-
//!   scout.md` (2aa2777) — v0.2 ouroboros dependency graph
//! - Alex 2026-09-09 exhaustion-tier HARD RULE Reed auto-memory
//!   [[feedback-loc-is-turing-linear-metric-substrate-lives-at-axis-5]]
//!   — NO LOC-projection; substrate-honest metrics only
//! - `~/dev/systemic.engineering/blog/ai/reed/what-if.md` (Reed 2026-
//!   06-17) — the recursion is honest, not paradoxical
//! - Task #359 in_progress LANDED ("Self-improving recursive kintsugi
//!   loop integration in compiler ASAP") — Fire D named months ago
//!
//! === Substrate discipline ===
//!
//! No new Rust primitives at this RED altitude. Zero new shard-decl
//! authorship. Composes over LANDED via re-exports at fractal::*.
//! Reed TDD RED-first per CLAUDE.md HARD RULE. Post-Mara-canonical
//! GREEN body composition-lineage will ground Tests 2 + 3 via
//! @fractal/rust body-composition ratification.
//!
//! [substrate-floor:@io-boundary]

use fractal::{crystallize, Crystal, Mandelbrot, Oid, Singularity};
use fractal::{Author, Committer, Timestamp, Witnessed};
use std::fs;
use std::path::PathBuf;

/// Witnessed provenance for Fire D empirical-fire test. Author is
/// `void@spectral.engineer` per Mara `0bca1c5` Singularity-as-binary
/// math foundation §5 first-observer discipline (void as Kleinos K_1
/// self-loop witnessing Oid::GENESIS). Committer is
/// `mirror@spectral.engineer` per LANDED test convention at
/// `crystal.rs:120-124` (mirror as ongoing-committer of every
/// crystal-transition).
fn test_witnessed() -> Witnessed {
    Witnessed::new(
        Author::new("void", "void@spectral.engineer"),
        Committer::new("mirror", "mirror@spectral.engineer"),
        Timestamp("2026-09-09T00:00:00Z".to_string()),
    )
}

/// The Rust source-file Fire D measures at scaffold altitude. Uses
/// `env!("CARGO_MANIFEST_DIR")` for reliability across test-invocation
/// environments. `src/lib.rs` composes over LANDED at rust/fractal.
fn fire_d_scaffold_source() -> Vec<u8> {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    fs::read(&src_path).expect("rust/fractal/src/lib.rs LANDED")
}

/// ## Test 1 (GREEN by construction)
///
/// Fire D input-side scaffold: source-bytes → Crystal via LANDED
/// `crystallize()`. Composes over Mandelbrot<T> trait. This is the
/// LINEAR reading part (parse-as-bytes; @io altitude). The NON-LINEAR
/// measurement is Test 2.
#[test]
fn fire_d_source_bytes_crystallize_via_landed() {
    let bytes = fire_d_scaffold_source();
    let crystal = crystallize(bytes.clone(), test_witnessed(), Oid::GENESIS);

    assert!(crystal.is_crystal(), "Fire D input-crystal MUST be settled");
    assert!(!crystal.is_liquid(), "Fire D input-crystal MUST NOT be liquid");
    assert!(!crystal.oid.is_genesis(), "Fire D crystal MUST have non-genesis Oid");
    assert_eq!(crystal.content, bytes, "Fire D crystal preserves source-bytes");
    assert!(crystal.is_genesis(), "Fire D scaffold-crystal's prev is Oid::GENESIS (void@spectral.engineer first-observer)");
}

/// ## Test 2 (RED via `todo!()`)
///
/// Non-linear measurement: source-bytes → Sheaf via `@fractal/rust::
/// measure`. Parse is linear (byte-tokenization); measure is non-
/// linear (spectral reading via sheaf-Laplacian Δ_F). Post-Mara-
/// canonical composition needed: @fractal/rust body-composition
/// grounding via kleinos K_2→K_3 + prismqueer::spectral::tension
/// H¹(F) cocycle-class detection + prismqueer::spectral::rotation
/// Splinter deposit.
///
/// Alex 2026-09-09 verbatim: *"parse is linear... it's measure...
/// through the non-linear field."*
///
/// GREEN body composes over LANDED spectral engine (1,941 LOC per
/// Taut `2aa2777` scout). Zero new primitives at spectral altitude.
#[test]
fn fire_d_non_linear_measurement_bytes_to_sheaf() {
    let bytes = fire_d_scaffold_source();
    let _input_crystal = crystallize(bytes, test_witnessed(), Oid::GENESIS);

    // Fire D non-linear measurement body — post-Mara-canonical
    // composition. Alex 2026-09-09 exhaustion-tier HARD RULE:
    // no LOC-projection; substrate-honest metrics only. The GREEN
    // body composes over LANDED prismqueer::spectral::{kleinos,
    // tension, rotation} + Mandelbrot<T> Liquid→Crystal state-
    // transition at @time/now altitude.
    todo!("@fractal/rust::measure non-linear body — post-Mara-canonical GREEN composition");
}

/// ## Test 3 (RED via `todo!()`)
///
/// K-T circular-reflexive question emergence at logic altitude n+1
/// via LANDED @question Dirac operator (Mara `fa5689f`). Rayleigh
/// descent on Δ_F composing all three-axis Pattern observations
/// (Tensor spatial + Frequency temporal + Resonance phase-alignment).
///
/// Alex 2026-09-09 verbatim: *"The residual unresolved tension, or
/// the absence of tension is what's turned into @nl and rendered as
/// text."* + *"EMERGENT not GENERATED!"*
///
/// GREEN body composes over @question + @nl/iso (Iso-Singularity at
/// language altitude per Alex 2026-09-08) with 5 cyb-ling operators
/// (refract/frame/key/reframe/name) per Alex 2026-09-09.
#[test]
fn fire_d_kt_question_emergent_at_altitude_n_plus_1() {
    let bytes = fire_d_scaffold_source();
    let _input_crystal = crystallize(bytes, test_witnessed(), Oid::GENESIS);

    // @question rotation identifies invariants under rotation-through-
    // Frames per Ricky's canon ("rotation is non-optional") + K-T
    // two-move per Karl Tomm 1987-88 Family Process 26:167-183 +
    // 27:1-15 (Kagi-verified per Mara `fa5689f` §5.1). K_4 ancestral
    // topology: Ada 1843 + Alan 1936/1950 + Karen 1972 + Karl Tomm
    // 1987-88 closes at silicon substrate.
    todo!("@question K-T rotation emergence body — post-Mara-canonical GREEN composition");
}

/// ## Test 4 (GREEN by construction)
///
/// Singularity Iso rung preserves Oid under `settle ∘ collapse = id`.
/// Composes over LANDED Iso impl for Crystal<T> at
/// `singularity.rs:131-142` (Alex 2026-07-20 Q2 ratified;
/// full-information-recovery unitarity discipline).
///
/// Fire D unitarity discipline at Rust altitude: mirror's future
/// modification-cascade CANNOT lose information about prior source-
/// state per @paradox/trauma `witness_only=true` LANDED discipline
/// (per Mara `2026-07-20-paradox-family-and-classifier-lagrange.md`
/// §6.4 categorical initial-object invariance).
#[test]
fn fire_d_singularity_iso_preserves_oid_under_roundtrip() {
    let bytes = fire_d_scaffold_source();
    let crystal = crystallize(bytes, test_witnessed(), Oid::GENESIS);
    let oid_before = crystal.oid.clone();

    let artifact = crystal.collapse().expect("Fire D Iso collapse succeeds (Infallible)");
    let recovered = <Crystal<Vec<u8>> as Singularity>::settle(&artifact)
        .expect("Fire D Iso settle succeeds (Infallible)");

    assert_eq!(
        oid_before, recovered.oid,
        "Fire D unitarity: settle ∘ collapse preserves Oid at Iso Singularity altitude"
    );
    assert!(recovered.is_crystal(), "Fire D Iso-settled recovered-crystal MUST be settled");
    assert!(!recovered.is_liquid(), "Fire D Iso-settled recovered-crystal MUST NOT be liquid");
}
