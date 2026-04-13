//! Language detection — maps language names to tree-sitter grammars and LSP servers.

/// Configuration for a supported language.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageConfig {
    /// Language name (e.g., "python").
    pub name: String,
    /// Tree-sitter grammar package name (e.g., "tree-sitter-python").
    pub tree_sitter: String,
    /// Command to launch the LSP server (e.g., `["pyright-langserver", "--stdio"]`).
    pub lsp_command: Vec<String>,
    /// File extensions for this language (e.g., `[".py"]`).
    pub file_extensions: Vec<String>,
}

/// LSP capabilities detected from a server's `initialize` response.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LspCapabilities {
    pub completion: bool,
    pub hover: bool,
    pub definition: bool,
    pub references: bool,
    pub diagnostics: bool,
    pub semantic_tokens: bool,
}

impl LspCapabilities {
    /// Returns an iterator over (action_name, mirror_signature) pairs
    /// for capabilities that are supported.
    pub fn actions(&self) -> Vec<(&'static str, &'static str)> {
        let mut actions = Vec::new();
        if self.completion {
            actions.push(("complete", "complete(position)"));
        }
        if self.diagnostics {
            actions.push(("diagnose", "diagnose(range)"));
        }
        if self.hover {
            actions.push(("hover", "hover(position)"));
        }
        if self.definition {
            actions.push(("definition", "definition(position)"));
        }
        if self.references {
            actions.push(("references", "references(position)"));
        }
        if self.semantic_tokens {
            actions.push(("tokens", "tokens(range)"));
        }
        actions
    }

    /// A default set with all capabilities enabled — used when no LSP
    /// server is available and we want to generate stubs for everything.
    pub fn all() -> Self {
        Self {
            completion: true,
            hover: true,
            definition: true,
            references: true,
            diagnostics: true,
            semantic_tokens: true,
        }
    }
}

/// Hardcoded language table. Returns `None` for unknown languages.
pub fn detect(name: &str) -> Option<LanguageConfig> {
    let normalized = name.to_lowercase();
    match normalized.as_str() {
        "python" | "py" => Some(LanguageConfig {
            name: "python".into(),
            tree_sitter: "tree-sitter-python".into(),
            lsp_command: vec!["pyright-langserver".into(), "--stdio".into()],
            file_extensions: vec![".py".into()],
        }),
        "rust" | "rs" => Some(LanguageConfig {
            name: "rust".into(),
            tree_sitter: "tree-sitter-rust".into(),
            lsp_command: vec!["rust-analyzer".into()],
            file_extensions: vec![".rs".into()],
        }),
        "gleam" => Some(LanguageConfig {
            name: "gleam".into(),
            tree_sitter: "tree-sitter-gleam".into(),
            lsp_command: vec!["gleam".into(), "lsp".into()],
            file_extensions: vec![".gleam".into()],
        }),
        "javascript" | "js" => Some(LanguageConfig {
            name: "javascript".into(),
            tree_sitter: "tree-sitter-javascript".into(),
            lsp_command: vec!["typescript-language-server".into(), "--stdio".into()],
            file_extensions: vec![".js".into(), ".mjs".into(), ".cjs".into()],
        }),
        "typescript" | "ts" => Some(LanguageConfig {
            name: "typescript".into(),
            tree_sitter: "tree-sitter-typescript".into(),
            lsp_command: vec!["typescript-language-server".into(), "--stdio".into()],
            file_extensions: vec![".ts".into(), ".tsx".into()],
        }),
        "nix" => Some(LanguageConfig {
            name: "nix".into(),
            tree_sitter: "tree-sitter-nix".into(),
            lsp_command: vec!["nil".into()],
            file_extensions: vec![".nix".into()],
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_python() {
        let config = detect("python").unwrap();
        assert_eq!(config.name, "python");
        assert_eq!(config.tree_sitter, "tree-sitter-python");
        assert_eq!(config.lsp_command, vec!["pyright-langserver", "--stdio"]);
        assert_eq!(config.file_extensions, vec![".py"]);
    }

    #[test]
    fn detect_python_alias() {
        let config = detect("py").unwrap();
        assert_eq!(config.name, "python");
    }

    #[test]
    fn detect_rust() {
        let config = detect("rust").unwrap();
        assert_eq!(config.name, "rust");
        assert_eq!(config.tree_sitter, "tree-sitter-rust");
    }

    #[test]
    fn detect_rust_alias() {
        assert_eq!(detect("rs").unwrap().name, "rust");
    }

    #[test]
    fn detect_gleam() {
        let config = detect("gleam").unwrap();
        assert_eq!(config.lsp_command, vec!["gleam", "lsp"]);
    }

    #[test]
    fn detect_javascript() {
        let config = detect("javascript").unwrap();
        assert_eq!(config.name, "javascript");
        assert!(config.file_extensions.contains(&".js".to_string()));
    }

    #[test]
    fn detect_js_alias() {
        assert_eq!(detect("js").unwrap().name, "javascript");
    }

    #[test]
    fn detect_typescript() {
        let config = detect("typescript").unwrap();
        assert_eq!(config.name, "typescript");
        assert!(config.file_extensions.contains(&".ts".to_string()));
    }

    #[test]
    fn detect_ts_alias() {
        assert_eq!(detect("ts").unwrap().name, "typescript");
    }

    #[test]
    fn detect_nix() {
        let config = detect("nix").unwrap();
        assert_eq!(config.lsp_command, vec!["nil"]);
    }

    #[test]
    fn detect_unknown() {
        assert!(detect("brainfuck").is_none());
    }

    #[test]
    fn detect_case_insensitive() {
        assert!(detect("Python").is_some());
        assert!(detect("RUST").is_some());
    }

    #[test]
    fn lsp_capabilities_all() {
        let caps = LspCapabilities::all();
        assert!(caps.completion);
        assert!(caps.hover);
        assert!(caps.definition);
        assert!(caps.references);
        assert!(caps.diagnostics);
        assert!(caps.semantic_tokens);
    }

    #[test]
    fn lsp_capabilities_actions_all() {
        let caps = LspCapabilities::all();
        let actions = caps.actions();
        assert_eq!(actions.len(), 6);
        assert_eq!(actions[0].0, "complete");
        assert_eq!(actions[5].0, "tokens");
    }

    #[test]
    fn lsp_capabilities_actions_partial() {
        let caps = LspCapabilities {
            completion: true,
            hover: true,
            definition: false,
            references: false,
            diagnostics: true,
            semantic_tokens: false,
        };
        let actions = caps.actions();
        assert_eq!(actions.len(), 3);
        let names: Vec<&str> = actions.iter().map(|(n, _)| *n).collect();
        assert_eq!(names, vec!["complete", "diagnose", "hover"]);
    }

    #[test]
    fn lsp_capabilities_default_empty() {
        let caps = LspCapabilities::default();
        assert!(!caps.completion);
        assert!(caps.actions().is_empty());
    }
}
