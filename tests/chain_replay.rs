//! Chain-as-shatter end-to-end test.
//!
//! This test asserts the load-bearing invariant of the chain-as-shatter
//! recognition: **replaying a captured chain against the same input is
//! bit-for-bit identical to running the live pipeline**.
//!
//! We don't compute. We crystallize. This test is the receipt that the
//! architecture is structurally honest at the runtime layer.
//!
//! The test runs the `mirror` binary as a subprocess three ways:
//!
//!   1. Live pipeline: `mirror ai abyss <file> | mirror ai pathfinder -`
//!   2. Capture pipeline: same, plus `--capture=<chain.shatter>` on the
//!      last stage. Writes a chain-shatter file to disk.
//!   3. Replay: `mirror replay <chain.shatter> <file>` re-runs the chain
//!      against the same input.
//!
//! Assertions:
//!   - The chain.shatter file is human-readable and contains `chain: abyss |> pathfinder`.
//!   - The replay output equals the live pipeline output, byte-for-byte.
//!   - Re-running the whole loop produces the same bytes (determinism).

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn mirror_bin() -> PathBuf {
    // cargo provides CARGO_BIN_EXE_<name> automatically during integration
    // tests. This is the canonical path to the compiled `mirror` binary and
    // works regardless of where CARGO_TARGET_DIR lives.
    PathBuf::from(env!("CARGO_BIN_EXE_mirror"))
}

fn run_live_pipeline(input_file: &str) -> Vec<u8> {
    let bin = mirror_bin();

    // Stage 1: mirror ai abyss <input>
    let stage1 = Command::new(&bin)
        .args(["ai", "abyss", input_file])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn stage 1");

    // Stage 2: mirror ai pathfinder - (reads stdin from stage 1)
    let mut stage2 = Command::new(&bin)
        .args(["ai", "pathfinder", "-"])
        .stdin(stage1.stdout.expect("stage 1 stdout"))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn stage 2");

    let mut out = Vec::new();
    if let Some(mut so) = stage2.stdout.take() {
        std::io::copy(&mut so, &mut out).expect("read stage 2 stdout");
    }
    let _ = stage2.wait();
    out
}

fn run_capture_pipeline(input_file: &str, capture_path: &str) -> Vec<u8> {
    let bin = mirror_bin();

    let stage1 = Command::new(&bin)
        .args(["ai", "abyss", input_file])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn stage 1");

    let mut stage2 = Command::new(&bin)
        .args([
            "ai",
            "pathfinder",
            "-",
            &format!("--capture={}", capture_path),
        ])
        .stdin(stage1.stdout.expect("stage 1 stdout"))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn stage 2");

    let mut out = Vec::new();
    if let Some(mut so) = stage2.stdout.take() {
        std::io::copy(&mut so, &mut out).expect("read stage 2 stdout");
    }
    let _ = stage2.wait();
    out
}

fn run_replay(shatter_file: &str, input_file: &str) -> Vec<u8> {
    let bin = mirror_bin();
    let out = Command::new(&bin)
        .args(["replay", shatter_file, input_file])
        .output()
        .expect("failed to run replay");
    out.stdout
}

#[test]
fn chain_replay_matches_live_pipeline() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let input_file = format!("{}/boot/00-form.mirror", manifest);

    // Write the chain-shatter capture to a tempfile under /tmp.
    let capture_path = "/tmp/mirror-chain-replay-test.shatter";
    let _ = std::fs::remove_file(capture_path);

    // 1. Run the capture pipeline. Writes the .shatter file AND produces
    //    the live output on stdout.
    let live_out = run_capture_pipeline(&input_file, capture_path);
    assert!(
        !live_out.is_empty(),
        "live pipeline output must not be empty"
    );

    // 2. The .shatter file must exist and be readable.
    let shatter_text = std::fs::read_to_string(capture_path)
        .expect("chain-shatter file must exist after --capture");

    // 3. The .shatter file is human-readable and contains the chain line.
    assert!(
        shatter_text.contains("chain: abyss |> pathfinder"),
        "chain-shatter must contain the chain line, got:\n{}",
        shatter_text
    );
    assert!(
        shatter_text.contains("input:"),
        "chain-shatter must contain an input: line"
    );
    assert!(
        shatter_text.contains("output:"),
        "chain-shatter must contain an output: line"
    );

    // 4. Replay the chain against the same input.
    let replayed_out = run_replay(capture_path, &input_file);

    // 5. THE INVARIANT: replay is bit-for-bit identical to the live pipeline.
    assert_eq!(
        live_out,
        replayed_out,
        "replay output must be byte-identical to live pipeline output.\n\
         live: {:?}\n\
         replay: {:?}",
        String::from_utf8_lossy(&live_out),
        String::from_utf8_lossy(&replayed_out),
    );

    // 6. Re-run the whole loop. Must produce the same bytes. Determinism.
    let live_out_2 = run_live_pipeline(&input_file);
    assert_eq!(
        live_out, live_out_2,
        "live pipeline must be deterministic across runs"
    );

    let replayed_out_2 = run_replay(capture_path, &input_file);
    assert_eq!(
        replayed_out, replayed_out_2,
        "replay must be deterministic across runs"
    );

    // 7. Clean up.
    let _ = std::fs::remove_file(capture_path);

    // Write a small confirmation to stderr so `cargo test -- --nocapture`
    // shows the artifact.
    let _ = writeln!(
        std::io::stderr(),
        "\nchain_replay_matches_live_pipeline: PASS\n  chain-shatter: {}\n  live bytes: {}\n  replay bytes: {}\n  diff: 0",
        shatter_text.lines().next().unwrap_or(""),
        live_out.len(),
        replayed_out.len(),
    );
}
