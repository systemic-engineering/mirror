// FROZEN -- see AGENTS.md. Do not modify without explicit approval.
// This file is Rust substrate. All extensions happen through .mirror grammars.
// If you're adding code here, you're probably wrong. Write a grammar instead.

//! @cli -- flag parsing as typed lambdas.
//!
//! Contract:
//! - in: raw CLI args ([&str])
//! - out: ParsedCommand with grammar refs
//! - bound: no external deps. The grammar IS the parser.
//!
//! Every `--flag` is a grammar reference:
//!   --strict      -> @cli/strict      (nullary)
//!   --format json -> @cli/format      (unary, value = "json")
//!   --git/commit  -> @git/commit      (namespaced)
//!
//! Flags compose as an applicative product, not a pipeline.
//! Each flag independently configures the command.

/// A parsed CLI flag mapped to a grammar reference.
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedFlag {
    /// The grammar reference this flag maps to (e.g. "@cli/strict", "@git/commit").
    pub grammar_ref: String,
    /// The value passed to this flag, if unary (e.g. Some("json") for --format json).
    pub value: Option<String>,
}

/// A parsed CLI command with positional args and flags.
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedCommand {
    /// The command name (e.g. "compile", "kintsugi").
    pub name: String,
    /// Positional arguments (non-flag args).
    pub positional: Vec<String>,
    /// Parsed flags mapped to grammar refs.
    pub flags: Vec<ParsedFlag>,
}

/// Parse flag arguments into grammar-ref-mapped ParsedFlags.
///
/// Rules:
/// - `--x/y` maps to `@x/y` (namespaced flag)
/// - `--x` maps to `@cli/x` (default namespace)
/// - If the next arg doesn't start with `--`, it's the flag's value (unary)
/// - If the next arg starts with `--` or there is no next arg, the flag is nullary
pub fn parse_flags(args: &[&str]) -> Vec<ParsedFlag> {
    let mut flags = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = args[i];

        if !arg.starts_with("--") {
            i += 1;
            continue;
        }

        let raw = &arg[2..]; // strip --

        // Map -- to @: --x/y -> @x/y, --x -> @cli/x
        let grammar_ref = if raw.contains('/') {
            format!("@{}", raw)
        } else {
            format!("@cli/{}", raw)
        };

        // Check if next arg is a value (doesn't start with --)
        let value = if i + 1 < args.len() && !args[i + 1].starts_with("--") {
            i += 1;
            Some(args[i].to_string())
        } else {
            None
        };

        flags.push(ParsedFlag {
            grammar_ref,
            value,
        });

        i += 1;
    }

    flags
}

/// Parse a full CLI invocation into a ParsedCommand.
///
/// Args format: command [flags...] [positional...]
/// Flags are extracted first, remaining args are positional.
pub fn parse_command(args: &[String]) -> ParsedCommand {
    if args.is_empty() {
        return ParsedCommand {
            name: String::new(),
            positional: vec![],
            flags: vec![],
        };
    }

    let name = args[0].clone();
    let rest: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

    let mut positional = Vec::new();
    let mut flag_args = Vec::new();
    let mut i = 0;

    // First pass: separate flags from positional args
    while i < rest.len() {
        if rest[i].starts_with("--") {
            flag_args.push(rest[i]);
            // If next arg is a value for this flag, include it
            if i + 1 < rest.len() && !rest[i + 1].starts_with("--") {
                flag_args.push(rest[i + 1]);
                i += 1;
            }
        } else {
            positional.push(rest[i].to_string());
        }
        i += 1;
    }

    let flags = parse_flags(&flag_args);

    ParsedCommand {
        name,
        positional,
        flags,
    }
}

/// Format a parsed command's flags as a composed pipeline string.
///
/// For display/debugging: `lift() . target("std") . format("json")`
pub fn format_pipeline(flags: &[ParsedFlag]) -> String {
    if flags.is_empty() {
        return String::from("identity");
    }

    flags
        .iter()
        .map(|f| {
            let name = f.grammar_ref.rsplit('/').next().unwrap_or(&f.grammar_ref);
            match &f.value {
                Some(v) => format!("{}(\"{}\")", name, v),
                None => format!("{}()", name),
            }
        })
        .collect::<Vec<_>>()
        .join(" . ")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_flag_maps_to_grammar_ref() {
        let flags = parse_flags(&["--strict"]);
        assert_eq!(flags[0].grammar_ref, "@cli/strict");
    }

    #[test]
    fn parse_namespaced_flag() {
        let flags = parse_flags(&["--git/commit", "fix: thing"]);
        assert_eq!(flags[0].grammar_ref, "@git/commit");
        assert_eq!(flags[0].value, Some("fix: thing".to_string()));
    }

    #[test]
    fn parse_nullary_flag() {
        let flags = parse_flags(&["--lift"]);
        assert_eq!(flags[0].grammar_ref, "@cli/lift");
        assert!(flags[0].value.is_none());
    }

    #[test]
    fn flags_compose_left_to_right() {
        let flags = parse_flags(&["--lift", "--target", "std", "--git/commit"]);
        assert_eq!(flags.len(), 3);
        assert_eq!(flags[0].grammar_ref, "@cli/lift");
        assert_eq!(flags[1].grammar_ref, "@cli/target");
        assert_eq!(flags[1].value, Some("std".to_string()));
        assert_eq!(flags[2].grammar_ref, "@git/commit");
    }

    #[test]
    fn parse_command_separates_flags_and_positional() {
        let args: Vec<String> = vec![
            "kintsugi", "--lift", "--target", "std", "src/mcp.rs",
        ].into_iter().map(String::from).collect();

        let cmd = parse_command(&args);
        assert_eq!(cmd.name, "kintsugi");
        assert_eq!(cmd.positional, vec!["src/mcp.rs"]);
        assert_eq!(cmd.flags.len(), 2);
        assert_eq!(cmd.flags[0].grammar_ref, "@cli/lift");
        assert_eq!(cmd.flags[1].grammar_ref, "@cli/target");
        assert_eq!(cmd.flags[1].value, Some("std".to_string()));
    }

    #[test]
    fn format_pipeline_nullary() {
        let flags = parse_flags(&["--strict"]);
        assert_eq!(format_pipeline(&flags), "strict()");
    }

    #[test]
    fn format_pipeline_mixed() {
        let flags = parse_flags(&["--lift", "--target", "std", "--format", "json"]);
        assert_eq!(
            format_pipeline(&flags),
            "lift() . target(\"std\") . format(\"json\")"
        );
    }

    #[test]
    fn format_pipeline_empty() {
        let flags: Vec<ParsedFlag> = vec![];
        assert_eq!(format_pipeline(&flags), "identity");
    }

    #[test]
    fn format_pipeline_namespaced() {
        let flags = parse_flags(&["--git/commit", "fix: thing"]);
        assert_eq!(format_pipeline(&flags), "commit(\"fix: thing\")");
    }

    #[test]
    fn parse_empty_args() {
        let flags = parse_flags(&[]);
        assert!(flags.is_empty());
    }

    #[test]
    fn parse_only_positional_no_flags() {
        let flags = parse_flags(&["src/main.rs", "src/lib.rs"]);
        assert!(flags.is_empty());
    }

    #[test]
    fn parse_multiple_nullary_flags() {
        let flags = parse_flags(&["--strict", "--verbose", "--lift"]);
        assert_eq!(flags.len(), 3);
        assert!(flags.iter().all(|f| f.value.is_none()));
        assert_eq!(flags[0].grammar_ref, "@cli/strict");
        assert_eq!(flags[1].grammar_ref, "@cli/verbose");
        assert_eq!(flags[2].grammar_ref, "@cli/lift");
    }

    #[test]
    fn parse_command_no_flags() {
        let args: Vec<String> = vec!["compile", "boot/std/cli.mirror"]
            .into_iter().map(String::from).collect();
        let cmd = parse_command(&args);
        assert_eq!(cmd.name, "compile");
        assert_eq!(cmd.positional, vec!["boot/std/cli.mirror"]);
        assert!(cmd.flags.is_empty());
    }

    #[test]
    fn parse_command_empty() {
        let args: Vec<String> = vec![];
        let cmd = parse_command(&args);
        assert!(cmd.name.is_empty());
    }
}
