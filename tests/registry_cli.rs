use std::process::Command;

#[test]
fn mirror_registry_command_lists_resolved_forms() {
    let bin = env!("CARGO_BIN_EXE_mirror");
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let store_dir = std::env::temp_dir().join(format!("mirror-cli-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&store_dir);

    let output = Command::new(bin)
        .args([
            "registry",
            &format!("{}/boot", manifest_dir),
            "--store",
            store_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run mirror registry");
    assert!(
        output.status.success(),
        "mirror registry exited non-zero: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("@prism"),
        "stdout missing @prism: {}",
        stdout
    );
    assert!(stdout.contains("@meta"), "stdout missing @meta: {}", stdout);
    assert!(
        stdout.contains("@actor"),
        "stdout missing @actor: {}",
        stdout
    );
    assert!(
        stdout.contains("05-property") && stdout.contains("FAIL"),
        "stdout should mark 05-property as FAIL: {}",
        stdout
    );
    assert!(
        stdout.contains(store_dir.to_str().unwrap()),
        "stdout should mention the store mount path: {}",
        stdout
    );

    assert!(store_dir.join("objects").exists());
    assert!(store_dir.join("refs").exists());
    assert!(store_dir.join("refs").join("@prism").exists());
}
