//! Actor status: list initialized actors.

use std::path::{Path, PathBuf};

/// Info about a discovered actor.
#[derive(Debug, Clone)]
pub struct ActorInfo {
    pub name: String,
    pub home: PathBuf,
    pub workspaces: Vec<String>,
    pub grammar_imports: Vec<String>,
}

/// Check if a directory is an actor identity repo.
pub fn is_actor_home(path: &Path) -> bool {
    path.join("main.conv").exists()
        && path.join(".conversation").is_dir()
        && path.join("workspace").is_dir()
}

/// Read actor info from an identity repo.
pub fn read_actor(path: &Path) -> Result<ActorInfo, String> {
    if !is_actor_home(path) {
        return Err(format!("{} is not an actor home", path.display()));
    }

    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .trim_start_matches('.')
        .to_string();

    // Read workspaces
    let workspaces = crate::actor::mount::list(path)
        .unwrap_or_default()
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    // Read grammar imports from main.conv
    let grammar_imports = read_grammar_imports(&path.join("main.conv"));

    Ok(ActorInfo {
        name,
        home: path.to_path_buf(),
        workspaces,
        grammar_imports,
    })
}

/// Extract `in @domain` imports from a .conv file.
fn read_grammar_imports(path: &Path) -> Vec<String> {
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    let mut imports = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(domain) = trimmed.strip_prefix("in @") {
            let domain = domain.trim();
            if !domain.is_empty() {
                imports.push(format!("@{domain}"));
            }
        }
    }
    imports.sort();
    imports
}

/// Discover actors from a list of candidate directories.
pub fn discover_actors(candidates: &[PathBuf]) -> Vec<ActorInfo> {
    let mut actors: Vec<ActorInfo> = candidates
        .iter()
        .filter(|p| is_actor_home(p))
        .filter_map(|p| read_actor(p).ok())
        .collect();
    actors.sort_by(|a, b| a.name.cmp(&b.name));
    actors
}

/// Get the default candidate directories (~/.<name> directories).
pub fn home_candidates() -> Vec<PathBuf> {
    let home = match std::env::var("HOME") {
        Ok(h) => PathBuf::from(h),
        Err(_) => return vec![],
    };

    let entries = match std::fs::read_dir(&home) {
        Ok(e) => e,
        Err(_) => return vec![],
    };

    entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with('.'))
                .unwrap_or(false)
        })
        .collect()
}

/// Format status output.
pub fn format_status(actors: &[ActorInfo]) -> String {
    if actors.is_empty() {
        return "no actors found".to_string();
    }

    let mut lines = Vec::new();
    lines.push(format!(
        "{:<12} {:<14} {}",
        "NAME", "WORKSPACES", "GRAMMARS"
    ));

    for actor in actors {
        let grammars = if actor.grammar_imports.is_empty() {
            "\u{2014}".to_string()
        } else {
            actor.grammar_imports.join(" ")
        };
        lines.push(format!(
            "{:<12} {:<14} {}",
            actor.name,
            actor.workspaces.len(),
            grammars
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_actor_home_true() {
        let dir = tempfile::tempdir().unwrap();
        let home = dir.path().join(".test");
        std::fs::create_dir_all(home.join(".conversation")).unwrap();
        std::fs::create_dir_all(home.join("workspace")).unwrap();
        std::fs::write(home.join("main.conv"), "grammar @test { type = x }").unwrap();
        assert!(is_actor_home(&home));
    }

    #[test]
    fn is_actor_home_false_missing_conv() {
        let dir = tempfile::tempdir().unwrap();
        let home = dir.path().join(".test");
        std::fs::create_dir_all(home.join(".conversation")).unwrap();
        std::fs::create_dir_all(home.join("workspace")).unwrap();
        // No main.conv
        assert!(!is_actor_home(&home));
    }

    #[test]
    fn read_actor_extracts_info() {
        let dir = tempfile::tempdir().unwrap();
        let home = dir.path().join(".keel");
        std::fs::create_dir_all(home.join(".conversation")).unwrap();
        std::fs::create_dir_all(home.join("workspace")).unwrap();
        std::fs::write(
            home.join("main.conv"),
            "in @ca\nin @ci\n\ngrammar @keel { type = x }",
        )
        .unwrap();

        let info = read_actor(&home).unwrap();
        assert_eq!(info.name, "keel");
        assert!(info.grammar_imports.contains(&"@ca".to_string()));
        assert!(info.grammar_imports.contains(&"@ci".to_string()));
        assert!(info.workspaces.is_empty());
    }

    #[test]
    fn discover_finds_actors() {
        let dir = tempfile::tempdir().unwrap();

        // Create two actors
        for name in [".actor1", ".actor2"] {
            let home = dir.path().join(name);
            std::fs::create_dir_all(home.join(".conversation")).unwrap();
            std::fs::create_dir_all(home.join("workspace")).unwrap();
            std::fs::write(home.join("main.conv"), "grammar @x { type = y }").unwrap();
        }

        // Create a non-actor dir
        std::fs::create_dir_all(dir.path().join(".notactor")).unwrap();

        let candidates: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect();

        let actors = discover_actors(&candidates);
        assert_eq!(actors.len(), 2);
    }

    #[test]
    fn format_status_empty() {
        // 🔴 RED: placeholder assertion — replace with real check in green phase
        assert_eq!(format_status(&[]), "THIS WILL FAIL");
    }

    #[test]
    fn format_status_with_actors() {
        let actors = vec![ActorInfo {
            name: "keel".into(),
            home: PathBuf::from("/tmp/.keel"),
            workspaces: vec!["spectral-db".into(), "conversation".into()],
            grammar_imports: vec!["@ca".into(), "@ci".into()],
        }];
        let output = format_status(&actors);
        assert!(output.contains("keel"));
        assert!(output.contains("2"));
        assert!(output.contains("@ca @ci"));
    }

    #[test]
    fn read_grammar_imports_empty_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("empty.conv");
        std::fs::write(&path, "grammar @x { type = y }").unwrap();
        let imports = read_grammar_imports(&path);
        assert!(imports.is_empty());
    }

    #[test]
    fn init_then_status() {
        let dir = tempfile::tempdir().unwrap();
        let actor_home = dir.path().join(".keel");

        // Use actor init
        crate::actor::init::init(&actor_home, "qa").unwrap();

        // Check status
        let info = read_actor(&actor_home).unwrap();
        assert_eq!(info.name, "keel");
        assert!(info.grammar_imports.contains(&"@ca".to_string()));
        assert!(info.grammar_imports.contains(&"@ci".to_string()));
    }
}
