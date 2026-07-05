//! Grammar loading + keyword lookup. Mirrors the C `grammar_t` API.

use crate::ast::AstKind;
use crate::Ctx;
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

/// Derive an @-ref from a grammar path like "boot/std/code/llvm/ir.mirror"
/// or "shards/mirror/grammar.mirror". Both substrate roots strip to the
/// same @-ref; `shards/` is the destination, `boot/std/` is the legacy
/// fallback (per `bootstrap-retirement-plan.md` shrinkage contract).
pub fn grammar_ref_from_path(path: &str) -> String {
    let mut p = path;
    // shards/ wins; boot/std/ is the legacy fallback. Order matters only
    // in that we try the more-specific path first; the two prefixes are
    // disjoint so the order is interchangeable in practice.
    if let Some(stripped) = p.strip_prefix("shards/") {
        p = stripped;
    } else if let Some(stripped) = p.strip_prefix("boot/std/") {
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
    // shards/-first resolution: when the primary grammar is loaded from
    // shards/, prefer companion sources from shards/ as well. The merger
    // already treats a missing companion file as a no-op (additive), so a
    // shards/ companion that hasn't been ported yet falls through cleanly.
    match path {
        "shards/mirror/grammar.mirror" => &[
            "shards/mirror/glass/ast/token.mirror",
            "boot/std/mirror/glass/ast/token.mirror",
        ],
        "boot/std/mirror/grammar.mirror" => &["boot/std/mirror/glass/ast/token.mirror"],
        "shards/mirror/spec.mirror" => &["shards/mirror/spec/keywords.mirror"],
        _ => &[],
    }
}

/// Merge keyword mappings from a companion source into the primary grammar.
/// Returns `Err` if a keyword is declared in both with conflicting ops; the
/// caller surfaces this to the operator (stop-and-report per the
/// keyword-harvester extension contract).
///
/// Companion paths are hardcoded relative (e.g. `shards/mirror/glass/ast/token.mirror`).
/// When `ctx` is `Some`, each is resolved against `ctx.cwd()` before reading; when
/// `None` the read is process-cwd-relative (legacy `load_grammar` shape).
fn merge_keyword_sources(
    g: &mut Grammar,
    companions: &[&str],
    ctx: Option<&Ctx>,
) -> std::io::Result<()> {
    for companion in companions {
        let read_path: std::path::PathBuf = match ctx {
            Some(c) => c.resolve(companion),
            None => std::path::PathBuf::from(companion),
        };
        let source = match fs::read_to_string(&read_path) {
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

/// Load a grammar from a path, resolving against process cwd.
///
/// Prefer [`load_grammar_in`] which takes an explicit `&Ctx` — that variant
/// resolves the grammar path (and companion sources) against `ctx.cwd()` so
/// callers threaded through the dispatch chain don't inherit the process cwd
/// implicitly. This function is retained as a compat wrapper for callers
/// that haven't been threaded yet.
pub fn load_grammar(path: &str) -> std::io::Result<Grammar> {
    let source = fs::read_to_string(path)?;
    let mut g = parse_grammar(&source);
    g.r#ref = grammar_ref_from_path(path);
    merge_keyword_sources(&mut g, companion_keyword_sources(path), None)?;
    Ok(g)
}

/// Load a grammar, resolving `path` and its companion keyword sources
/// against `ctx.cwd()`. The Arc 2 shape per /loop 2026-07-05: relative
/// grammar paths (`shards/mirror/spec.mirror`, `boot/std/mirror/grammar.mirror`,
/// etc.) resolve against the caller's dispatch context instead of the
/// process cwd. Absolute paths pass through unchanged.
pub fn load_grammar_in(path: &str, ctx: &Ctx) -> std::io::Result<Grammar> {
    let read_path = ctx.resolve(path);
    let source = fs::read_to_string(&read_path)?;
    let mut g = parse_grammar(&source);
    g.r#ref = grammar_ref_from_path(path);
    merge_keyword_sources(&mut g, companion_keyword_sources(path), Some(ctx))?;
    Ok(g)
}

/// Pick a grammar for a file based on extension. Mirrors C `grammar_for_file`.
///
/// Substrate-pull: `shards/` is source of truth; `boot/std/` is the
/// transitional legacy fallback. For each extension we check whether the
/// shards/ grammar exists on disk and prefer it; otherwise we return the
/// boot/std/ path (which may itself be missing, in which case `load_grammar`
/// surfaces the IO error to the caller — same contract as before).
///
/// Prefer [`grammar_for_file_in`] which takes a `&Ctx` — the presence check
/// resolves against `ctx.cwd()` rather than the process cwd. This function
/// is retained for callers that haven't been threaded yet.
pub fn grammar_for_file(path: &str) -> &'static str {
    let p = Path::new(path);
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
    match ext {
        "rs" => {
            if Path::new("shards/code/rust.mirror").exists() {
                "shards/code/rust.mirror"
            } else {
                "boot/std/code/rust.mirror"
            }
        }
        "spec" => {
            // `.spec` files declare project manifolds; their grammar is
            // `@mirror/spec` (shards/mirror/spec.mirror), declared in the
            // substrate per [[architecture-shards-as-substrate-source]].
            // Companion keyword bindings live at
            // `shards/mirror/spec/keywords.mirror` (registered in
            // `companion_keyword_sources`); together they let the existing
            // tokenize+parse infrastructure produce an AST the
            // kintsugi-spec walker walks directly, retiring the
            // hand-rolled `parse_spec_targets` byte scanner.
            "shards/mirror/spec.mirror"
        }
        "mirror" | "shard" | "shatter" => {
            if Path::new("shards/mirror/grammar.mirror").exists() {
                "shards/mirror/grammar.mirror"
            } else {
                "boot/std/mirror/grammar.mirror"
            }
        }
        "ll" => {
            if Path::new("shards/code/llvm/ir.mirror").exists() {
                "shards/code/llvm/ir.mirror"
            } else {
                "boot/std/code/llvm/ir.mirror"
            }
        }
        _ => {
            if Path::new("shards/code/rust.mirror").exists() {
                "shards/code/rust.mirror"
            } else {
                "boot/std/code/rust.mirror"
            }
        }
    }
}

/// Ctx-aware variant of [`grammar_for_file`]. Existence checks for
/// `shards/` vs `boot/std/` grammar files resolve against `ctx.cwd()`
/// instead of the process cwd, so callers threaded through Arc 2 pick
/// the correct file regardless of where the process was invoked.
pub fn grammar_for_file_in(path: &str, ctx: &Ctx) -> &'static str {
    let p = Path::new(path);
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
    match ext {
        "rs" => {
            if ctx.resolve("shards/code/rust.mirror").exists() {
                "shards/code/rust.mirror"
            } else {
                "boot/std/code/rust.mirror"
            }
        }
        "spec" => "shards/mirror/spec.mirror",
        "mirror" | "shard" | "shatter" => {
            if ctx.resolve("shards/mirror/grammar.mirror").exists() {
                "shards/mirror/grammar.mirror"
            } else {
                "boot/std/mirror/grammar.mirror"
            }
        }
        "ll" => {
            if ctx.resolve("shards/code/llvm/ir.mirror").exists() {
                "shards/code/llvm/ir.mirror"
            } else {
                "boot/std/code/llvm/ir.mirror"
            }
        }
        _ => {
            if ctx.resolve("shards/code/rust.mirror").exists() {
                "shards/code/rust.mirror"
            } else {
                "boot/std/code/rust.mirror"
            }
        }
    }
}

/// Resolve a grammar ref like "@code/llvm/ir" to a .mirror path.
///
/// Substrate-pull: try `shards/<ref>.mirror` first; if that file exists,
/// return that path. Otherwise fall back to `boot/std/<ref>.mirror`. This
/// operationalizes the `bootstrap-retirement-plan.md` shrinkage contract:
/// shards/ wins, boot/ is the legacy fallback.
pub fn grammar_path_for_ref(r#ref: &str) -> Option<String> {
    if !r#ref.starts_with('@') {
        return None;
    }
    let tail = &r#ref[1..];
    let shards_path = format!("shards/{}.mirror", tail);
    if Path::new(&shards_path).exists() {
        return Some(shards_path);
    }
    Some(format!("boot/std/{}.mirror", tail))
}

/// Ctx-aware variant of [`grammar_path_for_ref`]. Existence checks
/// resolve against `ctx.cwd()` instead of the process cwd.
pub fn grammar_path_for_ref_in(r#ref: &str, ctx: &Ctx) -> Option<String> {
    if !r#ref.starts_with('@') {
        return None;
    }
    let tail = &r#ref[1..];
    let shards_path = format!("shards/{}.mirror", tail);
    if ctx.resolve(&shards_path).exists() {
        return Some(shards_path);
    }
    Some(format!("boot/std/{}.mirror", tail))
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
        // through the meta-glass at boot/std/mirror/grammar.mirror (per the
        // 2026-05-25 substrate decision).
        assert_eq!(
            grammar_for_file("eigenboard.shard"),
            "boot/std/mirror/grammar.mirror"
        );
    }

    #[test]
    fn spec_extension_routes_to_spec_grammar() {
        // `.spec` declares a project manifold; it parses through
        // @mirror/spec (shards/mirror/spec.mirror). The companion
        // keyword bindings at `shards/mirror/spec/keywords.mirror`
        // are merged in by `load_grammar` so the tokenizer's keyword
        // table carries `project`, `target`, `settle_on`, etc.
        // (substrate-pull realize: the dispatch surface IS substrate-
        // declared, not hardcoded in Rust).
        assert_eq!(grammar_for_file("mirror.spec"), "shards/mirror/spec.mirror");
    }
}
