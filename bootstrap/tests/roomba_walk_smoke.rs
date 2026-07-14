//! End2end empirical test: `@roomba` walks the mirror substrate's OWN DAG
//! and records trajectory. Confirms the /loop end2end shipping per Alex
//! 2026-07-14 directive "we sharpen the /loop to an actual end2end run of
//! the roomba."
//!
//! This is a smoke test with `--nocapture` output: run via
//! `cargo test --test roomba_walk_smoke -- --nocapture` to see the full
//! trajectory printed to stdout.
//!
//! Substrate authority:
//! - Mara `9bbebd2` — @roomba canonical spec (Rung 10 substrate self-
//!   maintenance)
//! - Mara `e0a3e48` — @coherence species-shard (Foerster imperative
//!   operationalized)
//! - Reed `422076d` — @roomba + @coherence Rust runtime landing
//! - Alex 2026-07-14 in-transcript composition (@roomba walks → tension →
//!   @song → @kintsugi decides @knife | @peer at K+1)
//! - Alex 2026-07-14 in-transcript "ship it until unresolvable ambiguity"

use mirror::roomba::{summarize_trajectory, walk, WalkTermination};
use std::path::PathBuf;

fn mirror_root() -> PathBuf {
    // Walk from the mirror workspace root, not the bootstrap subdir.
    // Cargo runs tests with CARGO_MANIFEST_DIR = bootstrap/, so parent = mirror/.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR should have a parent")
        .to_path_buf()
}

#[test]
#[ignore = "slow: walks the full mirror substrate DAG; run with --ignored --nocapture"]
fn roomba_walks_mirror_substrate_end2end() {
    let root = mirror_root();
    println!("\n\n=== @roomba end2end empirical run ===\n");
    println!("substrate root: {}", root.display());
    println!("budget: 20 steps");
    println!("epsilon_pain: 0.1");
    println!();

    let trajectory = walk(&root, 20, 0.1);
    let summary = summarize_trajectory(&trajectory);
    print!("{}", summary);
    println!();

    // Substrate-honest assertions: minimum viable claim that walker ran.
    // The empirical observation (coherence climb, knife fires, tension
    // pattern) is for eyeballing; the test just asserts the walker did
    // *something* observable.
    assert!(
        trajectory.graph_node_count > 0,
        "substrate DAG should have >0 nodes"
    );
    assert!(
        !trajectory.steps.is_empty(),
        "walker should have taken at least one step"
    );
    assert!(
        matches!(
            trajectory.termination,
            WalkTermination::BudgetExhausted | WalkTermination::NoUnvisitedNeighbors
        ),
        "walker should terminate via budget or exhausted neighbors, got {:?}",
        trajectory.termination
    );

    println!("\n=== @roomba end2end empirical run: DONE ===\n");
}

#[test]
fn roomba_walks_substrate_at_least_one_step() {
    // Non-ignored quick smoke: fast walk with tiny budget to confirm the
    // integration surface compiles + links + executes without --nocapture.
    let root = mirror_root();
    let trajectory = walk(&root, 3, 0.5);
    assert!(
        trajectory.graph_node_count > 0,
        "substrate DAG has 0 nodes; expected the mirror repo to index"
    );
    assert!(trajectory.steps.len() >= 1);
    assert!(trajectory.steps.len() <= 3, "budget=3 respected");
}
