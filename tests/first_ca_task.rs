//! First CA task: 33 branches → crystal.
//!
//! RED: This test records the messy state (37 branches, broken main) and
//! asserts the projected outcome — after merge, main compiles clean, tests
//! pass, and branch count drops to working set.
//!
//! GREEN: After the merge cascade completes, this test passes.

use std::process::Command;

fn project_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn mirror_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mirror"))
}

/// Count local git branches in this repo.
fn count_branches() -> usize {
    let output = Command::new("git")
        .args(["branch", "--list"])
        .current_dir(project_root())
        .output()
        .expect("git branch --list");
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().count()
}

/// The merge happened: branch count should be reduced.
/// Merged branches get deleted. Only main + active work branches remain.
#[test]
fn first_ca_task_branches_merged() {
    let branch_count = count_branches();
    // After merge + cleanup, we should have far fewer than the original 37.
    // main + this task branch + any unmerged stragglers.
    assert!(
        branch_count < 20,
        "expected < 20 branches after merge cleanup, got {}",
        branch_count
    );
}

/// After merge, `mirror ci .` should succeed (compile + holonomy measurement).
#[test]
fn first_ca_task_ci_succeeds() {
    let output = mirror_bin()
        .args(["ci", "."])
        .current_dir(project_root())
        .output()
        .expect("mirror ci .");

    assert!(
        output.status.success(),
        "mirror ci . should succeed after merge. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// After merge, `mirror crystal --oid` should produce a content address.
#[test]
fn first_ca_task_crystal_materializes() {
    let output = mirror_bin()
        .args(["crystal", "--oid"])
        .current_dir(project_root())
        .output()
        .expect("mirror crystal --oid");

    assert!(
        output.status.success(),
        "crystal should materialize on clean main. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.trim().is_empty(), "crystal OID should not be empty");
}
