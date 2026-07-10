//! PREREQ-1 (Tick 2) RED — `spectral` as declaration head admitted by
//! `collect_declared_namespaces` at `bootstrap/src/lib.rs:2131-2140`.
//!
//! Substrate motion authority: Mara canonical spec `e764a32`
//! (`docs/specs/substrate-self-migration-via-spectral-typing.md` §5) +
//! Taut boundary scout `8a3b0a4` (three-prereq min-cut §6) + Seam
//! Phase D ratification `2b56977` (RATIFY-WITH-QUALIFICATIONS,
//! landing-order Q7: PREREQ-1 MUST precede PREREQ-2).
//!
//! The 3-line delta:
//! ```rust
//! } else if let Some(r) = line.strip_prefix("spectral ") {
//!     r
//! ```
//! inserted between the existing `grammar` and `else` clauses.
//!
//! Post-GREEN: `spectral @foo/bar { ... }` declarations at any shard
//! in the corpus register as declared namespaces at kintsugi --ci
//! resolution altitude — so downstream `in @foo/bar` imports resolve
//! and count_unresolved_imports returns 0.
//!
//! Pre-GREEN: `spectral @foo/bar` head is NOT recognized by
//! collect_declared_namespaces (only glass/prism/grammar recognized);
//! downstream `in @foo/bar` counts as unresolved.
//!
//! The self-migration architecture (Mara §8) requires this admittance
//! BEFORE Mara declares `@kintsugi/fracture/relocate` (PREREQ-2) as a
//! spectral-shaped species. Without PREREQ-1, PREREQ-2 can't compile
//! its declaration form.

use std::path::PathBuf;
use std::process::{Command, Output};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn mirror_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_mirror"))
}

/// Create a fixture dir at std::env::temp_dir()/mirror-prereq1-<pid>/
/// containing two shards — one `spectral @test_prereq1/head { ... }`
/// declaration + one consumer that imports it — then run
/// `mirror kintsugi --ci --out @data/json` on the fixture and return
/// the raw Output.
///
/// Fixture uses PID + nanos so parallel test runs don't collide;
/// caller may leak-clean via std::fs::remove_dir_all at end.
fn kintsugi_ci_on_spectral_fixture() -> (Output, PathBuf) {
    let mut dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    dir.push(format!("mirror-prereq1-{}-{}", std::process::id(), ts));
    std::fs::create_dir_all(&dir).expect("create fixture dir");

    // File A — declares `spectral @test_prereq1/head { ... }`.
    // Pre-GREEN: collect_declared_namespaces skips this line (only
    // glass/prism/grammar prefixes recognized); post-GREEN,
    // @test_prereq1/head enters the declared set.
    std::fs::write(
        dir.join("a_head.mirror"),
        "spectral @test_prereq1/head {\n  focus foo\n  project foo\n  split foo\n  shift foo\n  settle foo\n}\n\ntype foo = ref\n\nout foo\n",
    )
    .expect("write a_head");

    // File B — imports @test_prereq1/head. Pre-GREEN: import is
    // unresolved (head not declared). Post-GREEN: resolves cleanly.
    std::fs::write(
        dir.join("b_consumer.mirror"),
        "in @test_prereq1/head\n\nprism @test_prereq1/consumer {\n  focus bar\n  project bar\n  split bar\n  shift bar\n  settle bar\n}\n\ntype bar = ref\n\nout bar\n",
    )
    .expect("write b_consumer");

    let out = Command::new(mirror_bin())
        .current_dir(repo_root())
        .arg("kintsugi")
        .arg("--ci")
        .arg("--out")
        .arg("@data/json")
        .arg(dir.to_str().expect("utf-8 tempdir"))
        .output()
        .expect("mirror kintsugi --ci --out @data/json");

    (out, dir)
}

// === T1: spectral declaration head is recognized post-PREREQ-1 =========
//
// The primary lock. Post-GREEN: the corpus kintsugi --ci envelope's
// aggregate `verdict` field is `success` and `dark_count` is 0,
// because `@test_prereq1/head` is declared (via `spectral`) AND
// imported (via `in`) — the substrate closes cleanly.
//
// Pre-GREEN: `spectral @test_prereq1/head` is skipped by
// collect_declared_namespaces; the `in @test_prereq1/head` in b_consumer
// counts as unresolved; corpus verdict is `partial` with dark_count > 0.

#[test]
fn t01_spectral_head_recognized_by_collect_declared_namespaces() {
    let (out, fixture_dir) = kintsugi_ci_on_spectral_fixture();

    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    // Clean the fixture eagerly to avoid leaking temp dirs across runs.
    let _ = std::fs::remove_dir_all(&fixture_dir);

    assert!(
        out.status.success(),
        "T1: `mirror kintsugi --ci --out @data/json` on the spectral fixture must exit 0; \
         stdout=<{stdout}> stderr=<{stderr}>",
    );

    let v: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("T1: stdout must be valid JSON (kintsugi --ci --out @data/json); {e} — stdout=<{stdout}>"));

    // The b_consumer's per-file record isolates the effect. a_head
    // fails other pact predicates (missing `in @` ancestry,
    // path_matches_namespace against /tmp) that persist regardless of
    // PREREQ-1. b_consumer's ONLY variable is whether @test_prereq1/head
    // resolves — which flips solely on the collect_declared_namespaces
    // 3-line delta.
    let per_file = v["per_file"].as_array().unwrap_or_else(|| {
        panic!("T1: envelope must carry `per_file` array (corpus mode); got: {v}")
    });

    let b_consumer_record = per_file
        .iter()
        .find(|r| {
            r["path"]
                .as_str()
                .map(|p| p.ends_with("b_consumer.mirror"))
                .unwrap_or(false)
        })
        .unwrap_or_else(|| panic!("T1: per_file must include b_consumer.mirror; got: {v}"));

    let b_dark = b_consumer_record["dark_count"]
        .as_u64()
        .unwrap_or(u64::MAX);
    let b_verdict = b_consumer_record["verdict"].as_str().unwrap_or("<absent>");

    assert_eq!(
        b_dark, 0,
        "T1: b_consumer.mirror dark_count MUST be 0 post-PREREQ-1. Pre-GREEN: \
         dark_count=1 because `in @test_prereq1/head` is unresolved —\
         collect_declared_namespaces skips `spectral` prefix. Locks the 3-line delta \
         at bootstrap/src/lib.rs:~2140. b_consumer record: {b_consumer_record}"
    );

    assert_eq!(
        b_verdict, "success",
        "T1: b_consumer.mirror verdict MUST be `success` post-PREREQ-1 (its ONLY dark \
         source — the import — resolves cleanly). Pre-GREEN: verdict=`failure` because \
         @test_prereq1/head is unresolved. b_consumer record: {b_consumer_record}"
    );
}
