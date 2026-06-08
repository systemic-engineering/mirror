//! T22 — `mirror kintsugi <file.rs>` invokes the realisation discriminator.
//!
//! First kintsugi-on-Rust tick. T21 declared the discriminator surface
//! (`shards/mirror/realisation.mirror`'s `classify(f) -> realisable_file`);
//! T22 lands the boundary-Rust realisation (`bootstrap/src/realisation.rs`)
//! that runs the discriminator's MVP body — a hardcoded match table
//! mapping Rust file basename → target substrate altitude — and wires
//! `mirror kintsugi <file>.rs` to invoke it. The match table IS the
//! 29-instance substrate-pull training set made explicit, per Mara's
//! T22 brief.
//!
//! ## Why transient fixtures, not the real bootstrap files
//!
//! The discriminator dispatches on file BASENAME (T22 MVP body), not on
//! content. Pre-T22 the binary would still tokenize the file through the
//! rust grammar before any dispatch, which is O(size) — `oscillate.rs`
//! at 144KB hangs. Post-T22 the `.rs` short-circuit reaches
//! `cmd_kintsugi_rust` BEFORE tokenize, so the dispatch is O(1). To keep
//! the integration test fast and deterministic regardless of pre/post,
//! we write 1-line transient fixtures with the right basenames under
//! the OS tempdir and assert on the classification. The classification
//! is content-blind in the MVP; basename is the discriminator's input.
//!
//! Test corpus:
//!
//! - **substrate-realisable** files (`gap.rs`, `kintsugi.rs`,
//!   `oscillate.rs`, `score.rs`, etc.) emit a classification with
//!   `altitude=Substrate` and the expected `target` @mirror altitude
//!   on stderr, plus a fracture-proposal comment on stdout.
//! - **boundary** files (`main.rs`, `portal.rs`, `sheaf_laplacian.rs`)
//!   emit a classification with `altitude=Boundary` and the expected
//!   `@io/*` target on stderr.
//! - **unknown** Rust files emit a classification with `verdict=Partial`
//!   naming the substrate-pull gap.
//! - **preservation**: `mirror kintsugi <fixture>.mirror` (T19's
//!   settled-text path) is unaffected by the T22 dispatch.
//!
//! The fracture-proposal output is intentionally minimal this tick
//! (comment text on stdout); T23+ matures the discriminator's body to
//! emit structured fracture diffs against the target altitude's
//! grammar.

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> &'static Path {
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

/// Build a transient `.rs` fixture under the OS tempdir. Per the
/// module docstring: the MVP discriminator dispatches on basename,
/// not content; a one-line stub at the right basename suffices to
/// exercise the classification path end-to-end.
///
/// Returns `(tempdir, file_path)`. Caller is responsible for
/// removing both when the test ends.
fn write_fixture(basename: &str) -> (PathBuf, PathBuf) {
    // Per-test, per-process unique dir so parallel test execution
    // doesn't fight over the same path.
    let dir = std::env::temp_dir().join(format!(
        "mirror_t22_{}_{}_{:?}",
        basename.replace('/', "_").replace('.', "_"),
        std::process::id(),
        std::thread::current().id(),
    ));
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join(basename);
    if let Some(parent) = file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&file, b"// T22 transient fixture\n").expect("write fixture");
    (dir, file)
}

fn cleanup(dir: &Path) {
    let _ = std::fs::remove_dir_all(dir);
}

fn run_on_fixture(basename: &str) -> std::process::Output {
    let (dir, file) = write_fixture(basename);
    let path = file.to_string_lossy().into_owned();
    let out = run_kintsugi(&[&path]);
    cleanup(&dir);
    out
}

// ── T22.1 — substrate-realisable: gap.rs ──────────────────────────────

/// `mirror kintsugi gap.rs` runs the discriminator, emits a
/// classification on stderr with `altitude=Substrate` and
/// `target=@mirror/spectral/oscillate`, and writes a fracture-proposal
/// comment on stdout. The first kintsugi-on-Rust file in the cascade.
#[test]
fn gap_rs_classifies_as_substrate_realisable() {
    let out = run_on_fixture("gap.rs");
    assert_eq!(
        out.status.code(),
        Some(0),
        "kintsugi on a Rust file must exit 0; stderr: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("[realisation]"),
        "T22: classify must emit a `[realisation] ...` trace; got: {stderr}",
    );
    assert!(
        stderr.contains("altitude=Substrate"),
        "gap.rs is substrate-realisable; expected `altitude=Substrate`; got: {stderr}",
    );
    assert!(
        stderr.contains("@mirror/spectral/oscillate"),
        "gap.rs targets @mirror/spectral/oscillate per the T22 MVP table; got: {stderr}",
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("// kintsugi-on-Rust"),
        "T22: fracture proposal must surface as a `// kintsugi-on-Rust` \
         comment on stdout; got: {stdout}",
    );
    assert!(
        stdout.contains("// fracture proposal"),
        "T22: stdout must include a `// fracture proposal` line; got: {stdout}",
    );
}

// ── T22.2 — boundary: main.rs / portal.rs / sheaf_laplacian.rs ────────

/// `main.rs` is @io/cli — OS dispatch, argument parsing, process exit
/// codes. The discriminator classifies it as `Boundary`.
#[test]
fn main_rs_classifies_as_boundary() {
    let out = run_on_fixture("main.rs");
    assert_eq!(
        out.status.code(),
        Some(0),
        "kintsugi on a boundary Rust file must exit 0; stderr: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("altitude=Boundary"),
        "main.rs is @io; expected `altitude=Boundary`; got: {stderr}",
    );
    assert!(
        stderr.contains("@io/cli"),
        "main.rs targets @io/cli per the T22 MVP table; got: {stderr}",
    );
}

/// `portal.rs` is @io/socket — the SCM_RIGHTS handoff lives at the
/// permanent-@io altitude. Classify as `Boundary`.
#[test]
fn portal_rs_classifies_as_boundary() {
    let out = run_on_fixture("portal.rs");
    assert_eq!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("altitude=Boundary"),
        "portal.rs is @io; got: {stderr}",
    );
    assert!(
        stderr.contains("@io/socket"),
        "portal.rs targets @io/socket; got: {stderr}",
    );
}

/// `sheaf_laplacian.rs` is @io/flang — the LAPACK FFI boundary. The
/// substrate algebra lives at @epistemologic/math/sheaf_laplacian;
/// the *FFI realisation* stays in Rust at @io/flang.
#[test]
fn sheaf_laplacian_rs_classifies_as_boundary() {
    let out = run_on_fixture("sheaf_laplacian.rs");
    assert_eq!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("altitude=Boundary"),
        "sheaf_laplacian.rs is @io; got: {stderr}",
    );
    assert!(
        stderr.contains("@io/flang"),
        "sheaf_laplacian.rs targets @io/flang; got: {stderr}",
    );
}

// ── T22.3 — substrate-realisable: a few more from the MVP table ──────

#[test]
fn oscillate_rs_classifies_as_substrate_realisable() {
    let out = run_on_fixture("oscillate.rs");
    assert_eq!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("altitude=Substrate"));
    assert!(stderr.contains("@mirror/spectral/oscillate"));
}

#[test]
fn score_rs_classifies_as_substrate_realisable() {
    let out = run_on_fixture("score.rs");
    assert_eq!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("altitude=Substrate"));
    assert!(stderr.contains("@mirror/spectral/score"));
}

#[test]
fn tensor_rs_classifies_as_substrate_realisable() {
    let out = run_on_fixture("tensor.rs");
    assert_eq!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("altitude=Substrate"));
    assert!(stderr.contains("@epistemologic/math/sheaf_laplacian"));
}

#[test]
fn ast_rs_classifies_as_substrate_realisable() {
    let out = run_on_fixture("ast.rs");
    assert_eq!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("altitude=Substrate"));
    assert!(stderr.contains("@code/mirror"));
}

// ── T22.4 — unknown Rust file → uncertain ─────────────────────────────

/// A Rust filename not present in the MVP table emits an uncertain
/// classification — Partial verdict, naming the gap as a substrate-pull
/// candidate. The discriminator MUST NOT confidently classify a file
/// it has not been trained on.
#[test]
fn unknown_rust_file_classifies_as_uncertain() {
    let out = run_on_fixture("some_unknown_t22.rs");
    assert_eq!(
        out.status.code(),
        Some(0),
        "unknown Rust file must still exit 0 — the discriminator's miss \
         is a substrate gap, not a failure; stderr: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("verdict=Partial"),
        "unknown file must emit verdict=Partial (the substrate-pull surface); \
         got: {stderr}",
    );
}

// ── T22.5 — preservation: T19's .mirror path is unchanged ────────────

/// `mirror kintsugi <fixture>.mirror` (T19's settled-text path) still
/// runs the kintsugi loop, emits the canonical render on stdout, and
/// the `[settle]` trace on stderr. T22's Rust dispatch does NOT
/// touch the .mirror branch.
#[test]
fn mirror_file_path_is_preserved() {
    let out = run_kintsugi(&["bootstrap/tests/fixtures/kintsugi-pass/a.mirror"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T19 .mirror path must still work post-T22; stderr: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("[settle]"),
        "T19's `[settle]` stderr trace must survive T22's dispatch; got: {stderr}",
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("@prism"),
        "T19's settled render must still preserve the substrate header; got: {stdout}",
    );
    // The T22 Rust-dispatch stdout marker MUST NOT leak into the
    // .mirror path.
    assert!(
        !stdout.contains("// kintsugi-on-Rust"),
        ".mirror dispatch must NOT emit the T22 Rust comment marker; got: {stdout}",
    );
}
