//! Grammar loading + keyword lookup. Mirrors the C `grammar_t` API.

use crate::ast::AstKind;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Mapping {
    pub keyword: String,
    pub kind: AstKind,
}

#[derive(Debug, Clone, Default)]
pub struct Grammar {
    pub mappings: Vec<Mapping>,
    /// grammar ref like "@code/llvm/ir" or "@mirror/grammar"
    pub r#ref: String,
}

impl Grammar {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, keyword: &str, kind: AstKind) {
        let mut k = keyword.to_string();
        if k.len() > 63 {
            k.truncate(63);
        }
        self.mappings.push(Mapping { keyword: k, kind });
    }

    pub fn lookup(&self, word: &str) -> Option<AstKind> {
        for m in &self.mappings {
            if m.keyword == word {
                return Some(m.kind);
            }
        }
        None
    }

    /// Reverse lookup: given an AST kind, find the first declared keyword.
    pub fn keyword_for_kind(&self, kind: AstKind) -> Option<&str> {
        for m in &self.mappings {
            if m.kind == kind {
                return Some(&m.keyword);
            }
        }
        None
    }

    pub fn is_llvm_ir(&self) -> bool {
        self.r#ref == "@code/llvm/ir"
    }

    /// True when this grammar tokenizes `.mirror` source — used to gate the
    /// Spec A / Spec B forms (`io`, `match`, `select`, `~f`, etc.) so they
    /// only activate where the language actually defines them.
    pub fn is_mirror(&self) -> bool {
        self.r#ref == "@mirror/grammar"
    }
}

/// Parse a .mirror grammar source. Mirrors C `parse_grammar` semantics.
pub fn parse_grammar(source: &str) -> Grammar {
    let mut g = Grammar::new();
    let mut in_grammar_block = false;
    let mut brace_depth: i32 = 0;

    for raw_line in source.split('\n') {
        // Trim leading/trailing whitespace (and trailing \r).
        let line = raw_line.trim_matches(|c: char| c == ' ' || c == '\t');
        let line = line.trim_end_matches(['\r', ' ', '\t']);

        // Comments
        if line.starts_with("--") || line.starts_with('#') {
            continue;
        }

        if line.starts_with("grammar ") && line.contains('{') {
            in_grammar_block = true;
            brace_depth = 1;
            continue;
        }

        if !in_grammar_block {
            continue;
        }

        for ch in line.chars() {
            if ch == '{' {
                brace_depth += 1;
            } else if ch == '}' {
                brace_depth -= 1;
                if brace_depth == 0 {
                    in_grammar_block = false;
                }
            }
        }

        if !in_grammar_block && brace_depth <= 0 {
            continue;
        }

        // Skip lines with (), <>, {}. These contain action/abstract syntax.
        if line.contains('(') || line.contains('>') || line.contains('{') || line.contains('}') {
            continue;
        }

        // Strip trailing comment markers `#...` and ` --...`.
        let mut content = line.to_string();
        if let Some(idx) = content.find('#') {
            content.truncate(idx);
        }
        if let Some(idx) = content.find(" --") {
            content.truncate(idx);
        }
        // Trim trailing whitespace post-truncation.
        let content = content.trim_end_matches([' ', '\t']).to_string();

        // sscanf %63s %63s
        let mut words = content.split_ascii_whitespace();
        let w1 = match words.next() {
            Some(w) => w,
            None => continue,
        };
        let w2 = match words.next() {
            Some(w) => w,
            None => continue,
        };
        // The C version requires exactly two words. Anything beyond also has to
        // be rejected to match the `if (words == 2)` check.
        if words.next().is_some() {
            continue;
        }

        // Truncate to 63 like sscanf %63s.
        let w1: String = w1.chars().take(63).collect();
        let w2: String = w2.chars().take(63).collect();

        let kind = match w1.as_str() {
            "focus" => AstKind::Focus,
            "split" => AstKind::Split,
            "shift" => AstKind::Shift,
            "project" => AstKind::Project,
            "settle" => AstKind::Settle,
            _ => continue,
        };
        g.add(&w2, kind);
    }
    g
}

/// Derive an @-ref from a grammar path like "boot/std/code/llvm/ir.mirror".
pub fn grammar_ref_from_path(path: &str) -> String {
    let mut p = path;
    let pre = "boot/std/";
    if let Some(stripped) = p.strip_prefix(pre) {
        p = stripped;
    }
    if let Some(stripped) = p.strip_suffix(".mirror") {
        p = stripped;
    }
    let mut s = String::with_capacity(p.len() + 1);
    s.push('@');
    s.push_str(p);
    s
}

/// Companion grammar files whose `<op> <keyword>` declarations are merged
/// into the keyword table when the primary grammar is loaded. Substrate-pull
/// realization: keyword vocabulary accumulates from `.mirror` declarations,
/// not from hardcoded Rust. See AGENTS.md "Keywords Are Substrate
/// Declarations" and `boot/std/mirror/glass/ast/token.mirror`.
///
/// The mapping is keyed by the primary grammar path. Each entry lists
/// additional `.mirror` files whose keyword declarations compose into the
/// same table. Same keyword + same op in two files = no conflict (the
/// substrate carries the same declaration from both sources). Same keyword
/// + different op = conflict; `merge_keyword_sources` reports it.
fn companion_keyword_sources(path: &str) -> &'static [&'static str] {
    match path {
        "boot/std/mirror/grammar.mirror" => &["boot/std/mirror/glass/ast/token.mirror"],
        _ => &[],
    }
}

/// Merge keyword mappings from a companion source into the primary grammar.
/// Returns `Err` if a keyword is declared in both with conflicting ops; the
/// caller surfaces this to the operator (stop-and-report per the
/// keyword-harvester extension contract).
fn merge_keyword_sources(g: &mut Grammar, companions: &[&str]) -> std::io::Result<()> {
    for companion in companions {
        let source = match fs::read_to_string(companion) {
            Ok(s) => s,
            // Missing companion file is not fatal — the legacy file alone
            // remains a valid keyword source. This keeps the harvester
            // extension additive: removing the companion regresses to the
            // pre-extension behaviour.
            Err(_) => continue,
        };
        let extra = parse_grammar(&source);
        for m in extra.mappings {
            match g.lookup(&m.keyword) {
                Some(existing) if existing == m.kind => {
                    // Same keyword + same op in both files. The substrate
                    // carries the same declaration from both sources;
                    // no-op. (Legacy `focus grammar` / `focus prism` /
                    // etc. are mirrored verbatim in token.mirror.)
                    continue;
                }
                Some(existing) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!(
                            "keyword conflict: `{}` declared as {:?} in primary grammar but {:?} in companion `{}`",
                            m.keyword, existing, m.kind, companion
                        ),
                    ));
                }
                None => {
                    g.add(&m.keyword, m.kind);
                }
            }
        }
    }
    Ok(())
}

pub fn load_grammar(path: &str) -> std::io::Result<Grammar> {
    let source = fs::read_to_string(path)?;
    let mut g = parse_grammar(&source);
    g.r#ref = grammar_ref_from_path(path);
    merge_keyword_sources(&mut g, companion_keyword_sources(path))?;
    Ok(g)
}

/// Pick a grammar for a file based on extension. Mirrors C `grammar_for_file`.
pub fn grammar_for_file(path: &str) -> &'static str {
    let p = Path::new(path);
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
    match ext {
        "rs" => "boot/std/code/rust.mirror",
        "mirror" | "spec" | "shard" | "shatter" => "boot/std/mirror/grammar.mirror",
        "ll" => "boot/std/code/llvm/ir.mirror",
        _ => "boot/std/code/rust.mirror",
    }
}

/// Resolve a grammar ref like "@code/llvm/ir" to a .mirror path.
pub fn grammar_path_for_ref(r#ref: &str) -> Option<String> {
    if !r#ref.starts_with('@') {
        return None;
    }
    Some(format!("boot/std/{}.mirror", &r#ref[1..]))
}

/// Words to silently skip during tokenisation (Rust-grammar noise).
pub fn is_skip_word(word: &str) -> bool {
    matches!(
        word,
        "async"
            | "unsafe"
            | "const"
            | "extern"
            | "crate"
            | "super"
            | "self"
            | "where"
            | "mut"
            | "ref"
            | "static"
            | "let"
            | "if"
            | "else"
            | "for"
            | "while"
            | "loop"
            | "match"
            | "return"
            | "break"
            | "continue"
            | "as"
            | "in"
            | "type"
            | "dyn"
            | "move"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shard_extension_routes_to_mirror_grammar() {
        // .shard is the observer-relative deployment description; it parses
        // through the same meta-glass as .spec / .mirror / .shatter.
        // Coexists with .spec (per Alex 2026-05-25 substrate decision).
        assert_eq!(
            grammar_for_file("eigenboard.shard"),
            "boot/std/mirror/grammar.mirror"
        );
        assert_eq!(
            grammar_for_file("eigenboard.spec"),
            "boot/std/mirror/grammar.mirror"
        );
    }
}
