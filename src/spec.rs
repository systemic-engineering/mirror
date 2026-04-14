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
            if ch.is_whitespace()
                || matches!(ch, '{' | '}' | '(' | ')' | '[' | ']' | '#')
            {
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
                pos += 1;
                let (store, new_pos) = parse_store_block(&tokens, pos)?;
                spec.store = store;
                pos = new_pos;
            }
            "craft" => {
                pos += 1;
                let (craft, new_pos) = parse_craft_block(&tokens, pos)?;
                spec.craft = craft;
                pos = new_pos;
            }
            "kintsugi" => {
                pos += 1;
                let (kintsugi, new_pos) = parse_kintsugi_block(&tokens, pos)?;
                spec.kintsugi = kintsugi;
                pos = new_pos;
            }
            "properties" => {
                pos += 1;
                let (properties, new_pos) = parse_properties_block(&tokens, pos)?;
                spec.properties = properties;
                pos = new_pos;
            }
            _ => {
                pos += 1;
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

fn parse_name_block(
    tokens: &[String],
    pos: usize,
) -> Result<(Vec<String>, usize), SpecParseError> {
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
}
