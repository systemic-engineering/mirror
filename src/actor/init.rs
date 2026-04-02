//! Actor init: scaffold an identity repo from a role template.

use std::path::Path;

/// Initialize an actor identity repo.
pub fn init(_path: &Path, _role: &str) -> Result<(), String> {
    todo!("init not yet implemented")
}

fn role_template(_name: &str, _role: &str) -> String {
    todo!("role_template not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

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
