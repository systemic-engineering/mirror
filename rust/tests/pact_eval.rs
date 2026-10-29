//! `mirror pact eval <file>` through the binary
//! (docs/audits/2026-10-29-seam-pact-eval-and-ruler-envelope.md §1.5).
//!
//! Fixtures are `tests/pact/*.pact-text`, transcribed by hand from
//! `tests/pact/oracles.md` and the `.json` vectors at `5babfbad` (blob ids
//! in each fixture's header). Expected outputs are `tests/pact/*.expected`
//! in the audit §1.4 format, written from the oracle, never from the
//! binary. If a transcription and the oracle disagree, the oracle is right.
//!
//! The binary runs with the repo root as its working directory, so the
//! record's `file "<path as given>"` line is the repo-relative path.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_mirror");

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn mirror(args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("spawn mirror")
}

fn eval(path: &str) -> Output {
    mirror(&["pact", "eval", path])
}

fn stdout(o: &Output) -> String {
    String::from_utf8(o.stdout.clone()).expect("stdout is UTF-8")
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

fn code(o: &Output) -> i32 {
    o.status.code().expect("exited, not signalled")
}

fn expected(name: &str) -> String {
    std::fs::read_to_string(repo_root().join(format!("tests/pact/{name}.expected")))
        .expect("expected file")
}

fn fixture(name: &str) -> String {
    std::fs::read_to_string(repo_root().join(format!("tests/pact/{name}.pact-text")))
        .expect("fixture")
}

fn assert_vector(name: &str, exit: i32) {
    let o = eval(&format!("tests/pact/{name}.pact-text"));
    assert!(
        !stderr(&o).contains("panicked"),
        "{name}: panicked:\n{}",
        stderr(&o)
    );
    assert_eq!(stdout(&o), expected(name), "{name}: record");
    assert_eq!(code(&o), exit, "{name}: exit code; stderr:\n{}", stderr(&o));
}

/// The record without its `file` line (the path differs from the fixture's).
fn without_file_line(record: &str) -> String {
    record
        .lines()
        .filter(|l| !l.starts_with("file "))
        .map(|l| format!("{l}\n"))
        .collect()
}

/// All orders of the record lines (comments dropped), written one after the
/// other to the same path, give byte-identical stdout, and that stdout is
/// the oracle's record (up to the `file` line) with the oracle's exit.
fn assert_permutation_invariant(name: &str, exit: i32) -> usize {
    let lines: Vec<String> = fixture(name)
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(String::from)
        .collect();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("permuted.pact-text");
    let path = path.to_str().unwrap();
    let mut first: Option<(String, i32)> = None;
    let mut count = 0;
    permute(&mut lines.clone(), 0, &mut |order| {
        std::fs::write(path, order.join("\n") + "\n").unwrap();
        let o = eval(path);
        let got = (stdout(&o), code(&o));
        match &first {
            None => {
                assert_eq!(
                    without_file_line(&got.0),
                    without_file_line(&expected(name)),
                    "{name}: record"
                );
                assert_eq!(got.1, exit, "{name}: exit");
                first = Some(got);
            }
            Some(f) => assert_eq!(&got, f, "{name}: order {order:?}"),
        }
        count += 1;
    });
    count
}

fn permute(v: &mut Vec<String>, k: usize, f: &mut dyn FnMut(&[String])) {
    if k == v.len() {
        f(v);
        return;
    }
    for i in k..v.len() {
        v.swap(k, i);
        permute(v, k + 1, f);
        v.swap(k, i);
    }
}

// ── Vectors ──────────────────────────────────────────────────────────────

#[test]
fn vector_01_nine_paths_clear_exit_0() {
    assert_vector("01-determinism", 0);
}

#[test]
fn vector_01_every_order_of_its_lines_is_byte_identical() {
    // 6 record lines: all 720 orders, which include the 48 rule-order ×
    // fact-order products the audit names.
    assert_eq!(assert_permutation_invariant("01-determinism", 0), 720);
}

#[test]
fn vector_03_send_false_clear_exit_0_statuses_only() {
    assert_vector("03-false-with-provenance", 0);
}

#[test]
fn vector_04_opaque_exit_1() {
    assert_vector("04-negation-cycle", 1);
}

#[test]
fn vector_04_every_order_of_its_lines_is_byte_identical() {
    assert_eq!(assert_permutation_invariant("04-negation-cycle", 1), 24);
}

#[test]
fn vector_05_functional_violation_is_refused_exit_255() {
    assert_vector("05-functional-violation", 255);
}

#[test]
fn vector_05_control_is_clear_exit_0() {
    assert_vector("05-functional-violation-control", 0);
}

// ── Dark: the reader, one case per grammar rule broken ───────────────────

/// Write `body` to a temp file, run it, and require a dark record naming
/// `line:col`, exit 2, and no panic.
fn assert_dark(label: &str, body: &[u8], at: &str) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("dark.pact-text");
    std::fs::write(&path, body).unwrap();
    let o = eval(path.to_str().unwrap());
    let out = stdout(&o);
    assert!(
        !stderr(&o).contains("panicked"),
        "{label}: panicked:\n{}",
        stderr(&o)
    );
    assert_eq!(code(&o), 2, "{label}: exit; stdout:\n{out}");
    assert!(
        out.starts_with("verdict     dark\n"),
        "{label}: not a dark record:\n{out}"
    );
    let dark = out
        .lines()
        .find(|l| l.starts_with("dark "))
        .unwrap_or_else(|| panic!("{label}: no dark line:\n{out}"));
    assert!(
        dark.trim_start_matches("dark")
            .trim_start()
            .starts_with(&format!("{at} ")),
        "{label}: want {at}, got {dark:?}"
    );
}

const OK: &str = "rule r p(?x) requires e(?x)\nfact e(a)\n";

#[test]
fn dark_carriage_return() {
    assert_dark("CR", b"fact e(a)\r\n", "1:10");
}

#[test]
fn dark_tab() {
    assert_dark("TAB", b"fact\te(a)\n", "1:5");
}

#[test]
fn dark_non_ascii() {
    assert_dark("non-ASCII", "fact e(\u{e4})\n".as_bytes(), "1:8");
}

#[test]
fn dark_nul() {
    assert_dark("NUL", b"fact e(a)\0\n", "1:10");
}

#[test]
fn dark_trailing_comment() {
    assert_dark("trailing comment", b"fact e(a) # no\n", "1:11");
}

#[test]
fn dark_comment_not_at_column_1() {
    // A record (and a comment) starts at column 1; a non-blank line that
    // starts with a space is dark at 1:1.
    assert_dark("indented comment", b"  # no\n", "1:1");
}

#[test]
fn dark_empty_parentheses() {
    assert_dark("p()", b"fact p()\n", "1:8");
}

#[test]
fn dark_numeric_constant() {
    assert_dark("numeric constant", b"fact e(1)\n", "1:8");
}

#[test]
fn dark_not_as_a_predicate() {
    assert_dark("not as predicate", b"fact not(a)\n", "1:6");
}

#[test]
fn dark_requires_as_a_constant() {
    assert_dark("requires as sym", b"fact e(requires)\n", "1:8");
}

#[test]
fn dark_duplicate_rule_id() {
    assert_dark(
        "duplicate rule id",
        b"rule r p(?x) requires e(?x)\nrule r q(?x) requires e(?x)\n",
        "2:6",
    );
}

#[test]
fn dark_non_ground_query() {
    assert_dark(
        "non-ground query",
        format!("{OK}query p(?x)\n").as_bytes(),
        "3:9",
    );
}

#[test]
fn dark_variable_in_a_fact() {
    assert_dark("variable in fact", b"fact e(?x)\n", "1:8");
}

#[test]
fn dark_query_arity_mismatch() {
    assert_dark(
        "query arity",
        format!("{OK}query p(a, b)\n").as_bytes(),
        "3:7",
    );
}

#[test]
fn dark_empty_requires() {
    assert_dark("empty requires", b"rule r p requires\n", "1:18");
}

#[test]
fn dark_missing_requires() {
    assert_dark("missing requires", b"rule r p(?x) e(?x)\n", "1:14");
}

#[test]
fn dark_unknown_key() {
    assert_dark("unknown key", b"axiom e(a)\n", "1:1");
}

#[test]
fn dark_leading_space_before_a_record() {
    assert_dark("leading space", b" fact e(a)\n", "1:1");
}

#[test]
fn dark_uppercase_name() {
    assert_dark("uppercase", b"fact E(a)\n", "1:6");
}

#[test]
fn dark_space_inside_parentheses() {
    assert_dark("space after (", b"fact e( a)\n", "1:8");
}

#[test]
fn dark_unclosed_parenthesis() {
    assert_dark("unclosed", b"fact e(a\n", "1:9");
}

#[test]
fn dark_position_zero() {
    assert_dark(
        "position 0",
        format!("{OK}functional p(0) -> 1\n").as_bytes(),
        "3:14",
    );
}

#[test]
fn dark_functional_without_arrow() {
    assert_dark(
        "no arrow",
        format!("{OK}functional p(1) 1\n").as_bytes(),
        "3:17",
    );
}

#[test]
fn dark_bad_rule_id() {
    assert_dark("rule id", b"rule R p(?x) requires e(?x)\n", "1:6");
}

#[test]
fn dark_not_without_an_atom() {
    // `not` not followed by a space and an atom is `not` used as a name.
    assert_dark("bare not", b"rule r p(?x) requires e(?x), not\n", "1:30");
}

// ── Dark: the engine ─────────────────────────────────────────────────────

#[test]
fn dark_unsafe_rule_names_the_rule_and_the_variable() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("unsafe.pact-text");
    std::fs::write(&path, "rule bad p(?x) requires not q(?x)\n").unwrap();
    let o = eval(path.to_str().unwrap());
    let out = stdout(&o);
    assert!(
        !stderr(&o).contains("panicked"),
        "panicked:\n{}",
        stderr(&o)
    );
    assert_eq!(code(&o), 2);
    assert!(out.starts_with("verdict     dark\n"), "{out}");
    let dark = out
        .lines()
        .find(|l| l.starts_with("dark "))
        .expect("dark line");
    assert!(
        dark.contains("bad") && dark.contains("?x"),
        "names rule and variable: {dark:?}"
    );
}

#[test]
fn dark_arity_mismatch_between_rules_and_facts() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("arity.pact-text");
    std::fs::write(&path, "rule r p(?x) requires e(?x, ?y)\nfact e(a)\n").unwrap();
    let o = eval(path.to_str().unwrap());
    assert_eq!(code(&o), 2);
    assert!(stdout(&o).starts_with("verdict     dark\n"));
    assert!(!stderr(&o).contains("panicked"));
}

#[test]
fn dark_unreadable_file() {
    let o = eval("tests/pact/does-not-exist.pact-text");
    assert_eq!(code(&o), 2);
    assert!(
        stdout(&o).starts_with("verdict     dark\n"),
        "{}",
        stdout(&o)
    );
}

// ── Usage errors: stderr, no record, exit 2 ──────────────────────────────

#[test]
fn usage_errors_print_no_record() {
    for args in [
        vec!["pact"],
        vec!["pact", "eval"],
        vec!["pact", "frob", "tests/pact/01-determinism.pact-text"],
        vec![
            "pact",
            "eval",
            "--strict",
            "tests/pact/01-determinism.pact-text",
        ],
        vec![
            "pact",
            "eval",
            "tests/pact/01-determinism.pact-text",
            "extra",
        ],
    ] {
        let o = mirror(&args);
        assert_eq!(code(&o), 2, "{args:?}");
        assert_eq!(stdout(&o), "", "{args:?}: no record on stdout");
        assert!(!stderr(&o).is_empty(), "{args:?}: says why on stderr");
        assert!(!stderr(&o).contains("panicked"), "{args:?}");
    }
}

// ── No serde in the slice ────────────────────────────────────────────────

#[test]
fn pact_text_rs_names_no_serde() {
    let src = std::fs::read_to_string(repo_root().join("rust/src/pact_text.rs"))
        .expect("rust/src/pact_text.rs exists");
    for (i, line) in src.lines().enumerate() {
        assert!(
            !line.contains("serde"),
            "rust/src/pact_text.rs:{}: {line}",
            i + 1
        );
    }
}
