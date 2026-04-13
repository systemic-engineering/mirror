//! End-to-end CLI tests for the mirror binary.

use std::process::Command;

/// Project root — tests run from the manifest directory.
fn project_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn mirror_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mirror"))
}

// ---------------------------------------------------------------------------
// Basic invocation
// ---------------------------------------------------------------------------

#[test]
fn no_args_shows_usage() {
    let output = mirror_bin().output().unwrap();
    assert!(!output.status.success(), "should exit non-zero with no args");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("usage:"),
        "should print usage on stderr: {}",
        stderr
    );
}

#[test]
fn help_falls_through_to_query() {
    // "help" is not a subcommand — it falls through to cmd_query,
    // which parses it as a bare atom and prints it.
    let output = mirror_bin().arg("help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("help"),
        "should parse 'help' as an atom: {}",
        stdout
    );
}

// ---------------------------------------------------------------------------
// Compile command
// ---------------------------------------------------------------------------

#[test]
fn compile_valid_mirror_file() {
    let dir = tempfile::TempDir::new().unwrap();
    let file = dir.path().join("test.mirror");
    std::fs::write(&file, "type greeting\n").unwrap();

    let output = mirror_bin()
        .arg("compile")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "compile should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let oid = stdout.trim();
    // OID is a hex SHA — 64 chars for SHA-256
    assert!(
        oid.len() == 64 && oid.chars().all(|c| c.is_ascii_hexdigit()),
        "stdout should be a hex OID, got: {}",
        oid
    );
}

#[test]
fn compile_invalid_syntax_fails() {
    let dir = tempfile::TempDir::new().unwrap();
    let file = dir.path().join("bad.mirror");
    std::fs::write(&file, "").unwrap(); // empty file

    let output = mirror_bin()
        .arg("compile")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "compile of empty file should fail"
    );
}

#[test]
fn compile_nonexistent_file_fails() {
    let output = mirror_bin()
        .arg("compile")
        .arg("does_not_exist.mirror")
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("No such file"),
        "should mention missing file: {}",
        stderr
    );
}

#[test]
fn compile_no_file_arg_shows_usage() {
    let output = mirror_bin().arg("compile").output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("usage:"),
        "should print compile usage: {}",
        stderr
    );
}

// ---------------------------------------------------------------------------
// Crystal command
// ---------------------------------------------------------------------------

#[test]
fn crystal_produces_shatter_file() {
    let dir = tempfile::TempDir::new().unwrap();
    let output_file = dir.path().join("out.shatter");

    let output = mirror_bin()
        .current_dir(project_root())
        .arg("crystal")
        .arg(output_file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "crystal should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // The output file should exist and contain mirror syntax
    let content = std::fs::read_to_string(&output_file).unwrap();
    assert!(
        content.contains("form mirror"),
        "shatter should contain mirror form: {}",
        &content[..content.len().min(200)]
    );
}

#[test]
fn crystal_prints_oid_to_stdout() {
    let dir = tempfile::TempDir::new().unwrap();
    let output_file = dir.path().join("crystal.shatter");

    let output = mirror_bin()
        .current_dir(project_root())
        .arg("crystal")
        .arg(output_file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let oid = stdout.trim();
    assert!(
        oid.len() == 64 && oid.chars().all(|c| c.is_ascii_hexdigit()),
        "crystal should print hex OID to stdout, got: {}",
        oid
    );
}

// ---------------------------------------------------------------------------
// Query command (the fallback — parse and print AST)
// ---------------------------------------------------------------------------

#[test]
fn query_parses_type_declaration() {
    let output = mirror_bin()
        .arg("type greeting")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "query should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("greeting"),
        "should contain parsed atom: {}",
        stdout
    );
}

#[test]
fn query_parses_ref() {
    let output = mirror_bin()
        .arg("@prism")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("@prism"),
        "should contain ref: {}",
        stdout
    );
}

// ---------------------------------------------------------------------------
// Compile boot files — the boot sequence must compile
// ---------------------------------------------------------------------------

#[test]
fn compile_boot_prism() {
    let output = mirror_bin()
        .current_dir(project_root())
        .arg("compile")
        .arg("boot/00-prism.mirror")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "boot/00-prism.mirror should compile, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn compile_boot_meta() {
    let output = mirror_bin()
        .current_dir(project_root())
        .arg("compile")
        .arg("boot/01-meta.mirror")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "boot/01-meta.mirror should compile, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn compile_boot_actor() {
    let output = mirror_bin()
        .current_dir(project_root())
        .arg("compile")
        .arg("boot/03-actor.mirror")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "boot/03-actor.mirror should compile, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn compile_boot_property() {
    let output = mirror_bin()
        .current_dir(project_root())
        .arg("compile")
        .arg("boot/05-property.mirror")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "boot/05-property.mirror should compile, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// ---------------------------------------------------------------------------
// Compile the shatter file — the crystal IS a .mirror file
// ---------------------------------------------------------------------------

#[test]
fn compile_shatter_produces_oid() {
    let output = mirror_bin()
        .current_dir(project_root())
        .arg("compile")
        .arg("mirror.shatter")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "mirror.shatter should compile, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let oid = stdout.trim();
    assert!(
        oid.len() == 64 && oid.chars().all(|c| c.is_ascii_hexdigit()),
        "should produce hex OID, got: {}",
        oid
    );
}

// ---------------------------------------------------------------------------
// Round-trip: crystal → compile → same OID
// ---------------------------------------------------------------------------

#[test]
fn crystal_compile_roundtrip_same_oid() {
    let dir = tempfile::TempDir::new().unwrap();
    let shatter_file = dir.path().join("roundtrip.shatter");

    // Step 1: materialize crystal
    let crystal_output = mirror_bin()
        .current_dir(project_root())
        .arg("crystal")
        .arg(shatter_file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(crystal_output.status.success());
    let crystal_oid = String::from_utf8_lossy(&crystal_output.stdout).trim().to_string();

    // Step 2: compile the crystal back
    let compile_output = mirror_bin()
        .arg("compile")
        .arg(shatter_file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(compile_output.status.success());
    let compile_oid = String::from_utf8_lossy(&compile_output.stdout).trim().to_string();

    // The OIDs must match — the compiler can read its own output
    assert_eq!(
        crystal_oid, compile_oid,
        "crystal OID and compile OID should match (round-trip)"
    );
}

// ---------------------------------------------------------------------------
// AI subcommand
// ---------------------------------------------------------------------------

#[test]
fn ai_no_model_shows_usage() {
    let output = mirror_bin().arg("ai").output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("usage:"),
        "should print ai usage: {}",
        stderr
    );
}

#[test]
fn ai_known_model_exits_zero() {
    let output = mirror_bin()
        .arg("ai")
        .arg("abyss")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("abyss"),
        "should mention model name: {}",
        stderr
    );
}

#[test]
fn ai_unknown_model_exits_zero() {
    // Unknown models fall through to "ai fate <model>"
    let output = mirror_bin()
        .arg("ai")
        .arg("unknown_model")
        .output()
        .unwrap();

    assert!(output.status.success());
}

// ---------------------------------------------------------------------------
// Focus subcommand — not yet implemented (would be `mirror focus <file>`)
// ---------------------------------------------------------------------------

#[test]
#[ignore = "focus subcommand not yet implemented — falls through to query"]
fn focus_on_mirror_file_prints_nodes() {
    let dir = tempfile::TempDir::new().unwrap();
    let file = dir.path().join("test.mirror");
    std::fs::write(&file, "type greeting\ntype farewell\n").unwrap();

    let output = mirror_bin()
        .arg("focus")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("greeting"));
    assert!(stdout.contains("farewell"));
}

#[test]
#[ignore = "focus subcommand not yet implemented — falls through to query"]
fn focus_on_boot_prism_succeeds() {
    let output = mirror_bin()
        .current_dir(project_root())
        .arg("focus")
        .arg("boot/00-prism.mirror")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "focus on boot prism should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
