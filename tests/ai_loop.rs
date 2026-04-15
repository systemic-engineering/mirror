//! Integration tests for the AI loop: mirror ai <model> <file>

use std::path::Path;

#[test]
fn crystal_file_stops_immediately() {
    let dir = tempfile::TempDir::new().unwrap();
    let file = dir.path().join("crystal.mirror");
    std::fs::write(&file, "in @mirror\ntype x = a | b\n").unwrap();

    let result = mirror::ai::ai_loop(&file, 100).unwrap();
    assert_eq!(result.steps, 0, "crystal file should take 0 steps");
    assert_eq!(result.holonomy_start, 0.0);
    assert_eq!(result.holonomy_end, 0.0);
    assert_eq!(result.health, "crystal");
    assert!(result.models_used.is_empty());
}

#[test]
fn partial_file_reports_model() {
    let dir = tempfile::TempDir::new().unwrap();
    let file = dir.path().join("partial.mirror");
    // 'io' is unrecognized, generates parse warnings
    std::fs::write(
        &file,
        "in @mirror\nio tick(features) => imperfect\ntype x = a\n",
    )
    .unwrap();

    let result = mirror::ai::ai_loop(&file, 100).unwrap();
    assert!(
        result.holonomy_start > 0.0,
        "partial file should have holonomy > 0"
    );
    assert_eq!(result.steps, 1, "observe-only: one step");
    assert!(!result.models_used.is_empty(), "should report a model");
}

#[test]
fn real_boot_file_runs_without_crashing() {
    let boot_file = Path::new("boot/std/new.mirror");
    if !boot_file.exists() {
        // Skip if not in the right directory
        return;
    }

    let result = mirror::ai::ai_loop(boot_file, 100);
    assert!(
        result.is_ok(),
        "ai_loop on boot file should not crash: {:?}",
        result.err()
    );
}

#[test]
fn fate_grammar_runs_without_crashing() {
    let fate_file = Path::new("boot/std/fate.mirror");
    if !fate_file.exists() {
        return;
    }

    let result = mirror::ai::ai_loop(fate_file, 100);
    assert!(
        result.is_ok(),
        "ai_loop on fate.mirror should not crash: {:?}",
        result.err()
    );
    let output = result.unwrap();
    // fate.mirror has unrecognized keywords (io, property), so holonomy > 0
    assert!(output.holonomy_start > 0.0);
}
