//! Tick 1 Landing 3 DELAYED-GREEN — `command X { command Y { ... } }`
//! subcommand nesting grammar admissibility at the compile altitude.
//!
//! DELAYED-GREEN (not RED): the compile pipeline is permissive to
//! nested declaration heads inside command bodies today — the substrate
//! admits the recursive-command form directly at compile altitude
//! without a depth cap. This test therefore locks the invariant
//! going forward: if a future compile-altitude tightening rejects
//! nested `command` bodies, this test flags the regression. The
//! substrate-decl grammar (Landing 1) declares the geometry; this
//! test discharges the geometric ground truth as a preserved
//! compile-altitude invariant. Runtime dispatch semantics (i.e.,
//! whether the binary recognizes `mirror peer beam <peer_home>`)
//! land at Tick 3/4 of the 6-tick cascade with their own
//! RED-first tests.
//!
//! Landed alongside:
//!   - Landing 1: `shards/mirror/lens/cli.mirror` docblock + grammar
//!     extension — recursive-command form (Option A), depth-2 admitted
//!     directly. (OID via `mirror compile shards/mirror/lens/cli.mirror`.)
//!   - Landing 2: `docs/specs/cli-as-prism.md` §3.2 — depth-2 promoted
//!     from RESERVED to LANDED; recursive-command form named as the
//!     Mara-choice; Reed's `subcommand(name)` sketch superseded.
//!
//! Mara-choice at collapse: Option A (recursive-command) over Option B
//! (new `subcommand` head). See `shards/mirror/lens/cli.mirror`
//! docblock "Tick 1 landing" section for rationale.
//!
//! This test locks the SURFACE contract at the compile altitude: a
//! shard fixture containing a `command peer { command beam { ... } }`
//! nesting IS admissible substrate — it compiles cleanly and produces
//! an OID. The test does NOT assert the runtime dispatcher realizes
//! the nested action; that's Tick 3/4 of the 6-tick cascade.
//!
//! Substrate discipline: the substrate-decl grammar is authoritative;
//! the compile-altitude admissibility discharges the geometric ground
//! truth (Alex 2026-07-07:
//! feedback-cli-subcommand-nesting-is-geometric-ground-truth memory).
//! The consumer landing (`command peer { command beam { ... } }` at
//! `mirror.spec`'s cli-block) is the follow-up tick.
//!
//! RED-first: this test asserts compile-level admissibility. The
//! grammar admits recursion by the extension in Landing 1; if the
//! `mirror compile` pipeline enforces a depth cap or rejects nested
//! `command` bodies at parse time, this test fails.
//!
//! Pattern mirrors `bootstrap/tests/shatter_target_shard.rs` on the
//! subprocess side but scoped to compile-altitude (not runtime
//! dispatch): shard fixture -> `mirror compile <path>` -> assert OID.

use std::io::Write;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn run_mirror(args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

/// Write a shard fixture with `command X { command Y { ... } }` nesting.
/// Returns the tempfile path.
fn write_nested_command_fixture(tag: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let pid = std::process::id();
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    path.push(format!(
        "mirror-cli-subcommand-nesting-{}-{}-{}.mirror",
        tag, pid, stamp
    ));
    let content = r#"in @prism
in @mirror/lens/cli

# Test fixture: depth-2 subcommand nesting via recursive-command.
# Per shards/mirror/lens/cli.mirror docblock (Tick 1 landing,
# 2026-07-08 Mara): `command peer { command beam { ... } }` is
# admissible substrate. This fixture exercises the grammar's
# recursive-command admissibility at compile altitude.

command peer {
  # peer-scope: the enclosing command declares no args of its own;
  # its body composes of nested subcommands.
  command beam {
    # peer beam: beam + persistent-identity-context. See
    # docs/specs/beam-as-substrate-primitive.md §3.2.
    arg peer_home: ~d
    flag mission: ~f
  }
}
"#;
    let mut f = std::fs::File::create(&path).expect("create fixture");
    f.write_all(content.as_bytes()).expect("write fixture");
    path
}

// ── T1: nested `command Y` inside `command X` compiles cleanly ────────
//
// The recursive-command grammar (Landing 1) admits `command` inside
// a `command` body. RED asserts compile succeeds and emits an OID.
// If the parser rejects the nesting with an
// "unexpected `command` inside command body" diagnostic, this fails.

#[test]
fn t01_nested_command_compiles_cleanly() {
    let fixture = write_nested_command_fixture("t01");
    let out = run_mirror(&["compile", fixture.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T1: `mirror compile <fixture-with-nested-command>` must exit 0; stdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
    // A well-formed OID is 64 hex chars on stdout.
    let trimmed = stdout.trim();
    assert_eq!(
        trimmed.len(),
        64,
        "T1: `mirror compile` must emit a 64-hex OID on stdout; got {} chars: {:?}",
        trimmed.len(),
        trimmed
    );
    assert!(
        trimmed.chars().all(|c| c.is_ascii_hexdigit()),
        "T1: `mirror compile` OID must be lowercase hex; got: {:?}",
        trimmed
    );
    let _ = std::fs::remove_file(&fixture);
}

// ── T2: no substrate-honest "unknown head" diagnostic on nested command ──
//
// The recursive-command form means `command` inside a command body
// is the SAME head noun — not "unknown." If the parser mis-reports
// the inner `command` as an unknown declaration head, this fails.

#[test]
fn t02_nested_command_not_reported_as_unknown() {
    let fixture = write_nested_command_fixture("t02");
    let out = run_mirror(&["compile", fixture.to_str().unwrap()]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("unknown") || !stderr.contains("command"),
        "T2: nested `command` in a command body must not be reported as unknown; got stderr:\n{}",
        stderr
    );
    let _ = std::fs::remove_file(&fixture);
}
