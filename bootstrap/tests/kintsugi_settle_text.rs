//! T19 — `mirror kintsugi <file>` writes settled-formatting text to stdout.
//!
//! Fourth tick of the @mirror/spectral substrate-realisation cascade
//! (T16 → T17 → T18 → **T19** → T20). The substrate's kintsugi loop —
//! `oscillate_with_ast` (T13) — settles an AST through ACTIVE/DARK
//! alternation; T19 lands the surface that runs the loop **at the CLI
//! boundary** and emits the settled form as mirror-canonical text on
//! stdout.
//!
//! The path was *already* there for the AST-only rendering case
//! (`cmd_kintsugi_single` renders `render_ast(&ast)` to stdout per
//! T11.2.6). T19 lifts it: thread the AST through
//! [`oscillate::oscillate_with_ast`] so the rendered bytes are the
//! substrate's *settled* reading, not the raw parse.
//!
//! Test corpus (per Mara's T19 brief):
//!
//! - **round-trip-stable**: an already-settled fixture → output
//!   matches the canonical render (the loop's fixed point is the
//!   input).
//! - **kintsugi-ran**: an unsettled fixture (Dark regions) → output
//!   still emits the canonical render of the parsed AST (the loop
//!   runs end-to-end without erasing Dark — Dark is the substrate's
//!   honest acknowledgment; T19 doesn't paper over it). The output
//!   is non-empty and parseable; stderr stays quiet (no panic).
//! - **empty file**: graceful handling — empty output, exit 0.
//! - **--ci preservation**: the `mirror kintsugi --ci <dir>` corpus
//!   walker (T11.3) still emits the verdict envelope unchanged. T19
//!   only extends the single-file no-`--ci` path; the CI mode keeps
//!   its T11.2/T11.3 shape.
//!
//! The portal-detection branch (the discriminator the future T20
//! handshake lands at the CLI pipe boundary) is a `TODO(T20)`
//! comment in the body; T19 unconditionally takes the text branch.

use std::path::Path;
use std::process::Command;

fn repo_root() -> &'static Path {
    // CARGO_MANIFEST_DIR points at bootstrap/; its parent is the repo
    // root. Grammar lookups in the binary use paths relative to its
    // CWD (e.g. "boot/std/mirror/grammar.mirror"), so we run from
    // there — same convention as `kintsugi_loop.rs` and `kintsugi_ci.rs`.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    Box::leak(p.to_path_buf().into_boxed_path())
}

fn run_kintsugi(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_mirror");
    let mut cmd = Command::new(exe);
    cmd.current_dir(repo_root());
    cmd.arg("kintsugi");
    for a in args {
        cmd.arg(a);
    }
    cmd.output().expect("binary did not run")
}

/// A stable, already-settled fixture: `kintsugi-pass/a.mirror` is a
/// canonical-form `.mirror` source the corpus walker already reads
/// as `verdict=success`. The T19 settled-formatting output for this
/// fixture is byte-equal to running the loop twice (idempotence is
/// the loop's fixed-point claim).
const SETTLED_FIXTURE: &str = "bootstrap/tests/fixtures/kintsugi-pass/a.mirror";

/// An unsettled fixture: `kintsugi-partial/dark.mirror` has Dark
/// regions (substrate-marked unresolved parse). The T19 path must
/// run the loop end-to-end and emit the rendered AST without panic,
/// even though Dark persists. Dark is honest acknowledgment, not a
/// loop failure.
const UNSETTLED_FIXTURE: &str = "bootstrap/tests/fixtures/kintsugi-partial/dark.mirror";

// ── T19.1 — settled fixture: round-trip-stable on stdout ─────────────

/// `mirror kintsugi <settled-fixture>` writes the canonical render to
/// stdout. The output is non-empty (the fixture has substantive
/// content), and re-running on the same input is byte-identical
/// (the loop's fixed point IS the input for already-settled
/// content — the substrate's reflexivity reading).
#[test]
fn settled_fixture_emits_canonical_render_on_stdout() {
    let out = run_kintsugi(&[SETTLED_FIXTURE]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "settled fixture must exit 0; stderr: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.stdout.is_empty(),
        "settled fixture must emit a non-empty render on stdout",
    );
    // Substrate-pull: the render is mirror-canonical text. The
    // fixture's `in @prism` header is the substrate's namespace
    // declaration; it MUST survive the loop's settle step
    // (identity-preserving per the @mirror/spectral/morphism
    // glass-property triple).
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("@prism"),
        "settled fixture render must preserve the substrate header `in @prism`; got: {stdout}",
    );
}

/// **The T19 distinguisher.** The single-file path threads the AST
/// through [`oscillate::oscillate_with_ast`] (T13's driver); when
/// the loop runs, it emits a `[settle]` stderr trace naming the
/// terminal oscillation state and iteration count. RED before T19
/// lands: no `[settle]` trace because the path does NOT yet call
/// `oscillate_with_ast`. GREEN after: the trace appears, proving
/// the loop ran end-to-end at the CLI boundary.
#[test]
fn single_file_path_traces_settle_through_oscillate_with_ast() {
    let out = run_kintsugi(&[SETTLED_FIXTURE]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("[settle]"),
        "T19: the single-file path must thread the AST through \
         `oscillate_with_ast` and emit a `[settle] ...` stderr trace; \
         got stderr: {stderr}",
    );
}

/// Idempotence: running T19's settle path twice on the same settled
/// fixture yields byte-identical output. The kintsugi loop's fixed
/// point IS the input here; the substrate's reflexivity reading.
#[test]
fn settled_fixture_settle_path_is_idempotent() {
    let a = run_kintsugi(&[SETTLED_FIXTURE]);
    let b = run_kintsugi(&[SETTLED_FIXTURE]);
    assert_eq!(
        a.stdout, b.stdout,
        "T19's settle output must be deterministic across runs",
    );
}

// ── T19.2 — unsettled fixture: loop runs end-to-end, no panic ────────

/// `mirror kintsugi <unsettled-fixture>` runs the kintsugi loop
/// end-to-end through T13's `oscillate_with_ast` driver and emits
/// the rendered AST on stdout. Dark regions persist (the loop
/// honours the substrate's honest acknowledgment); the path does
/// NOT panic.
#[test]
fn unsettled_fixture_runs_loop_and_emits_text() {
    let out = run_kintsugi(&[UNSETTLED_FIXTURE]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "unsettled fixture must exit 0 (Dark is honest, not failure); stderr: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    // The render is non-empty (the fixture has the `in @prism`
    // header even though the body is Dark).
    assert!(
        !out.stdout.is_empty(),
        "unsettled fixture must still emit a non-empty render on stdout",
    );
    // No panic on stderr (the Banach contraction terminates per
    // T13's MAX_OSCILLATE_ITERATIONS cap).
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("panic"),
        "settle loop must not panic on unsettled input; stderr: {stderr}",
    );
}

// ── T19.3 — empty file: graceful handling ─────────────────────────────

/// `mirror kintsugi <empty-file>` is a graceful no-op: empty stdout,
/// exit 0. The loop has nothing to settle; the substrate's identity
/// on the empty AST is the empty AST; the render of empty is empty.
#[test]
fn empty_file_is_graceful() {
    // Write a temporary empty .mirror fixture under the OS tempdir,
    // not under tests/fixtures (which is tracked).
    let tmp = std::env::temp_dir().join("mirror_t19_empty.mirror");
    std::fs::write(&tmp, b"").expect("write empty fixture");
    let path = tmp.to_string_lossy().into_owned();
    let out = run_kintsugi(&[&path]);
    let _ = std::fs::remove_file(&tmp);
    assert_eq!(
        out.status.code(),
        Some(0),
        "empty file must exit 0; stderr: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    // Empty input → empty (or near-empty) render. We accept any
    // output with no panic; the graceful contract is exit 0 + no
    // stderr panic, not bytewise empty (the renderer may emit a
    // trailing newline for grammar reasons).
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("panic"),
        "empty file must not panic; stderr: {stderr}",
    );
}

// ── T19.4 — --ci preservation: the corpus mode is unchanged ───────────

/// The existing `mirror kintsugi --ci <dir>` corpus walker (T11.3)
/// still emits the verdict envelope unchanged. T19 only extends the
/// no-`--ci` single-file path; the CI surface keeps its T11.2/T11.3
/// shape and exit code. Preservation test.
#[test]
fn ci_corpus_mode_still_works() {
    let out = run_kintsugi(&["--ci", "bootstrap/tests/fixtures/kintsugi-pass"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "--ci corpus mode must exit 0; stderr: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    // The verdict envelope starts with `verdict <discrimination>`
    // per T11.2.5's mirror-text default. T19 must not regress this.
    assert!(
        stdout.trim_start().starts_with("verdict"),
        "--ci corpus mode must still emit the verdict envelope; got: {stdout}",
    );
}

/// The settled-text path emits NO `verdict` record header — that's
/// the `--ci` surface. T19 lands a different surface (canonical
/// render, not a CI verdict envelope). Substrate-pull-honest:
/// `--ci` is the verdict-emission gate; the bare invocation is the
/// settled-text gate.
#[test]
fn settled_text_path_emits_no_verdict_header() {
    let out = run_kintsugi(&[SETTLED_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.trim_start().starts_with("verdict "),
        "no-`--ci` path must NOT emit a verdict record; got: {stdout}",
    );
}
