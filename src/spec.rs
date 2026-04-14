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
    /// Every top-level block, in declaration order. The command registry.
    pub blocks: Vec<SpecBlock>,
}

/// A generic spec block — one top-level `name { ... }` entry.
/// Each block IS a CLI command. The block's contents ARE the command's config.
#[derive(Clone, Debug)]
pub struct SpecBlock {
    /// Block name (e.g. "craft", "kintsugi", "properties").
    pub name: String,
    /// Flags declared with `--flag` syntax.
    pub flags: Vec<String>,
    /// Key-value settings declared with `key = value` syntax.
    pub settings: Vec<(String, String)>,
}

impl SpecConfig {
    /// Resolve a command name to its spec block.
    /// The spec IS the command registry.
    pub fn resolve_command(&self, name: &str) -> Option<&SpecBlock> {
        self.blocks.iter().find(|b| b.name == name)
    }

    /// All command names declared in the spec, in declaration order.
    pub fn command_names(&self) -> Vec<&str> {
        self.blocks.iter().map(|b| b.name.as_str()).collect()
    }

    /// Generate help text from spec blocks. The help IS the spec rendered.
    pub fn help_text(&self) -> String {
        let mut out = String::new();
        out.push_str("mirror -- an honest compiler\n\n");
        out.push_str("commands:\n");
        for block in &self.blocks {
            let summary = block.summary();
            if summary.is_empty() {
                out.push_str(&format!("  {}\n", block.name));
            } else {
                out.push_str(&format!("  {:16}{}\n", block.name, summary));
            }
        }
        out
    }

    /// Discover and parse the nearest `mirror.spec` file.
    /// Walks up from the current directory looking for `mirror.spec`.
    /// Returns `Default` if no spec file is found.
    pub fn discover() -> Self {
        let mut dir = std::env::current_dir().ok();
        while let Some(d) = dir {
            let candidate = d.join("mirror.spec");
            if candidate.exists() {
                return parse_spec(candidate.to_str().unwrap_or("mirror.spec")).unwrap_or_default();
            }
            dir = d.parent().map(|p| p.to_path_buf());
        }
        Self::default()
    }
}

impl SpecBlock {
    /// Brief summary for help text — first flag or setting, or empty.
    pub fn summary(&self) -> String {
        if !self.flags.is_empty() {
            format!("flags: {}", self.flags.join(", "))
        } else if !self.settings.is_empty() {
            format!(
                "{}",
                self.settings
                    .iter()
                    .map(|(k, v)| format!("{} = {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            String::new()
        }
    }

    /// Check if a flag is declared in this block.
    pub fn accepts_flag(&self, flag: &str) -> bool {
        self.flags.iter().any(|f| f == flag)
    }

    /// Look up a setting value by key.
    pub fn setting(&self, key: &str) -> Option<&str> {
        self.settings
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
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
// Tokenizer
// ---------------------------------------------------------------------------

fn tokenize(source: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        // # comments
        if c == '#' {
            while let Some(&ch) = chars.peek() {
                chars.next();
                if ch == '\n' {
                    break;
                }
            }
            continue;
        }

        // -- line comments (but not --flags)
        if c == '-' {
            let mut peek_chars = chars.clone();
            peek_chars.next();
            if peek_chars.peek() == Some(&'-') {
                peek_chars.next();
                let next_after = peek_chars.peek().copied();
                if next_after.is_none()
                    || next_after == Some(' ')
                    || next_after == Some('\n')
                    || next_after == Some('\t')
                {
                    while let Some(&ch) = chars.peek() {
                        chars.next();
                        if ch == '\n' {
                            break;
                        }
                    }
                    continue;
                }
            }
        }

        // Quoted string
        if c == '"' {
            chars.next();
            let mut s = String::new();
            while let Some(&ch) = chars.peek() {
                chars.next();
                if ch == '"' {
                    break;
                }
                s.push(ch);
            }
            tokens.push(format!("\"{}\"", s));
            continue;
        }

        // Single-char delimiters
        if matches!(c, '{' | '}' | '(' | ')' | '[' | ']') {
            tokens.push(c.to_string());
            chars.next();
            continue;
        }

        // => operator
        if c == '=' {
            chars.next();
            if chars.peek() == Some(&'>') {
                chars.next();
                tokens.push("=>".to_string());
            } else {
                tokens.push("=".to_string());
            }
            continue;
        }

        // Word (identifier, @ref, --flag, path with dots/slashes)
        let mut word = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() || matches!(ch, '{' | '}' | '(' | ')' | '[' | ']' | '#') {
                break;
            }
            if ch == '=' {
                let mut peek = chars.clone();
                peek.next();
                if peek.peek() == Some(&'>') {
                    break;
                }
                break;
            }
            word.push(ch);
            chars.next();
        }
        if !word.is_empty() {
            tokens.push(word);
        }
    }

    tokens
}

fn skip_token(tokens: &[String], pos: usize, expected: &str) -> Result<usize, SpecParseError> {
    if pos >= tokens.len() || tokens[pos] != expected {
        return Err(SpecParseError(format!(
            "expected '{}' at position {}, got {:?}",
            expected,
            pos,
            tokens.get(pos)
        )));
    }
    Ok(pos + 1)
}

fn unquote(s: &str) -> String {
    s.trim_matches('"').to_string()
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

/// Parse a spec from source text.
pub fn parse_spec_source(source: &str) -> Result<SpecConfig, SpecParseError> {
    let mut spec = SpecConfig::default();
    let tokens = tokenize(source);
    let mut pos = 0;

    while pos < tokens.len() {
        match tokens[pos].as_str() {
            "@oid" => {
                pos += 1;
                pos = skip_token(&tokens, pos, "(")?;
                spec.oid = unquote(&tokens[pos]);
                pos += 1;
                pos = skip_token(&tokens, pos, ")")?;
            }
            "store" => {
                let block_name = tokens[pos].clone();
                pos += 1;
                let block_start = pos;
                let (store, new_pos) = parse_store_block(&tokens, pos)?;
                spec.store = store;
                let block = parse_generic_block(&tokens, block_start)?;
                spec.blocks.push(SpecBlock {
                    name: block_name,
                    flags: block.0,
                    settings: block.1,
                });
                pos = new_pos;
            }
            "craft" => {
                let block_name = tokens[pos].clone();
                pos += 1;
                let block_start = pos;
                let (craft, new_pos) = parse_craft_block(&tokens, pos)?;
                spec.craft = craft;
                let block = parse_generic_block(&tokens, block_start)?;
                spec.blocks.push(SpecBlock {
                    name: block_name,
                    flags: block.0,
                    settings: block.1,
                });
                pos = new_pos;
            }
            "kintsugi" => {
                let block_name = tokens[pos].clone();
                pos += 1;
                let block_start = pos;
                let (kintsugi, new_pos) = parse_kintsugi_block(&tokens, pos)?;
                spec.kintsugi = kintsugi;
                let block = parse_generic_block(&tokens, block_start)?;
                spec.blocks.push(SpecBlock {
                    name: block_name,
                    flags: block.0,
                    settings: block.1,
                });
                pos = new_pos;
            }
            "properties" => {
                let block_name = tokens[pos].clone();
                pos += 1;
                let block_start = pos;
                let (properties, new_pos) = parse_properties_block(&tokens, pos)?;
                spec.properties = properties;
                let block = parse_generic_block(&tokens, block_start)?;
                spec.blocks.push(SpecBlock {
                    name: block_name,
                    flags: block.0,
                    settings: block.1,
                });
                pos = new_pos;
            }
            _ => {
                // Unknown top-level token: if followed by `{`, parse as generic block
                let tok = tokens[pos].clone();
                if !tok.starts_with('@')
                    && !tok.starts_with('-')
                    && pos + 1 < tokens.len()
                    && tokens[pos + 1] == "{"
                {
                    pos += 1;
                    let block = parse_generic_block(&tokens, pos)?;
                    spec.blocks.push(SpecBlock {
                        name: tok,
                        flags: block.0,
                        settings: block.1,
                    });
                    // Skip past the block
                    pos = skip_block(&tokens, pos)?;
                } else {
                    pos += 1;
                }
            }
        }
    }

    Ok(spec)
}

/// Parse a spec from a file path.
pub fn parse_spec(path: &str) -> Result<SpecConfig, SpecParseError> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| SpecParseError(format!("cannot read {}: {}", path, e)))?;
    parse_spec_source(&source)
}

// ---------------------------------------------------------------------------
// Generic block parser — extracts flags and settings from any `{ ... }` block
// ---------------------------------------------------------------------------

/// Parse a generic block starting at `{`. Returns (flags, settings).
/// Does NOT consume the tokens — used alongside the specific parser.
fn parse_generic_block(
    tokens: &[String],
    pos: usize,
) -> Result<(Vec<String>, Vec<(String, String)>), SpecParseError> {
    let mut pos = skip_token(tokens, pos, "{")?;
    let mut flags = Vec::new();
    let mut settings = Vec::new();
    let mut depth = 1;

    while pos < tokens.len() && depth > 0 {
        if tokens[pos] == "{" {
            depth += 1;
            pos += 1;
            continue;
        }
        if tokens[pos] == "}" {
            depth -= 1;
            if depth == 0 {
                break;
            }
            pos += 1;
            continue;
        }
        // Only collect flags/settings at depth 1
        if depth == 1 {
            let tok = &tokens[pos];
            if tok.starts_with("--") {
                flags.push(tok.clone());
                pos += 1;
            } else if pos + 2 < tokens.len() && tokens[pos + 1] == "=" {
                let key = tok.clone();
                let value = tokens[pos + 2].clone();
                settings.push((key, value));
                pos += 3;
            } else {
                pos += 1;
            }
        } else {
            pos += 1;
        }
    }

    Ok((flags, settings))
}

/// Skip past a `{ ... }` block, handling nested braces.
fn skip_block(tokens: &[String], pos: usize) -> Result<usize, SpecParseError> {
    let mut pos = skip_token(tokens, pos, "{")?;
    let mut depth = 1;
    while pos < tokens.len() && depth > 0 {
        if tokens[pos] == "{" {
            depth += 1;
        } else if tokens[pos] == "}" {
            depth -= 1;
        }
        pos += 1;
    }
    Ok(pos)
}

// ---------------------------------------------------------------------------
// Block parsers
// ---------------------------------------------------------------------------

fn parse_store_block(
    tokens: &[String],
    pos: usize,
) -> Result<(StoreConfig, usize), SpecParseError> {
    let mut pos = skip_token(tokens, pos, "{")?;
    let mut store = StoreConfig::default();

    while pos < tokens.len() && tokens[pos] != "}" {
        if tokens[pos] == "path" {
            pos += 1;
            pos = skip_token(tokens, pos, "=")?;
            store.path = Some(tokens[pos].clone());
            pos += 1;
        } else {
            pos += 1;
        }
    }

    pos = skip_token(tokens, pos, "}")?;
    Ok((store, pos))
}

fn parse_craft_block(
    tokens: &[String],
    pos: usize,
) -> Result<(CraftConfig, usize), SpecParseError> {
    let mut pos = skip_token(tokens, pos, "{")?;
    let mut craft = CraftConfig::default();

    while pos < tokens.len() && tokens[pos] != "}" {
        match tokens[pos].as_str() {
            "target" => {
                pos += 1;
                let (target, new_pos) = parse_target(tokens, pos)?;
                craft.targets.push(target);
                pos = new_pos;
            }
            "default" => {
                pos += 1;
                while pos < tokens.len()
                    && tokens[pos] != "}"
                    && tokens[pos] != "target"
                    && tokens[pos] != "default"
                {
                    craft.default.push(tokens[pos].clone());
                    pos += 1;
                }
            }
            _ => {
                pos += 1;
            }
        }
    }

    pos = skip_token(tokens, pos, "}")?;
    Ok((craft, pos))
}

fn parse_target(tokens: &[String], pos: usize) -> Result<(TargetConfig, usize), SpecParseError> {
    let mut target = TargetConfig::default();
    let mut pos = pos;

    if pos >= tokens.len() {
        return Err(SpecParseError("expected target name".to_string()));
    }
    let first_name = tokens[pos].clone();
    pos += 1;

    // Check for flow syntax: name => target_name
    if pos < tokens.len() && tokens[pos] == "=>" {
        target.source = Some(first_name);
        pos += 1;
        if pos >= tokens.len() {
            return Err(SpecParseError("expected target name after =>".to_string()));
        }
        target.name = tokens[pos].clone();
        pos += 1;

        // Check for `out @lens("path")`
        if pos < tokens.len() && tokens[pos] == "out" {
            pos += 1;
            if pos >= tokens.len() {
                return Err(SpecParseError("expected lens after 'out'".to_string()));
            }
            target.lens = Some(tokens[pos].clone());
            pos += 1;
            if pos < tokens.len() && tokens[pos] == "(" {
                pos += 1;
                target.output_path = Some(unquote(&tokens[pos]));
                pos += 1;
                pos = skip_token(tokens, pos, ")")?;
            }
        }
    } else {
        target.name = first_name;
    }

    // Check for glob: ("glob")
    if pos < tokens.len() && tokens[pos] == "(" {
        pos += 1;
        target.glob = Some(unquote(&tokens[pos]));
        pos += 1;
        pos = skip_token(tokens, pos, ")")?;
    }

    // Parse grammar block { @prism @meta ... }
    if pos < tokens.len() && tokens[pos] == "{" {
        pos += 1;
        while pos < tokens.len() && tokens[pos] != "}" {
            if tokens[pos] == "{" {
                let mut depth = 1;
                pos += 1;
                while pos < tokens.len() && depth > 0 {
                    if tokens[pos] == "{" {
                        depth += 1;
                    } else if tokens[pos] == "}" {
                        depth -= 1;
                    }
                    pos += 1;
                }
                continue;
            }
            let tok = &tokens[pos];
            if tok.starts_with('@') {
                target.grammars.push(tok.clone());
            }
            pos += 1;
        }
        pos = skip_token(tokens, pos, "}")?;
    }

    Ok((target, pos))
}

fn parse_kintsugi_block(
    tokens: &[String],
    pos: usize,
) -> Result<(KintsugiConfig, usize), SpecParseError> {
    let mut pos = skip_token(tokens, pos, "{")?;
    let mut kintsugi = KintsugiConfig::default();

    while pos < tokens.len() && tokens[pos] != "}" {
        let tok = &tokens[pos];
        if tok.starts_with("--") {
            kintsugi.flags.push(tok.clone());
            pos += 1;
        } else if pos + 2 < tokens.len() && tokens[pos + 1] == "=" {
            let key = tok.clone();
            let value = tokens[pos + 2].clone();
            kintsugi.settings.push((key, value));
            pos += 3;
        } else {
            pos += 1;
        }
    }

    pos = skip_token(tokens, pos, "}")?;
    Ok((kintsugi, pos))
}

fn parse_properties_block(
    tokens: &[String],
    pos: usize,
) -> Result<(PropertiesConfig, usize), SpecParseError> {
    let mut pos = skip_token(tokens, pos, "{")?;
    let mut props = PropertiesConfig::default();

    while pos < tokens.len() && tokens[pos] != "}" {
        match tokens[pos].as_str() {
            "requires" => {
                pos += 1;
                let (items, new_pos) = parse_name_block(tokens, pos)?;
                props.requires = items;
                pos = new_pos;
            }
            "invariant" => {
                pos += 1;
                let (items, new_pos) = parse_name_block(tokens, pos)?;
                props.invariant = items;
                pos = new_pos;
            }
            "ensures" => {
                pos += 1;
                let (items, new_pos) = parse_name_block(tokens, pos)?;
                props.ensures = items;
                pos = new_pos;
            }
            _ => {
                pos += 1;
            }
        }
    }

    pos = skip_token(tokens, pos, "}")?;
    Ok((props, pos))
}

fn parse_name_block(tokens: &[String], pos: usize) -> Result<(Vec<String>, usize), SpecParseError> {
    let mut pos = skip_token(tokens, pos, "{")?;
    let mut items = Vec::new();

    while pos < tokens.len() && tokens[pos] != "}" {
        items.push(tokens[pos].clone());
        pos += 1;
    }

    pos = skip_token(tokens, pos, "}")?;
    Ok((items, pos))
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

    #[test]
    fn parse_spec_full_spec() {
        let spec = parse_spec_source(
            r#"
# mirror.spec

@oid("@mirror-lang")

store {
  path = .git/mirror
}

craft {
  target boot("boot/*.mirror") {
    @prism
    @meta
    @shatter
    @property
  }

  target boot => mirror out @code/rust("rust/mirror/") {
    @prism
    @meta
    @property
    @list
  }

  target boot => cli out @code/rust("rust/mirror-cli/") {
    @lsp
    @git
    @store
    @code
    @shatter
  }

  default boot mirror cli
}

kintsugi {
  --hoist
  --sort-deps
  --normalize
  --align
  naming = snake_case
  indent = 2
}

properties {
  requires {
    types_lowercase
    action_is_named_type
    unique_variants
    every_type_reachable
    no_dead_variants
  }
  invariant {
    deterministic
    pure
    no_cycles
  }
  ensures {
    always_halts
  }
}
"#,
        )
        .unwrap();

        assert_eq!(spec.oid, "@mirror-lang");
        assert_eq!(spec.store.path.as_deref(), Some(".git/mirror"));
        assert_eq!(spec.craft.targets.len(), 3);
        assert_eq!(spec.craft.default, vec!["boot", "mirror", "cli"]);
        assert_eq!(spec.kintsugi.flags.len(), 4);
        assert_eq!(spec.properties.requires.len(), 5);
        assert_eq!(spec.properties.invariant.len(), 3);
        assert_eq!(spec.properties.ensures.len(), 1);
    }

    #[test]
    fn parse_spec_comments_are_skipped() {
        let spec = parse_spec_source(
            r#"
# This is a comment
@oid("@test")
-- This is also a comment
store {
  # inline comment
  path = .git/mirror
}
"#,
        )
        .unwrap();
        assert_eq!(spec.oid, "@test");
        assert_eq!(spec.store.path.as_deref(), Some(".git/mirror"));
    }

    #[test]
    fn parse_spec_empty_is_ok() {
        let spec = parse_spec_source("").unwrap();
        assert_eq!(spec.oid, "");
        assert!(spec.craft.targets.is_empty());
    }

    #[test]
    fn tokenize_basic() {
        let tokens = tokenize(r#"@oid("@test") craft { }"#);
        assert_eq!(
            tokens,
            vec!["@oid", "(", "\"@test\"", ")", "craft", "{", "}"]
        );
    }

    #[test]
    fn tokenize_flow_arrow() {
        let tokens = tokenize("target boot => mirror");
        assert_eq!(tokens, vec!["target", "boot", "=>", "mirror"]);
    }

    #[test]
    fn tokenize_flag() {
        let tokens = tokenize("--hoist --sort-deps");
        assert_eq!(tokens, vec!["--hoist", "--sort-deps"]);
    }

    #[test]
    fn tokenize_key_value() {
        let tokens = tokenize("naming = snake_case");
        assert_eq!(tokens, vec!["naming", "=", "snake_case"]);
    }

    #[test]
    fn parse_spec_target_without_glob() {
        let spec = parse_spec_source(
            r#"
craft {
  target shatter {
    @shatter
  }
}
"#,
        )
        .unwrap();
        assert_eq!(spec.craft.targets.len(), 1);
        assert_eq!(spec.craft.targets[0].name, "shatter");
        assert!(spec.craft.targets[0].glob.is_none());
        assert_eq!(spec.craft.targets[0].grammars, vec!["@shatter"]);
    }

    #[test]
    fn parse_spec_flow_target_without_lens() {
        let spec = parse_spec_source(
            r#"
craft {
  target boot => combined {
    @prism
  }
}
"#,
        )
        .unwrap();
        let target = &spec.craft.targets[0];
        assert_eq!(target.name, "combined");
        assert_eq!(target.source.as_deref(), Some("boot"));
        assert!(target.lens.is_none());
        assert!(target.output_path.is_none());
    }

    #[test]
    fn spec_parse_error_display() {
        let err = SpecParseError("test error".to_string());
        assert_eq!(format!("{}", err), "test error");
    }

    #[test]
    fn spec_parse_error_is_error() {
        let err = SpecParseError("test".to_string());
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn parse_spec_file_not_found() {
        let result = parse_spec("/nonexistent/mirror.spec");
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains("cannot read"));
    }

    #[test]
    fn parse_real_mirror_spec() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("mirror.spec");
        let spec = parse_spec(path.to_str().unwrap()).unwrap();
        assert_eq!(spec.oid, "@mirror-lang");
        assert_eq!(spec.store.path.as_deref(), Some(".git/mirror"));
        assert!(
            spec.craft.targets.len() >= 4,
            "should have at least 4 targets"
        );
        assert!(
            spec.craft.targets.iter().any(|t| t.name == "boot"),
            "should have boot target"
        );
        assert!(
            spec.craft.targets.iter().any(|t| t.name == "mirror"),
            "should have mirror target"
        );
        assert!(
            spec.craft.targets.iter().any(|t| t.name == "cli"),
            "should have cli target"
        );
        assert!(!spec.properties.requires.is_empty());
    }

    // -----------------------------------------------------------------------
    // Phase 1: SpecConfig as command registry
    // -----------------------------------------------------------------------

    #[test]
    fn spec_blocks_populated_from_known_blocks() {
        let spec = parse_spec_source(
            r#"
store { path = .git/mirror }
craft { default boot }
kintsugi { --hoist naming = snake_case }
properties { requires { unique_variants } }
"#,
        )
        .unwrap();
        assert_eq!(spec.blocks.len(), 4);
        assert_eq!(spec.blocks[0].name, "store");
        assert_eq!(spec.blocks[1].name, "craft");
        assert_eq!(spec.blocks[2].name, "kintsugi");
        assert_eq!(spec.blocks[3].name, "properties");
    }

    #[test]
    fn spec_blocks_capture_flags_and_settings() {
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
        let block = spec.resolve_command("kintsugi").unwrap();
        assert_eq!(block.flags, vec!["--hoist", "--sort-deps"]);
        assert_eq!(block.settings.len(), 2);
        assert_eq!(block.setting("naming"), Some("snake_case"));
        assert_eq!(block.setting("indent"), Some("2"));
    }

    #[test]
    fn resolve_command_returns_none_for_unknown() {
        let spec = parse_spec_source("store { path = .git/mirror }").unwrap();
        assert!(spec.resolve_command("nonexistent").is_none());
    }

    #[test]
    fn command_names_returns_all_block_names() {
        let spec = parse_spec_source(
            r#"
store { path = .git/mirror }
craft { default boot }
kintsugi { --hoist }
"#,
        )
        .unwrap();
        let names = spec.command_names();
        assert_eq!(names, vec!["store", "craft", "kintsugi"]);
    }

    #[test]
    fn spec_block_summary_with_flags() {
        let block = SpecBlock {
            name: "kintsugi".into(),
            flags: vec!["--hoist".into(), "--sort-deps".into()],
            settings: vec![],
        };
        assert_eq!(block.summary(), "flags: --hoist, --sort-deps");
    }

    #[test]
    fn spec_block_summary_with_settings() {
        let block = SpecBlock {
            name: "store".into(),
            flags: vec![],
            settings: vec![("path".into(), ".git/mirror".into())],
        };
        assert_eq!(block.summary(), "path = .git/mirror");
    }

    #[test]
    fn spec_block_summary_empty() {
        let block = SpecBlock {
            name: "empty".into(),
            flags: vec![],
            settings: vec![],
        };
        assert_eq!(block.summary(), "");
    }

    #[test]
    fn spec_block_accepts_flag() {
        let block = SpecBlock {
            name: "kintsugi".into(),
            flags: vec!["--hoist".into(), "--sort-deps".into()],
            settings: vec![],
        };
        assert!(block.accepts_flag("--hoist"));
        assert!(!block.accepts_flag("--unknown"));
    }

    #[test]
    fn unknown_block_parsed_generically() {
        let spec = parse_spec_source(
            r#"
infer {
  --classify
  --learn
}
"#,
        )
        .unwrap();
        let block = spec.resolve_command("infer").unwrap();
        assert_eq!(block.flags, vec!["--classify", "--learn"]);
    }

    #[test]
    fn real_spec_has_blocks() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("mirror.spec");
        let spec = parse_spec(path.to_str().unwrap()).unwrap();
        let names = spec.command_names();
        assert!(names.contains(&"store"), "should have store block");
        assert!(names.contains(&"craft"), "should have craft block");
        assert!(names.contains(&"kintsugi"), "should have kintsugi block");
        assert!(
            names.contains(&"properties"),
            "should have properties block"
        );
    }

    // -----------------------------------------------------------------------
    // Phase 2: Help from spec
    // -----------------------------------------------------------------------

    #[test]
    fn spec_block_detail_help() {
        let block = SpecBlock {
            name: "kintsugi".into(),
            flags: vec!["--hoist".into(), "--sort-deps".into()],
            settings: vec![("naming".into(), "snake_case".into())],
        };
        let help = block.detail_help();
        assert!(
            help.contains("kintsugi"),
            "detail help should include command name"
        );
        assert!(
            help.contains("--hoist"),
            "detail help should list flags"
        );
        assert!(
            help.contains("naming = snake_case"),
            "detail help should list settings"
        );
    }

    #[test]
    fn spec_help_text_from_blocks() {
        let spec = parse_spec_source(
            r#"
craft { default boot }
kintsugi { --hoist }
"#,
        )
        .unwrap();
        let help = spec.help_text();
        assert!(help.contains("craft"), "help should list craft");
        assert!(help.contains("kintsugi"), "help should list kintsugi");
    }
}
