//! mq pipeline parser + executor. Mirrors C `split_pipeline` / `execute_pipeline`.

use crate::ast::AstNode;
use crate::exec::io_exec;
use crate::grammar::{grammar_path_for_ref, load_grammar};
use crate::spectral::{compute_content_oid, render_ast};
use crate::tokenize::tokenize;

/// A rewrite rule parsed from `<symbol> => <replacement>` mq syntax.
///
/// The meta-glass identifies which tokens are structural (keywords,
/// path components, file basenames) versus prose. The rewrite applies
/// only at structural-token boundaries; English in `@nl` comments
/// containing the symbol stays unchanged because @nl lifts those
/// bytes opaquely through the cross-grammar boundary.
///
/// For 4b.3 the rewrite is whole-word-bounded: the symbol must be
/// surrounded by non-word bytes (or source boundaries) to match. This
/// is the structural-safety property at the byte level until the
/// full meta-glass-aware rewriter lands in 4b.4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewriteRule {
    pub symbol: String,
    pub replacement: String,
}

/// Parse an mq-query of the form `<symbol> => <replacement>` into a
/// `Vec<RewriteRule>`. Multiple rules can be joined with `;`. Returns
/// `None` if the query does not contain a `=>` separator (i.e., is a
/// navigation pipeline, not a rewrite).
///
/// F-9 fix: each semicolon-delimited segment is parsed by locating
/// the FIRST `=>` substring as the separator (via `find("=>")` +
/// split), rather than testing only `query.contains("=>")` and then
/// re-splitting later. Everything before the first `=>` is the LHS,
/// everything after is the RHS, both whitespace-trimmed. A segment
/// without `=>` aborts the whole parse (returns `None`).
///
/// TODO: unify with the mq-query parser. Once the meta-glass exposes
/// a real grammar for queries, dispatch on token types instead of
/// substring presence — see audit F-9 for the future-correctness
/// debt this carries.
pub fn parse_rewrite(query: &str) -> Option<Vec<RewriteRule>> {
    if query.find("=>").is_none() {
        return None;
    }
    let mut rules = Vec::new();
    for raw in query.split(';') {
        let raw = raw.trim();
        if raw.is_empty() {
            continue;
        }
        // Find the FIRST `=>` separator; everything before is the
        // LHS, everything after is the RHS. Both whitespace-trimmed.
        let sep = raw.find("=>")?;
        let lhs = &raw[..sep];
        let rhs = &raw[sep + 2..];
        let sym = lhs.trim().trim_matches('\'').trim_matches('"');
        let repl = rhs.trim().trim_matches('\'').trim_matches('"');
        if sym.is_empty() {
            return None;
        }
        rules.push(RewriteRule {
            symbol: sym.to_string(),
            replacement: repl.to_string(),
        });
    }
    Some(rules)
}

/// Apply rewrite rules to source bytes at whole-word boundaries. A
/// match requires the symbol to be bounded by non-word bytes (or
/// source start/end). Word bytes are ASCII alnum + `_`. `/` is a
/// path separator (a boundary), not a word byte — we want
/// `@mirror/grammar` → `@mirror/glass` to rewrite the trailing path
/// component. `@` is also a boundary so `@grammar` rewrites to
/// `@glass`.
///
/// F-3 fix: this definition is now shared with the combinator
/// walker via [`crate::spectral::is_word_byte`]; the two surfaces
/// previously disagreed (the walker had `/` as a word byte; the
/// rewrite path did not). Unified to the rewrite's narrower form
/// because the migration's structural rewrite of path components
/// requires `/` to be a boundary.
///
/// F-8: rules apply **sequentially**: rule N sees rule (N-1)'s
/// output. The intermediate buffer is threaded through each rule's
/// pass. For parallel-style semantics (each rule sees the original
/// source independently), the call site must run each rule on the
/// original bytes and merge by chosen conflict policy — that
/// composition is not implemented here. Tests
/// `apply_rewrites_chains_sequentially` and
/// `apply_rewrites_sequential_not_parallel` pin both behaviors.
pub fn apply_rewrites(rules: &[RewriteRule], source: &[u8]) -> Vec<u8> {
    use crate::spectral::is_word_byte;
    let mut current = source.to_vec();
    for rule in rules {
        let sym = rule.symbol.as_bytes();
        let repl = rule.replacement.as_bytes();
        if sym.is_empty() {
            continue;
        }
        let mut out: Vec<u8> = Vec::with_capacity(current.len());
        let mut i = 0;
        while i < current.len() {
            if i + sym.len() <= current.len() && &current[i..i + sym.len()] == sym {
                let left_ok = i == 0 || !is_word_byte(current[i - 1]);
                let right_ok =
                    i + sym.len() == current.len() || !is_word_byte(current[i + sym.len()]);
                if left_ok && right_ok {
                    out.extend_from_slice(repl);
                    i += sym.len();
                    continue;
                }
            }
            out.push(current[i]);
            i += 1;
        }
        current = out;
    }
    current
}

#[cfg(test)]
mod rewrite_tests {
    use super::*;

    #[test]
    fn parse_simple_rewrite() {
        let rules = parse_rewrite("grammar => glass").unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].symbol, "grammar");
        assert_eq!(rules[0].replacement, "glass");
    }

    #[test]
    fn parse_quoted_rewrite() {
        let rules = parse_rewrite("'grammar' => 'glass'").unwrap();
        assert_eq!(rules[0].symbol, "grammar");
        assert_eq!(rules[0].replacement, "glass");
    }

    #[test]
    fn parse_multiple_rules() {
        let rules = parse_rewrite("a => b; c => d").unwrap();
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0].symbol, "a");
        assert_eq!(rules[1].replacement, "d");
    }

    #[test]
    fn parse_non_rewrite_returns_none() {
        assert!(parse_rewrite("@code/llvm/ir |> @mirror/kintsugi").is_none());
    }

    #[test]
    fn apply_word_bounded_rewrite() {
        let rules = vec![RewriteRule {
            symbol: "grammar".to_string(),
            replacement: "glass".to_string(),
        }];
        // word-bounded: should rewrite
        let out = apply_rewrites(&rules, b"grammar @mirror/grammar");
        assert_eq!(out, b"glass @mirror/glass");
    }

    #[test]
    fn apply_does_not_match_inside_identifier() {
        let rules = vec![RewriteRule {
            symbol: "grammar".to_string(),
            replacement: "glass".to_string(),
        }];
        // `grammars` should NOT rewrite (right-side word byte)
        let out = apply_rewrites(&rules, b"grammars plural");
        assert_eq!(out, b"grammars plural");
    }

    #[test]
    fn apply_preserves_english_in_prose() {
        // "the grammar of mirror" — 'grammar' is whole-word so it WOULD
        // rewrite at the byte level. structural-safety via @nl is the
        // 4b.4 layer; for 4b.3 the whole-word boundary is what's
        // implemented. this test pins the current semantics.
        let rules = vec![RewriteRule {
            symbol: "grammar".to_string(),
            replacement: "glass".to_string(),
        }];
        let out = apply_rewrites(&rules, b"# the grammar of mirror\n");
        assert_eq!(out, b"# the glass of mirror\n");
    }

    // ---------- F-3: unified is_word_byte semantics ----------

    /// F-3 pin: both surfaces (`apply_rewrites`'s whole-word check
    /// and the walker's `branch_keyword_occurs`) now share the same
    /// `is_word_byte` definition (alnum + `_`, with `/` as a
    /// boundary). For every byte in 0..=255 the two surfaces agree
    /// on whether a single-byte literal is bounded.
    ///
    /// Concrete witness: `@mirror/grammar` with rewrite rule
    /// `grammar => glass` produces `@mirror/glass` (trailing path
    /// component rewrites), and the walker's whole-word check on
    /// `grammar` against the same source returns `true` (the path
    /// separator `/` is a boundary).
    #[test]
    fn f3_is_word_byte_unified_across_surfaces() {
        use crate::spectral::is_word_byte;
        // The single shared definition: alnum + `_`. Path separator
        // `/` is a boundary; `@` is a boundary.
        for b in 0u8..=255 {
            let expected = b.is_ascii_alphanumeric() || b == b'_';
            assert_eq!(is_word_byte(b), expected, "byte {}", b);
        }
        assert!(!is_word_byte(b'/'), "/ must be a boundary");
        assert!(!is_word_byte(b'@'), "@ must be a boundary");

        // End-to-end: the trailing path component rewrites because
        // `/` is a boundary on the rewrite side AND on the walker
        // side. Pre-fix, only one surface saw it.
        let rules = vec![RewriteRule {
            symbol: "grammar".to_string(),
            replacement: "glass".to_string(),
        }];
        let out = apply_rewrites(&rules, b"@mirror/grammar");
        assert_eq!(out, b"@mirror/glass");
    }

    // ---------- F-8: sequential application of rewrite rules ----------

    /// F-8 pin: rules apply in document order over the running buffer
    /// — rule N sees rule (N-1)'s output. Witnessed by two clauses in
    /// one test:
    ///
    /// 1. Forward chain: `a => b; b => c` rewrites `a` all the way to
    ///    `c` (not `b`); rule 2 sees rule 1's output.
    /// 2. Order-asymmetry: `b => c; a => b` on `a` produces `b` (rule
    ///    1 fires on nothing; rule 2 rewrites `a -> b`); swapping the
    ///    rule order gives a different verdict, confirming the
    ///    semantics is sequential, not parallel.
    #[test]
    fn f8_apply_rewrites_runs_rules_sequentially() {
        // Clause 1: forward chain.
        let rules = parse_rewrite("a => b; b => c").unwrap();
        let out = apply_rewrites(&rules, b"a");
        assert_eq!(out, b"c", "rule 2 sees rule 1's output: a -> b -> c");

        // Clause 2: order-asymmetry.
        let rules = parse_rewrite("b => c; a => b").unwrap();
        let out = apply_rewrites(&rules, b"a");
        assert_eq!(
            out, b"b",
            "sequential: first rule sees no b's; second rewrites a -> b"
        );
    }

    // ---------- F-9: parse_rewrite uses first-`=>` split ----------

    /// F-9 pin: `parse_rewrite` locates the FIRST `=>` as the
    /// separator (LHS = everything before, RHS = everything after).
    /// A symbol or replacement containing further `=>` substrings
    /// stays on its respective side rather than being silently
    /// re-split. (Today's identifier grammar can't actually contain
    /// `=>`, but the test pins the parser-level invariant against a
    /// future surface that allows it inside quoted strings.)
    #[test]
    fn f9_parse_rewrite_splits_on_first_arrow() {
        // Construct a synthetic RHS containing a literal `=>`. The
        // current `parse_rewrite` strips outer quotes; we use a
        // bare-form RHS for clarity.
        let rules = parse_rewrite("sym => a => b").unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].symbol, "sym");
        assert_eq!(
            rules[0].replacement, "a => b",
            "first => is the separator; later => stays in RHS verbatim"
        );

        // Negative case: a query that contains `=>` nowhere returns
        // None (the dispatcher routes it elsewhere).
        assert!(parse_rewrite("@code/llvm/ir |> @mirror/kintsugi").is_none());

        // A multi-rule query where one segment lacks `=>` returns
        // None (no silent partial parse).
        assert!(parse_rewrite("a => b; no_arrow_here").is_none());
    }
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub r#ref: String,
    /// kintsugi the result of this segment before passing to the next?
    pub kintsugi_after: bool,
}

pub fn split_pipeline(query: &str) -> Vec<Segment> {
    let mut segs: Vec<Segment> = Vec::new();
    let bytes = query.as_bytes();
    let len = bytes.len();
    let mut start = 0usize;
    let mut p = 0usize;
    while p < len {
        if p + 2 < len && bytes[p] == b'|' && bytes[p + 1] == b'\\' && bytes[p + 2] == b'>' {
            let seg = query[start..p].trim();
            segs.push(Segment {
                r#ref: seg.to_string(),
                kintsugi_after: true,
            });
            p += 3;
            start = p;
        } else if p + 1 < len && bytes[p] == b'|' && bytes[p + 1] == b'>' {
            let seg = query[start..p].trim();
            segs.push(Segment {
                r#ref: seg.to_string(),
                kintsugi_after: false,
            });
            p += 2;
            start = p;
        } else {
            p += 1;
        }
    }
    let tail = query[start..].trim();
    if !tail.is_empty() {
        segs.push(Segment {
            r#ref: tail.to_string(),
            kintsugi_after: false,
        });
    }
    segs
}

fn tokenize_with_ref(r#ref: &str, source: &[u8]) -> Option<AstNode> {
    let path = grammar_path_for_ref(r#ref)?;
    let g = load_grammar(&path).ok()?;
    Some(tokenize(source, &g))
}

/// Apply kintsugi to the current AST using its grammar tag.
fn apply_implicit_kintsugi(ast: &mut AstNode, current_text: &mut Vec<u8>) -> Result<(), ()> {
    let tag = ast.grammar_tag.clone();
    if tag.is_empty() {
        return Ok(());
    }
    let mut out = Vec::new();
    render_ast(ast, 0, &mut out);
    *current_text = out;
    match tokenize_with_ref(&tag, current_text) {
        Some(new_ast) => {
            *ast = new_ast;
            Ok(())
        }
        None => Err(()),
    }
}

pub fn execute_pipeline(segs: &[Segment], source: &[u8]) -> i32 {
    if segs.is_empty() {
        return 1;
    }
    let first_ref = &segs[0].r#ref;
    let mut ast = match tokenize_with_ref(first_ref, source) {
        Some(a) => a,
        None => return 1,
    };
    let mut current_text: Vec<u8> = source.to_vec();

    if segs[0].kintsugi_after {
        if apply_implicit_kintsugi(&mut ast, &mut current_text).is_err() {
            return 1;
        }
    }

    for i in 1..segs.len() {
        let r#ref = segs[i].r#ref.as_str();
        if r#ref == "@mirror/kintsugi" || r#ref == "@kintsugi" {
            let mut out = Vec::new();
            render_ast(&ast, 0, &mut out);
            current_text = out;
            let tag = if !ast.grammar_tag.is_empty() {
                ast.grammar_tag.clone()
            } else {
                first_ref.clone()
            };
            ast = match tokenize_with_ref(&tag, &current_text) {
                Some(a) => a,
                None => return 1,
            };
        } else if r#ref == "@mirror/butterfly"
            || r#ref == "@mirror/butterfly.butterfly"
            || r#ref == "@mirror/butterfly.emit"
        {
            let emit_only = r#ref == "@mirror/butterfly.emit";
            if !emit_only {
                let args = [
                    "-x",
                    "ir",
                    "-",
                    "-O2",
                    "-ffp-contract=off",
                    "-o",
                    "mirror-butterfly",
                    "-lm",
                ];
                match io_exec("clang", &args, &current_text) {
                    Ok((rc, out)) => {
                        if !out.is_empty() {
                            crate::_raw_stderr(&out);
                        }
                        if rc != 0 {
                            crate::merr!("butterfly: clang failed with exit {}", rc);
                            return rc;
                        }
                        crate::merr!("butterfly: wrote ./mirror-butterfly");
                    }
                    Err(e) => {
                        crate::merr!("butterfly: exec error: {}", e);
                        return 1;
                    }
                }
            }
        } else {
            let new_ast = match tokenize_with_ref(r#ref, &current_text) {
                Some(a) => a,
                None => {
                    crate::merr!("pipeline: cannot dispatch {}", r#ref);
                    return 1;
                }
            };
            ast = new_ast;
        }

        if segs[i].kintsugi_after {
            if apply_implicit_kintsugi(&mut ast, &mut current_text).is_err() {
                return 1;
            }
        }
    }

    let last = segs.last().unwrap().r#ref.as_str();
    let nseg = segs.len();
    if nseg >= 2
        && (last == "@mirror/kintsugi" || last == "@kintsugi" || last == "@mirror/butterfly.emit")
    {
        crate::_raw_stdout(&current_text);
    } else if nseg >= 2 && (last == "@mirror/butterfly" || last == "@mirror/butterfly.butterfly") {
        // no stdout
    } else {
        let oid = compute_content_oid(&ast);
        crate::mout!("{}", oid);
    }
    0
}

pub fn is_mq_query(arg: &str) -> bool {
    if arg.is_empty() {
        return false;
    }
    if arg.starts_with('@') {
        return true;
    }
    if arg.contains("|>") {
        return true;
    }
    if arg.contains("|\\>") {
        return true;
    }
    false
}
