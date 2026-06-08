//! `@mirror/realisation` — the per-Rust-file discriminator (boundary Rust).
//!
//! This module is the boundary-Rust realisation of T21's substrate
//! declaration (`shards/mirror/realisation.mirror`). T21 named the
//! discriminator surface; T22 lands the body that runs it against the
//! bootstrap's own Rust source.
//!
//! ## Discipline
//!
//! Per the shard's prose, the discriminator answers — for each
//! `bootstrap/src/<file>.rs` — *does the substrate already declare
//! what this file does, at some `@mirror/*` altitude?*
//!
//! - YES → `altitude = Substrate`; `target = <the @mirror altitude>`.
//!   The file is a **candidate for kintsugi-on-Rust migration**: a
//!   later tick replaces the Rust body with a `.mirror` declaration
//!   that lifts the file's behaviour into the substrate.
//! - NO (but the file reaches into a non-mirror surface — syscall,
//!   FFI, subprocess) → `altitude = Boundary`; `target = <the @io/*
//!   species>`. The file **stays in Rust** at the @io altitude.
//! - UNCERTAIN → `altitude = Substrate` with a `Partial` opacity:
//!   the discriminator's miss IS a substrate gap (the substrate-
//!   honest reading per `[[feedback-substrate-already-had-the-word]]`).
//!
//! ## MVP body — the hardcoded match table
//!
//! Per Mara's T22 brief: the body is a hardcoded `match` on the
//! file's basename. Each row in the table IS one of the 26-30
//! substrate-pull recognitions logged in `roadmap/2026-06-08-rug-
//! pull.md` §5, made explicit at the boundary altitude. The table
//! is small, audited, and reviewable — every row carries a
//! `rationale` string naming the spec/insight/feedback note that
//! justifies the classification.
//!
//! ## Why not a real Rust AST parse?
//!
//! Per the shard's docstring: "for v0 a hardcoded match table on
//! filename → target_altitude is the smallest move". Real AST
//! parsing is a separate substantial tick (T23+). The match table
//! is the kintsugi-on-Rust pilot's proof-of-concept; once the
//! pipe works end-to-end (substrate decl → Rust impl → CLI dispatch
//! → emitted classification + fracture proposal), T23 grows the
//! body — first per-function refinement, then real `@code/rust`
//! AST walking against the target altitude's grammar.
//!
//! ## Path → Ref convention
//!
//! Following T19's `@kintsugi/<sanitized-basename>` precedent for
//! anchor refs, T22 uses `@realisation/<sanitized-basename>` for the
//! discriminator's `path` field. The realisation altitude reads
//! "this is `gap.rs` as classified at the realisation altitude" —
//! the prefix names the altitude that ran the classification, not
//! the file's filesystem location.
//!
//! ## Verdict construction
//!
//! Both `Substrate` and `Boundary` classifications use
//! `Transparency::Clear` (no opacities). The classification IS
//! clean — the file is unambiguously substrate-realisable or
//! unambiguously @io. The `Partial` arm is reserved for unknown
//! files (the substrate-pull surface for T23: each `Partial` is a
//! candidate row to add to the table once classified by hand).
//!
//! Substrate decisions: `[[architecture-shards-as-substrate-source]]`
//!                       (the body realises the substrate's typed decl,
//!                       not the other way around);
//!                       `[[architecture-fragmentation-is-the-rust-
//!                       substrate]]` (this module lives in bootstrap,
//!                       not prism_core — bootstrap-specific concern).
//! Substrate recognitions: `[[feedback-substrate-already-had-the-word]]`
//!                       (the 30th-instance recognition T21 logged;
//!                       each row in the MVP table is one prior
//!                       recognition made explicit).

use prism_core::{Diagnostic, PropertyVerdict, Ref, Transparency};

/// The boundary partition. Same two-state carrier as the substrate's
/// `type altitude = | boundary | substrate` declaration in
/// `shards/mirror/realisation.mirror`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Altitude {
    /// The file IS @io — it stays in Rust permanently.
    Boundary,
    /// The file is substrate-realisable at some `@mirror/*` altitude —
    /// it is a candidate for kintsugi-on-Rust migration.
    Substrate,
}

impl std::fmt::Display for Altitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Altitude::Boundary => write!(f, "Boundary"),
            Altitude::Substrate => write!(f, "Substrate"),
        }
    }
}

/// The discriminator's full output carrier. Mirrors T21's substrate
/// declaration:
///
/// ```text
/// type realisable_file = {
///   path: ref,
///   altitude: altitude,
///   target: ref,
///   verdict: transparency,
/// }
/// ```
///
/// - `path` — the `@realisation/<basename>` anchor naming the
///   classified file at the realisation altitude.
/// - `altitude` — the binary partition (`Boundary` vs `Substrate`).
/// - `target` — the `@mirror/*` altitude that subsumes a
///   substrate-realisable file, or the `@io/*` species a boundary
///   file targets.
/// - `verdict` — the classification's confidence. `Clear` when the
///   table entry is unambiguous; `Opaque(...)` with a located opacity
///   when the file is unknown to the MVP table (the substrate-pull
///   surface).
#[derive(Clone, Debug, PartialEq)]
pub struct RealisableFile {
    pub path: Ref,
    pub altitude: Altitude,
    pub target: Ref,
    pub verdict: Transparency<Ref>,
}

/// `classify(f) -> realisable_file` — the structural primitive (per
/// `shards/mirror/realisation.mirror`'s `classify` action).
///
/// MVP body: hardcoded `match` on the file's basename. The table IS
/// the 26-30 substrate-pull recognitions made explicit. Each row
/// carries an inline rationale (the spec/insight reference that
/// justifies the classification).
///
/// Unknown basenames return a `Substrate` altitude with a `Partial`
/// verdict — the discriminator's miss IS a substrate-realisable
/// surface (the substrate gap is "we don't yet know what altitude
/// declares this Rust file"). T23+ grows the table as each unknown
/// is hand-classified.
pub fn classify(path: &str) -> RealisableFile {
    let basename = path_basename(path);
    let anchor = anchor_ref(&basename);

    // Match against the basename string. Each arm encodes one row of
    // the MVP table. Substrate-realisable rows → `substrate(target,
    // rationale)`; boundary rows → `boundary(target, rationale)`;
    // unknown → `uncertain(path, rationale)`.
    //
    // The trailing path segment after `bootstrap/src/` is the match
    // discriminator — sub-modules under `bootstrap/src/<dir>/<file>.rs`
    // (currently only `music/mod.rs`) match on the full last-two-segment
    // path; everything else matches on the basename alone.
    let two_seg = two_segment_tail(path);

    match (two_seg.as_str(), basename.as_str()) {
        // ── substrate-realisable rows ────────────────────────────────
        //
        // Per the brief's MVP table. Each row is one of the 26-30
        // substrate-pull recognitions made explicit at the boundary.

        // `gap.rs` — gap algebra at the verdict altitude. The substrate
        // declares the same shape at `@mirror/spectral/oscillate`'s
        // gap-tension-tensor sub-grammar.
        (_, "gap.rs") => substrate(
            anchor,
            "@mirror/spectral/oscillate",
            "via gap-tension-tensor-substrate.md",
        ),

        // `oscillate.rs` — the kintsugi driver. Substrate declares the
        // ACTIVE/DARK alternation at `@mirror/spectral/oscillate`.
        (_, "oscillate.rs") => substrate(
            anchor,
            "@mirror/spectral/oscillate",
            "kintsugi driver — Banach contraction over morphism contexts",
        ),

        // `kintsugi.rs` — the SDRF fracture loop. Same target as
        // `oscillate.rs` (the kintsugi loop is the oscillate loop's
        // structural-side companion).
        (_, "kintsugi.rs") => substrate(
            anchor,
            "@mirror/spectral/oscillate",
            "SDRF kintsugi loop — fracture minimisation per descent",
        ),

        // `score.rs` — score / pending / metalogue session shape.
        // Substrate declares `@mirror/spectral/score` (T11.1).
        (_, "score.rs") => substrate(
            anchor,
            "@mirror/spectral/score",
            "score-of / pending / metalogue session — T11.1",
        ),

        // `tensor.rs` — sheaf-Laplacian tensor algebra at the
        // gap-tension altitude. Substrate declares the operator at
        // `@epistemologic/math/sheaf_laplacian`.
        (_, "tensor.rs") => substrate(
            anchor,
            "@epistemologic/math/sheaf_laplacian",
            "T8 + gap-tension-tensor §3 — Fiedler-eigenvector tension",
        ),

        // `curvature.rs` — balanced-Forman curvature on K_2 / K_3 / K_4
        // / barbell bridges. Substrate declares `@epistemologic/math/
        // curvature` (T9 SDRF).
        (_, "curvature.rs") => substrate(
            anchor,
            "@epistemologic/math/curvature",
            "T9 SDRF — balanced Forman curvature on graph edges",
        ),

        // `music/mod.rs` — audible altitude (cadences, dissonance,
        // verdict ↔ cadence_kind projection). Substrate declares
        // `@epistemologic/math/music` (audible-altitude cascade).
        ("music/mod.rs", _) => substrate(
            anchor,
            "@epistemologic/math/music",
            "audible-altitude cascade — cadence ↔ verdict morphism",
        ),

        // `ast.rs` — AST node / kind / structural primitives. Substrate
        // declares `@code/mirror` (the AST altitude is the mirror code's
        // own grammar).
        (_, "ast.rs") => substrate(
            anchor,
            "@code/mirror",
            "AST altitude — node kind / line-col structural primitives",
        ),

        // `tokenize.rs` — the bootstrap tokenizer. Substrate declares
        // `@code/mirror` (the grammar reads its own source).
        (_, "tokenize.rs") => substrate(
            anchor,
            "@code/mirror",
            "tokenizer — the grammar reads its own source",
        ),

        // `property.rs` — gap collection over an AST (gaps_of). Substrate
        // declares `@epistemologic/property/gap`.
        (_, "property.rs") => substrate(
            anchor,
            "@epistemologic/property",
            "via shards/epistemologic/property/* — gaps_of(ast)",
        ),

        // `grammar.rs` — grammar loading + keyword harvesting. The
        // substrate-pull is *uncertain*: bootstrap reads its own
        // grammar at startup (per `[[feedback-tokenizer-is-grammar-
        // bootstrapped]]`), so grammar.rs is the boundary between
        // "grammar source" and "grammar in memory". For T22 we
        // classify as substrate-realisable at `@code/mirror` with
        // the uncertainty annotated in the rationale; the body of
        // grammar.rs IS a mirror-altitude concern (per
        // [[feedback-no-new-rust]]).
        (_, "grammar.rs") => substrate(
            anchor,
            "@code/mirror",
            "parser — uncertain; bootstrap reads its own grammar",
        ),

        // `pipeline.rs` — pipeline rewrites + execution composition.
        // Substrate declares this at `@mirror/lens` (the
        // observation/projection family per shards/mirror/lens.mirror;
        // pipeline composition is lens composition).
        (_, "pipeline.rs") => substrate(
            anchor,
            "@mirror/lens",
            "pipeline = lens composition — observation/projection",
        ),

        // `spectral.rs` — spectral algebra (compose_a, fold5, quantize,
        // content-OID prism). Substrate declares this at
        // `@mirror/spectral` (the agent-coordination family root).
        (_, "spectral.rs") => substrate(
            anchor,
            "@mirror/spectral",
            "spectral algebra — compose_a / fold5 / content-OID prism",
        ),

        // ── boundary rows ────────────────────────────────────────────
        //
        // Permanent-@io files per the glass-wall / cross-wall-kintsugi
        // partition. Each reaches into a non-mirror surface that the
        // substrate cannot fold past.

        // `main.rs` — OS dispatch, argv parsing, process exit codes.
        // Permanent @io/cli — the CLI binary entry point. (`@io/cli`
        // is a future species the substrate has not yet declared at a
        // shard; T23+ lifts the species when a consumer pulls it. T22
        // names the target so the substrate-pull surface is logged.)
        (_, "main.rs") => boundary(anchor, "@io/cli", "OS dispatch — argv parsing + process exit"),

        // `sheaf_laplacian.rs` — LAPACK FFI (dsyev / dgesvd) over BLAS.
        // The substrate algebra lives at `@epistemologic/math/sheaf_
        // laplacian`; the FFI realisation stays in Rust permanently
        // at `@io/flang`.
        (_, "sheaf_laplacian.rs") => boundary(
            anchor,
            "@io/flang",
            "LAPACK FFI — dsyev / dgesvd over gfortran + Accelerate",
        ),

        // `portal.rs` — SCM_RIGHTS fd-passing at the CLI pipe boundary.
        // Permanent @io/socket — the kernel's typed-capability tag
        // surface (per T20's portal-as-eigenvalue-stream-gen_prism lift).
        (_, "portal.rs") => boundary(
            anchor,
            "@io/socket",
            "SCM_RIGHTS handoff — kernel-managed fd transfer",
        ),

        // `exec.rs` — subprocess execution (Command, exit-code → verdict).
        // Permanent @io/process — fork+exec surface per UNIX (1969).
        (_, "exec.rs") => boundary(
            anchor,
            "@io/process",
            "subprocess execution — fork+exec exit-code projection",
        ),

        // `git.rs` — git plumbing FFI (hash-object / cat-file /
        // update-ref). Permanent @io/git — content-addressed object
        // database primitives per Torvalds 2005.
        (_, "git.rs") => boundary(
            anchor,
            "@io/git",
            "git plumbing — hash-object / cat-file / update-ref",
        ),

        // `hash.rs` — SHA-256 canonical hash + CoincidenceHash detector.
        // The substrate algebra is `@io/crypto` (vendor crypto surface).
        // Permanent @io.
        (_, "hash.rs") => boundary(
            anchor,
            "@io/crypto",
            "SHA-256 canonical hash — vendor crypto surface",
        ),

        // `crystallize.rs` — content-addressed storage with BLAKE3
        // (Splinter, Text, Content carriers). The substrate declares
        // `@mirror/store` as the open content-addressed gate; the
        // Rust impl IS the @io/store realisation of that gate.
        // Boundary because the disk/git-backed storage is @io.
        (_, "crystallize.rs") => boundary(
            anchor,
            "@io/store",
            "content-addressed storage — @mirror/store realised at @io/store",
        ),

        // ── unknown — Partial verdict, substrate-pull surface ────────

        _ => uncertain(anchor, &basename, "no match in T22 MVP table"),
    }
}

/// `is_substrate_realisable(f) -> verdict` — the verdict projection.
///
/// Declared because the substrate shard
/// (`shards/mirror/realisation.mirror`) names this action; T22 lands it
/// alongside `classify` so the boundary surface matches the substrate
/// declaration even though main.rs's CLI dispatch only calls `classify`
/// this tick. Future consumers (mosaic.settle_on, audition altitude,
/// CI gates per the shard's docstring) pull this surface directly.
#[allow(dead_code)]
///
/// Returns `Imperfect`-shaped verdict for consumers that want only the
/// binary projection. Mirrors the shard's
/// `is_substrate_realisable(f: ref) -> verdict` action.
///
/// For T22 the projection is structural:
/// - `Substrate` altitude → `Success(())` (the file is migratable)
/// - `Boundary` altitude → `Success(())` too (the classification is
///   clean; the file is correctly @io). Per Mara's brief, boundary
///   classifications use Success — the discriminator's verdict is
///   about classification confidence, not about whether the file is
///   "good". A clean boundary classification IS a clean classification.
/// - `Opaque(_)` verdict → `Partial((), opacity)` (the discriminator
///   surfaces its uncertainty as a located opacity for T23 refinement).
pub fn is_substrate_realisable(path: &str) -> terni::Imperfect<(), &'static str, Transparency<Ref>> {
    let cls = classify(path);
    match cls.verdict {
        Transparency::Clear => terni::Imperfect::Success(()),
        opaque @ Transparency::Opaque(_) => terni::Imperfect::Partial((), opaque),
    }
}

/// `target_altitude(f) -> ref` — the targeting projection.
///
/// Declared for the same reason as `is_substrate_realisable` above —
/// the substrate shard names this action. T22's CLI dispatch reads
/// `classify(...).target` directly; T23+ consumers (the structured
/// fracture-diff tick) call `target_altitude` to drive the grammar
/// diff against the named altitude.
#[allow(dead_code)]
///
/// Returns the `@mirror/*` altitude that subsumes the file when
/// substrate-realisable; the `@io/*` species when boundary. T22
/// (this tick) consumes this directly via `classify(...).target` in
/// the CLI dispatch's `[realisation]` trace; T23+ consumes it to
/// drive the structured fracture diff against the target altitude's
/// grammar.
pub fn target_altitude(path: &str) -> Ref {
    classify(path).target
}

// ── helpers ─────────────────────────────────────────────────────────

/// Extract the basename of a path. Borrowed from `cmd_kintsugi_single`'s
/// basename-extraction precedent (T19). Returns the OS-string component
/// after the last separator; falls back to `"file"` on empty / non-UTF-8.
fn path_basename(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file")
        .to_string()
}

/// Extract the last two segments of a path for sub-module
/// disambiguation. `bootstrap/src/music/mod.rs` → `"music/mod.rs"`;
/// `bootstrap/src/gap.rs` → `"gap.rs"` (the second segment back is
/// `src`, which doesn't match any table row). Used to distinguish
/// `music/mod.rs` from a hypothetical top-level `mod.rs`.
fn two_segment_tail(path: &str) -> String {
    let p = std::path::Path::new(path);
    let parent = p
        .parent()
        .and_then(|pp| pp.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let base = path_basename(path);
    if parent.is_empty() || parent == "src" || parent == "." {
        base
    } else {
        format!("{}/{}", parent, base)
    }
}

/// Build the `@realisation/<sanitized-basename>` anchor ref. Sanitization
/// borrowed from `cmd_kintsugi_single`'s T19 precedent: drop whitespace
/// and control characters so `Ref::new` accepts the result.
fn anchor_ref(basename: &str) -> Ref {
    let safe: String = basename
        .chars()
        .filter(|c| !c.is_whitespace() && !c.is_control())
        .collect();
    let safe = if safe.is_empty() { "file".into() } else { safe };
    let s = format!("@realisation/{}", safe);
    // `unwrap_or_else` to a sentinel: if the sanitized form still fails
    // Ref's validator (defensive — shouldn't happen given the sanitiser),
    // fall back to a stable sentinel that round-trips. The classify
    // contract is total — every Rust file gets a classification.
    Ref::new(s).unwrap_or_else(|_| {
        Ref::new("@realisation/file").expect("sentinel anchor must construct")
    })
}

/// Construct a substrate-realisable classification. The `target` and
/// `rationale` strings are baked into the boundary code at the call
/// site; the helper just builds the `RealisableFile` record. `Clear`
/// verdict — the classification is unambiguous.
fn substrate(path: Ref, target: &str, _rationale: &'static str) -> RealisableFile {
    let target_ref = Ref::new(target.to_string())
        .unwrap_or_else(|_| panic!("substrate target ref invalid: {}", target));
    RealisableFile {
        path,
        altitude: Altitude::Substrate,
        target: target_ref,
        verdict: Transparency::Clear,
    }
}

/// Construct a boundary classification — the file stays in Rust at
/// some `@io/*` species. `Clear` verdict — the classification is
/// unambiguous.
fn boundary(path: Ref, target: &str, _rationale: &'static str) -> RealisableFile {
    let target_ref = Ref::new(target.to_string())
        .unwrap_or_else(|_| panic!("boundary target ref invalid: {}", target));
    RealisableFile {
        path,
        altitude: Altitude::Boundary,
        target: target_ref,
        verdict: Transparency::Clear,
    }
}

/// Construct an uncertain classification — the discriminator does not
/// know which altitude this file lives at. Per T22's substrate-honest
/// reading: the miss IS a substrate gap, surfaced as a `Partial`
/// opacity at the file's own anchor location. T23+ refines.
///
/// Altitude defaults to `Substrate` because an unknown file is *more
/// likely* substrate-realisable than @io (the @io species are
/// enumerated; the substrate-realisable space is open). The opacity
/// carries the diagnostic naming the gap.
fn uncertain(path: Ref, basename: &str, rationale: &'static str) -> RealisableFile {
    let diagnostic = Diagnostic::new(format!("{}: {}", basename, rationale));
    let opacity = PropertyVerdict::Partial {
        confidence: 0.0,
        diagnostics: vec![diagnostic],
    };
    // Target points back at @mirror/realisation itself — the gap IS a
    // substrate-pull-realize on the discriminator's own table.
    let target = Ref::new("@mirror/realisation")
        .expect("@mirror/realisation must construct as a valid ref");
    RealisableFile {
        path: path.clone(),
        altitude: Altitude::Substrate,
        target,
        verdict: Transparency::single(path, opacity),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gap_rs_classifies_as_substrate_oscillate() {
        let cls = classify("bootstrap/src/gap.rs");
        assert_eq!(cls.altitude, Altitude::Substrate);
        assert_eq!(cls.target.as_str(), "@mirror/spectral/oscillate");
        assert!(matches!(cls.verdict, Transparency::Clear));
    }

    #[test]
    fn main_rs_classifies_as_boundary_cli() {
        let cls = classify("bootstrap/src/main.rs");
        assert_eq!(cls.altitude, Altitude::Boundary);
        assert_eq!(cls.target.as_str(), "@io/cli");
        assert!(matches!(cls.verdict, Transparency::Clear));
    }

    #[test]
    fn portal_rs_classifies_as_boundary_socket() {
        let cls = classify("bootstrap/src/portal.rs");
        assert_eq!(cls.altitude, Altitude::Boundary);
        assert_eq!(cls.target.as_str(), "@io/socket");
    }

    #[test]
    fn sheaf_laplacian_rs_classifies_as_boundary_flang() {
        let cls = classify("bootstrap/src/sheaf_laplacian.rs");
        assert_eq!(cls.altitude, Altitude::Boundary);
        assert_eq!(cls.target.as_str(), "@io/flang");
    }

    #[test]
    fn music_mod_rs_classifies_as_substrate_music() {
        let cls = classify("bootstrap/src/music/mod.rs");
        assert_eq!(cls.altitude, Altitude::Substrate);
        assert_eq!(cls.target.as_str(), "@epistemologic/math/music");
    }

    #[test]
    fn unknown_rs_classifies_as_partial() {
        let cls = classify("bootstrap/src/quux_unknown.rs");
        // Unknown defaults to Substrate altitude (the open space) with
        // a Partial opacity at the anchor location.
        assert_eq!(cls.altitude, Altitude::Substrate);
        assert_eq!(cls.target.as_str(), "@mirror/realisation");
        match cls.verdict {
            Transparency::Opaque(_) => { /* expected */ }
            Transparency::Clear => panic!("unknown must surface a Partial opacity, not Clear"),
        }
    }

    #[test]
    fn target_altitude_projection_matches_classify() {
        let t = target_altitude("bootstrap/src/score.rs");
        assert_eq!(t.as_str(), "@mirror/spectral/score");
    }

    #[test]
    fn is_substrate_realisable_clear_is_success() {
        let v = is_substrate_realisable("bootstrap/src/gap.rs");
        assert!(matches!(v, terni::Imperfect::Success(())));
    }

    #[test]
    fn is_substrate_realisable_boundary_is_also_success() {
        // Per the docstring: boundary classifications also use Success
        // because the verdict measures classification confidence, not
        // membership in the substrate. A clean boundary IS clean.
        let v = is_substrate_realisable("bootstrap/src/main.rs");
        assert!(matches!(v, terni::Imperfect::Success(())));
    }

    #[test]
    fn is_substrate_realisable_unknown_is_partial() {
        let v = is_substrate_realisable("bootstrap/src/zzz_unknown.rs");
        assert!(matches!(v, terni::Imperfect::Partial((), _)));
    }

    #[test]
    fn anchor_ref_starts_with_realisation_prefix() {
        let cls = classify("bootstrap/src/gap.rs");
        assert!(cls.path.as_str().starts_with("@realisation/"));
        assert!(cls.path.as_str().contains("gap.rs"));
    }

    #[test]
    fn two_segment_tail_handles_top_level_files() {
        assert_eq!(two_segment_tail("bootstrap/src/gap.rs"), "gap.rs");
    }

    #[test]
    fn two_segment_tail_handles_sub_modules() {
        assert_eq!(
            two_segment_tail("bootstrap/src/music/mod.rs"),
            "music/mod.rs"
        );
    }
}
