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
    for op in [
        "focus crystal",
        "project crystal",
        "split crystal",
        "shift crystal",
        "settle crystal",
    ] {
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

// ── Stage-2: substrate-level resolution through the kintsugi pipeline ────────────
//
// Stage-1 pinned the textual floor: the shard exists, declares the right
// glass at the right path, names the five fields, exports its surface.
// Stage-2 lifts the gate to substrate-level resolution: the shard parses
// cleanly through the production mirror grammar (no dark regions — the
// tokenizer classifies every span), AND a consumer that imports
// `@mirror/store/crystal` and rides the `crystal` carrier in downstream
// type positions also parses cleanly. If the substrate-decl left a gap the
// consumer's surface cannot reach, that gap surfaces as dark regions in
// the consumer fixture's CI record.
//
// Per [[feedback-substrate-already-had-the-word]]: if everything goes
// green on first run, the deeper RED is just naming what wasn't yet
// explicit — the substrate's consumption surface IS the next altitude.

use std::collections::HashMap;
use std::process::Output;

fn run_ki(args: &[&str]) -> Output {
    use std::os::unix::process::ExitStatusExt;
    let mut argv: Vec<String> = vec!["mirror".to_string(), "kintsugi".to_string()];
    for a in args {
        argv.push((*a).to_string());
    }
    let out = mirror::kintsugi_main_in(&argv, repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

fn parse_record(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut it = line.splitn(2, char::is_whitespace);
        let key = it.next().unwrap_or("").trim().to_string();
        let value = it.next().unwrap_or("").trim().to_string();
        if !key.is_empty() {
            map.insert(key, value);
        }
    }
    map
}

fn split_records(stdout: &str) -> Vec<HashMap<String, String>> {
    stdout
        .split("\n\n")
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(parse_record)
        .collect()
}

// ── Route A: crystal.mirror itself parses cleanly through the corpus walker.
//
// The textual floor (Stage-1) could be satisfied by a file containing the
// right substrings in any order. This test pins that the shard ALSO
// tokenizes to a clean AST under the production mirror grammar — zero
// dark regions, success verdict, deterministic.

#[test]
fn crystal_shard_resolves_through_corpus_walker() {
    let out = run_ki(&["--ci", "shards/mirror/store/crystal.mirror"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "kintsugi --ci on crystal.mirror must exit 0; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    assert!(
        !records.is_empty(),
        "corpus walker must emit at least one record; got: {stdout}"
    );
    let r = &records[0];
    assert_eq!(
        r.get("verdict").map(String::as_str),
        Some("success"),
        "crystal.mirror must parse to a success verdict through the production \
         grammar (substrate-level resolution, not textual presence); got: {stdout}"
    );
    assert_eq!(
        r.get("dark_count").map(String::as_str),
        Some("0"),
        "crystal.mirror must have zero dark regions — every span classifies; \
         got: {stdout}"
    );
}

// ── Route B: a consumer shard that rides the crystal carrier resolves cleanly.
//
// The consumer fixture imports `@mirror/store/crystal`, names
// `build_artifact = crystal`, and projects `composition_graph: mosaic(@code)`
// through a downstream `polyglot_dag` alias. If crystal's substrate-decl
// left any vocabulary gap the consumer's surface cannot reach (an undeclared
// carrier, a mis-shaped parametric form, a sub-prism declaration that
// doesn't compose), that gap surfaces here as dark regions on the consumer.
//
// Stricter than route A: route A pins crystal.mirror's own grammar
// compliance; route B pins the CONSUMPTION surface — what crystal looks
// like from the outside, riding the same grammar as every other shard.

#[test]
fn crystal_consumer_fixture_resolves_through_corpus_walker() {
    let out = run_ki(&["--ci", "bootstrap/tests/fixtures/crystal-consumer"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "kintsugi --ci on the crystal-consumer fixture must exit 0; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    assert!(
        !records.is_empty(),
        "corpus walker must emit at least one record; got: {stdout}"
    );
    let agg = &records[0];
    assert_eq!(
        agg.get("verdict").map(String::as_str),
        Some("success"),
        "the crystal-consumer fixture must aggregate to success — the consumer's \
         use of `crystal` as a type, of `mosaic(@code)` as a polyglot DAG \
         carrier, and of the `@mirror/store/crystal` import must all classify \
         under the production grammar; got: {stdout}"
    );
    assert_eq!(
        agg.get("dark_count").map(String::as_str),
        Some("0"),
        "the crystal-consumer fixture must have zero dark regions across the \
         corpus — if the consumption surface has a gap, it surfaces here; \
         got: {stdout}"
    );
}

// ── Stage-2 (cont.): the typed-field surface IS what the preamble said it is.
//
// The textual Stage-1 floor only pinned field NAMES. The preamble of
// crystal.mirror commits to specific substrate-pull field TYPES grounded
// in existing carriers:
//
//   section            : [splinter(@code)]      — @glass's splinter(altitude)
//   derived_predicates : [property_verdict]     — @glass's property_verdict
//   fracture_calendar  : transparency(au)       — @glass's transparency(p)
//   composition_graph  : mosaic(@code)          — @mirror/mosaic's mosaic(altitude)
//
// These are the substrate-pull bindings: each field rides an EXISTING
// substrate carrier (the substrate-already-had-the-word discipline). The
// following tests pin the typed bindings so a future tick can't drift a
// field to a bare or invented carrier without surfacing the regression.

#[test]
fn crystal_section_field_binds_splinter_at_code() {
    let src = shard_src();
    // The record's `section` field must ride `splinter(@code)` — the
    // @glass splinter(altitude) carrier specialized to the @code namespace.
    // Anything else (bare `[ref]`, an invented `language_section` type,
    // a stringly `[string]`) is a substrate-pull violation.
    assert!(
        src.contains("section: [splinter(@code)]"),
        "expected `section: [splinter(@code)]` — the parametric splinter \
         carrier from @glass specialized to the @code namespace; got:\n---\n{src}\n---"
    );
}

#[test]
fn crystal_derived_predicates_field_binds_property_verdict() {
    let src = shard_src();
    assert!(
        src.contains("derived_predicates: [property_verdict]"),
        "expected `derived_predicates: [property_verdict]` — the @glass \
         property_verdict carrier (verdict + location); got:\n---\n{src}\n---"
    );
}

#[test]
fn crystal_fracture_calendar_field_binds_transparency_of_au() {
    let src = shard_src();
    assert!(
        src.contains("fracture_calendar: transparency(au)"),
        "expected `fracture_calendar: transparency(au)` — @glass's \
         transparency(p) loss carrier parametric over au; got:\n---\n{src}\n---"
    );
}

#[test]
fn crystal_composition_graph_field_binds_mosaic_at_code() {
    let src = shard_src();
    // composition_graph IS mosaic(@code) — the polyglot-by-construction
    // structural witness. The @mirror/mosaic mosaic(altitude) carrier
    // specialized to @code. Critical: this binding is the entire
    // polyglot-by-construction claim made structural.
    assert!(
        src.contains("composition_graph: mosaic(@code)"),
        "expected `composition_graph: mosaic(@code)` — the mosaic(altitude) \
         carrier from @mirror/mosaic specialized to @code; this is the \
         polyglot-by-construction structural witness; got:\n---\n{src}\n---"
    );
}

#[test]
fn crystal_oid_field_binds_store_oid_carrier() {
    let src = shard_src();
    // The crystal's oid field rides @mirror/store/oid's `type oid = ref`
    // — the substrate's content-address primitive. Bare `string` or
    // `ref` directly here would erase the sibling sub-prism's identity
    // carrier and break the [[feedback-no-bare-types]] discipline.
    assert!(
        src.contains("oid: oid"),
        "expected `oid: oid` — the field rides @mirror/store/oid's typed \
         carrier, not bare `ref` or `string`; got:\n---\n{src}\n---"
    );
}

#[test]
fn crystal_imports_mosaic_for_composition_graph() {
    let src = shard_src();
    // composition_graph's mosaic(@code) carrier comes from @mirror/mosaic.
    // The substrate-pull discipline requires the import path be explicit
    // so future Seam review can trace where the carrier was declared.
    assert!(
        src.contains("in @mirror/mosaic"),
        "expected `in @mirror/mosaic` to source the mosaic(altitude) carrier \
         composition_graph rides; got:\n---\n{src}\n---"
    );
}

#[test]
fn crystal_imports_au_for_fracture_calendar() {
    let src = shard_src();
    // fracture_calendar's transparency(au) parametrizes over @mirror/au's
    // au type. The au import is the substrate-pull witness for the
    // parametric carrier's argument.
    assert!(
        src.contains("in @mirror/au"),
        "expected `in @mirror/au` to source the au type fracture_calendar \
         parametrizes the transparency carrier over; got:\n---\n{src}\n---"
    );
}

#[test]
fn crystal_imports_code_namespace() {
    let src = shard_src();
    // Both `section` and `composition_graph` specialize their parametric
    // carriers to @code. The @code namespace must be imported.
    assert!(
        src.contains("in @code"),
        "expected `in @code` to make the @code namespace available to \
         `splinter(@code)` and `mosaic(@code)`; got:\n---\n{src}\n---"
    );
}

#[test]
fn crystal_consumer_fixture_per_file_record_is_success() {
    // The aggregate is success only if every per-file record is success.
    // This test pins the per-file invariant explicitly so a future fixture
    // expansion (a second .mirror file in the consumer dir) can't drift the
    // aggregate to partial silently.
    let out = run_ki(&["--ci", "bootstrap/tests/fixtures/crystal-consumer"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    // First record is the aggregate; subsequent records are per-file.
    let per_file: Vec<&HashMap<String, String>> = records.iter().skip(1).collect();
    assert!(
        !per_file.is_empty(),
        "corpus walker must emit at least one per-file record after the \
         aggregate; got: {stdout}"
    );
    for r in &per_file {
        assert_eq!(
            r.get("verdict").map(String::as_str),
            Some("success"),
            "every per-file consumer record must be success; got: {stdout}"
        );
        assert_eq!(
            r.get("dark_count").map(String::as_str),
            Some("0"),
            "every per-file consumer record must have zero dark regions; \
             got: {stdout}"
        );
    }
}
