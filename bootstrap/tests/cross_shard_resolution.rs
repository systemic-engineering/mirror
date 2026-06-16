//! Cross-shard semantic import resolution RED — the gate Mara's Stage-2
//! crystal report (commit 9c358a2) named as the load-bearing next tick.
//!
//! Today the bootstrap's `--ci` corpus walker performs tokenization-
//! altitude resolution only (dark AST nodes via grammar action). A
//! dangling `in @<namespace>` declaration parses as syntax and is never
//! checked against the corpus's `out` declarations. The resolver does
//! not know what's reachable.
//!
//! Per task #268 follow-up tick #1 (Mara): cross-shard semantic name
//! resolution at the `mirror compile` altitude. The substrate-pull
//! recognition: a `.mirror` shard with a dangling import IS a corpus
//! with a structural dark region; the resolver's job is to surface it
//! as a failure verdict, not let it pass silently.
//!
//! GREEN: when this RED test passes, the resolver verifies every
//! `in <path>` resolves to a known `out <path>` somewhere in the loaded
//! shard corpus, and emits a failure verdict for unresolved imports.
//!
//! Mara owns the GREEN. Reed wrote this RED.

use std::collections::HashMap;
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::process::Output;

fn repo_root() -> &'static Path {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    Box::leak(p.to_path_buf().into_boxed_path())
}

fn run_ki(args: &[&str]) -> Output {
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

const BOGUS_FIXTURE: &str = "bootstrap/tests/fixtures/cross-shard-bogus-import";

// ── The gate ──────────────────────────────────────────────────────────────────
//
// A fixture that imports a nonexistent namespace must produce verdict:
// failure. Today the resolver lets it pass; tomorrow (after GREEN) it
// catches the unresolved reference.

#[test]
fn unresolved_cross_shard_import_emits_failure_verdict() {
    let out = run_ki(&["--ci", BOGUS_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    assert!(
        !records.is_empty(),
        "--ci walker must emit at least one record; got stdout:\n{stdout}"
    );
    let agg = &records[0];
    assert_eq!(
        agg.get("verdict").map(String::as_str),
        Some("failure"),
        "cross-shard resolver must catch `in @bogus/namespace/does/not/exist` \
         (the import in {BOGUS_FIXTURE}/bogus.mirror has no corresponding `out` \
         declaration in any loaded shard); got verdict={:?}; full stdout:\n{stdout}",
        agg.get("verdict")
    );
}

#[test]
fn unresolved_import_surfaces_in_dark_count() {
    // A dangling import is a structural dark region in the dependency
    // graph. The kintsugi loop's `dark_count` should count it.
    let out = run_ki(&["--ci", BOGUS_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    let agg = &records[0];
    let dark_count_str = agg.get("dark_count").map(String::as_str).unwrap_or("0");
    let dark_count: usize = dark_count_str.trim_matches('"').parse().unwrap_or(0);
    assert!(
        dark_count >= 1,
        "expected `dark_count >= 1` for unresolved cross-shard import \
         (the dangling `in @bogus/namespace/does/not/exist` IS a dark region); \
         got dark_count={dark_count}; full stdout:\n{stdout}"
    );
}

// ── Sanity: the GREEN side must also be honored ──────────────────────────────
//
// The resolver must be DISCRIMINATING, not just paranoid. Mara's
// Stage-2 fixture (crystal-consumer) imports `@mirror/store/crystal`
// which IS exported by shards/mirror/store/crystal.mirror. That fixture
// must continue to resolve successfully — the GREEN side proves the
// resolver doesn't false-positive on valid imports.
//
// This test guards against "flag every import as failure" as a cheap
// GREEN. The resolver must distinguish resolved from unresolved.

const CRYSTAL_CONSUMER_FIXTURE: &str = "bootstrap/tests/fixtures/crystal-consumer";

#[test]
fn resolved_cross_shard_import_still_succeeds() {
    let out = run_ki(&["--ci", CRYSTAL_CONSUMER_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    assert!(
        !records.is_empty(),
        "--ci walker must emit at least one record for crystal-consumer; \
         got stdout:\n{stdout}"
    );
    let agg = &records[0];
    assert_eq!(
        agg.get("verdict").map(String::as_str),
        Some("success"),
        "crystal-consumer's `in @mirror/store/crystal` IS resolvable \
         (shards/mirror/store/crystal.mirror exports it); resolver must NOT \
         false-positive on valid imports; got verdict={:?}; full stdout:\n{stdout}",
        agg.get("verdict")
    );
}
