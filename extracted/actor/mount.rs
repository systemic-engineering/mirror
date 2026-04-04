//! Actor mount/unmount: workspace management.

use std::path::Path;

/// Mount a workspace into an actor's workspace directory.
pub fn mount(actor_home: &Path, workspace_path: &Path) -> Result<String, String> {
    let actor_home = actor_home
        .canonicalize()
        .map_err(|e| format!("actor home: {e}"))?;
    let workspace_path = workspace_path
        .canonicalize()
        .map_err(|e| format!("workspace: {e}"))?;

    let workspace_dir = actor_home.join("workspace");
    if !workspace_dir.exists() {
        return Err(format!(
            "{} has no workspace/ directory — run actor init first",
            actor_home.display()
        ));
    }

    let name = workspace_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "cannot derive workspace name from path".to_string())?;

    let link_path = workspace_dir.join(name);
    if link_path.exists() {
        return Err(format!("workspace '{}' already mounted", name));
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(&workspace_path, &link_path).map_err(|e| format!("symlink: {e}"))?;

    #[cfg(not(unix))]
    return Err("mount requires unix (symlinks)".into());

    Ok(name.to_string())
}

/// Unmount a workspace from an actor.
pub fn unmount(actor_home: &Path, name: &str) -> Result<(), String> {
    let workspace_dir = actor_home.join("workspace");
    let link_path = workspace_dir.join(name);

    if !link_path.exists() {
        return Err(format!("workspace '{}' not mounted", name));
    }

    // Verify it's a symlink before removing
    if !link_path
        .symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
    {
        return Err(format!("'{}' is not a symlink — refusing to remove", name));
    }

    std::fs::remove_file(&link_path).map_err(|e| format!("remove: {e}"))?;

    Ok(())
}

/// List mounted workspaces.
pub fn list(actor_home: &Path) -> Result<Vec<(String, std::path::PathBuf)>, String> {
    let workspace_dir = actor_home.join("workspace");
    if !workspace_dir.exists() {
        return Ok(vec![]);
    }

    let mut mounts = Vec::new();
    let entries = std::fs::read_dir(&workspace_dir).map_err(|e| format!("read workspace: {e}"))?;

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path
            .symlink_metadata()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
        {
            let name = entry.file_name().to_string_lossy().to_string();
            let target = std::fs::read_link(&path).unwrap_or_default();
            mounts.push((name, target));
        }
    }

    mounts.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(mounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_actor() -> (tempfile::TempDir, std::path::PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let actor_home = dir.path().join("actor");
        std::fs::create_dir_all(actor_home.join("workspace")).unwrap();
        std::fs::write(actor_home.join("main.conv"), "grammar @test { type = x }").unwrap();
        (dir, actor_home)
    }

    #[test]
    fn mount_creates_symlink() {
        let (_dir, actor_home) = setup_actor();
        let workspace = tempfile::tempdir().unwrap();
        std::fs::write(
            workspace.path().join("test.conv"),
            "grammar @app { type = y }",
        )
        .unwrap();

        let name = mount(&actor_home, workspace.path()).unwrap();
        assert_eq!(
            name,
            workspace.path().file_name().unwrap().to_str().unwrap()
        );

        let link = actor_home.join("workspace").join(&name);
        assert!(link.exists());
        assert!(link.symlink_metadata().unwrap().file_type().is_symlink());
    }

    #[test]
    fn mount_fails_if_already_mounted() {
        let (_dir, actor_home) = setup_actor();
        let workspace = tempfile::tempdir().unwrap();

        mount(&actor_home, workspace.path()).unwrap();
        assert!(mount(&actor_home, workspace.path()).is_err());
    }

    #[test]
    fn mount_fails_without_workspace_dir() {
        let dir = tempfile::tempdir().unwrap();
        let actor_home = dir.path().join("no-actor");
        std::fs::create_dir_all(&actor_home).unwrap();
        // No workspace/ dir

        let workspace = tempfile::tempdir().unwrap();
        assert!(mount(&actor_home, workspace.path()).is_err());
    }

    #[test]
    fn unmount_removes_symlink() {
        let (_dir, actor_home) = setup_actor();
        let workspace = tempfile::tempdir().unwrap();

        let name = mount(&actor_home, workspace.path()).unwrap();
        unmount(&actor_home, &name).unwrap();

        let link = actor_home.join("workspace").join(&name);
        assert!(!link.exists());
    }

    #[test]
    fn unmount_fails_if_not_mounted() {
        let (_dir, actor_home) = setup_actor();
        assert!(unmount(&actor_home, "nonexistent").is_err());
    }

    #[test]
    fn list_shows_mounted_workspaces() {
        let (_dir, actor_home) = setup_actor();
        let ws1 = tempfile::tempdir().unwrap();
        let ws2 = tempfile::tempdir().unwrap();

        mount(&actor_home, ws1.path()).unwrap();
        mount(&actor_home, ws2.path()).unwrap();

        let mounts = list(&actor_home).unwrap();
        assert_eq!(mounts.len(), 2);
    }

    #[test]
    fn list_empty_workspace() {
        let (_dir, actor_home) = setup_actor();
        let mounts = list(&actor_home).unwrap();
        assert!(mounts.is_empty());
    }

    #[test]
    fn mount_fails_if_actor_home_nonexistent() {
        let dir = tempfile::tempdir().unwrap();
        let actor_home = dir.path().join("does-not-exist");
        let workspace = tempfile::tempdir().unwrap();
        let err = mount(&actor_home, workspace.path()).unwrap_err();
        assert!(err.contains("actor home"), "got: {err}");
    }

    #[test]
    fn mount_fails_if_workspace_nonexistent() {
        let (_dir, actor_home) = setup_actor();
        let workspace = std::path::Path::new("/tmp/conv-test-nonexistent-workspace-xyz");
        let err = mount(&actor_home, workspace).unwrap_err();
        assert!(err.contains("workspace"), "got: {err}");
    }

    #[test]
    fn list_skips_non_symlink_entries() {
        let (_dir, actor_home) = setup_actor();
        // Place a real file in workspace/ — list should ignore it (return 0 mounts)
        std::fs::write(actor_home.join("workspace").join("not-a-link"), "data").unwrap();
        let mounts = list(&actor_home).unwrap();
        assert!(mounts.is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn list_fails_when_workspace_unreadable() {
        use std::os::unix::fs::PermissionsExt;
        let (_dir, actor_home) = setup_actor();
        let workspace_dir = actor_home.join("workspace");
        std::fs::set_permissions(&workspace_dir, std::fs::Permissions::from_mode(0o000)).unwrap();
        let result = list(&actor_home);
        std::fs::set_permissions(&workspace_dir, std::fs::Permissions::from_mode(0o755)).unwrap();
        assert!(result.is_err());
    }

    #[cfg(unix)]
    #[test]
    fn mount_fails_symlink_when_workspace_dir_unwritable() {
        use std::os::unix::fs::PermissionsExt;
        // Workspace dir exists (passes existence check) but is not writable — symlink fails.
        let (_dir, actor_home) = setup_actor();
        let workspace = tempfile::tempdir().unwrap();
        let workspace_dir = actor_home.join("workspace");
        std::fs::set_permissions(&workspace_dir, std::fs::Permissions::from_mode(0o555)).unwrap();
        let result = mount(&actor_home, workspace.path());
        std::fs::set_permissions(&workspace_dir, std::fs::Permissions::from_mode(0o755)).unwrap();
        let err = result.unwrap_err();
        assert!(err.contains("symlink"), "got: {err}");
    }

    #[cfg(unix)]
    #[test]
    fn unmount_fails_when_remove_blocked() {
        use std::os::unix::fs::PermissionsExt;
        // Mount a workspace, then make workspace/ unwritable so remove_file fails.
        let (_dir, actor_home) = setup_actor();
        let workspace = tempfile::tempdir().unwrap();
        let name = mount(&actor_home, workspace.path()).unwrap();
        let workspace_dir = actor_home.join("workspace");
        std::fs::set_permissions(&workspace_dir, std::fs::Permissions::from_mode(0o555)).unwrap();
        let result = unmount(&actor_home, &name);
        std::fs::set_permissions(&workspace_dir, std::fs::Permissions::from_mode(0o755)).unwrap();
        let err = result.unwrap_err();
        assert!(err.contains("remove"), "got: {err}");
    }

    #[test]
    fn mount_fails_when_workspace_has_no_name_component() {
        // The root path "/" canonicalizes to "/" which has no file_name() — ok_or_else fires.
        let (_dir, actor_home) = setup_actor();
        let err = mount(&actor_home, std::path::Path::new("/")).unwrap_err();
        assert!(err.contains("cannot derive"), "got: {err}");
    }

    #[test]
    fn unmount_refuses_non_symlink() {
        let (_dir, actor_home) = setup_actor();
        // Place a real file (not a symlink) in workspace/
        std::fs::write(actor_home.join("workspace").join("realfile"), "data").unwrap();
        let err = unmount(&actor_home, "realfile").unwrap_err();
        assert!(err.contains("not a symlink"), "got: {err}");
    }

    #[test]
    fn list_no_workspace_dir_returns_empty() {
        let dir = tempfile::tempdir().unwrap();
        let actor_home = dir.path().join("bare-actor");
        std::fs::create_dir_all(&actor_home).unwrap();
        // No workspace/ dir — list should return empty vec, not error
        let mounts = list(&actor_home).unwrap();
        assert!(mounts.is_empty());
    }

    #[test]
    fn mount_then_observe_finds_deps() {
        let (_dir, actor_home) = setup_actor();
        let workspace = tempfile::tempdir().unwrap();
        std::fs::write(
            workspace.path().join("app.conv"),
            "in @admin\ngrammar @app { type = page }",
        )
        .unwrap();

        let name = mount(&actor_home, workspace.path()).unwrap();

        // Observe the mounted workspace through the actor's workspace dir
        let mounted_path = actor_home.join("workspace").join(&name);
        let deps = crate::actor::observe::scan_repo(&mounted_path).unwrap();
        assert!(deps.iter().any(|d| d.name == "admin"));
    }
}
