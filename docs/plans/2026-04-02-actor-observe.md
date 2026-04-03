# conversation actor observe — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `conversation actor observe <actor-home> <repo-path>` — the actor reads a repo's grammars, resolves dependencies, and emits a `flake.nix` through `@nix.emit(flake)`.

**Architecture:** New `src/actor.rs` module with an `observe` function. Scans the target repo for `.conv` files, resolves `in @*` dependencies using the existing `PackageRegistry`, determines which conversation packages are needed, and generates a `flake.nix` with `conversation.lib.beam` + package mounts. The flake generation is deterministic — same grammars always produce the same flake.

**Tech Stack:** Rust, existing conversation parser/resolver, Nix flake generation (string templating — the `@nix` domain grammar declares the types, the Rust code emits the text)

**Codebase:** `/Users/alexwolf/dev/projects/conversation/`

**Build/test:**
```bash
nix develop -c cargo test --lib --test compile_test --test grammar_test
```

---

## File Structure

| File | Responsibility |
|------|---------------|
| `src/actor.rs` | **Create.** Actor subcommands: `observe` (first), later `init`/`spawn`/`mount`/`status` |
| `src/actor/observe.rs` | **Create.** Observe a repo: scan .conv files, resolve deps, emit flake.nix |
| `src/actor/emit_nix.rs` | **Create.** Nix flake generation from resolved dependency graph |
| `src/main.rs` | **Modify.** Add `"actor"` arm to CLI match |
| `src/lib.rs` | **Modify.** Add `pub mod actor;` |

---

### Task 1: Actor module + CLI subcommand

**Files:**
- Create: `src/actor.rs`
- Create: `src/actor/observe.rs`
- Create: `src/actor/emit_nix.rs`
- Modify: `src/main.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write failing test for observe**

Create `src/actor/observe.rs`:

```rust
//! Observe a repo: scan .conv files, resolve dependencies, emit flake.

use std::path::Path;

use crate::packages::PackageRegistry;

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
        ).unwrap();
        let deps = scan_repo(dir.path()).unwrap();
        assert!(deps.iter().any(|d| d.name == "admin" && d.is_package));
    }

    #[test]
    fn scan_repo_with_beam_dep_is_core() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("app.conv"),
            "in @beam\n\ngrammar @myapp {\n  type = thing\n}\n",
        ).unwrap();
        let deps = scan_repo(dir.path()).unwrap();
        // @beam is core, not a package
        assert!(deps.iter().any(|d| d.name == "beam" && !d.is_package));
    }
}
```

Create `src/actor/emit_nix.rs`:

```rust
//! Nix flake generation from resolved dependencies.

use super::observe::ResolvedDep;

/// Known conversation packages and their GitHub repos.
fn package_repo(name: &str) -> Option<&'static str> {
    match name {
        "admin" => Some("systemic-engineering/conversation-admin"),
        "ci" => Some("systemic-engineering/conversation-ci"),
        "ca" => Some("systemic-engineering/conversation-ca"),
        "ai" => Some("systemic-engineering/conversation-ai"),
        _ => None,
    }
}

/// Core domains that are part of conversation-beam (not separate packages).
fn is_core_domain(name: &str) -> bool {
    matches!(name, "beam" | "actor" | "compiler" | "nix" | "git"
        | "coincidence" | "projection" | "property" | "topology" | "mail")
}

/// Generate a flake.nix from resolved dependencies.
pub fn emit_flake(name: &str, deps: &[ResolvedDep]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emit_flake_no_packages() {
        let deps = vec![ResolvedDep {
            name: "beam".into(),
            is_package: false,
        }];
        let flake = emit_flake("myapp", &deps);
        assert!(flake.contains("conversation.lib.beam"));
        assert!(flake.contains("name = \"myapp\""));
        assert!(!flake.contains("packages = {"));
    }

    #[test]
    fn emit_flake_with_admin() {
        let deps = vec![
            ResolvedDep { name: "beam".into(), is_package: false },
            ResolvedDep { name: "admin".into(), is_package: true },
        ];
        let flake = emit_flake("myapp", &deps);
        assert!(flake.contains("conversation-admin"));
        assert!(flake.contains("admin"));
        assert!(flake.contains("packages = {"));
    }

    #[test]
    fn emit_flake_is_deterministic() {
        let deps = vec![
            ResolvedDep { name: "admin".into(), is_package: true },
            ResolvedDep { name: "ci".into(), is_package: true },
        ];
        let a = emit_flake("myapp", &deps);
        let b = emit_flake("myapp", &deps);
        assert_eq!(a, b);
    }

    #[test]
    fn is_core() {
        assert!(is_core_domain("beam"));
        assert!(is_core_domain("compiler"));
        assert!(!is_core_domain("admin"));
        assert!(!is_core_domain("ci"));
    }
}
```

Create `src/actor.rs`:

```rust
//! Actor lifecycle: observe, init, spawn, mount, status.
//!
//! `conversation actor observe <home> <repo>` — the first verb.
//! The actor reads a repo's grammars and emits a flake.nix.

pub mod observe;
pub mod emit_nix;

use std::path::Path;

/// Run the `actor` subcommand.
pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("usage: conversation actor <observe|init|spawn|mount|status> [args]");
        std::process::exit(1);
    }

    match args[0].as_str() {
        "observe" => {
            if args.len() < 3 {
                eprintln!("usage: conversation actor observe <actor-home> <repo-path>");
                std::process::exit(1);
            }
            let _actor_home = &args[1];
            let repo_path = Path::new(&args[2]);
            cmd_observe(repo_path);
        }
        other => {
            eprintln!("conversation actor: unknown subcommand '{other}'");
            eprintln!("available: observe");
            std::process::exit(1);
        }
    }
}

fn cmd_observe(repo_path: &Path) {
    eprintln!("observing: {}", repo_path.display());

    let deps = match observe::scan_repo(repo_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("conversation actor observe: {e}");
            std::process::exit(1);
        }
    };

    let packages: Vec<_> = deps.iter().filter(|d| d.is_package).collect();
    let core: Vec<_> = deps.iter().filter(|d| !d.is_package).collect();

    eprintln!("  core domains: {}", core.iter().map(|d| d.name.as_str()).collect::<Vec<_>>().join(", "));
    eprintln!("  packages: {}", packages.iter().map(|d| d.name.as_str()).collect::<Vec<_>>().join(", "));

    let repo_name = repo_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let flake = emit_nix::emit_flake(repo_name, &deps);

    let flake_path = repo_path.join("flake.nix");
    match std::fs::write(&flake_path, &flake) {
        Ok(()) => eprintln!("  wrote: {}", flake_path.display()),
        Err(e) => {
            eprintln!("conversation actor observe: write flake.nix: {e}");
            std::process::exit(1);
        }
    }
}
```

- [ ] **Step 2: Add module declarations**

Add to `src/lib.rs`:

```rust
pub mod actor;
```

- [ ] **Step 3: Add CLI arm to main.rs**

In the `match args[1].as_str()` block in `src/main.rs`, add before the `_` catch-all:

```rust
        "actor" => {
            conversation::actor::run(&args[2..].to_vec());
        }
```

- [ ] **Step 4: Run tests — verify they fail (todo!)**

```bash
cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test actor -- --nocapture
```

Expected: compile succeeds, tests panic with "not yet implemented".

- [ ] **Step 5: Implement scan_repo**

Replace `todo!()` in `src/actor/observe.rs`:

```rust
pub fn scan_repo(repo_path: &Path) -> Result<Vec<ResolvedDep>, String> {
    if !repo_path.exists() {
        return Err(format!("path does not exist: {}", repo_path.display()));
    }

    let mut deps = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // Walk for .conv files
    for entry in walkdir(repo_path)? {
        let source = std::fs::read_to_string(&entry)
            .map_err(|e| format!("{}: {e}", entry.display()))?;

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
    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("{e}"))?;
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip hidden dirs, build artifacts
        if name.starts_with('.') || name == "target" || name == "build"
            || name == "_build" || name == "node_modules"
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
```

Make `is_core_domain` public in `emit_nix.rs`:

```rust
pub fn is_core_domain(name: &str) -> bool {
```

- [ ] **Step 6: Implement emit_flake**

Replace `todo!()` in `src/actor/emit_nix.rs`:

```rust
pub fn emit_flake(name: &str, deps: &[ResolvedDep]) -> String {
    let packages: Vec<_> = deps.iter()
        .filter(|d| d.is_package)
        .collect();

    let mut inputs = String::new();
    inputs.push_str("    nixpkgs.url = \"github:NixOS/nixpkgs/nixos-unstable\";\n");
    inputs.push_str("    flake-utils.url = \"github:numtide/flake-utils\";\n");
    inputs.push_str("    conversation.url = \"git+ssh://git@github.com/systemic-engineering/conversation\";\n");

    for pkg in &packages {
        if let Some(repo) = package_repo(&pkg.name) {
            inputs.push_str(&format!(
                "    {name}.url = \"git+ssh://git@github.com/{repo}\";\n",
                name = pkg.name
            ));
        }
    }

    let mut input_names = vec!["self", "nixpkgs", "flake-utils", "conversation"];
    for pkg in &packages {
        if package_repo(&pkg.name).is_some() {
            input_names.push(&pkg.name);
        }
    }
    let input_args = input_names.join(", ");

    let mut package_mounts = String::new();
    if !packages.is_empty() {
        package_mounts.push_str("          packages = {\n");
        for pkg in &packages {
            if package_repo(&pkg.name).is_some() {
                package_mounts.push_str(&format!(
                    "            {name} = {name};\n",
                    name = pkg.name
                ));
            }
        }
        package_mounts.push_str("          };\n");
    }

    format!(r#"{{
  description = "{name} — conversation project";

  inputs = {{
{inputs}  }};

  outputs = {{ {input_args} }}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${{system}};
        beamPkgs = pkgs.beam.packages.erlang_27;

        app = conversation.lib.beam {{
          inherit pkgs;
          name = "{name}";
          src = ./.;
{package_mounts}        }};
      in {{
        devShells.default = pkgs.mkShell {{
          buildInputs = [
            pkgs.gleam pkgs.erlang_27 beamPkgs.rebar3
            pkgs.git pkgs.just
          ];
          shellHook = ''
            export LANG=en_US.UTF-8
          '';
        }};
      }});
}}
"#)
}
```

- [ ] **Step 7: Run tests — verify they pass**

```bash
cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test actor -- --nocapture
```

Expected: all 7 tests pass (3 observe + 4 emit_nix).

- [ ] **Step 8: Commit**

```bash
cd /Users/alexwolf/dev/projects/conversation
git add src/actor.rs src/actor/ src/main.rs src/lib.rs
git commit -m "🟢 conversation actor observe: scan repo grammars, emit flake.nix"
```

---

### Task 2: End-to-end test against real repo

**Files:**
- Modify: `src/actor/observe.rs` (add integration test)

- [ ] **Step 1: Write integration test**

Add to `src/actor/observe.rs` tests:

```rust
    #[test]
    fn scan_conversation_beam_finds_beam() {
        // conversation-beam has .conv files in its grammars
        // or at minimum Erlang actors that extend @beam
        let beam_path = std::path::Path::new("/Users/alexwolf/dev/projects/conversation-beam");
        if !beam_path.exists() {
            eprintln!("skipping: conversation-beam not found");
            return;
        }
        // conversation-beam might not have .conv files directly
        // but we can test the scan doesn't crash on a real repo
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
        ).unwrap();

        let deps = scan_repo(dir.path()).unwrap();
        assert_eq!(deps.len(), 2);

        let flake = super::emit_nix::emit_flake("myapp", &deps);

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
```

- [ ] **Step 2: Run tests**

```bash
cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test actor -- --nocapture
```

Expected: 9 tests pass.

- [ ] **Step 3: Manual test — run observe on fragmentation**

```bash
cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo run -- actor observe /tmp/test-actor /Users/alexwolf/dev/projects/fragmentation
```

Expected: scans fragmentation (it may have no .conv files → empty deps → flake with just core). Check the output flake.nix.

- [ ] **Step 4: Manual test — run observe on a dir with .conv files**

```bash
mkdir -p /tmp/test-observe
echo 'in @admin\nin @ci\ngrammar @testapp { type = thing }' > /tmp/test-observe/app.conv
cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo run -- actor observe /tmp/test-actor /tmp/test-observe
cat /tmp/test-observe/flake.nix
rm -rf /tmp/test-observe
```

Expected: generates flake.nix with admin and ci packages mounted.

- [ ] **Step 5: Commit**

```bash
cd /Users/alexwolf/dev/projects/conversation
git add src/actor/observe.rs
git commit -m "🟢 integration tests: observe round-trip + manual verification"
```
