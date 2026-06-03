//! T11.2.6 — closing the substrate-pull loop with a typed `Verdict`.
//!
//! T11.2.5 shipped untyped mirror-text key-value records as the
//! default `mirror kintsugi --ci` emission. That was substrate-pull-
//! aligned in spirit (mirror-shaped text the tokenizer can read) but
//! NOT in form: the substrate did not know what a verdict IS — it
//! just saw text it happened to tokenize as identifiers.
//!
//! T11.2.6 closes the loop by declaring `verdict` (and supporting
//! types) in `boot/std/kintsugi.mirror`. The Rust emitter's wire
//! shape stays IDENTICAL — what changes is that the substrate now
//! authoritatively declares the type those records belong to. The
//! emitter writes canonical instances of a typed mirror declaration,
//! not just "some text".
//!
//! The load-bearing assertion: the field set declared in the
//! `boot/std/kintsugi.mirror` `type verdict = {...}` block matches
//! the field set emitted by `mirror kintsugi --ci`, by direct
//! comparison. Drift on either side breaks the test — that's the
//! property we want: the wire shape and the typed declaration are
//! linked at the substrate's source of truth.
//!
//! These tests RED before the grammar declaration lands; GREEN after.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::Command;

fn repo_root() -> &'static Path {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    Box::leak(p.to_path_buf().into_boxed_path())
}

fn read_kintsugi_grammar() -> String {
    let path = repo_root().join("boot/std/kintsugi.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn run_ci(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_mirror");
    let mut cmd = Command::new(exe);
    cmd.current_dir(repo_root());
    cmd.arg("kintsugi");
    for a in args {
        cmd.arg(a);
    }
    cmd.output().expect("binary did not run")
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

/// Locate a `type <name> = { <body> }` block in a mirror source and
/// return its body (everything between the outer braces, verbatim).
/// Returns `None` if no such block exists. The scan is brace-balanced
/// so nested types don't trip it up.
fn find_record_type_body<'a>(source: &'a str, type_name: &str) -> Option<&'a str> {
    let needle = format!("type {} = {{", type_name);
    let start = source.find(&needle)?;
    let body_start = start + needle.len();
    let bytes = source.as_bytes();
    let mut pos = body_start;
    let mut depth = 1i32;
    while pos < bytes.len() && depth > 0 {
        match bytes[pos] {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            return Some(&source[body_start..pos]);
        }
        pos += 1;
    }
    None
}

/// Extract the set of field names declared inside a `type X = { ... }`
/// record body. Field lines are `<name>: <type>,` so we keep the token
/// before the first `:` on each line, ignoring blanks and comments.
fn record_fields(body: &str) -> HashSet<String> {
    let mut fields = HashSet::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("--") {
            continue;
        }
        if let Some((name, _)) = line.split_once(':') {
            let name = name.trim().trim_end_matches(',').trim();
            if !name.is_empty() && !name.contains(' ') {
                fields.insert(name.to_string());
            }
        }
    }
    fields
}

// ── Type declarations live in the substrate ─────────────────────────────────

#[test]
fn kintsugi_mirror_declares_discrimination_type() {
    // T11.2.6: the substrate names the three positions of the kintsugi
    // verdict — success, partial, failure. The wire's `verdict <pos>`
    // value must be one of these three; the type is the source of truth.
    let src = read_kintsugi_grammar();
    assert!(
        src.contains("type discrimination = success | partial | failure"),
        "boot/std/kintsugi.mirror must declare `type discrimination = success | partial | failure`; got:\n{src}"
    );
}

#[test]
fn kintsugi_mirror_declares_verdict_type() {
    // T11.2.6: the substrate declares the single-file verdict as a
    // typed record. Field set: verdict, target, objective, iterations,
    // dark_count. This is the canonical mirror-text record shape.
    let src = read_kintsugi_grammar();
    let body = find_record_type_body(&src, "verdict").unwrap_or_else(|| {
        panic!("boot/std/kintsugi.mirror must declare `type verdict = {{ ... }}`; got:\n{src}")
    });
    let fields = record_fields(body);
    let expected: HashSet<String> = ["verdict", "target", "objective", "iterations", "dark_count"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(
        fields, expected,
        "declared `type verdict` field set must match the wire contract; \
         declared: {fields:?}, expected: {expected:?}; body:\n{body}"
    );
}

#[test]
fn kintsugi_mirror_declares_verdict_entry_type() {
    // T11.2.6: per-file entry inside a corpus verdict. Differs from
    // `verdict` only in the leading field name (`file`, not `target`).
    let src = read_kintsugi_grammar();
    let body = find_record_type_body(&src, "verdict_entry").unwrap_or_else(|| {
        panic!("boot/std/kintsugi.mirror must declare `type verdict_entry = {{ ... }}`; got:\n{src}")
    });
    let fields = record_fields(body);
    let expected: HashSet<String> = ["file", "verdict", "objective", "iterations", "dark_count"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(
        fields, expected,
        "declared `type verdict_entry` field set must match per-file wire contract; \
         declared: {fields:?}, expected: {expected:?}; body:\n{body}"
    );
}

#[test]
fn kintsugi_mirror_declares_corpus_verdict_type() {
    // T11.2.6: corpus verdict aggregate envelope. Adds files_processed
    // and a per_file list of verdict_entry values to the single-file
    // shape.
    let src = read_kintsugi_grammar();
    let body = find_record_type_body(&src, "corpus_verdict").unwrap_or_else(|| {
        panic!("boot/std/kintsugi.mirror must declare `type corpus_verdict = {{ ... }}`; got:\n{src}")
    });
    let fields = record_fields(body);
    let expected: HashSet<String> = [
        "verdict",
        "target",
        "objective",
        "iterations",
        "dark_count",
        "files_processed",
        "per_file",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(
        fields, expected,
        "declared `type corpus_verdict` field set must match the corpus wire contract; \
         declared: {fields:?}, expected: {expected:?}; body:\n{body}"
    );
}

// ── Round-trip: emitter output matches declared field set ───────────────────

const FIXTURE: &str = "boot/std/nl.mirror";
const PASS_FIXTURE: &str = "bootstrap/tests/fixtures/kintsugi-pass";

#[test]
fn emitted_single_file_verdict_field_set_matches_declared_type() {
    // T11.2.6 round-trip: parse the emitter output, parse the declared
    // type, assert the field sets coincide. The wire and the substrate
    // are linked at the source of truth — drift on either side breaks
    // this assertion.
    let out = run_ci(&["--ci", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    assert!(!records.is_empty(), "must emit at least one record");
    let wire_fields: HashSet<String> = records[0].keys().cloned().collect();

    let src = read_kintsugi_grammar();
    let body = find_record_type_body(&src, "verdict")
        .expect("type verdict declaration must exist in boot/std/kintsugi.mirror");
    let declared_fields = record_fields(body);

    assert_eq!(
        wire_fields, declared_fields,
        "wire field set must match declared `type verdict` field set; \
         wire: {wire_fields:?}, declared: {declared_fields:?}"
    );
}

#[test]
fn emitted_corpus_aggregate_field_set_matches_declared_type() {
    // T11.2.6 round-trip for the corpus aggregate envelope.
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    assert!(!records.is_empty(), "must emit at least an aggregate record");
    let wire_fields: HashSet<String> = records[0].keys().cloned().collect();

    let src = read_kintsugi_grammar();
    let body = find_record_type_body(&src, "corpus_verdict")
        .expect("type corpus_verdict declaration must exist in boot/std/kintsugi.mirror");
    let declared_fields = record_fields(body);

    // The declared type carries `per_file: [verdict_entry]`; the
    // aggregate envelope on the wire omits `per_file` because each
    // entry is its own blank-line-separated record (the wire shape
    // flattens the list). So the round-trip equality is on the
    // envelope-level fields: all declared fields EXCEPT `per_file`.
    let mut declared_envelope = declared_fields.clone();
    declared_envelope.remove("per_file");

    assert_eq!(
        wire_fields, declared_envelope,
        "aggregate wire field set must match declared `type corpus_verdict` \
         envelope field set (per_file flattens to records); \
         wire: {wire_fields:?}, declared envelope: {declared_envelope:?}"
    );
}

#[test]
fn emitted_corpus_per_file_field_set_matches_declared_type() {
    // T11.2.6 round-trip for the per-file entry in corpus mode.
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    assert!(
        records.len() >= 2,
        "expected aggregate + at least one per-file record"
    );
    let wire_fields: HashSet<String> = records[1].keys().cloned().collect();

    let src = read_kintsugi_grammar();
    let body = find_record_type_body(&src, "verdict_entry")
        .expect("type verdict_entry declaration must exist in boot/std/kintsugi.mirror");
    let declared_fields = record_fields(body);

    assert_eq!(
        wire_fields, declared_fields,
        "per-file wire field set must match declared `type verdict_entry` field set; \
         wire: {wire_fields:?}, declared: {declared_fields:?}"
    );
}

// ── Discriminator value range ──────────────────────────────────────────────

#[test]
fn emitted_discrimination_value_is_one_declared_position() {
    // T11.2.6: the wire's `verdict <pos>` value must match one of the
    // positions declared by `type discrimination = success | partial | failure`.
    let out = run_ci(&["--ci", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let r = &split_records(&stdout)[0];
    let pos = r.get("verdict").expect("verdict field present");

    // Pull the declared positions from the substrate, not a hardcoded list.
    let src = read_kintsugi_grammar();
    let decl = "type discrimination = success | partial | failure";
    assert!(
        src.contains(decl),
        "discrimination type declaration not found verbatim; \
         substrate-pull contract: the type lives in the substrate"
    );
    let positions: HashSet<&str> = ["success", "partial", "failure"].iter().copied().collect();
    assert!(
        positions.contains(pos.as_str()),
        "wire `verdict {pos}` must be one of the declared discrimination positions {positions:?}"
    );
}

// ── Substrate loads cleanly (no regression in kintsugi.mirror tokenizing) ──

#[test]
fn kintsugi_mirror_still_compiles_strict_after_verdict_declaration() {
    // T11.2.6 must not regress kintsugi.mirror's substrate health. The
    // file compiled with zero dark regions under --strict before; the
    // typed verdict declarations must not introduce dark regressions.
    let exe = env!("CARGO_BIN_EXE_mirror");
    let out = Command::new(exe)
        .current_dir(repo_root())
        .args(["compile", "--strict", "--no-cache", "boot/std/kintsugi.mirror"])
        .output()
        .expect("binary did not run");
    let code = out.status.code();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        code,
        Some(0),
        "boot/std/kintsugi.mirror must compile --strict with zero dark regions; \
         exit code: {code:?}; stderr:\n{stderr}"
    );
}
