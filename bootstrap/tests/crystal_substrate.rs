//! Crystal substrate-decl RED — `@mirror/store/crystal` declares the
//! polyglot, content-addressed, self-executable artifact type that drops
//! out of the kintsugi build at the @mirror/store altitude.
//!
//! Per the 2026-06-16 recognition cascade: stage_play's Story → Play →
//! Narrative is the build-lifecycle inspiration; mirror's altitudes are
//! `spec → settle → verdict → crystal`. The crystal is the load-bearing
//! OUTPUT type — the thing a settle produces, the thing `shift` relocates
//! to `@io/runtime`, the thing whose OID becomes
//! `org.stagefreight.plan.spectral_coordinate` at the StageFreight wire
//! surface.
//!
//! Per task #268 (Crystal substrate-decl, pending). These assertions go
//! RED until Mara declares the shard at `shards/mirror/store/crystal.mirror`
//! with the five-field record:
//!
//!   type crystal = {
//!     oid: oid,                            // content address of the settled artifact
//!     section: ?,                          // language-section structure preserving @code/<lang> boundaries
//!     derived_predicates: ?,               // properties the kintsugi loop verified during settlement
//!     fracture_calendar: ?,                // open opacities the kintsugi loop chose not to close (au<T>'s gold-cracks)
//!     composition_graph: ?,                // DAG over @code/<lang> grammars — "polyglot by construction" made structural
//!   }
//!
//! Mara picks the exact sub-types for the four non-oid fields, grounded
//! in the existing substrate vocabulary (`shard`, `splinter_graph`,
//! `verdict`, `imperfect`, `transparency`, the `@code/*` namespace).
//!
//! The RED is read-only — these assertions don't invoke the bootstrap's
//! compile machinery; they just check the substrate source. Once Mara's
//! shard lands, a follow-up GREEN test should exercise resolution via
//! `mirror compile` and crystallization via a kintsugi settle. That
//! follow-up is out of scope for this RED; it depends on substrate
//! choices Mara will make here.

use std::path::{Path, PathBuf};

fn repo_root() -> &'static Path {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    Box::leak(p.to_path_buf().into_boxed_path())
}

fn shard_path() -> PathBuf {
    repo_root().join("shards/mirror/store/crystal.mirror")
}

fn shard_src() -> String {
    let p = shard_path();
    std::fs::read_to_string(&p).unwrap_or_else(|_| {
        panic!(
            "expected substrate file at {} (task #268 — Crystal substrate-decl)",
            p.display()
        )
    })
}

// ── Existence ───────────────────────────────────────────────────────────

#[test]
fn crystal_shard_file_exists() {
    assert!(
        shard_path().exists(),
        "expected {} to be declared (task #268; Mara owns)",
        shard_path().display()
    );
}

// ── Glass declaration at the @mirror/store/crystal sub-prism path ─────────────
//
// Per the path-namespace property declared in glass.mirror and lifted at
// store.mirror's @mirror/store/oid sub-prism, the crystal sub-prism must
// be named `@mirror/store/crystal` and live at the file path
// shards/mirror/store/crystal.mirror.

#[test]
fn crystal_shard_declares_glass_at_correct_path() {
    let src = shard_src();
    assert!(
        src.contains("glass @mirror/store/crystal"),
        "expected `glass @mirror/store/crystal {{ ... }}` declaration in the \
         crystal shard (mirroring the @mirror/store/oid sub-prism pattern \
         at shards/mirror/store.mirror); got:\n---\n{src}\n---"
    );
}

#[test]
fn crystal_glass_declares_five_operations() {
    let src = shard_src();
    // The glass declaration carries all five operations on `crystal`,
    // matching the pattern of @mirror/store/oid at shards/mirror/store.mirror.
    for op in ["focus crystal", "project crystal", "split crystal", "shift crystal", "settle crystal"] {
        assert!(
            src.contains(op),
            "expected glass @mirror/store/crystal to declare `{op}` \
             (per the five-operation algebra in shards/prism.mirror); \
             got:\n---\n{src}\n---"
        );
    }
}

// ── The five-field record ────────────────────────────────────────────────
//
// Per the 2026-06-16 substrate recognition: crystal carries
// {oid, section, derived_predicates, fracture_calendar, composition_graph}.
// The four non-oid fields are sub-types Mara chooses, grounded in existing
// substrate vocabulary. Sub-type choice is Mara's call; field NAMES are
// load-bearing and fixed by recognition.

#[test]
fn crystal_record_declares_all_five_fields() {
    let src = shard_src();
    for field in [
        "oid",
        "section",
        "derived_predicates",
        "fracture_calendar",
        "composition_graph",
    ] {
        assert!(
            src.contains(field),
            "expected the crystal record to declare field `{field}` \
             (per the 2026-06-16 recognition: crystal = \
             {{oid, section, derived_predicates, fracture_calendar, \
             composition_graph}}); got:\n---\n{src}\n---"
        );
    }
}

#[test]
fn crystal_declares_record_type_alias() {
    let src = shard_src();
    // Either `type crystal = { ... }` (record form, like splinter_graph
    // at shards/mirror/store.mirror) or a sequence of typed fields under
    // a record declaration. The substring `type crystal` is the simplest
    // structural witness.
    assert!(
        src.contains("type crystal"),
        "expected `type crystal = {{ ... }}` (mirroring `type splinter_graph = \
         {{ root: oid, children: [oid] }}` at shards/mirror/store.mirror); \
         got:\n---\n{src}\n---"
    );
}

// ── composition_graph is the load-bearing structural claim ──────────────────
//
// "Polyglot by construction" means: the crystal cannot be non-polyglot
// because the spec doesn't compile to one. composition_graph is the
// substrate's structural enforcement — a DAG whose nodes are @code/<lang>
// grammars and whose edges are compilation dependencies. The substrate
// rejects a crystal whose composition_graph is malformed (cycles,
// dangling references, missing @code/* nodes).

#[test]
fn composition_graph_references_code_namespace() {
    let src = shard_src();
    // composition_graph must structurally relate to @code/<lang> grammars.
    // The shard's prose or type signature must name @code somewhere —
    // either as an import (`in @code`), a type parameter (`@code/<lang>`),
    // or a comment-anchored reference Mara is free to phrase.
    assert!(
        src.contains("@code"),
        "expected the crystal shard to relate composition_graph to the \
         @code/* namespace (the polyglot-by-construction claim); \
         got:\n---\n{src}\n---"
    );
}

// ── Exports: the shard must make crystal consumable elsewhere ──────────────

#[test]
fn crystal_exports_namespace_and_type() {
    let src = shard_src();
    assert!(
        src.contains("out @mirror/store/crystal"),
        "expected `out @mirror/store/crystal` so other shards can \
         `in @mirror/store/crystal`; got:\n---\n{src}\n---"
    );
    assert!(
        src.contains("out crystal"),
        "expected `out crystal` so the type alias is consumable at call sites; \
         got:\n---\n{src}\n---"
    );
}

// ── Imports: crystal builds on existing substrate ───────────────────────────
//
// crystal cannot stand alone — oid lives at @mirror/store/oid (in the
// sibling shard), and the carrier vocabulary (verdict / imperfect /
// transparency) lives at @glass. The shard must import at least these
// two upstream substrates explicitly.

#[test]
fn crystal_imports_glass_substrate() {
    let src = shard_src();
    assert!(
        src.contains("in @glass"),
        "expected `in @glass` for the carrier vocabulary (verdict, \
         imperfect, transparency); got:\n---\n{src}\n---"
    );
}

#[test]
fn crystal_imports_store_oid() {
    let src = shard_src();
    // Either `in @mirror/store` (the parent, which exports oid) or
    // `in @mirror/store/oid` (the sub-prism directly).
    assert!(
        src.contains("in @mirror/store"),
        "expected `in @mirror/store` (or `in @mirror/store/oid`) to \
         pull in the oid type the crystal's first field references; \
         got:\n---\n{src}\n---"
    );
}

// ── Substrate-pull markers ─────────────────────────────────────────────────
//
// The shard's preamble must name the recognition that motivated the
// substrate-decl. This is structural documentation — future agents
// (Seam, Reflection) need to trace back to the 2026-06-16 cascade.

#[test]
fn crystal_preamble_names_the_recognition() {
    let src = shard_src();
    // Either "stage_play" (the inspiration) or "polyglot by construction"
    // (the load-bearing claim) or "spec → settle → verdict → crystal"
    // (the four-altitude shape) must appear in the preamble.
    let has_marker = src.contains("stage_play")
        || src.contains("polyglot by construction")
        || src.contains("polyglot-by-construction")
        || (src.contains("settle") && src.contains("verdict") && src.contains("crystal"));
    assert!(
        has_marker,
        "expected the shard preamble to anchor the recognition (stage_play, \
         polyglot-by-construction, or the spec→settle→verdict→crystal \
         four-altitude shape) so Seam can trace it back to the 2026-06-16 \
         cascade; got:\n---\n{src}\n---"
    );
}
