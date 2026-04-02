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
pub fn emit_flake(_name: &str, _deps: &[ResolvedDep]) -> String {
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
}
