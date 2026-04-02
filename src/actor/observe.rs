//! Observe a repo: scan .conv files, resolve dependencies, emit flake.

use std::path::Path;

/// A resolved dependency from scanning a repo's .conv files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedDep {
    /// The domain name (e.g., "admin", "ci", "ca")
    pub name: String,
    /// Whether this is a garden package or a core domain
    pub is_package: bool,
}

/// Scan a repo for .conv files and resolve which packages it needs.
pub fn scan_repo(_repo_path: &Path) -> Result<Vec<ResolvedDep>, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn scan_empty_repo_returns_empty() {
        let dir = tempfile::tempdir().unwrap();
        let deps = scan_repo(dir.path()).unwrap();
        assert!(deps.is_empty());
    }

    #[test]
    fn scan_repo_with_conv_finds_deps() {
        let dir = tempfile::tempdir().unwrap();
        // Write a .conv file that imports @admin
        fs::write(
            dir.path().join("app.conv"),
            "in @admin\n\ngrammar @myapp {\n  type = page\n}\n",
        )
        .unwrap();
        let deps = scan_repo(dir.path()).unwrap();
        assert!(deps.iter().any(|d| d.name == "admin" && d.is_package));
    }

    #[test]
    fn scan_repo_with_beam_dep_is_core() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("app.conv"),
            "in @beam\n\ngrammar @myapp {\n  type = thing\n}\n",
        )
        .unwrap();
        let deps = scan_repo(dir.path()).unwrap();
        // @beam is core, not a package
        assert!(deps.iter().any(|d| d.name == "beam" && !d.is_package));
    }

    #[test]
    fn scan_conversation_beam_finds_beam() {
        let beam_path = std::path::Path::new("/Users/alexwolf/dev/projects/conversation-beam");
        if !beam_path.exists() {
            eprintln!("skipping: conversation-beam not found");
            return;
        }
        let deps = scan_repo(beam_path).unwrap();
        eprintln!("conversation-beam deps: {:?}", deps);
    }

    #[test]
    fn observe_and_emit_round_trip() {
        let dir = tempfile::tempdir().unwrap();

        std::fs::write(
            dir.path().join("myapp.conv"),
            "in @admin\nin @ci\n\ngrammar @myapp {\n  type = page | check\n}\n",
        )
        .unwrap();

        let deps = scan_repo(dir.path()).unwrap();
        assert_eq!(deps.len(), 2);

        let flake = super::super::emit_nix::emit_flake("myapp", &deps);

        let flake_path = dir.path().join("flake.nix");
        std::fs::write(&flake_path, &flake).unwrap();

        let content = std::fs::read_to_string(&flake_path).unwrap();
        assert!(content.contains("conversation.lib.beam"));
        assert!(content.contains("conversation-admin"));
        assert!(content.contains("conversation-ci"));
        assert!(content.contains("packages = {"));
        assert!(content.contains("admin = admin"));
        assert!(content.contains("ci = ci"));
    }
}
