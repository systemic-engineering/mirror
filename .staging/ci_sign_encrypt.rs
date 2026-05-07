//! Integration test: full CI pipeline using only environment variables.
//!
//! Exercises compile → sign → verify using `MIRROR_CI_SIGN_KEY` and
//! `MIRROR_CI_ENCRYPT_KEY` env vars only. No filesystem key dependencies.

use std::process::Command;

fn mirror_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mirror"))
}

/// Generate a fresh Ed25519 keypair, returning (private_pem, public_openssh).
fn generate_test_keypair() -> (String, String) {
    use ssh_key::{Algorithm, LineEnding, PrivateKey};
    let key = PrivateKey::random(&mut ssh_key::rand_core::OsRng, Algorithm::Ed25519)
        .expect("keygen should succeed");
    let pem = key.to_openssh(LineEnding::LF).expect("to_openssh");
    let pub_line = key.public_key().to_openssh().expect("pub to_openssh");
    (pem.to_string(), pub_line)
}

// ---------------------------------------------------------------------------
// Full pipeline: compile --sign → verify → tamper → verify fails
// ---------------------------------------------------------------------------

#[test]
fn ci_pipeline_compile_sign_verify_env_only() {
    let dir = tempfile::TempDir::new().unwrap();

    // 1. Generate test keypair
    let (private_pem, _public_ssh) = generate_test_keypair();

    // 2. Write private key to temp file (MIRROR_CI_SIGN_KEY accepts file path)
    let key_path = dir.path().join("ci_sign_key");
    std::fs::write(&key_path, &private_pem).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o600)).unwrap();
    }

    // 3. Write a simple .mirror file
    let mirror_file = dir.path().join("test.mirror");
    std::fs::write(&mirror_file, "type greeting\n").unwrap();

    // 4. Compile with --sign, using only env vars (no ~/.ssh dependency)
    let compile_output = mirror_bin()
        .arg("compile")
        .arg(mirror_file.to_str().unwrap())
        .arg("--sign")
        .env("MIRROR_CI_SIGN_KEY", key_path.to_str().unwrap())
        .env(
            "MIRROR_CI_ENCRYPT_KEY",
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPlaceholderEncryptKeyForCITest ci@test",
        )
        // Isolate from host keys
        .env("CONVERSATION_KEYS", "/nonexistent")
        .env("CONVERSATION_KEYS_PRIVATE", "")
        .env("CONVERSATION_KEYS_PUBLIC", "")
        .env_remove("HOME")
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&compile_output.stderr);
    assert!(
        compile_output.status.success(),
        "compile --sign should succeed, stderr: {}",
        stderr
    );

    // 5. Verify .shatter and .shatter.sig exist
    let shatter_path = dir.path().join("test.shatter");
    let sig_path = dir.path().join("test.shatter.sig");
    assert!(shatter_path.exists(), ".shatter should exist");
    assert!(sig_path.exists(), ".shatter.sig should exist");

    // 6. Verify the signature is valid SSH signature PEM
    let sig_content = std::fs::read_to_string(&sig_path).unwrap();
    assert!(
        sig_content.contains("BEGIN SSH SIGNATURE"),
        "sig should be SSH signature PEM"
    );

    // 7. Run `mirror verify` — should succeed
    let verify_output = mirror_bin()
        .arg("verify")
        .arg(shatter_path.to_str().unwrap())
        .env("MIRROR_CI_SIGN_KEY", key_path.to_str().unwrap())
        .env("CONVERSATION_KEYS", "/nonexistent")
        .env_remove("HOME")
        .output()
        .unwrap();

    let verify_stdout = String::from_utf8_lossy(&verify_output.stdout);
    let verify_stderr = String::from_utf8_lossy(&verify_output.stderr);
    assert!(
        verify_output.status.success(),
        "verify should succeed for untampered content, stdout: {}, stderr: {}",
        verify_stdout,
        verify_stderr
    );
    assert!(
        verify_stdout.contains("verified"),
        "stdout should contain 'verified': {}",
        verify_stdout
    );

    // 8. Tamper with the .shatter content
    std::fs::write(&shatter_path, "type TAMPERED\n").unwrap();

    // 9. Run `mirror verify` again — should FAIL
    let tampered_output = mirror_bin()
        .arg("verify")
        .arg(shatter_path.to_str().unwrap())
        .env("MIRROR_CI_SIGN_KEY", key_path.to_str().unwrap())
        .env("CONVERSATION_KEYS", "/nonexistent")
        .env_remove("HOME")
        .output()
        .unwrap();

    assert!(
        !tampered_output.status.success(),
        "verify should fail after tampering"
    );
    let tampered_stderr = String::from_utf8_lossy(&tampered_output.stderr);
    assert!(
        tampered_stderr.contains("verification failed"),
        "should report verification failure: {}",
        tampered_stderr
    );
}

// ---------------------------------------------------------------------------
// Verify that MIRROR_CI_ENCRYPT_KEY is resolvable from env
// ---------------------------------------------------------------------------

#[test]
fn ci_encrypt_key_env_is_visible() {
    // This test confirms the env var is picked up without filesystem keys.
    // The @encrypt filter itself is tested in unit tests; here we just confirm
    // the CI env path doesn't crash when set.
    let dir = tempfile::TempDir::new().unwrap();
    let mirror_file = dir.path().join("test.mirror");
    std::fs::write(&mirror_file, "type greeting\n").unwrap();

    // Compile WITHOUT --sign but WITH MIRROR_CI_ENCRYPT_KEY set
    // This should succeed — the encrypt key is only used if @encrypt filter fires
    let output = mirror_bin()
        .arg("compile")
        .arg(mirror_file.to_str().unwrap())
        .env(
            "MIRROR_CI_ENCRYPT_KEY",
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPlaceholderEncryptKeyForCITest ci@test",
        )
        .env("CONVERSATION_KEYS", "/nonexistent")
        .env_remove("HOME")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "compile should succeed with MIRROR_CI_ENCRYPT_KEY set, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// ---------------------------------------------------------------------------
// Compile without --sign produces no .sig file
// ---------------------------------------------------------------------------

#[test]
fn ci_compile_without_sign_no_sig() {
    let dir = tempfile::TempDir::new().unwrap();
    let mirror_file = dir.path().join("test.mirror");
    std::fs::write(&mirror_file, "type greeting\n").unwrap();

    let output = mirror_bin()
        .arg("compile")
        .arg(mirror_file.to_str().unwrap())
        .env("CONVERSATION_KEYS", "/nonexistent")
        .env_remove("HOME")
        .output()
        .unwrap();

    assert!(output.status.success());
    let sig_path = dir.path().join("test.shatter.sig");
    assert!(
        !sig_path.exists(),
        ".shatter.sig should NOT exist without --sign"
    );
}
