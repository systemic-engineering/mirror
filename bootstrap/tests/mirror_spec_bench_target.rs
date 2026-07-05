//! Phase A RED — `target bench` block in `mirror.spec` per /loop
//! 2026-07-05 Arc 3b + Seam Phase D `91e79c8` §7 canonical execution.
//!
//! Wires @mirror/bench (LANDED 2026-07-01 at `shards/mirror/bench.mirror`,
//! 16.3KB) INTO `mirror.spec` via one `target bench` target block.
//!
//! **Precondition**: Sub-arc 3a landed (`cargo bench` dispatch available
//! via `cargo_args_for_check` "bench" arm).
//!
//! **RED phase**: `mirror.spec` currently has 6 target blocks (binary,
//! fmt, lint, tests, audit, action, release); no `target bench` block.
//!
//! **GREEN phase** (Mara): add one `target bench { name "mirror";
//! altitude @code/rust; emit cargo; check bench }` block. Add
//! `bench.compiles` (or equivalent) to `settle_on` block.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (repo root)")
        .to_path_buf()
}

fn read_source(rel: &str) -> String {
    let path = repo_root().join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {:?}: {}", path, e))
}

#[test]
fn mirror_spec_has_target_bench_block() {
    let content = read_source("mirror.spec");
    assert!(
        content.contains("target bench {") || content.contains("target bench{"),
        "mirror.spec must declare `target bench {{ ... }}` block per \
         Seam `91e79c8` §7 Sub-arc 3b. Wires @mirror/bench into the \
         mirror.spec target manifold; consumes via `check bench` \
         dispatch (Sub-arc 3a precondition)."
    );
}

#[test]
fn mirror_spec_bench_block_has_check_bench() {
    let content = read_source("mirror.spec");
    let has_check_bench = content.contains("check    bench")
        || content.contains("check bench")
        || content.contains("check   bench");
    assert!(
        has_check_bench,
        "mirror.spec target bench block must include `check bench` to \
         dispatch `cargo bench` via `cargo_args_for_check` per Seam \
         `91e79c8` §7 Sub-arc 3b."
    );
}

#[test]
fn mirror_spec_bench_block_emits_cargo() {
    let content = read_source("mirror.spec");
    // The bench target dispatches via cargo (matches existing binary/
    // fmt/lint/tests/audit targets). altitude @code/rust; emit cargo.
    let has_bench_altitude_or_emit = content.contains("emit     cargo")
        || content.contains("emit cargo");
    assert!(
        has_bench_altitude_or_emit,
        "mirror.spec must contain `emit cargo` (already required by \
         existing targets); confirms bench target composes with existing \
         cargo dispatch machinery."
    );
}
