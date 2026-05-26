//! Tests for `mirror compile --strict`: every source byte must either
//! enter the AST or trigger a strict-mode error.
//!
//! Regression for #91 (the silent `--strict` no-op on unrecognized syntax).
//! Before the fix, `mirror compile --strict` over a source containing the
//! `imperfect<portal>` drift returned the vacuously-empty OID with exit 0
//! — the action-decl rule's return-type reader uses `is_name_char` which
//! doesn't recognize `<`/`>`, so the rule failed to match; the body bytes
//! were silently consumed byte-by-byte through the unrecognized-char
//! fallback in `bootstrap/src/tokenize.rs`; no `Dark` node was emitted;
//! `enforce_strict` saw zero darks and reported success.
//!
//! `--strict` is the contract that every source byte either enters an AST
//! node, is comment/whitespace, OR produces an error. The Dark-node
//! mechanism handles the `<keyword> { ... }` shape; this test pins the
//! orthogonal shape — bytes between recognized tokens that never enter
//! the AST and never trigger an error.

use std::path::Path;
use std::process::Command;

fn repo_root() -> &'static Path {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    Box::leak(p.to_path_buf().into_boxed_path())
}

fn run_compile(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_mirror");
    let mut cmd = Command::new(exe);
    cmd.current_dir(repo_root());
    cmd.arg("compile");
    for a in args {
        cmd.arg(a);
    }
    cmd.output().expect("binary did not run")
}

/// The fixture that surfaced #91 in the field: an action declaration whose
/// return type uses foreign-language angle-bracket parametric notation.
/// `imperfect<portal>` is unrecognized by the action-decl rule, so the
/// body's `\` obligation marker never enters the AST as part of an
/// IoBinding; the bytes between `imperfect` and the final `}` are silently
/// consumed by the byte-by-byte fallback. `--strict` must error.
const DRIFT_FIXTURE: &str = "\
in @prism

grammar @test/drift {
  open(remote: ~uri) -> imperfect<portal> { \\ }
}

out open
";

#[test]
/// The regression. The drift fixture contains unrecognized bytes (the
/// `<portal>` between `imperfect` and `{`). `--strict` must refuse to
/// crystallize it. Before the fix this assertion fails because the
/// bootstrap returns exit 0 with the vacuously-empty OID.
fn strict_errors_on_unrecognized_bytes_between_tokens() {
    let path = std::env::temp_dir().join("mirror_strict_drift_91.mirror");
    std::fs::write(&path, DRIFT_FIXTURE).unwrap();
    let out = run_compile(&["--strict", "--no-cache", path.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "--strict must error on unrecognized bytes; got success.\n\
         stdout: {}\n\
         stderr: {}",
        stdout,
        stderr,
    );
}

#[test]
/// Diagnostic shape: the error must name a byte offset and surrounding
/// context so the developer can find the drift. Pinning the shape keeps
/// future tightening of the error format honest.
fn strict_error_names_offset_and_context() {
    let path = std::env::temp_dir().join("mirror_strict_drift_91_diag.mirror");
    std::fs::write(&path, DRIFT_FIXTURE).unwrap();
    let out = run_compile(&["--strict", "--no-cache", path.to_str().unwrap()]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "--strict must error; cannot assert diagnostic shape on success"
    );
    // The diagnostic must point at a location (line:col or byte offset)
    // so the developer can navigate to the drift. We accept either form;
    // the line/col rendering is the existing total_classification shape.
    let has_location = stderr.contains("line ") || stderr.contains("byte ");
    assert!(
        has_location,
        "strict error must include a location (line N or byte N); got: {}",
        stderr
    );
}

#[test]
/// Default (non-strict) compile must still succeed on the drift fixture
/// — the fix is opt-in via `--strict`. The compile yields *some* OID
/// (possibly different from the buggy one once the fix lands); the test
/// pins the success contract, not the OID value.
fn default_compile_still_succeeds_on_drift() {
    let path = std::env::temp_dir().join("mirror_strict_drift_91_default.mirror");
    std::fs::write(&path, DRIFT_FIXTURE).unwrap();
    let out = run_compile(&["--no-cache", path.to_str().unwrap()]);
    assert!(
        out.status.success(),
        "default compile must still succeed; --strict is opt-in.\n\
         stderr: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

#[test]
/// Sanity: `--strict` on a canonical fixture (no drift) still succeeds.
/// Without this, a regression that errors on ALL input would pass the
/// first test. The canonical form uses `(T)` not `<T>` per
/// `@kintsugi/fracture/generic-brackets`.
fn strict_accepts_canonical_parens() {
    let source = "\
in @prism

grammar @test/canonical {
  open(remote: ~uri) -> imperfect(portal) { \\ }
}

out open
";
    let path = std::env::temp_dir().join("mirror_strict_drift_91_canonical.mirror");
    std::fs::write(&path, source).unwrap();
    let out = run_compile(&["--strict", "--no-cache", path.to_str().unwrap()]);
    assert!(
        out.status.success(),
        "--strict must accept the canonical (T) form.\n\
         stderr: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}
