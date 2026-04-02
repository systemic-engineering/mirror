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
pub fn is_core_domain(name: &str) -> bool {
    matches!(
        name,
        "beam"
            | "actor"
            | "compiler"
            | "nix"
            | "git"
            | "coincidence"
            | "projection"
            | "property"
            | "topology"
            | "mail"
    )
}

/// Generate a flake.nix from resolved dependencies.
pub fn emit_flake(name: &str, deps: &[ResolvedDep]) -> String {
    let packages: Vec<_> = deps.iter().filter(|d| d.is_package).collect();

    let mut inputs = String::new();
    inputs.push_str("    nixpkgs.url = \"github:NixOS/nixpkgs/nixos-unstable\";\n");
    inputs.push_str("    flake-utils.url = \"github:numtide/flake-utils\";\n");
    inputs.push_str(
        "    conversation.url = \"git+ssh://git@github.com/systemic-engineering/conversation\";\n",
    );

    for pkg in &packages {
        if let Some(repo) = package_repo(&pkg.name) {
            inputs.push_str(&format!(
                "    {pname}.url = \"git+ssh://git@github.com/{repo}\";\n",
                pname = pkg.name
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

    let known_packages: Vec<_> = packages
        .iter()
        .filter(|p| package_repo(&p.name).is_some())
        .collect();

    let mut package_mounts = String::new();
    if !known_packages.is_empty() {
        package_mounts.push_str("          packages = {\n");
        for pkg in &known_packages {
            package_mounts.push_str(&format!(
                "            {pname} = {pname};\n",
                pname = pkg.name
            ));
        }
        package_mounts.push_str("          };\n");
    }

    format!(
        r#"{{
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
"#
    )
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
            ResolvedDep {
                name: "beam".into(),
                is_package: false,
            },
            ResolvedDep {
                name: "admin".into(),
                is_package: true,
            },
        ];
        let flake = emit_flake("myapp", &deps);
        assert!(flake.contains("conversation-admin"));
        assert!(flake.contains("admin"));
        assert!(flake.contains("packages = {"));
    }

    #[test]
    fn emit_flake_is_deterministic() {
        let deps = vec![
            ResolvedDep {
                name: "admin".into(),
                is_package: true,
            },
            ResolvedDep {
                name: "ci".into(),
                is_package: true,
            },
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

    #[test]
    fn emit_flake_with_ca_and_ai() {
        let deps = vec![
            ResolvedDep {
                name: "ca".into(),
                is_package: true,
            },
            ResolvedDep {
                name: "ai".into(),
                is_package: true,
            },
        ];
        let flake = emit_flake("myapp", &deps);
        assert!(flake.contains("conversation-ca"));
        assert!(flake.contains("conversation-ai"));
        assert!(flake.contains("packages = {"));
    }

    #[test]
    fn emit_flake_unknown_package_excluded() {
        // Unknown packages have no repo mapping — excluded from inputs + mounts
        let deps = vec![ResolvedDep {
            name: "unknown-pkg".into(),
            is_package: true,
        }];
        let flake = emit_flake("myapp", &deps);
        // No input or mount generated for unknown packages
        assert!(!flake.contains("unknown-pkg.url"));
        assert!(!flake.contains("packages = {"));
    }
}
