//! Actor mount/unmount: workspace management.

use std::path::Path;

/// Mount a workspace into an actor's workspace directory.
pub fn mount(actor_home: &Path, workspace_path: &Path) -> Result<String, String> {
    todo!("mount not yet implemented")
}

/// Unmount a workspace from an actor.
pub fn unmount(actor_home: &Path, name: &str) -> Result<(), String> {
    todo!("unmount not yet implemented")
}

/// List mounted workspaces.
pub fn list(actor_home: &Path) -> Result<Vec<(String, std::path::PathBuf)>, String> {
    todo!("list not yet implemented")
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
        std::fs::write(workspace.path().join("test.conv"), "grammar @app { type = y }").unwrap();

        let name = mount(&actor_home, workspace.path()).unwrap();
        assert_eq!(name, workspace.path().file_name().unwrap().to_str().unwrap());

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
