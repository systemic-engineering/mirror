//! Actor init: scaffold an identity repo from a role template.

use std::path::Path;

// Thread-local override for the git binary path. Tests inject a non-existent
// path here to exercise spawn-failure branches without touching process-wide PATH.
thread_local! {
    static GIT_BIN_OVERRIDE: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

/// Return the git binary to use for subprocess calls.
fn git_bin() -> String {
    GIT_BIN_OVERRIDE.with(|ov| ov.borrow().clone().unwrap_or_else(|| "git".to_string()))
}

/// Run a process command and return its output, mapping spawn errors to a string.
fn run_cmd(cmd: &mut std::process::Command, context: &str) -> Result<std::process::Output, String> {
    cmd.output().map_err(|e| format!("{context}: {e}"))
}

/// Derive the actor name from a path, stripping a leading dot.
fn actor_name(path: &Path) -> Result<&str, String> {
    let raw = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "cannot derive actor name from path".to_string())?;
    let name = raw.trim_start_matches('.');
    if name.is_empty() {
        return Err("actor name is empty".into());
    }
    Ok(name)
}

/// Run `git init` in `dir`.
fn git_init(dir: &Path) -> Result<(), String> {
    let output = run_cmd(
        std::process::Command::new(git_bin())
            .args(["init"])
            .current_dir(dir),
        "git init",
    )?;
    if !output.status.success() {
        return Err(format!(
            "git init failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

/// Run `git commit -m <msg> --no-verify` in `dir`.
fn git_commit(dir: &Path, msg: &str) -> Result<(), String> {
    let output = run_cmd(
        std::process::Command::new(git_bin())
            .args(["commit", "-m", msg, "--no-verify"])
            .current_dir(dir),
        "git commit",
    )?;
    if !output.status.success() {
        return Err(format!(
            "git commit failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

/// Create the actor directory structure and write the grammar file.
fn scaffold_files(path: &Path, name: &str, role: &str) -> Result<(), String> {
    std::fs::create_dir_all(path).map_err(|e| format!("create dir: {e}"))?;
    std::fs::create_dir_all(path.join(".conversation"))
        .map_err(|e| format!("create .conversation: {e}"))?;
    std::fs::create_dir_all(path.join("workspace"))
        .map_err(|e| format!("create workspace: {e}"))?;
    let grammar = role_template(name, role);
    std::fs::write(path.join("main.conv"), &grammar)
        .map_err(|e| format!("write main.conv: {e}"))?;
    Ok(())
}

/// Configure git user identity.
fn git_configure_identity(dir: &Path, name: &str) -> Result<(), String> {
    for (key, val) in [
        ("user.name", name),
        ("user.email", &format!("{name}@systemic.engineer")),
    ] {
        run_cmd(
            std::process::Command::new(git_bin())
                .args(["config", "--local", key, val])
                .current_dir(dir),
            "git config",
        )?;
    }
    Ok(())
}

/// Stage all files and make the initial commit.
fn git_stage_and_commit(dir: &Path, msg: &str) -> Result<(), String> {
    run_cmd(
        std::process::Command::new(git_bin())
            .args(["add", "-A"])
            .current_dir(dir),
        "git add",
    )?;
    git_commit(dir, msg)
}

/// Initialize an actor identity repo.
pub fn init(path: &Path, role: &str) -> Result<(), String> {
    let name = actor_name(path)?;
    if path.exists() {
        return Err(format!("{} already exists", path.display()));
    }
    scaffold_files(path, name, role)?;
    git_init(path)?;
    git_configure_identity(path, name)?;
    let msg = format!("🌱 init: {name} — {role}");
    git_stage_and_commit(path, &msg)?;
    Ok(())
}

fn role_template(name: &str, role: &str) -> String {
    match role {
        "qa" => format!(
            r#"grammar @{name} {{
  type = signal | observation | fix | report
  type signal = drift | regression | coverage | pressure
  type observation = test | build | lint | type_check
  type fix = patch | refactor | test_fix
  type report = summary | finding | recommendation
}}

in @ca
in @ci

out {name} {{
  observe {{}}
  fix {{}}
  report {{}}
}}
"#
        ),
        "maintenance" => format!(
            r#"grammar @{name} {{
  type = signal | observation | action
  type signal = drift | stale | dependency
  type observation = lint | format | update
  type action = fix | notify | log
}}

in @ca
in @ci

out {name} {{
  observe {{}}
  maintain {{}}
}}
"#
        ),
        _ => format!(
            r#"grammar @{name} {{
  type = signal | observation | action
}}

in @ca

out {name} {{
  observe {{}}
}}
"#
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- actor_name ---

    #[test]
    fn actor_name_strips_leading_dot() {
        assert_eq!(
            actor_name(std::path::Path::new("/tmp/.keel")).unwrap(),
            "keel"
        );
    }

    #[test]
    fn actor_name_no_leading_dot() {
        assert_eq!(
            actor_name(std::path::Path::new("/tmp/keel")).unwrap(),
            "keel"
        );
    }

    #[test]
    fn actor_name_empty_after_strip_errors() {
        let err = actor_name(std::path::Path::new("/tmp/...")).unwrap_err();
        assert!(err.contains("actor name is empty"), "got: {err}");
    }

    #[test]
    fn actor_name_no_file_name_component_errors() {
        // ".." has no file_name() — triggers the ok_or_else closure
        let err = actor_name(std::path::Path::new("..")).unwrap_err();
        assert!(err.contains("cannot derive actor name"), "got: {err}");
    }

    // --- run_cmd ---

    #[test]
    fn run_cmd_spawn_failure_formats_error() {
        let dir = tempfile::tempdir().unwrap();
        let err = run_cmd(
            std::process::Command::new("/nonexistent/binary/that/does/not/exist")
                .current_dir(dir.path()),
            "test context",
        )
        .unwrap_err();
        assert!(err.contains("test context"), "got: {err}");
    }

    // --- spawn-failure tests (via thread-local git binary override) ---

    #[test]
    fn git_init_spawn_failure() {
        GIT_BIN_OVERRIDE.with(|ov| *ov.borrow_mut() = Some("/nonexistent/git".to_string()));
        let dir = tempfile::tempdir().unwrap();
        let err = git_init(dir.path()).unwrap_err();
        GIT_BIN_OVERRIDE.with(|ov| *ov.borrow_mut() = None);
        assert!(err.contains("git init"), "got: {err}");
    }

    #[test]
    fn git_commit_spawn_failure() {
        GIT_BIN_OVERRIDE.with(|ov| *ov.borrow_mut() = Some("/nonexistent/git".to_string()));
        let dir = tempfile::tempdir().unwrap();
        let err = git_commit(dir.path(), "msg").unwrap_err();
        GIT_BIN_OVERRIDE.with(|ov| *ov.borrow_mut() = None);
        assert!(err.contains("git commit"), "got: {err}");
    }

    #[test]
    fn git_configure_identity_spawn_failure() {
        GIT_BIN_OVERRIDE.with(|ov| *ov.borrow_mut() = Some("/nonexistent/git".to_string()));
        let dir = tempfile::tempdir().unwrap();
        let err = git_configure_identity(dir.path(), "tester").unwrap_err();
        GIT_BIN_OVERRIDE.with(|ov| *ov.borrow_mut() = None);
        assert!(err.contains("git config"), "got: {err}");
    }

    #[test]
    fn git_stage_and_commit_spawn_failure() {
        GIT_BIN_OVERRIDE.with(|ov| *ov.borrow_mut() = Some("/nonexistent/git".to_string()));
        let dir = tempfile::tempdir().unwrap();
        let err = git_stage_and_commit(dir.path(), "msg").unwrap_err();
        GIT_BIN_OVERRIDE.with(|ov| *ov.borrow_mut() = None);
        assert!(err.contains("git add"), "got: {err}");
    }

    // --- git_init ---

    #[test]
    fn git_init_succeeds_in_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        git_init(dir.path()).unwrap();
        assert!(dir.path().join(".git").is_dir());
    }

    #[test]
    fn git_init_fails_when_git_dir_is_file() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(".git"), "not a dir").unwrap();
        let err = git_init(dir.path()).unwrap_err();
        assert!(err.contains("git init failed"), "got: {err}");
    }

    // --- git_commit ---

    #[test]
    fn git_commit_fails_with_nothing_to_commit() {
        let dir = tempfile::tempdir().unwrap();
        git_init(dir.path()).unwrap();
        for (key, val) in [("user.name", "test"), ("user.email", "test@test.com")] {
            std::process::Command::new("git")
                .args(["config", "--local", key, val])
                .current_dir(dir.path())
                .output()
                .unwrap();
        }
        let err = git_commit(dir.path(), "🌱 init: test — qa").unwrap_err();
        assert!(err.contains("git commit failed"), "got: {err}");
    }

    // --- scaffold_files ---

    #[cfg(unix)]
    #[test]
    fn scaffold_files_fails_on_unwritable_parent() {
        use std::os::unix::fs::PermissionsExt;
        let parent = tempfile::tempdir().unwrap();
        std::fs::set_permissions(parent.path(), std::fs::Permissions::from_mode(0o555)).unwrap();
        let actor_path = parent.path().join("myactor");
        let result = scaffold_files(&actor_path, "myactor", "qa");
        std::fs::set_permissions(parent.path(), std::fs::Permissions::from_mode(0o755)).unwrap();
        let err = result.unwrap_err();
        assert!(err.contains("create dir"), "got: {err}");
    }

    #[cfg(unix)]
    #[test]
    fn scaffold_files_fails_when_conversation_dir_blocked() {
        use std::os::unix::fs::PermissionsExt;
        let parent = tempfile::tempdir().unwrap();
        let actor_path = parent.path().join("myactor");
        std::fs::create_dir_all(&actor_path).unwrap();
        std::fs::set_permissions(&actor_path, std::fs::Permissions::from_mode(0o555)).unwrap();
        let result = scaffold_files(&actor_path, "myactor", "qa");
        std::fs::set_permissions(&actor_path, std::fs::Permissions::from_mode(0o755)).unwrap();
        let msg = result.unwrap_err();
        // Either .conversation or workspace creation fails (macOS: create_dir_all
        // on existing dir succeeds, then the subdirectory creation fails)
        assert!(msg.contains("create"), "got: {msg}");
    }

    #[test]
    fn scaffold_files_fails_when_workspace_is_file() {
        // Create workspace as a FILE — create_dir_all will fail (not a directory)
        let parent = tempfile::tempdir().unwrap();
        let actor_path = parent.path().join("myactor");
        std::fs::create_dir_all(actor_path.join(".conversation")).unwrap();
        // Create "workspace" as a plain FILE, not a directory
        std::fs::write(actor_path.join("workspace"), "not a dir").unwrap();
        let result = scaffold_files(&actor_path, "myactor", "qa");
        let msg = result.unwrap_err();
        assert!(msg.contains("create workspace"), "got: {msg}");
    }

    #[cfg(unix)]
    #[test]
    fn scaffold_files_fails_when_write_blocked() {
        use std::os::unix::fs::PermissionsExt;
        let parent = tempfile::tempdir().unwrap();
        let actor_path = parent.path().join("myactor");
        std::fs::create_dir_all(actor_path.join(".conversation")).unwrap();
        std::fs::create_dir_all(actor_path.join("workspace")).unwrap();
        std::fs::set_permissions(&actor_path, std::fs::Permissions::from_mode(0o555)).unwrap();
        let result = scaffold_files(&actor_path, "myactor", "qa");
        std::fs::set_permissions(&actor_path, std::fs::Permissions::from_mode(0o755)).unwrap();
        let msg = result.unwrap_err();
        assert!(msg.contains("write main.conv"), "got: {msg}");
    }

    // --- git_stage_and_commit ---

    #[test]
    fn git_stage_and_commit_fails_with_nothing_to_commit() {
        let dir = tempfile::tempdir().unwrap();
        git_init(dir.path()).unwrap();
        git_configure_identity(dir.path(), "test").unwrap();
        let err = git_stage_and_commit(dir.path(), "🌱 init: test — qa").unwrap_err();
        assert!(err.contains("git commit failed"), "got: {err}");
    }

    // --- init error paths ---

    #[test]
    fn init_fails_bad_actor_name() {
        // ".." has no file_name() — actor_name returns Err, ? propagates it
        let err = init(std::path::Path::new(".."), "qa").unwrap_err();
        assert!(err.contains("cannot derive actor name"), "got: {err}");
    }

    #[cfg(unix)]
    #[test]
    fn init_fails_propagating_scaffold_error() {
        use std::os::unix::fs::PermissionsExt;
        // Make parent unwritable — scaffold_files fails, ? propagates the error
        let parent = tempfile::tempdir().unwrap();
        std::fs::set_permissions(parent.path(), std::fs::Permissions::from_mode(0o555)).unwrap();
        let actor_path = parent.path().join(".myactor");
        let result = init(&actor_path, "qa");
        std::fs::set_permissions(parent.path(), std::fs::Permissions::from_mode(0o755)).unwrap();
        let err = result.unwrap_err();
        assert!(err.contains("create dir"), "got: {err}");
    }

    // --- init ---

    #[test]
    fn init_creates_structure() {
        let dir = tempfile::tempdir().unwrap();
        let actor_path = dir.path().join(".testactor");
        init(&actor_path, "qa").unwrap();

        assert!(actor_path.join("main.conv").exists());
        assert!(actor_path.join(".conversation").is_dir());
        assert!(actor_path.join("workspace").is_dir());
        assert!(actor_path.join(".git").is_dir());
    }

    #[test]
    fn init_grammar_contains_name() {
        let dir = tempfile::tempdir().unwrap();
        let actor_path = dir.path().join(".keel");
        init(&actor_path, "qa").unwrap();

        let grammar = std::fs::read_to_string(actor_path.join("main.conv")).unwrap();
        assert!(grammar.contains("grammar @keel"));
        assert!(grammar.contains("out keel"));
        assert!(grammar.contains("in @ca"));
        assert!(grammar.contains("in @ci"));
    }

    #[test]
    fn init_maintenance_role() {
        let dir = tempfile::tempdir().unwrap();
        let actor_path = dir.path().join(".tender");
        init(&actor_path, "maintenance").unwrap();

        let grammar = std::fs::read_to_string(actor_path.join("main.conv")).unwrap();
        assert!(grammar.contains("grammar @tender"));
        assert!(grammar.contains("maintain"));
    }

    #[test]
    fn init_default_role() {
        let dir = tempfile::tempdir().unwrap();
        let actor_path = dir.path().join(".agent");
        init(&actor_path, "unknown").unwrap();

        let grammar = std::fs::read_to_string(actor_path.join("main.conv")).unwrap();
        assert!(grammar.contains("grammar @agent"));
        assert!(grammar.contains("in @ca"));
    }

    #[test]
    fn init_fails_if_exists() {
        let dir = tempfile::tempdir().unwrap();
        let actor_path = dir.path().join(".existing");
        std::fs::create_dir_all(&actor_path).unwrap();
        assert!(init(&actor_path, "qa").is_err());
    }

    #[test]
    fn init_git_commit_exists() {
        let dir = tempfile::tempdir().unwrap();
        let actor_path = dir.path().join(".keel");
        init(&actor_path, "qa").unwrap();

        let output = std::process::Command::new("git")
            .args(["log", "--oneline", "-1"])
            .current_dir(&actor_path)
            .output()
            .unwrap();
        let log = String::from_utf8_lossy(&output.stdout);
        assert!(log.contains("init: keel"), "got: {log}");
    }

    #[test]
    fn observe_after_init() {
        // Init an actor, then observe its own repo — should find the grammar deps
        let dir = tempfile::tempdir().unwrap();
        let actor_path = dir.path().join(".keel");
        init(&actor_path, "qa").unwrap();

        let deps = super::super::observe::scan_repo(&actor_path).unwrap();
        assert!(deps.iter().any(|d| d.name == "ca"), "should find @ca dep");
        assert!(deps.iter().any(|d| d.name == "ci"), "should find @ci dep");
    }
}
