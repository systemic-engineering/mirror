//! Phase A RED — spawn-parent-family-lift.
//!
//! Per Mara `docs/math/spawn/spawn-as-loop-monad.md` §1.3 + Seam audit
//! `docs/audits/2026-07-02-seam-spawn-as-loop-monad.md` §9: additive
//! extension of `shards/loop.mirror` with four new action signatures at
//! `@loop` family-root altitude so `@spawn <= @loop` species-of
//! inheritance is clean.
//!
//! Actions being added:
//!   - advance(state, input) → moi(state)
//!         The tick that consumes budget by one and advances the peer's
//!         wave function by one bind step per Mara §2.2 substrate-honest
//!         right-identity law.
//!
//!   - halt(state) → imperfect<value, exhausted, ref>
//!         Terminal projection: returns the peer's value at halt or
//!         `exhausted` when budget hit zero without target-reach or
//!         kintsugi convergence per Mara §3.1 halting theorem.
//!
//!   - budget_of(state) → ref
//!         Extracts remaining reduction budget. Typed ref per
//!         `[[feedback-no-bare-types]]`; underlying carrier constrained
//!         to ℕ by substrate admission-rules per Mara §3.1.
//!
//!   - trajectory_of(state) → ref
//!         Extracts the crystallized trajectory (blob-chain) the peer
//!         has walked. Content-addressed history witness for the
//!         un-cite-ability theorem's composition with @spawn per Mara
//!         §8.4 candidate #133.
//!
//! **RED phase**: `shards/loop.mirror` does not yet declare these
//! actions at family-root altitude. Text-check tests fail by asserting
//! their presence. GREEN adds the substrate-decl action declarations
//! per Mara subagent (Phase B of the loop).
//!
//! Test strategy: text-level presence check on the shard file. The
//! substrate compiles at grammar altitude; parsing-level assertions
//! require the full substrate boot cycle. Text presence is the
//! smallest-tick RED that verifies the family-root lift landed without
//! over-scoping into grammar compilation.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_loop_shard() -> String {
    let path = repo_root().join("shards/loop.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/loop.mirror at {:?}: {}", path, e))
}

#[test]
fn loop_shard_declares_advance_action() {
    let content = read_loop_shard();
    assert!(
        content.contains("advance("),
        "@loop must declare `advance` action for tick-with-input per Mara §1.3 + Seam §9"
    );
}

#[test]
fn loop_shard_declares_halt_action() {
    let content = read_loop_shard();
    assert!(
        content.contains("halt("),
        "@loop must declare `halt` action returning imperfect<value, exhausted, ref>"
    );
}

#[test]
fn loop_shard_declares_budget_of_action() {
    let content = read_loop_shard();
    assert!(
        content.contains("budget_of("),
        "@loop must declare `budget_of` action extracting remaining ticks (typed ref, not bare u32)"
    );
}

#[test]
fn loop_shard_declares_trajectory_of_action() {
    let content = read_loop_shard();
    assert!(
        content.contains("trajectory_of("),
        "@loop must declare `trajectory_of` action extracting crystallized trajectory (blob-chain)"
    );
}
