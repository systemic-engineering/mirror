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
fn no_args_shows_help() {
    let output = mirror_bin().output().unwrap();
    assert!(
        !output.status.success(),
        "should exit non-zero with no args"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("mirror -- an honest compiler"),
        "should print help on stderr: {}",
        stderr
    );
    assert!(
        stderr.contains("usage:"),
        "should contain usage section: {}",
        stderr
    );
}

#[test]
fn help_subcommand_shows_help() {
    let output = mirror_bin().arg("help").output().unwrap();
    assert!(output.status.success(), "help should exit zero");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("mirror -- an honest compiler"),
        "help should print help text: {}",
        stdout
    );
    assert!(
        stdout.contains("focus"),
        "help should list focus: {}",
        stdout
    );
}

#[test]
fn dashdash_help_shows_help() {
    let output = mirror_bin().arg("--help").output().unwrap();
    assert!(output.status.success(), "--help should exit zero");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("mirror -- an honest compiler"),
        "--help should print help text: {}",
        stdout
    );
}

#[test]
fn per_command_help() {
    let output = mirror_bin().arg("compile").arg("--help").output().unwrap();
    assert!(output.status.success(), "compile --help should exit zero");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("compile"),
        "compile --help should describe compile: {}",
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

#[test]
fn crystal_oid_flag_prints_oid() {
    // spec.mirror exists in the project root, so --oid should print it
    let output = mirror_bin()
        .current_dir(project_root())
        .arg("crystal")
        .arg("--oid")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "crystal --oid should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let oid = stdout.trim();
    assert!(
        oid.len() == 64 && oid.chars().all(|c| c.is_ascii_hexdigit()),
        "crystal --oid should print hex OID, got: {}",
        oid
    );
}

// ---------------------------------------------------------------------------
// Query command (the fallback — parse and print AST)
// ---------------------------------------------------------------------------

#[test]
fn query_parses_type_declaration() {
    let output = mirror_bin().arg("type greeting").output().unwrap();

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
    let output = mirror_bin().arg("@prism").output().unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("@prism"), "should contain ref: {}", stdout);
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
        .arg("boot/01a-meta-actor.mirror")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "boot/01a-meta-actor.mirror should compile, stderr: {}",
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
    let crystal_oid = String::from_utf8_lossy(&crystal_output.stdout)
        .trim()
        .to_string();

    // Step 2: compile the crystal back
    let compile_output = mirror_bin()
        .arg("compile")
        .arg(shatter_file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(compile_output.status.success());
    let compile_oid = String::from_utf8_lossy(&compile_output.stdout)
        .trim()
        .to_string();

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
    let output = mirror_bin().arg("ai").arg("abyss").output().unwrap();

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
// Focus subcommand
// ---------------------------------------------------------------------------

#[test]
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

// ---------------------------------------------------------------------------
// mirror merge — the tool runs on itself
// ---------------------------------------------------------------------------

#[test]
fn merge_summary_on_self() {
    // Run `mirror merge main` (without --ai) on the mirror repo itself.
    // This produces a structural diff summary without modifying git state.
    let output = mirror_bin()
        .current_dir(project_root())
        .args(["merge", "main"])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // The merge summary should either:
    // 1. Succeed with a crystal delta (if we're ahead of main)
    // 2. Fail because we're already on main
    // Both are valid — the tool ran on itself.
    if output.status.success() {
        assert!(
            stdout.contains("crystal delta")
                || stdout.contains("added")
                || stdout.contains("removed")
                || stdout.contains("unchanged"),
            "merge summary must contain structural diff info.\nstdout: {}\nstderr: {}",
            stdout,
            stderr
        );
    } else {
        // If we're ON main, it should say so
        let combined = format!("{}{}", stdout, stderr);
        assert!(
            combined.contains("already on")
                || combined.contains("same branch")
                || combined.contains("main"),
            "merge failure must explain why.\nstdout: {}\nstderr: {}",
            stdout,
            stderr
        );
    }
}

#[test]
fn merge_help() {
    let output = mirror_bin()
        .current_dir(project_root())
        .args(["merge", "--help"])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("merge") && stdout.contains("crystal delta"),
        "merge help must describe the command.\nstdout: {}",
        stdout
    );
}

// ---------------------------------------------------------------------------
// git subcommands
// ---------------------------------------------------------------------------

#[test]
fn cli_git_refs() {
    let output = mirror_bin()
        .current_dir(project_root())
        .args(["git", "refs"])
        .output()
        .unwrap();
    assert!(output.status.success(), "git refs should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("main"),
        "git refs must show main: {}",
        stdout
    );
    assert!(
        stdout.contains("HEAD"),
        "git refs must show HEAD: {}",
        stdout
    );
}

#[test]
fn cli_git_tree() {
    let output = mirror_bin()
        .current_dir(project_root())
        .args(["git", "tree", "main"])
        .output()
        .unwrap();
    assert!(output.status.success(), "git tree should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("boot"),
        "git tree must show boot/: {}",
        stdout
    );
    assert!(
        stdout.contains("src"),
        "git tree must show src/: {}",
        stdout
    );
}

#[test]
fn cli_git_show_from_main() {
    let output = mirror_bin()
        .current_dir(project_root())
        .args(["git", "show", "main:boot/00-prism.mirror"])
        .output()
        .unwrap();
    assert!(output.status.success(), "git show should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("focus id"),
        "must read prism.mirror from main: {}",
        stdout
    );
}

#[test]
fn cli_git_log() {
    let output = mirror_bin()
        .current_dir(project_root())
        .args(["git", "log"])
        .output()
        .unwrap();
    assert!(output.status.success(), "git log should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty(), "git log must produce output");
}

#[test]
fn cli_git_diff() {
    let output = mirror_bin()
        .current_dir(project_root())
        .args(["git", "diff", "main", "HEAD"])
        .output()
        .unwrap();
    assert!(output.status.success(), "git diff should succeed");
}

#[test]
fn cli_git_no_subcommand_shows_help() {
    let output = mirror_bin()
        .current_dir(project_root())
        .args(["git"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git with no subcommand should show help"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("refs"),
        "git help must mention refs: {}",
        stdout
    );
}
