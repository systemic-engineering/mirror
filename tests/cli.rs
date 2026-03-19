//! End-to-end tests for the conversation binary.

use std::io::Write;
use std::process::{Command, Stdio};

fn conversation_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_conversation"))
}

#[test]
fn no_args_prints_usage_and_exits_1() {
    let output = conversation_bin().output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("usage:"));
}

#[test]
fn missing_file_exits_1() {
    let output = conversation_bin().arg("nonexistent.conv").output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No such file"));
}

#[test]
#[cfg_attr(not(feature = "slow"), ignore = "needs --features slow")]
fn systemic_engineering_conv_produces_json() {
    let se_path = std::env::var("SYSTEMIC_ENGINEERING")
        .unwrap_or_else(|_| "/Users/alexwolf/dev/systemic.engineering".into());

    let output = conversation_bin()
        .arg("systemic.engineering.conv")
        .arg(format!("{}/blog", se_path))
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");

    // Output shape matches .conv declaration
    let pieces = &value["blog"]["pieces"];
    assert!(pieces["draft"].is_array());
    assert!(pieces["published"].is_array());
    assert!(pieces["archived"].is_array());

    let published = pieces["published"].as_array().unwrap();
    assert!(!published.is_empty(), "should have published pieces");

    for entry in published {
        assert!(entry["slug"].is_string());
        assert!(entry["headlines"].is_array());
    }
}

#[test]
fn inline_expression_produces_json() {
    let output = conversation_bin()
        .arg("-e")
        .arg("@json")
        .arg(".")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(value.is_object());
}

#[test]
fn fixture_json_conv_produces_output() {
    let output = conversation_bin()
        .arg("fixtures/json.conv")
        .arg(".")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(value.is_object());
}

#[test]
fn shell_evaluates_piped_expressions() {
    let mut child = conversation_bin()
        .arg("shell")
        .arg(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().unwrap();
        writeln!(stdin, "@json").unwrap();
        writeln!(stdin, "@json").unwrap();
    }

    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Each expression produces a JSON object; prompt prefixes each
    // Count opening braces that start JSON objects (after "conversation> ")
    let json_count = stdout.matches("\"@json\"").count();
    assert_eq!(json_count, 2, "expected two JSON outputs, got: {}", stdout);
}

#[test]
fn e_flag_missing_expression_exits_1() {
    let output = conversation_bin().arg("-e").output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("usage:"));
}

#[test]
fn e_flag_invalid_expression_exits_1() {
    // Unclosed block → parse error → from_source Err → exits 1
    let output = conversation_bin()
        .arg("-e")
        .arg("{ unclosed")
        .arg(".")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("conversation:"),
        "expected error on stderr: {}",
        stderr
    );
}

#[test]
fn shell_breaks_on_read_error() {
    // Invalid UTF-8 bytes cause io::Lines to return Err — shell breaks cleanly
    let mut child = conversation_bin()
        .arg("shell")
        .arg(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(b"\xFF\xFF\xFF\n").unwrap();
    }

    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "shell should exit cleanly after break"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("read error"),
        "expected read error on stderr: {}",
        stderr
    );
}

#[test]
fn shell_skips_empty_lines() {
    let mut child = conversation_bin()
        .arg("shell")
        .arg(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().unwrap();
        // Empty line is silently skipped; valid expression produces output
        writeln!(stdin, "").unwrap();
        writeln!(stdin, "@json").unwrap();
    }

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"@json\""),
        "should produce output: {}",
        stdout
    );
}

#[test]
fn loads_packages_from_env() {
    let dir = tempfile::TempDir::new().unwrap();
    // Packages live in a tier subdirectory (private/protected/public)
    let private_dir = dir.path().join("private");
    std::fs::create_dir(&private_dir).unwrap();
    std::fs::write(
        private_dir.join("@beam"),
        "grammar @beam {\n  type = process | module\n}\n",
    )
    .unwrap();

    let output = conversation_bin()
        .env("CONVERSATION_PACKAGES", dir.path())
        .arg("-e")
        .arg("@json")
        .arg(".")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn bad_packages_dir_falls_back() {
    // "private" exists as a file (not a dir) — walk_dir fails, falls back gracefully
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(dir.path().join("private"), "content").unwrap();

    let output = conversation_bin()
        .env("CONVERSATION_PACKAGES", dir.path())
        .arg("-e")
        .arg("@json")
        .arg(".")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "should fall back to empty resolve: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("packages:"), "should log error: {}", stderr);
}

#[test]
fn bad_package_source_falls_back() {
    let dir = tempfile::TempDir::new().unwrap();
    // Invalid .conv source inside a tier dir — to_namespace returns Err
    let private_dir = dir.path().join("private");
    std::fs::create_dir(&private_dir).unwrap();
    std::fs::write(private_dir.join("@bad"), ">>> invalid\n").unwrap();

    let output = conversation_bin()
        .env("CONVERSATION_PACKAGES", dir.path())
        .arg("-e")
        .arg("@json")
        .arg(".")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "should fall back to empty resolve: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("packages:"), "should log error: {}", stderr);
}

#[test]
fn test_subcommand_runs_property_tests() {
    let dir = tempfile::TempDir::new().unwrap();
    // Grammar package in tier dir
    let private_dir = dir.path().join("private");
    std::fs::create_dir(&private_dir).unwrap();
    std::fs::write(private_dir.join("@x"), "grammar @x {\n  type = a | b\n}\n").unwrap();

    // .conv file with test section
    let conv_file = dir.path().join("test.conv");
    std::fs::write(
        &conv_file,
        "grammar @t {\n  type = a | b\n}\n---\ntest \"types\" {\n  @t has a\n  @t has b\n}\n",
    )
    .unwrap();

    let output = conversation_bin()
        .env("CONVERSATION_PACKAGES", dir.path())
        .arg("test")
        .arg(conv_file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("PASS") || stdout.contains("pass") || stdout.contains("ok"),
        "expected pass indication: {}",
        stdout
    );
}

#[test]
fn test_subcommand_missing_arg() {
    let output = conversation_bin().arg("test").output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("usage:"));
}

#[test]
fn test_subcommand_missing_file() {
    let output = conversation_bin()
        .arg("test")
        .arg("nonexistent.conv")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No such file"));
}

#[test]
fn test_subcommand_bad_test_section() {
    // Unclosed test block → parse_test_section error → check_all returns Err
    let dir = tempfile::TempDir::new().unwrap();
    let conv_file = dir.path().join("bad_test.conv");
    std::fs::write(
        &conv_file,
        "grammar @g {\n  type = a\n}\n---\ntest \"unclosed {\n  @g has a\n",
    )
    .unwrap();

    let output = conversation_bin()
        .arg("test")
        .arg(conv_file.to_str().unwrap())
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("test:"), "expected test error: {}", stderr);
}

#[test]
fn test_subcommand_parse_error_still_runs_tests() {
    // Source has invalid spec but valid test section — still runs tests
    let dir = tempfile::TempDir::new().unwrap();
    let conv_file = dir.path().join("bad.conv");
    std::fs::write(
        &conv_file,
        ">>> invalid\n---\ntest \"t\" {\n  @g has a\n}\n",
    )
    .unwrap();

    let output = conversation_bin()
        .arg("test")
        .arg(conv_file.to_str().unwrap())
        .output()
        .unwrap();
    // Grammar won't be found but tests still execute
    assert!(!output.status.success()); // test fails (unknown domain)
}

#[test]
fn test_subcommand_empty_test_section() {
    let dir = tempfile::TempDir::new().unwrap();
    let conv_file = dir.path().join("empty.conv");
    std::fs::write(
        &conv_file,
        "grammar @g {\n  type = a\n}\n---\n# only comments\n",
    )
    .unwrap();

    let output = conversation_bin()
        .arg("test")
        .arg(conv_file.to_str().unwrap())
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("no tests"),
        "expected 'no tests': {}",
        stdout
    );
}

#[test]
fn test_subcommand_no_test_section() {
    let dir = tempfile::TempDir::new().unwrap();
    let conv_file = dir.path().join("nope.conv");
    std::fs::write(&conv_file, "grammar @g {\n  type = a\n}\n").unwrap();

    let output = conversation_bin()
        .arg("test")
        .arg(conv_file.to_str().unwrap())
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("no test section"));
}

#[test]
fn test_subcommand_reports_failures() {
    let dir = tempfile::TempDir::new().unwrap();
    let conv_file = dir.path().join("fail.conv");
    std::fs::write(
        &conv_file,
        "grammar @f {\n  type = a\n}\n---\ntest \"missing\" {\n  @f has nonexistent\n}\n",
    )
    .unwrap();

    let output = conversation_bin()
        .arg("test")
        .arg(conv_file.to_str().unwrap())
        .output()
        .unwrap();

    // Should exit with non-zero on test failure
    assert!(
        !output.status.success(),
        "should fail when tests fail, stdout: {}, stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn shell_reports_errors_and_continues() {
    let mut child = conversation_bin()
        .arg("shell")
        .arg(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().unwrap();
        // Unclosed block triggers parse error, then valid expression
        writeln!(stdin, "x {{").unwrap();
        writeln!(stdin, "@json").unwrap();
    }

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("error"),
        "should report error on stderr: {}",
        stderr
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"@json\""),
        "should still produce output for valid expression: {}",
        stdout
    );
}
