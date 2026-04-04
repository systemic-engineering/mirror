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
pub fn scan_repo(repo_path: &Path) -> Result<Vec<ResolvedDep>, String> {
    if !repo_path.exists() {
        return Err(format!("path does not exist: {}", repo_path.display()));
    }

    let mut deps = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // Walk for .conv files
    for entry in walkdir(repo_path)? {
        let source =
            std::fs::read_to_string(&entry).map_err(|e| format!("{}: {e}", entry.display()))?;

        // Extract `in @domain` declarations
        for line in source.lines() {
            let trimmed = line.trim();
            if let Some(domain) = trimmed.strip_prefix("in @") {
                let domain = domain.trim();
                if !domain.is_empty() && seen.insert(domain.to_string()) {
                    deps.push(ResolvedDep {
                        name: domain.to_string(),
                        is_package: !super::emit_nix::is_core_domain(domain),
                    });
                }
            }
        }
    }

    // Sort for determinism
    deps.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(deps)
}

/// Walk a directory for .conv files, skipping hidden dirs and build artifacts.
fn walkdir(root: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let mut files = Vec::new();
    walk_recursive(root, &mut files)?;
    files.sort();
    Ok(files)
}

fn walk_recursive(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip hidden dirs, build artifacts
        if name.starts_with('.')
            || name == "target"
            || name == "build"
            || name == "_build"
            || name == "node_modules"
        {
            continue;
        }

        if path.is_dir() {
            walk_recursive(&path, files)?;
        } else if name.ends_with(".conv") {
            files.push(path);
        }
    }

    Ok(())
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
    fn scan_repo_skips_non_conv_files() {
        let dir = tempfile::tempdir().unwrap();
        // Write a non-.conv file — should be ignored
        fs::write(dir.path().join("readme.md"), "# readme").unwrap();
        fs::write(dir.path().join("config.toml"), "[package]").unwrap();
        let deps = scan_repo(dir.path()).unwrap();
        assert!(deps.is_empty());
    }

    #[test]
    fn scan_repo_skips_hidden_and_build_dirs() {
        let dir = tempfile::tempdir().unwrap();
        // Create skipped directories with .conv files inside
        for skip_dir in &[".git", "target", "build", "_build", "node_modules"] {
            let skip_path = dir.path().join(skip_dir);
            fs::create_dir_all(&skip_path).unwrap();
            fs::write(skip_path.join("hidden.conv"), "in @admin\n").unwrap();
        }
        let deps = scan_repo(dir.path()).unwrap();
        // None of the skipped dirs' conv files should be picked up
        assert!(deps.is_empty());
    }

    #[test]
    fn scan_repo_deduplicates_same_domain() {
        let dir = tempfile::tempdir().unwrap();
        // Two files both importing @admin — should only appear once
        fs::write(
            dir.path().join("a.conv"),
            "in @admin\ngrammar @a { type = x }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("b.conv"),
            "in @admin\ngrammar @b { type = y }\n",
        )
        .unwrap();
        let deps = scan_repo(dir.path()).unwrap();
        let admin_count = deps.iter().filter(|d| d.name == "admin").count();
        assert_eq!(admin_count, 1);
    }

    #[test]
    fn scan_nonexistent_path_returns_error() {
        let err = scan_repo(std::path::Path::new(
            "/nonexistent/path/that/does/not/exist",
        ))
        .unwrap_err();
        assert!(err.contains("path does not exist"));
    }

    #[test]
    fn scan_repo_io_errors_propagate() {
        // Test that I/O errors during directory walk are returned as errors
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            // Test 1: unreadable subdir causes read_dir error
            let dir = tempfile::tempdir().unwrap();
            let subdir = dir.path().join("subdir");
            fs::create_dir_all(&subdir).unwrap();
            fs::set_permissions(&subdir, fs::Permissions::from_mode(0o000)).unwrap();
            let result = scan_repo(dir.path());
            fs::set_permissions(&subdir, fs::Permissions::from_mode(0o755)).unwrap();
            // Should error due to unreadable subdir
            assert!(result.is_err());

            // Test 2: unreadable .conv file causes read_to_string error
            let dir2 = tempfile::tempdir().unwrap();
            let conv_path = dir2.path().join("app.conv");
            fs::write(&conv_path, "in @admin\n").unwrap();
            fs::set_permissions(&conv_path, fs::Permissions::from_mode(0o000)).unwrap();
            let result2 = scan_repo(dir2.path());
            fs::set_permissions(&conv_path, fs::Permissions::from_mode(0o644)).unwrap();
            // Should error due to unreadable .conv file
            assert!(result2.is_err());
        }
    }

    #[test]
    fn scan_conversation_beam_finds_beam() {
        // conversation-beam exists — scan it and verify it doesn't crash
        let beam_path = std::path::Path::new("/Users/alexwolf/dev/projects/conversation-beam");
        let deps = scan_repo(beam_path).unwrap();
        eprintln!("conversation-beam deps: {:?}", deps);
    }

    #[test]
    fn observe_and_emit_round_trip() {
        let dir = tempfile::tempdir().unwrap();

        // Write a .conv that imports @admin and @ci
        std::fs::write(
            dir.path().join("myapp.conv"),
            "in @admin\nin @ci\n\ngrammar @myapp {\n  type = page | check\n}\n",
        )
        .unwrap();

        let deps = scan_repo(dir.path()).unwrap();
        assert_eq!(deps.len(), 2);

        let flake = super::super::emit_nix::emit_flake("myapp", &deps);

        // Write it
        let flake_path = dir.path().join("flake.nix");
        std::fs::write(&flake_path, &flake).unwrap();

        // Verify it's valid nix (basic structure check)
        let content = std::fs::read_to_string(&flake_path).unwrap();
        assert!(content.contains("conversation.lib.beam"));
        assert!(content.contains("conversation-admin"));
        assert!(content.contains("conversation-ci"));
        assert!(content.contains("packages = {"));
        assert!(content.contains("admin = admin"));
        assert!(content.contains("ci = ci"));
    }
}
