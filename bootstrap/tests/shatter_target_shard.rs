//! Tick 0 Landing 4 RED — `mirror shatter <oid> <out> [--target <ref>]`
//! cli surface.
//!
//! Prerequisite for the 6-tick cascade discharge per Taut's LRM scout
//! `1658b95`: today the binary has NO `"shatter"` dispatch arm at all
//! (only compile/craft/kintsugi/init/recall/spawn are wired). This
//! test file names the contract the follow-up GREEN (Landing 2) must
//! satisfy — the RED runs against the pre-Landing-2 binary and every
//! test fails because the subcommand isn't recognized.
//!
//! Composes with:
//! - Mara `583b939` (@shatter × @io canonical spec §4.1): @shatter IS
//!   the @io linearization operator; `--target @<X>` selects one of
//!   the (possibly many) @data/* / @code/* / @io/* projections.
//! - mirror.spec cli-block `command shatter { flag target: str = "auto" }`
//!   (Landing 1 this tick; OID
//!   `796f3289629835f2e05cb1ac98559b5584fae8b2b76c2c9167eb91f4921cefb8`).
//! - `parse_substrate_ref_to_format` (kintsugi-scoped at `:1826-1832`
//!   today; lifted to shatter-scoped in Landing 3).
//!
//! Substrate discipline: the RED locks the SURFACE contract (arg
//! parsing, error paths, exit-code shape). Landing 2's stub body is
//! forward-promised; the real projection emission is a follow-up tick.
//! Tests here assert what the dispatch layer must handle, not what
//! the projection body produces.
//!
//! Pattern mirrors `bootstrap/tests/spawn_task_shard.rs`: in-process
//! dispatch via `mirror::kintsugi_main_in` wrapping the returned
//! `ExitOutput` in `std::process::Output`.

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn run_shatter(args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string(), "shatter".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

/// A well-formed 64-hex OID string. Content doesn't have to resolve to
/// a real stored shard — these tests assert dispatch/arg-parse
/// behavior, not projection semantics.
const TEST_OID: &str = "796f3289629835f2e05cb1ac98559b5584fae8b2b76c2c9167eb91f4921cefb8";

fn tmp_out_path(tag: &str) -> String {
    let mut base = std::env::temp_dir();
    let pid = std::process::id();
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    base.push(format!("mirror-shatter-target-{}-{}-{}", tag, pid, stamp));
    base.to_string_lossy().to_string()
}

// ── T1: subcommand exists ─────────────────────────────────────────────
//
// Today the binary rejects `mirror shatter` with `unknown: shatter` —
// the dispatcher has arms for compile / craft / kintsugi / init /
// recall / spawn only. RED: this assertion fails until Landing 2.

#[test]
fn t01_shatter_subcommand_is_recognized() {
    let out = run_shatter(&[TEST_OID, &tmp_out_path("t01")]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("unknown: shatter"),
        "T1: `mirror shatter` must be a recognized subcommand (not `unknown: shatter`); stderr:\n{}",
        stderr
    );
}

// ── T2: usage line names --target ─────────────────────────────────────
//
// Per Landing 1 mirror.spec cli-block: shatter carries `flag target:
// str = "auto"`. The dispatcher's usage() line must expose --target as
// a recognized flag so operators see it on missing-args.

#[test]
fn t02_usage_line_names_target_flag() {
    let out = run_shatter(&[]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_ne!(
        out.status.code(),
        Some(0),
        "T2: `mirror shatter` with no args must exit non-zero"
    );
    assert!(
        stderr.contains("--target") || stderr.contains("target"),
        "T2: usage line for `mirror shatter` must mention --target; got stderr:\n{}",
        stderr
    );
}

// ── T3: --target accepted without dispatch-layer error ────────────────
//
// The dispatch must ACCEPT `--target @data/json` as a well-formed
// flag+value pair AND route through Landing 3's lifted
// parse_substrate_ref_to_format. Whether the body succeeds on OID
// resolution is Landing 2's stub concern; T3 asserts the ARG-PARSE
// layer knows the flag. RED: fails because `shatter` isn't dispatched
// at all today.

#[test]
fn t03_target_flag_parsed_without_dispatch_layer_error() {
    let out = run_shatter(&[TEST_OID, &tmp_out_path("t03"), "--target", "@data/json"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("unknown: shatter"),
        "T3: `mirror shatter <oid> <out> --target @data/json` — shatter must be a known subcommand; stderr:\n{}",
        stderr
    );
    // Pre-Landing-2: the dispatcher's `--target` parser is craft's
    // `--target-kind` alias, so `@data/json` fails `parse_target` and
    // errors with `unknown --target-kind value: @data/json`. Post-
    // Landing-2 shatter routing, `--target @data/json` is a valid
    // substrate ref via the lifted parse_substrate_ref_to_format.
    assert!(
        !stderr.contains("--target-kind value"),
        "T3: `--target @data/json` must not surface craft's `--target-kind value:` diagnostic in a shatter context; got stderr:\n{}",
        stderr
    );
}

// ── T4: --target with unknown ref produces substrate-honest error ─────
//
// Per Landing 3 (parse_substrate_ref_to_format lift): unknown substrate
// refs must fail with a diagnostic that names the parse failure —
// substrate-honest surface, not a silent default. This mirrors the
// kintsugi `--out` chain's behavior at `dispatch_out_substrate_ref`.

#[test]
fn t04_target_with_unknown_ref_rejects_substrate_honestly() {
    let out = run_shatter(&[TEST_OID, &tmp_out_path("t04"), "--target", "@nonexistent/nope"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_ne!(
        out.status.code(),
        Some(0),
        "T4: `mirror shatter --target @nonexistent/nope` must exit non-zero (unknown substrate ref); stderr:\n{}",
        stderr
    );
    // The pre-Landing-2 dispatcher's `--target` parser is craft's
    // `--target-kind` alias and emits `unknown --target-kind value: ...`.
    // Post-Landing-2, `--target` in a shatter context must route
    // through the lifted `parse_substrate_ref_to_format` (Landing 3)
    // and emit a shatter-scoped diagnostic naming the shatter-target
    // parse failure. RED: the craft-scoped `--target-kind value:`
    // string is the WRONG diagnostic for a shatter dispatch.
    assert!(
        !stderr.contains("--target-kind value"),
        "T4: shatter dispatch must NOT surface craft's `--target-kind value:` diagnostic; --target here is the shatter substrate-ref selector. Got stderr:\n{}",
        stderr
    );
    // Positive shape: the diagnostic must name the ref (@nonexistent) or
    // the substrate-ref concept. Substrate-honest — mirror kintsugi's
    // `dispatch_out_substrate_ref` prints "no projection registered for
    // substrate ref: ..." or similar; shatter follows the same pattern.
    let mentions_ref = stderr.contains("@nonexistent")
        || stderr.contains("substrate ref")
        || stderr.contains("projection")
        || stderr.contains("shatter target");
    assert!(
        mentions_ref,
        "T4: unknown --target must produce a substrate-honest diagnostic naming the ref / substrate-ref / projection / shatter target; got stderr:\n{}",
        stderr
    );
}

// ── T5: --target defaults to "auto" when omitted ──────────────────────
//
// Per Landing 1 mirror.spec: `flag target: str = "auto"`. The
// dispatcher must accept omitting --target (default behavior applies).
// Omitting the flag must not trigger an "expected --target" error.

#[test]
fn t05_target_flag_is_optional() {
    let out = run_shatter(&[TEST_OID, &tmp_out_path("t05")]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    // Same-as-T1 shape check, but scoped to arg-parse: omitting --target
    // must not cause a "requires --target" or "missing target" error.
    assert!(
        !stderr.contains("--target requires") && !stderr.contains("missing target"),
        "T5: --target is optional (default \"auto\"); omitting it must not error; got stderr:\n{}",
        stderr
    );
}

// ── T6: --target=<value> equals-form parses ────────────────────────────
//
// Every other --flag in the binary accepts both `--flag value` and
// `--flag=value` forms (see e.g. `--shatter=N`, `--out=@data/json`,
// `--target-kind=binary` in `dispatch`). `--target=@data/json` must
// join the pattern.

#[test]
fn t06_target_equals_form_parses() {
    let out = run_shatter(&[TEST_OID, &tmp_out_path("t06"), "--target=@data/json"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("unknown: shatter"),
        "T6: `--target=@data/json` (equals form) — shatter must be a known subcommand; stderr:\n{}",
        stderr
    );
    // Same discrimination as T3: pre-Landing-2 routes to craft's
    // --target-kind alias and errors. Post-Landing-2 routes to
    // substrate-ref parse.
    assert!(
        !stderr.contains("--target-kind value"),
        "T6: `--target=@data/json` (equals form) must not surface craft's `--target-kind value:` diagnostic in a shatter context; got stderr:\n{}",
        stderr
    );
}
