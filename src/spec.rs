//! spec — Parse `mirror.spec` files into structured configuration.
//!
//! A `.spec` file mirrors the CLI surface. Each top-level block IS a CLI command.
//! Each block's contents ARE that command's configuration.

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Top-level spec configuration parsed from a `mirror.spec` file.
#[derive(Clone, Debug, Default)]
pub struct SpecConfig {
    pub oid: String,
    pub store: StoreConfig,
    pub craft: CraftConfig,
    pub kintsugi: KintsugiConfig,
    pub properties: PropertiesConfig,
}

#[derive(Clone, Debug, Default)]
pub struct StoreConfig {
    pub path: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct CraftConfig {
    pub targets: Vec<TargetConfig>,
    pub default: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct TargetConfig {
    pub name: String,
    pub source: Option<String>,
    pub lens: Option<String>,
    pub output_path: Option<String>,
    pub glob: Option<String>,
    pub grammars: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct KintsugiConfig {
    pub flags: Vec<String>,
    pub settings: Vec<(String, String)>,
}

#[derive(Clone, Debug, Default)]
pub struct PropertiesConfig {
    pub requires: Vec<String>,
    pub invariant: Vec<String>,
    pub ensures: Vec<String>,
}

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct SpecParseError(pub String);

impl std::fmt::Display for SpecParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SpecParseError {}

// ---------------------------------------------------------------------------
// Parser — stub
// ---------------------------------------------------------------------------

/// Parse a spec from source text.
pub fn parse_spec_source(_source: &str) -> Result<SpecConfig, SpecParseError> {
    todo!("spec parser not yet implemented")
}

/// Parse a spec from a file path.
pub fn parse_spec(path: &str) -> Result<SpecConfig, SpecParseError> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| SpecParseError(format!("cannot read {}: {}", path, e)))?;
    parse_spec_source(&source)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_spec_reads_oid() {
        let spec = parse_spec_source(r#"@oid("@mirror-lang")"#).unwrap();
        assert_eq!(spec.oid, "@mirror-lang");
    }

    #[test]
    fn parse_spec_reads_store() {
        let spec = parse_spec_source(
            r#"
store {
  path = .git/mirror
}
"#,
        )
        .unwrap();
        assert_eq!(spec.store.path.as_deref(), Some(".git/mirror"));
    }

    #[test]
    fn parse_spec_reads_simple_target() {
        let spec = parse_spec_source(
            r#"
craft {
  target boot("boot/*.mirror") {
    @prism
    @meta
  }
  default boot
}
"#,
        )
        .unwrap();
        assert_eq!(spec.craft.targets.len(), 1);
        assert_eq!(spec.craft.targets[0].name, "boot");
        assert_eq!(spec.craft.targets[0].glob.as_deref(), Some("boot/*.mirror"));
        assert_eq!(spec.craft.targets[0].grammars, vec!["@prism", "@meta"]);
        assert_eq!(spec.craft.default, vec!["boot"]);
    }

    #[test]
    fn parse_spec_reads_flow_target() {
        let spec = parse_spec_source(
            r#"
craft {
  target boot => mirror out @code/rust("rust/mirror/") {
    @prism
    @meta
  }
}
"#,
        )
        .unwrap();
        let target = &spec.craft.targets[0];
        assert_eq!(target.name, "mirror");
        assert_eq!(target.source.as_deref(), Some("boot"));
        assert_eq!(target.lens.as_deref(), Some("@code/rust"));
        assert_eq!(target.output_path.as_deref(), Some("rust/mirror/"));
        assert_eq!(target.grammars, vec!["@prism", "@meta"]);
    }

    #[test]
    fn parse_spec_reads_multiple_targets() {
        let spec = parse_spec_source(
            r#"
craft {
  target boot("boot/*.mirror") {
    @prism
    @meta
  }
  target boot => mirror out @code/rust("rust/mirror/") {
    @prism
    @meta
    @list
  }
  target boot => cli out @code/rust("rust/mirror-cli/") {
    @code
    @shatter
  }
  default boot mirror cli
}
"#,
        )
        .unwrap();
        assert_eq!(spec.craft.targets.len(), 3);
        assert_eq!(spec.craft.targets[0].name, "boot");
        assert_eq!(spec.craft.targets[1].name, "mirror");
        assert_eq!(spec.craft.targets[2].name, "cli");
        assert_eq!(spec.craft.default, vec!["boot", "mirror", "cli"]);
    }

    #[test]
    fn parse_spec_reads_kintsugi() {
        let spec = parse_spec_source(
            r#"
kintsugi {
  --hoist
  --sort-deps
  naming = snake_case
  indent = 2
}
"#,
        )
        .unwrap();
        assert_eq!(spec.kintsugi.flags, vec!["--hoist", "--sort-deps"]);
        assert_eq!(spec.kintsugi.settings.len(), 2);
        assert_eq!(
            spec.kintsugi.settings[0],
            ("naming".to_string(), "snake_case".to_string())
        );
    }

    #[test]
    fn parse_spec_reads_properties() {
        let spec = parse_spec_source(
            r#"
properties {
  requires {
    types_lowercase
    unique_variants
  }
  invariant {
    deterministic
    pure
  }
  ensures {
    always_halts
  }
}
"#,
        )
        .unwrap();
        assert_eq!(
            spec.properties.requires,
            vec!["types_lowercase", "unique_variants"]
        );
        assert_eq!(spec.properties.invariant, vec!["deterministic", "pure"]);
        assert_eq!(spec.properties.ensures, vec!["always_halts"]);
    }
}
