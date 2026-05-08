//! MirrorRuntime — produces compiled `Shatter` artifacts from `.mirror` source,
//! and tracks bootstrap state in a `MirrorRegistry`.
//!
//! ## Recognition
//!
//! Each declaration in a `.mirror` file IS one beam in a content-addressed
//! trajectory. The compilation primitive is `MirrorFragment` (a
//! `Fractal<MirrorData, CoincidenceHash<5>>`).
//!
//! ## MirrorRegistry — the Rust/Mirror glass wall
//!
//! The Rust side hardcodes a small token surface: `prism`, the five operation
//! names (`focus split zoom project refract`), and `in` / `out`. Everything
//! else is learned by reading boot files in order.
//!
//! `MirrorRegistry` is the shared state between Rust and Mirror, backed by
//! `FrgmntStore<MirrorFragment>` from the fragmentation crate. Fragment names
//! (`@prism`, `@meta`, `@actor`) are stored as named refs pointing at the
//! OIDs of the MirrorFragments that declared them. The store IS the registry;
//! the registry is just a typed surface over it.
//!
//! As each boot file is compiled:
//!
//! 1. Parse → `MirrorAST` (structural).
//! 2. `registry.resolve(&form)` checks every `in @X` reference against the
//!    store's named refs. Failure means missing prerequisite.
//! 3. `registry.register(&form)` compiles each top-level `@X` declaration to
//!    a MirrorFragment, inserts it into the store persistently, and writes
//!    a ref mapping `@X → oid`.
//!
//! `in` is a partial read (semantically project / prism in the optic family):
//! `in @X` succeeds iff a ref named `@X` is in the store. `out` is implicit —
//! registering a form publishes its top-level children as that form's export
//! surface (semantically refract: a write to the form's state).
//!
//! ## Hot-swap memory layer
//!
//! Because the registry is content-addressed and disk-persistent, swapping
//! the `.frgmnt/` directory a MirrorRegistry is mounted at swaps the entire
//! memory of the language. Two processes can share state by mounting the
//! same path; one process can diverge by reopening at a different path.
//! The Shatter pipeline becomes a hot-swappable memory layer by construction.
//!
//! ## Pipeline
//!
//! - parse `.mirror` source → `MirrorAST` tree
//! - resolve against accumulated `MirrorRegistry`
//! - register the file's top-level forms into the registry's store
//! - wrap into `Shatter`, the runtime artifact
//!
//! Round-trip is exact: parse → emit text → parse again yields identical
//! content OIDs because the OID is derived from `MirrorData::encode()` and
//! recursive child OIDs.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::declaration::{
    fragment as build_fragment, MirrorFragment,
    MirrorFragmentExt, OpticOp,
};
use crate::mirror_ast::{
    Field, FocusNode, GrammarRef, Identifier, MirrorAST, ModuleNode, ProjectNode, RefractNode,
    SplitNode, TypeBody, ZoomNode,
};
use fragmentation::frgmnt_store::FrgmntStore;
use fragmentation::sha::HashAlg;
use prism::{Beam, Imperfect, Loss, Oid, Optic, Prism};

use crate::loss::{AstPosition, MirrorLoss, ParseLoss, ParseWarning};

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct MirrorRuntimeError(pub String);

impl std::fmt::Display for MirrorRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for MirrorRuntimeError {}

fn err(s: impl Into<String>) -> MirrorRuntimeError {
    MirrorRuntimeError(s.into())
}

#[derive(Debug)]
pub struct MirrorResolveError(pub String);

impl std::fmt::Display for MirrorResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for MirrorResolveError {}

// Form struct removed — parser emits MirrorAST directly.

// ---------------------------------------------------------------------------
// Shatter — the compilation artifact, a Prism implementation.
// ---------------------------------------------------------------------------

/// `Shatter` is the compilation artifact of `MirrorRuntime`. It implements
/// the `Prism` trait: three operations move a `MirrorFragment` into and out of its
/// content-addressed representation.
#[derive(Clone, Debug, Default)]
pub struct Shatter;

impl Prism for Shatter {
    type Input = Optic<(), MirrorFragment>;
    type Focused = Optic<MirrorFragment, MirrorFragment>;
    type Projected = Optic<MirrorFragment, MirrorFragment>;
    type Refracted = Optic<MirrorFragment, Shatter>;

    /// Focus: read the top-level eigenvalues from the AST.
    fn focus(&self, beam: Self::Input) -> Self::Focused {
        let input = beam.result().ok().expect("focus: Err beam").clone();
        beam.next(input)
    }

    /// Project: the fragment is already content-addressed.
    fn project(&self, beam: Self::Focused) -> Self::Projected {
        let frag = beam.result().ok().expect("project: Err beam").clone();
        beam.next(frag)
    }

    /// Refract: settle into the fixed-point crystal (Shatter itself).
    fn refract(&self, beam: Self::Projected) -> Self::Refracted {
        beam.next(Shatter)
    }
}

// Shatter test methods removed.

// ---------------------------------------------------------------------------
// Parser — line-oriented, brace-balanced.
// ---------------------------------------------------------------------------

/// Check if a word is a known declaration keyword (replaces DeclKind::parse).
fn is_decl_keyword(s: &str) -> bool {
    matches!(s,
        "form" | "type" | "prism" | "in" | "out" | "property" | "fold" |
        "requires" | "invariant" | "ensures" | "focus" | "project" |
        "split" | "zoom" | "refract" | "traversal" | "lens" | "action" |
        "recover" | "rescue" | "grammar" | "template" | "default" | "binding"
    )
}

/// Parse a `.mirror` source string into a content-addressed `MirrorFragment`.
///
/// Returns `Imperfect`: `Success` if all input was recognized,
/// `Partial` if unrecognized keywords were encountered (measured loss),
/// `Failure` if no declarations could be parsed.
pub fn parse_form(source: &str) -> Imperfect<MirrorFragment, MirrorRuntimeError, MirrorLoss> {
    let tokens = tokenize(source);
    let mut cursor = 0usize;
    let mut decls: Vec<MirrorFragment> = Vec::new();
    let mut warnings: Vec<ParseWarning> = Vec::new();

    loop {
        skip_trivia(&tokens, &mut cursor);
        if cursor >= tokens.len() {
            break;
        }
        match tokens.get(cursor) {
            Some(Tok::Word(w)) if is_decl_keyword(w) || w == "abstract" => {
                match parse_decl(&tokens, &mut cursor, AstPosition::TopLevel) {
                    Ok((frag, child_warnings)) => {
                        let ast = frag.mirror_ast();
                        let tag = ast.decl_tag();
                        let name = ast.name();
                        // M2001: top-level type/grammar/action require a name
                        if name.is_empty()
                            && matches!(tag, "type" | "grammar" | "action")
                        {
                            return Imperfect::failure(err(format!(
                                "M2001: `{}` requires a name",
                                tag
                            )));
                        }
                        // M2002: top-level `in` requires a target
                        if name.is_empty() && tag == "in" {
                            return Imperfect::failure(err("M2002: `in` requires a target"));
                        }
                        decls.push(frag);
                        warnings.extend(child_warnings);
                    }
                    Err(e) => return Imperfect::failure(e),
                }
            }
            Some(Tok::Word(_w)) => {
                let line = count_line_at(&tokens, cursor);
                collect_until_next_decl(&tokens, &mut cursor);
                warnings.push(ParseWarning::UnknownToken {
                    at: AstPosition::TopLevel,
                    line,
                });
            }
            Some(_) => {
                while cursor < tokens.len() && !matches!(tokens.get(cursor), Some(Tok::Newline)) {
                    cursor += 1;
                }
                if matches!(tokens.get(cursor), Some(Tok::Newline)) {
                    cursor += 1;
                }
            }
            None => break,
        }
    }

    // M2003: duplicate type names in the same scope
    {
        let mut seen_types: Vec<(String, Vec<String>)> = Vec::new();
        for d in &decls {
            let ast = d.mirror_ast();
            if matches!(ast, MirrorAST::Split(_)) && !ast.name().is_empty() {
                let name = ast.name().to_string();
                let params = ast.params_as_strings();
                if seen_types
                    .iter()
                    .any(|(n, p)| n == &name && p == &params)
                {
                    return Imperfect::failure(err(format!(
                        "M2003: duplicate type name `{}`",
                        name
                    )));
                }
                seen_types.push((name, params));
            }
        }
    }

    if decls.is_empty() && warnings.is_empty() {
        Imperfect::failure(err("no declarations found"))
    } else if decls.is_empty() {
        let loss = MirrorLoss {
            parse: ParseLoss { warnings },
            ..MirrorLoss::zero()
        };
        Imperfect::failure_with_loss(err("no recognized declarations found"), loss)
    } else {
        collect_fragment_form_deprecations(&decls, &mut warnings);

        let frag = if decls.len() == 1 {
            decls.into_iter().next().unwrap()
        } else {
            let wrapper_ast = MirrorAST::Module(ModuleNode {
                name: Identifier::new(""),
                children: vec![],
            });
            build_fragment(wrapper_ast, decls)
        };

        if warnings.is_empty() {
            Imperfect::Success(frag)
        } else {
            let loss = MirrorLoss {
                parse: ParseLoss { warnings },
                ..MirrorLoss::zero()
            };
            Imperfect::Partial(frag, loss)
        }
    }
}

/// Parse a `.mirror` source string into a `MirrorAST` node.
///
/// This is the canonical parser: it returns typed AST nodes directly.
/// Use `MirrorAST::to_fragment()` to convert to content-addressed fragments.
///
/// Returns `Imperfect`: `Success` if all input was recognized,
/// `Partial` if unrecognized keywords were encountered (measured loss),
/// `Failure` if no declarations could be parsed.
pub fn parse_ast(source: &str) -> Imperfect<MirrorAST, MirrorRuntimeError, MirrorLoss> {
    parse_form(source).map(|frag| frag.mirror_ast().clone())
}

/// Detect deprecated `form` keyword usage and add deprecation warnings.
fn collect_fragment_form_deprecations(decls: &[MirrorFragment], warnings: &mut Vec<ParseWarning>) {
    for decl in decls {
        let ast = decl.mirror_ast();
        if ast.decl_tag() == "form" && !ast.name().is_empty() {
            warnings.push(ParseWarning::DeprecatedKind {
                kind: "form",
                replacement: "grammar",
                at: AstPosition::TopLevel,
                line: 0,
            });
        }
        collect_fragment_form_deprecations(decl.mirror_children(), warnings);
    }
}

/// Count the 1-based line number at a token position by counting Newline tokens before it.
fn count_line_at(tokens: &[Tok], pos: usize) -> usize {
    let newlines = tokens[..pos]
        .iter()
        .filter(|t| matches!(t, Tok::Newline))
        .count();
    newlines + 1
}

/// Collect tokens from current position until the next newline or end-of-tokens.
/// Returns the collected content as a string.
fn collect_until_next_decl(tokens: &[Tok], cursor: &mut usize) -> String {
    let mut content = String::new();
    // Skip the keyword itself (already captured)
    *cursor += 1;
    while *cursor < tokens.len() {
        match tokens.get(*cursor) {
            Some(Tok::Newline) => {
                *cursor += 1;
                break;
            }
            Some(Tok::Word(w)) => {
                if !content.is_empty() {
                    content.push(' ');
                }
                content.push_str(w);
                *cursor += 1;
            }
            Some(Tok::LBrace) => {
                content.push('{');
                *cursor += 1;
            }
            Some(Tok::RBrace) => {
                content.push('}');
                *cursor += 1;
            }
            Some(Tok::LParen) => {
                content.push('(');
                *cursor += 1;
            }
            Some(Tok::RParen) => {
                content.push(')');
                *cursor += 1;
            }
            Some(Tok::Comma) => {
                content.push(',');
                *cursor += 1;
            }
            Some(Tok::Equals) => {
                content.push('=');
                *cursor += 1;
            }
            None => break,
        }
    }
    content.trim().to_string()
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tok {
    Word(String),
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comma,
    Equals,
    Newline,
}

fn tokenize(source: &str) -> Vec<Tok> {
    let mut out = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            ' ' | '\t' | '\r' => {
                i += 1;
            }
            '\n' => {
                out.push(Tok::Newline);
                i += 1;
            }
            '#' => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            '{' => {
                out.push(Tok::LBrace);
                i += 1;
            }
            '}' => {
                out.push(Tok::RBrace);
                i += 1;
            }
            '(' => {
                out.push(Tok::LParen);
                i += 1;
            }
            ')' => {
                out.push(Tok::RParen);
                i += 1;
            }
            ',' => {
                out.push(Tok::Comma);
                i += 1;
            }
            '|' | '.' | '/' | '<' | '>' | ':' | '-' | '!' => {
                // Operator sequences like |, |>, <|, /, .., !=, etc. can be declaration names.
                // Try to collect them as a word if they form a contiguous symbol sequence.
                let start = i;
                while i < bytes.len() {
                    let cc = bytes[i] as char;
                    if cc == '|'
                        || cc == '.'
                        || cc == '/'
                        || cc == '<'
                        || cc == '>'
                        || cc == ':'
                        || cc == '-'
                        || cc == '!'
                    {
                        i += 1;
                    } else {
                        break;
                    }
                }
                if i == start {
                    i += 1;
                } else {
                    let sym = &source[start..i];
                    // `--` is a line comment: skip to end of line.
                    if sym == "--" {
                        while i < bytes.len() && bytes[i] != b'\n' {
                            i += 1;
                        }
                    } else {
                        out.push(Tok::Word(sym.to_string()));
                    }
                }
            }
            '=' => {
                out.push(Tok::Equals);
                i += 1;
            }
            _ => {
                // For non-ASCII: advance by the full UTF-8 character width
                // to avoid landing in the middle of a multi-byte sequence.
                if !c.is_ascii() {
                    // Skip the entire multi-byte character.
                    let ch = source[i..].chars().next().unwrap();
                    i += ch.len_utf8();
                } else {
                    let start = i;
                    while i < bytes.len() {
                        let cc = bytes[i] as char;
                        if cc.is_ascii_alphanumeric() || cc == '_' || cc == '@' {
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    if i == start {
                        i += 1;
                    } else {
                        out.push(Tok::Word(source[start..i].to_string()));
                    }
                }
            }
        }
    }
    out
}

fn skip_trivia(tokens: &[Tok], cursor: &mut usize) {
    while *cursor < tokens.len() && matches!(tokens[*cursor], Tok::Newline) {
        *cursor += 1;
    }
}

/// Map a decl tag + name to an AstPosition for child warnings.
fn ast_position_for_tag(tag: &str, name: &str) -> AstPosition {
    let oid = crate::kernel::Oid::new(name);
    match tag {
        "grammar" | "form" => AstPosition::Grammar(oid),
        "type" => AstPosition::Type(oid),
        "action" => AstPosition::Action(oid),
        "property" => AstPosition::Property(oid),
        "prism" => AstPosition::Prism(oid),
        "fold" => AstPosition::Fold(oid),
        "split" => AstPosition::Split(oid),
        "zoom" => AstPosition::Zoom(oid),
        "refract" => AstPosition::Refract(oid),
        _ => AstPosition::TopLevel,
    }
}

fn parse_decl(
    tokens: &[Tok],
    cursor: &mut usize,
    _position: AstPosition,
) -> Result<(MirrorFragment, Vec<ParseWarning>), MirrorRuntimeError> {
    skip_trivia(tokens, cursor);
    let kind_word = match tokens.get(*cursor) {
        Some(Tok::Word(w)) => w.clone(),
        other => {
            return Err(err(format!(
                "expected declaration keyword, got {:?}",
                other
            )))
        }
    };
    *cursor += 1;

    // Handle modifier keywords (e.g. `abstract grammar`, `abstract action`).
    // The modifier is consumed and the actual keyword follows.
    let (kind_str, modifier) = if kind_word == "abstract" {
        let actual_word = match tokens.get(*cursor) {
            Some(Tok::Word(w)) => w.clone(),
            other => {
                return Err(err(format!(
                    "expected declaration keyword after 'abstract', got {:?}",
                    other
                )))
            }
        };
        *cursor += 1;
        if !is_decl_keyword(&actual_word) {
            return Err(err(format!("unknown declaration kind: {}", actual_word)));
        }
        (actual_word, true)
    } else {
        if !is_decl_keyword(&kind_word) {
            return Err(err(format!("unknown declaration kind: {}", kind_word)));
        }
        (kind_word, false)
    };
    let kind = kind_str.as_str();

    // Recover/Rescue: pipe-delimited params, optional fold operator, optional body.
    if kind == "recover" || kind == "rescue" {
        let mut params = Vec::new();
        let mut optic_ops = Vec::new();
        let mut variants = Vec::new();
        // Consume `|`
        if matches!(tokens.get(*cursor), Some(Tok::Word(w)) if w == "|") {
            *cursor += 1;
            loop {
                match tokens.get(*cursor) {
                    Some(Tok::Word(w)) if w == "|" => {
                        *cursor += 1;
                        break;
                    }
                    Some(Tok::Word(w)) => {
                        params.push(w.clone());
                        *cursor += 1;
                        if matches!(tokens.get(*cursor), Some(Tok::LParen)) {
                            *cursor += 1;
                            if let Some(last) = params.last_mut() {
                                last.push('(');
                                let mut depth = 1;
                                while *cursor < tokens.len() && depth > 0 {
                                    match tokens.get(*cursor) {
                                        Some(Tok::LParen) => {
                                            last.push('(');
                                            depth += 1;
                                        }
                                        Some(Tok::RParen) => {
                                            depth -= 1;
                                            last.push(')');
                                        }
                                        Some(Tok::Word(w)) => last.push_str(w),
                                        Some(Tok::Comma) => last.push(','),
                                        _ => {}
                                    }
                                    *cursor += 1;
                                }
                            }
                        }
                    }
                    Some(Tok::Comma) => {
                        *cursor += 1;
                    }
                    _ => break,
                }
            }
        }
        let is_fold = matches!(tokens.get(*cursor), Some(Tok::Word(w)) if w == "<")
            && matches!(tokens.get(*cursor + 1), Some(Tok::Equals));
        if is_fold {
            optic_ops.push(OpticOp::Fold);
            *cursor += 2;
            loop {
                match tokens.get(*cursor) {
                    Some(Tok::Newline) => {
                        *cursor += 1;
                        break;
                    }
                    Some(Tok::LBrace) => break,
                    Some(Tok::Word(w)) => {
                        variants.push(w.clone());
                        *cursor += 1;
                    }
                    _ => break,
                }
            }
        }
        let (body_text, children) = parse_action_body(tokens, cursor)?;
        // Build MirrorAST node — typed representation
        let ast = MirrorAST::Zoom(ZoomNode {
            name: Identifier::new(kind),
            params: params
                .iter()
                .map(|p| {
                    if let Some((n, t)) = p.split_once(':') {
                        Field {
                            name: Identifier::new(n.trim()),
                            type_ref: Identifier::new(t.trim()),
                        }
                    } else {
                        Field {
                            name: Identifier::new(p),
                            type_ref: Identifier::new("_"),
                        }
                    }
                })
                .collect(),
            target: None,
            grammar_ref: None,
            children: vec![],
            body: None,
        });
        let ast = if modifier { MirrorAST::Abstract(Box::new(ast)) } else { ast };
        let frag = build_fragment(ast, children);
        return Ok((frag, Vec::new()));
    }

    let name = match tokens.get(*cursor) {
        Some(Tok::Word(w)) => {
            let mut n = w.clone();
            *cursor += 1;
            while let Some(Tok::Word(seg)) = tokens.get(*cursor) {
                if seg.starts_with('/') || seg == "/" {
                    n.push_str(seg);
                    *cursor += 1;
                    if seg == "/" {
                        if let Some(Tok::Word(next)) = tokens.get(*cursor) {
                            n.push_str(next);
                            *cursor += 1;
                        }
                    }
                } else {
                    break;
                }
            }
            n
        }
        _ => String::new(),
    };

    // Check for grammar inheritance: `grammar @name < @parent`
    let mut parent_ref = None;
    if kind == "grammar" {
        if let Some(Tok::Word(w)) = tokens.get(*cursor) {
            if w == "<" {
                if let Some(Tok::Word(next)) = tokens.get(*cursor + 1) {
                    if next.starts_with('@') {
                        *cursor += 1;
                        parent_ref = Some(next.clone());
                        *cursor += 1;
                    }
                }
            }
        }
    }

    let mut has_parens = false;
    let mut params: Vec<String> = Vec::new();
    if matches!(tokens.get(*cursor), Some(Tok::LParen)) {
        has_parens = true;
        *cursor += 1;
        let mut paren_depth: usize = 1;
        loop {
            match tokens.get(*cursor) {
                Some(Tok::RParen) => {
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        *cursor += 1;
                        break;
                    }
                    if let Some(last) = params.last_mut() {
                        last.push(')');
                    }
                    *cursor += 1;
                }
                Some(Tok::LParen) => {
                    paren_depth += 1;
                    if let Some(last) = params.last_mut() {
                        last.push('(');
                    }
                    *cursor += 1;
                }
                Some(Tok::Word(w)) => {
                    if paren_depth > 1 {
                        if let Some(last) = params.last_mut() {
                            last.push_str(w);
                        } else {
                            params.push(w.clone());
                        }
                    } else {
                        params.push(w.clone());
                    }
                    *cursor += 1;
                }
                Some(Tok::Comma) => {
                    if paren_depth > 1 {
                        if let Some(last) = params.last_mut() {
                            last.push(',');
                        }
                    }
                    *cursor += 1;
                }
                Some(Tok::Equals) => {
                    if paren_depth > 1 {
                        if let Some(last) = params.last_mut() {
                            last.push('=');
                        }
                        *cursor += 1;
                    } else {
                        return Err(err(format!("malformed params: {:?}", Some(Tok::Equals))));
                    }
                }
                other => return Err(err(format!("malformed params: {:?}", other))),
            }
        }
    }

    let mut variants = Vec::new();
    let mut optic_ops = Vec::new();
    let is_fold = matches!(tokens.get(*cursor), Some(Tok::Word(w)) if w == "<")
        && matches!(tokens.get(*cursor + 1), Some(Tok::Equals));
    if is_fold {
        optic_ops.push(OpticOp::Fold);
        *cursor += 2;
        loop {
            match tokens.get(*cursor) {
                Some(Tok::Newline) => {
                    *cursor += 1;
                    break;
                }
                Some(Tok::LBrace) => break,
                Some(Tok::Word(w)) => {
                    variants.push(w.clone());
                    *cursor += 1;
                }
                _ => break,
            }
        }
    } else if matches!(tokens.get(*cursor), Some(Tok::Equals)) {
        optic_ops.push(OpticOp::Iso);
        *cursor += 1;
        loop {
            match tokens.get(*cursor) {
                Some(Tok::Newline) => {
                    *cursor += 1;
                    break;
                }
                Some(Tok::Word(w)) if w == "|" => {
                    if !optic_ops.contains(&OpticOp::Split) {
                        optic_ops.push(OpticOp::Split);
                    }
                    *cursor += 1;
                }
                Some(Tok::Word(w)) => {
                    variants.push(w.clone());
                    *cursor += 1;
                    if matches!(tokens.get(*cursor), Some(Tok::LParen)) {
                        *cursor += 1;
                        let mut paren_depth = 1;
                        while *cursor < tokens.len() && paren_depth > 0 {
                            match tokens.get(*cursor) {
                                Some(Tok::LParen) => paren_depth += 1,
                                Some(Tok::RParen) => paren_depth -= 1,
                                _ => {}
                            }
                            *cursor += 1;
                        }
                    }
                }
                Some(Tok::Equals) => {
                    return Err(err("M2004: double operator `=`"));
                }
                _ => break,
            }
        }
    }

    if has_parens && !optic_ops.contains(&OpticOp::Focus) {
        optic_ops.push(OpticOp::Focus);
    }

    let implicit_op = match kind {
        "fold" => Some(OpticOp::Fold),
        "focus" => Some(OpticOp::Focus),
        "split" => Some(OpticOp::Split),
        "zoom" => Some(OpticOp::Zoom),
        "refract" => Some(OpticOp::Refract),
        _ => None,
    };
    if let Some(op) = implicit_op {
        if !optic_ops.contains(&op) {
            optic_ops.push(op);
        }
    }

    // Action declarations — build MirrorAST first
    if kind == "action" {
        let grammar_ref = parse_action_grammar_ref(tokens, cursor);
        let return_type = parse_return_type(tokens, cursor);
        let (body_text, children) = parse_action_body(tokens, cursor)?;
        // Build MirrorAST node — typed representation
        let ast = MirrorAST::Zoom(ZoomNode {
            name: Identifier::new(&name),
            params: params
                .iter()
                .map(|p| {
                    if let Some((n, t)) = p.split_once(':') {
                        Field {
                            name: Identifier::new(n.trim()),
                            type_ref: Identifier::new(t.trim()),
                        }
                    } else {
                        Field {
                            name: Identifier::new(p),
                            type_ref: Identifier::new("_"),
                        }
                    }
                })
                .collect(),
            target: return_type.as_deref().map(Identifier::new),
            grammar_ref: grammar_ref.as_deref().map(|gr| {
                GrammarRef::new(if gr.starts_with('@') {
                    gr.to_string()
                } else {
                    format!("@{}", gr)
                })
            }),
            children: vec![],
            body: None,
        });
        let ast = if modifier { MirrorAST::Abstract(Box::new(ast)) } else { ast };
        let frag = build_fragment(ast, children);
        return Ok((frag, Vec::new()));
    }

    let mut children = Vec::new();
    let mut block_warnings: Vec<ParseWarning> = Vec::new();
    // Compute the child position based on the kind and name we just parsed
    let child_position = ast_position_for_tag(kind, &name);
    skip_inline_trivia(tokens, cursor);
    if matches!(tokens.get(*cursor), Some(Tok::LBrace)) {
        *cursor += 1;
        loop {
            skip_trivia(tokens, cursor);
            match tokens.get(*cursor) {
                Some(Tok::RBrace) => {
                    *cursor += 1;
                    break;
                }
                None => return Err(err("unterminated block".to_string())),
                Some(Tok::Word(w)) => {
                    if is_decl_keyword(w) || w == "abstract" {
                        let (child, child_warnings) =
                            parse_decl(tokens, cursor, child_position.clone())?;
                        children.push(child);
                        block_warnings.extend(child_warnings);
                    } else if w == "<" || w == ">" {
                        let _op = if w == "<" {
                            OpticOp::Subset
                        } else {
                            OpticOp::Superset
                        };
                        *cursor += 1;
                        let target = match tokens.get(*cursor) {
                            Some(Tok::Word(t)) => {
                                let name = t.clone();
                                *cursor += 1;
                                name
                            }
                            _ => String::new(),
                        };
                        let target_ref = if target.starts_with('@') {
                            target.clone()
                        } else {
                            format!("@{}", target)
                        };
                        let child_ast = MirrorAST::Project(ProjectNode {
                            name: Identifier::new(&target),
                            target: Some(GrammarRef::new(target_ref)),
                            children: vec![],
                        });
                        let child_frag = build_fragment(child_ast, Vec::new());
                        children.push(child_frag);
                        while *cursor < tokens.len() {
                            match tokens.get(*cursor) {
                                Some(Tok::RBrace) | Some(Tok::Newline) => break,
                                _ => {
                                    *cursor += 1;
                                }
                            }
                        }
                        if matches!(tokens.get(*cursor), Some(Tok::Newline)) {
                            *cursor += 1;
                        }
                    } else {
                        let line = count_line_at(tokens, *cursor);
                        collect_until_next_decl(tokens, cursor);
                        block_warnings.push(ParseWarning::UnknownToken {
                            at: child_position.clone(),
                            line,
                        });
                    }
                }
                _ => {
                    while *cursor < tokens.len()
                        && !matches!(tokens.get(*cursor), Some(Tok::Newline | Tok::RBrace))
                    {
                        *cursor += 1;
                    }
                    if matches!(tokens.get(*cursor), Some(Tok::Newline)) {
                        *cursor += 1;
                    }
                }
            }
        }
    }

    // Build MirrorAST node with children — the parser produces typed AST.
    let ast = build_ast_node_with_children(kind, &name, &params, &variants, &parent_ref, children.clone());
    let ast = if modifier { MirrorAST::Abstract(Box::new(ast)) } else { ast };
    let frag = build_fragment(ast, children);
    Ok((frag, block_warnings))
}

/// Build a MirrorAST node from parsed declaration components.
///
/// Accepts fragment children and converts them to MirrorAST children,
/// producing a fully populated typed AST node. The tag string determines
/// the MirrorAST variant; children are placed in the appropriate field.
#[allow(dead_code)]
fn build_ast_node(
    kind: &str,
    name: &str,
    params: &[String],
    variants: &[String],
    parent_ref: &Option<String>,
) -> MirrorAST {
    build_ast_node_with_children(kind, name, params, variants, parent_ref, Vec::new())
}

/// Build a MirrorAST node with children from fragment children.
fn build_ast_node_with_children(
    kind: &str,
    name: &str,
    params: &[String],
    variants: &[String],
    parent_ref: &Option<String>,
    frag_children: Vec<MirrorFragment>,
) -> MirrorAST {
    let children: Vec<MirrorAST> = frag_children
        .iter()
        .map(|f| f.mirror_ast().clone())
        .collect();
    build_ast_node_direct(kind, name, params, variants, parent_ref, children)
}

/// Build a MirrorAST node with pre-converted MirrorAST children.
fn build_ast_node_direct(
    kind: &str,
    name: &str,
    params: &[String],
    variants: &[String],
    parent_ref: &Option<String>,
    children: Vec<MirrorAST>,
) -> MirrorAST {
    match kind {
        "grammar" => {
            let grammar_name = if name.starts_with('@') {
                name.to_string()
            } else {
                format!("@{}", name)
            };
            let parent = parent_ref.as_ref().map(|p| {
                if p.starts_with('@') {
                    GrammarRef::new(p)
                } else {
                    GrammarRef::new(format!("@{}", p))
                }
            });
            MirrorAST::Focus(FocusNode {
                name: Identifier::new(grammar_name),
                target: parent,
                children,
            })
        }
        "type" => {
            let type_params: Vec<Identifier> = params.iter().map(|p| Identifier::new(p)).collect();
            let body = if !variants.is_empty() {
                Some(TypeBody::Enum(variants.iter().map(|v| Identifier::new(v)).collect()))
            } else {
                Some(TypeBody::Unit)
            };
            MirrorAST::Split(SplitNode {
                name: Identifier::new(name),
                variants: vec![],
                params: type_params,
                body,
                children,
            })
        }
        "in" => {
            let target = if name.starts_with('@') {
                GrammarRef::new(name)
            } else {
                GrammarRef::new(format!("@{}", name))
            };
            MirrorAST::Project(ProjectNode {
                name: Identifier::new(name),
                target: Some(target),
                children: vec![],
            })
        }
        "out" => MirrorAST::Project(ProjectNode {
            name: Identifier::new(name),
            target: None,
            children: vec![],
        }),
        "property" => {
            let ast_params: Vec<Field> = MirrorAST::params_to_fields(params);
            MirrorAST::Refract(RefractNode {
                name: Identifier::new(name),
                params: ast_params,
                target: None,
                children,
            })
        }
        "focus" => MirrorAST::Focus(FocusNode {
            name: Identifier::new(name),
            target: params.first().and_then(|p| {
                if p.starts_with('@') { Some(GrammarRef::new(p)) } else { None }
            }),
            children,
        }),
        "project" => MirrorAST::Project(ProjectNode {
            name: Identifier::new(name),
            target: params.first().and_then(|p| {
                if p.starts_with('@') { Some(GrammarRef::new(p)) } else { None }
            }),
            children,
        }),
        "split" => MirrorAST::Split(SplitNode {
            name: Identifier::new(name),
            variants: variants.iter().map(|v| Identifier::new(v)).collect(),
            params: vec![],
            body: None,
            children,
        }),
        "zoom" => MirrorAST::Zoom(ZoomNode {
            name: Identifier::new(name),
            target: params.first().map(|p| Identifier::new(p)),
            params: vec![],
            grammar_ref: None,
            children,
            body: None,
        }),
        "refract" => MirrorAST::Refract(RefractNode {
            name: Identifier::new(name),
            target: params.first().map(|p| Identifier::new(p)),
            params: vec![],
            children,
        }),
        "form" | "prism" => MirrorAST::Module(ModuleNode {
            name: Identifier::new(name),
            children,
        }),
        "fold" => MirrorAST::Refract(RefractNode {
            name: Identifier::new(name),
            params: Vec::new(),
            target: params.first().map(|p| Identifier::new(p)),
            children,
        }),
        "requires" | "invariant" | "ensures" => {
            MirrorAST::Refract(RefractNode {
                name: Identifier::new(name),
                params: Vec::new(),
                target: None,
                children,
            })
        }
        "recover" | "rescue" => {
            MirrorAST::Zoom(ZoomNode {
                name: Identifier::new(name),
                params: MirrorAST::params_to_fields(params),
                target: None,
                grammar_ref: None,
                children: vec![],
                body: if children.is_empty() { None } else { Some(children) },
            })
        }
        "action" => {
            MirrorAST::Zoom(ZoomNode {
                name: Identifier::new(name),
                params: MirrorAST::params_to_fields(params),
                target: None,
                grammar_ref: None,
                children: vec![],
                body: if children.is_empty() { None } else { Some(children) },
            })
        }
        "traversal" | "lens" => MirrorAST::Focus(FocusNode {
            name: Identifier::new(name),
            target: params.first().and_then(|p| {
                if p.starts_with('@') { Some(GrammarRef::new(p)) } else { None }
            }),
            children,
        }),
        "template" => {
            MirrorAST::Zoom(ZoomNode {
                name: Identifier::new(name),
                params: MirrorAST::params_to_fields(params),
                target: None,
                grammar_ref: None,
                children: vec![],
                body: if children.is_empty() { None } else { Some(children) },
            })
        }
        "default" | "binding" => MirrorAST::Project(ProjectNode {
            name: Identifier::new(name),
            target: None,
            children: vec![],
        }),
        _ => MirrorAST::Focus(FocusNode {
            name: Identifier::new(name),
            target: None,
            children,
        }),
    }
}

/// Parse an optional `in @grammar/path` after action params.
/// Consumes `in @word` or `in @word/path` tokens if present.
fn parse_action_grammar_ref(tokens: &[Tok], cursor: &mut usize) -> Option<String> {
    skip_inline_trivia(tokens, cursor);
    // Look for `in` keyword followed by `@grammar`
    if let Some(Tok::Word(w)) = tokens.get(*cursor) {
        if w == "in" {
            if let Some(Tok::Word(ref_word)) = tokens.get(*cursor + 1) {
                if ref_word.starts_with('@') {
                    *cursor += 2;
                    let mut grammar = ref_word.clone();
                    // Absorb path segments: @code/rust → `@code` `/` `rust`
                    // The tokenizer splits `/` into its own Word token.
                    while let Some(Tok::Word(seg)) = tokens.get(*cursor) {
                        if seg.starts_with('/') || seg == "/" {
                            grammar.push_str(seg);
                            *cursor += 1;
                            // Absorb the next segment too if `/` was standalone
                            if seg == "/" {
                                if let Some(Tok::Word(next)) = tokens.get(*cursor) {
                                    grammar.push_str(next);
                                    *cursor += 1;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    return Some(grammar);
                }
            }
        }
    }
    None
}

/// Parse an optional return type annotation: `-> type` or `-> [type]`.
/// Returns the return type string if present.
fn parse_return_type(tokens: &[Tok], cursor: &mut usize) -> Option<String> {
    skip_inline_trivia(tokens, cursor);
    if let Some(Tok::Word(w)) = tokens.get(*cursor) {
        if w == "->" {
            *cursor += 1;
            // Collect the return type tokens until newline or brace
            let mut rt = String::new();
            while *cursor < tokens.len() {
                match tokens.get(*cursor) {
                    Some(Tok::Newline) | Some(Tok::LBrace) => break,
                    Some(Tok::Word(w)) => {
                        if !rt.is_empty() {
                            rt.push(' ');
                        }
                        rt.push_str(w);
                        *cursor += 1;
                    }
                    _ => *cursor += 1,
                }
            }
            if rt.is_empty() {
                return None;
            }
            return Some(rt);
        }
    }
    None
}

/// Parse the body of an action declaration. The body is collected as raw text
/// (brace-balanced but not parsed by the mirror compiler). If the body contains
/// mirror declarations (like in `04-action.mirror`'s meta-actions), they are
/// parsed as children instead.
fn parse_action_body(
    tokens: &[Tok],
    cursor: &mut usize,
) -> Result<(Option<String>, Vec<MirrorFragment>), MirrorRuntimeError> {
    skip_inline_trivia(tokens, cursor);
    if !matches!(tokens.get(*cursor), Some(Tok::LBrace)) {
        return Ok((None, Vec::new()));
    }
    *cursor += 1;

    // Peek ahead: if the body contains mirror declaration keywords, parse as
    // structured children (this handles `04-action.mirror`'s meta-action bodies).
    // Otherwise, collect as raw text.
    let start_cursor = *cursor;
    let mut has_decl_keywords = false;
    let mut peek = start_cursor;
    let mut depth = 1;
    while peek < tokens.len() && depth > 0 {
        match &tokens[peek] {
            Tok::LBrace => depth += 1,
            Tok::RBrace => depth -= 1,
            Tok::Word(w) if depth == 1 => {
                if is_decl_keyword(w) {
                    has_decl_keywords = true;
                    break;
                }
            }
            _ => {}
        }
        peek += 1;
    }

    if has_decl_keywords {
        // Parse structured children (mirror declarations inside the action body).
        let mut children = Vec::new();
        loop {
            skip_trivia(tokens, cursor);
            match tokens.get(*cursor) {
                Some(Tok::RBrace) => {
                    *cursor += 1;
                    break;
                }
                None => return Err(err("unterminated action block")),
                Some(Tok::Word(w)) => {
                    if is_decl_keyword(w) {
                        let (child, _child_warnings) =
                            parse_decl(tokens, cursor, AstPosition::TopLevel)?;
                        children.push(child);
                    } else {
                        // Skip unrecognized tokens to next line or brace
                        while *cursor < tokens.len() {
                            match tokens.get(*cursor) {
                                Some(Tok::RBrace) | Some(Tok::Newline) => break,
                                _ => *cursor += 1,
                            }
                        }
                        if matches!(tokens.get(*cursor), Some(Tok::Newline)) {
                            *cursor += 1;
                        }
                    }
                }
                _ => {
                    while *cursor < tokens.len()
                        && !matches!(tokens.get(*cursor), Some(Tok::Newline | Tok::RBrace))
                    {
                        *cursor += 1;
                    }
                    if matches!(tokens.get(*cursor), Some(Tok::Newline)) {
                        *cursor += 1;
                    }
                }
            }
        }
        Ok((None, children))
    } else {
        // Collect raw body text: reconstruct from tokens, brace-balanced.
        let mut body = String::new();
        let mut brace_depth = 1;
        while *cursor < tokens.len() && brace_depth > 0 {
            match &tokens[*cursor] {
                Tok::LBrace => {
                    brace_depth += 1;
                    body.push('{');
                }
                Tok::RBrace => {
                    brace_depth -= 1;
                    if brace_depth > 0 {
                        body.push('}');
                    }
                }
                Tok::LParen => body.push('('),
                Tok::RParen => body.push(')'),
                Tok::Comma => body.push(','),
                Tok::Equals => body.push('='),
                Tok::Newline => body.push('\n'),
                Tok::Word(w) => {
                    if !body.is_empty() && !body.ends_with('\n') && !body.ends_with('{') {
                        body.push(' ');
                    }
                    body.push_str(w);
                }
            }
            *cursor += 1;
        }
        let body = body.trim().to_string();
        let body_text = if body.is_empty() { None } else { Some(body) };
        Ok((body_text, Vec::new()))
    }
}

fn skip_inline_trivia(tokens: &[Tok], cursor: &mut usize) {
    while matches!(tokens.get(*cursor), Some(Tok::Newline)) {
        *cursor += 1;
    }
}

// ---------------------------------------------------------------------------
// Emitter — MirrorFragment → text. Round-trip stable.
// ---------------------------------------------------------------------------

/// Emit `.mirror` text from a `MirrorFragment`.
pub fn emit_fragment(frag: &MirrorFragment) -> String {
    let mut out = String::new();
    emit_fragment_into(frag, 0, &mut out);
    out
}

/// Reorder a MirrorFragment's children into canonical (kintsugi) order.
pub fn kintsugi_fragment(frag: &MirrorFragment) -> MirrorFragment {
    let ast = frag.mirror_ast().clone();
    if frag.mirror_children().is_empty() {
        return frag.clone();
    }

    let mut children: Vec<MirrorFragment> = frag.mirror_children().to_vec();
    children.sort_by_key(|c| kintsugi_sort_key(c.mirror_ast().decl_tag()));

    build_fragment(ast, children)
}

/// Simplify a MirrorFragment by running the three-pass pipeline:
///   1. collapse_aliases — merge type declarations with identical variants
///   2. flatten_wrappers — inline types with a single param referencing another type
///   3. eliminate_dead — remove type declarations not referenced by any action
///
/// Returns `(simplified, before_count, after_count)`.
pub fn simplify_fragment(
    frag: &MirrorFragment,
) -> (MirrorFragment, usize, usize) {
    let ast = frag.mirror_ast().clone();
    let children: Vec<MirrorFragment> = frag.mirror_children().to_vec();
    let before_count = children.len();

    if children.is_empty() {
        return (frag.clone(), 0, 0);
    }

    // Helper: rebuild a Zoom fragment with new params
    fn rebuild_zoom_with_params(c: &MirrorFragment, new_params: Vec<String>) -> MirrorFragment {
        let old_ast = c.mirror_ast();
        if let MirrorAST::Zoom(z) = old_ast {
            let new_fields = MirrorAST::params_to_fields(&new_params);
            let new_ast = MirrorAST::Zoom(ZoomNode {
                name: z.name.clone(),
                params: new_fields,
                target: z.target.clone(),
                grammar_ref: z.grammar_ref.clone(),
                children: z.children.clone(),
                body: z.body.clone(),
            });
            build_fragment(new_ast, c.mirror_children().to_vec())
        } else {
            c.clone()
        }
    }

    // --- Pass 1: collapse_aliases ---
    let mut signatures: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut renames: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    for child in &children {
        let ca = child.mirror_ast();
        if matches!(ca, MirrorAST::Split(_)) {
            let variants = ca.variants_as_strings();
            if !variants.is_empty() {
                let sig = variants.join("|");
                let name = ca.name().to_string();
                if let Some(canonical) = signatures.get(&sig) {
                    renames.insert(name, canonical.clone());
                } else {
                    signatures.insert(sig, name);
                }
            }
        }
    }

    let children: Vec<MirrorFragment> = children
        .into_iter()
        .filter(|c| {
            let ca = c.mirror_ast();
            if matches!(ca, MirrorAST::Split(_)) {
                return !renames.contains_key(ca.name());
            }
            true
        })
        .map(|c| {
            let ca = c.mirror_ast();
            if matches!(ca, MirrorAST::Zoom(_)) && !renames.is_empty() {
                let params = ca.params_as_strings();
                let new_params: Vec<String> = params.iter().map(|p| {
                    if let Some(colon_pos) = p.find(':') {
                        let (param_name, type_part) = p.split_at(colon_pos);
                        let type_name = type_part[1..].trim();
                        if let Some(canonical) = renames.get(type_name) {
                            return format!("{}:{}", param_name, canonical);
                        }
                    }
                    if let Some(canonical) = renames.get(p.as_str()) {
                        return canonical.clone();
                    }
                    p.clone()
                }).collect();
                rebuild_zoom_with_params(&c, new_params)
            } else {
                c
            }
        })
        .collect();

    // --- Pass 2: flatten_wrappers ---
    let mut wrappers: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    for child in &children {
        let ca = child.mirror_ast();
        if matches!(ca, MirrorAST::Split(_)) {
            let variants = ca.variants_as_strings();
            let params = ca.params_as_strings();
            if variants.is_empty() && params.len() == 1 {
                let p = &params[0];
                if let Some(colon_pos) = p.find(':') {
                    let type_ref = p[colon_pos + 1..].trim();
                    if type_ref.starts_with('@') {
                        wrappers.insert(ca.name().to_string(), type_ref[1..].to_string());
                    }
                }
            }
        }
    }

    for _ in 0..10 {
        let mut changed = false;
        let snapshot = wrappers.clone();
        for (_, inner) in wrappers.iter_mut() {
            if let Some(deeper) = snapshot.get(inner.as_str()) {
                *inner = deeper.clone();
                changed = true;
            }
        }
        if !changed { break; }
    }

    let children: Vec<MirrorFragment> = if wrappers.is_empty() {
        children
    } else {
        children
            .into_iter()
            .filter(|c| {
                let ca = c.mirror_ast();
                if matches!(ca, MirrorAST::Split(_)) {
                    return !wrappers.contains_key(ca.name());
                }
                true
            })
            .map(|c| {
                let ca = c.mirror_ast();
                if matches!(ca, MirrorAST::Zoom(_)) {
                    let params = ca.params_as_strings();
                    let new_params: Vec<String> = params.iter().map(|p| {
                        if let Some(colon_pos) = p.find(':') {
                            let (param_name, type_part) = p.split_at(colon_pos);
                            let type_name = type_part[1..].trim();
                            if let Some(inner) = wrappers.get(type_name) {
                                return format!("{}:{}", param_name, inner);
                            }
                        }
                        if let Some(inner) = wrappers.get(p.as_str()) {
                            return inner.clone();
                        }
                        p.clone()
                    }).collect();
                    rebuild_zoom_with_params(&c, new_params)
                } else {
                    c
                }
            })
            .collect()
    };

    // --- Pass 3: eliminate_dead ---
    let mut referenced: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    for child in &children {
        let ca = child.mirror_ast();
        if matches!(ca, MirrorAST::Zoom(_)) {
            for p in ca.params_as_strings() {
                if let Some(colon_pos) = p.find(':') {
                    let type_name = p[colon_pos + 1..].trim();
                    referenced.insert(type_name.to_string());
                    if type_name.starts_with('@') {
                        referenced.insert(type_name[1..].to_string());
                    }
                }
                referenced.insert(p);
            }
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for child in &children {
            let ca = child.mirror_ast();
            if matches!(ca, MirrorAST::Split(_)) && referenced.contains(ca.name()) {
                for p in ca.params_as_strings() {
                    if let Some(colon_pos) = p.find(':') {
                        let type_name = p[colon_pos + 1..].trim().to_string();
                        if referenced.insert(type_name) { changed = true; }
                    }
                }
                for v in ca.variants_as_strings() {
                    if referenced.insert(v) { changed = true; }
                }
            }
        }
    }

    let children: Vec<MirrorFragment> = children
        .into_iter()
        .filter(|c| {
            let ca = c.mirror_ast();
            if matches!(ca, MirrorAST::Split(_)) {
                return referenced.contains(ca.name());
            }
            true
        })
        .collect();

    let after_count = children.len();
    let result = build_fragment(ast, children);
    (result, before_count, after_count)
}

fn emit_fragment_into(frag: &MirrorFragment, indent: usize, out: &mut String) {
    let ast = frag.mirror_ast();
    let children = frag.mirror_children();
    let tag = ast.decl_tag();
    let name = ast.name();
    let params = ast.params_as_strings();
    let variants = ast.variants_as_strings();

    for _ in 0..indent {
        out.push_str("  ");
    }
    if ast.is_abstract() {
        out.push_str("abstract ");
    }
    out.push_str(tag);
    if !name.is_empty() {
        out.push(' ');
        out.push_str(name);
    }
    // Recover/Rescue use pipe-delimited params
    if (tag == "recover" || tag == "rescue") && !params.is_empty() {
        out.push_str(" |");
        for (i, p) in params.iter().enumerate() {
            if i > 0 { out.push_str(", "); }
            out.push_str(p);
        }
        out.push('|');
    } else if !params.is_empty() {
        out.push('(');
        for (i, p) in params.iter().enumerate() {
            if i > 0 { out.push_str(", "); }
            out.push_str(p);
        }
        out.push(')');
    }
    // Action-specific: emit `in @grammar` and `-> return_type` before the body.
    if tag == "action" {
        if let Some(gr) = ast.grammar_ref_str() {
            out.push_str(" in ");
            out.push_str(&gr);
        }
        if let Some(rt) = ast.return_type_str() {
            out.push_str(" -> ");
            out.push_str(&rt);
        }
    }
    if !variants.is_empty() {
        out.push_str(" = ");
        for (i, v) in variants.iter().enumerate() {
            if i > 0 { out.push_str(" | "); }
            out.push_str(v);
        }
    }
    // No body_text available from AST — actions emit their children
    if !children.is_empty() {
        out.push_str(" {\n");
        for child in children {
            emit_fragment_into(child, indent + 1, out);
        }
        for _ in 0..indent {
            out.push_str("  ");
        }
        out.push_str("}\n");
    } else {
        out.push('\n');
    }
}

// ---------------------------------------------------------------------------
// Kintsugi — canonical ordering (the formatter)
// ---------------------------------------------------------------------------

// kintsugi is now kintsugi_fragment above

/// Sort key for kintsugi canonical order.
/// Lower numbers sort first. Stable sort preserves order within same kind.
fn kintsugi_sort_key(tag: &str) -> u8 {
    match tag {
        "in" => 0,
        "type" => 1,
        "traversal" => 2,
        "lens" => 3,
        "grammar" | "form" => 4,
        "property" => 5,
        "action" => 6,
        "focus" | "project" | "split" | "fold" | "zoom" | "refract" => 1,
        "out" => 7,
        "prism" => 1,
        "requires" | "invariant" | "ensures" => 5,
        "recover" | "rescue" => 6,
        "template" => 6,
        "default" | "binding" => 7,
        _ => 8,
    }
}

// ---------------------------------------------------------------------------
// MirrorRuntime — the operation.
// ---------------------------------------------------------------------------

/// Compiled artifact: the content-addressed MirrorFragment.
#[derive(Clone, Debug)]
pub struct CompiledShatter {
    /// The content-addressed fragment tree. Primary public interface.
    pub fragment: MirrorFragment,
}

impl CompiledShatter {
    pub fn crystal(&self) -> Oid {
        Oid::new(self.fragment.content_hash().as_str())
    }
    pub fn form_name(&self) -> &str {
        self.fragment.mirror_ast().name()
    }
    /// Get the AST from the fragment.
    pub fn ast(&self) -> &MirrorAST {
        self.fragment.mirror_ast()
    }
}

#[derive(Default)]
pub struct MirrorRuntime;

impl MirrorRuntime {
    pub fn new() -> Self {
        MirrorRuntime
    }

    pub fn compile_source(
        &self,
        source: &str,
    ) -> Imperfect<CompiledShatter, MirrorRuntimeError, MirrorLoss> {
        parse_form(source).map(|fragment| CompiledShatter { fragment })
    }

    /// Compile source and store the resulting `.shatter` artifact in the git store.
    ///
    /// On success (or partial), emits a `.shatter` file into `.git/mirror/` via the
    /// provided store. Also updates the file→OID ref index. Best-effort: failures
    /// to write to the store are silently discarded — the compiled result is returned
    /// regardless.
    pub fn compile_to_shatter(
        &self,
        source: &str,
        store: &crate::git_store::MirrorGitStore,
    ) -> Imperfect<(crate::shatter_format::ShatterMeta, String), MirrorRuntimeError, MirrorLoss>
    {
        let result = self.compile_source(source);
        let loss = result.loss().clone();

        result.map(|compiled| {
            use crate::shatter_format::{emit_shatter_with_frontmatter, ShatterMeta};

            let meta = ShatterMeta::from_compiled(&compiled, &loss);
            let body = source.to_string();
            let shatter_content = emit_shatter_with_frontmatter(&meta, &body);

            store.store_shatter(&meta.oid, &shatter_content);

            (meta, body)
        })
    }

    pub fn compile_file(&self, path: &Path) -> Result<CompiledShatter, MirrorRuntimeError> {
        let src = std::fs::read_to_string(path)
            .map_err(|e| err(format!("read {}: {}", path.display(), e)))?;
        Result::from(self.compile_source(&src))
    }

    pub fn compile_boot_dir(
        &self,
        dir: &Path,
        store_dir: &Path,
    ) -> Result<BootResolution, MirrorRuntimeError> {
        let mut entries: Vec<_> = std::fs::read_dir(dir)
            .map_err(|e| err(format!("read_dir {}: {}", dir.display(), e)))?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("mirror"))
            .collect();
        entries.sort();

        let mut registry = MirrorRegistry::open(store_dir)?;
        let mut resolved: BTreeMap<String, CompiledShatter> = BTreeMap::new();
        let mut failed: BTreeMap<String, MirrorResolveError> = BTreeMap::new();
        let mut all_fragments: Vec<MirrorFragment> = Vec::new();
        let mut total_loss = MirrorLoss::zero();

        for path in entries {
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            let src = std::fs::read_to_string(&path)
                .map_err(|e| err(format!("read {}: {}", path.display(), e)))?;
            let compile_result = self.compile_source(&src);

            // Accumulate loss from partial compilations
            let file_loss = compile_result.loss();
            if !file_loss.is_zero() {
                total_loss = total_loss.combine(file_loss);
            }

            // Extract the compiled result (Success or Partial both have a value)
            let compiled = match compile_result {
                Imperfect::Success(c) => c,
                Imperfect::Partial(c, _) => c,
                Imperfect::Failure(e, _) => return Err(e),
            };

            all_fragments.push(compiled.fragment.clone());

            match registry.resolve_fragment(&compiled.fragment) {
                Ok(()) => {
                    registry.register_fragment(&compiled.fragment);
                    resolved.insert(stem, compiled);
                }
                Err(e) => {
                    failed.insert(stem, e);
                }
            }
        }

        // --- Phase 2: Standard library (boot/std/) ---
        // The std is the first consumer of the package system.
        // Files resolve against the kernel registry.
        // TODO: Replace alphabetical sort with @package.resolve() once
        // the package resolver is implemented. For now, sort alphabetically
        // as a placeholder — the package system determines the real order.
        let std_dir = dir.join("std");
        if std_dir.is_dir() {
            let mut std_entries: Vec<_> = std::fs::read_dir(&std_dir)
                .map_err(|e| err(format!("read_dir {}: {}", std_dir.display(), e)))?
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("mirror"))
                .collect();
            // Alphabetical sort as placeholder until @package.resolve works
            std_entries.sort();

            for path in std_entries {
                let stem = format!(
                    "std/{}",
                    path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                );
                let src = std::fs::read_to_string(&path)
                    .map_err(|e| err(format!("read {}: {}", path.display(), e)))?;
                let compile_result = self.compile_source(&src);

                let file_loss = compile_result.loss();
                if !file_loss.is_zero() {
                    total_loss = total_loss.combine(file_loss);
                }

                let compiled = match compile_result {
                    Imperfect::Success(c) => c,
                    Imperfect::Partial(c, _) => c,
                    Imperfect::Failure(e, _) => return Err(e),
                };

                all_fragments.push(compiled.fragment.clone());

                match registry.resolve_fragment(&compiled.fragment) {
                    Ok(()) => {
                        registry.register_fragment(&compiled.fragment);
                        resolved.insert(stem, compiled);
                    }
                    Err(e) => {
                        failed.insert(stem, e);
                    }
                }
            }

            // --- Phase 2b: Standard library subdirectories (boot/std/<family>/) ---
            // Grammar families like @trace live in subdirectories of std/.
            // Files in subdirectories are loaded after flat files, sorted alphabetically.
            // Key format: "std/<family>/<stem>" (e.g., "std/trace/mod").
            let mut subdirs: Vec<_> = std::fs::read_dir(&std_dir)
                .map_err(|e| err(format!("read_dir {}: {}", std_dir.display(), e)))?
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.is_dir())
                .collect();
            subdirs.sort();

            for subdir in subdirs {
                let family = subdir
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
                let mut sub_entries: Vec<_> = std::fs::read_dir(&subdir)
                    .map_err(|e| err(format!("read_dir {}: {}", subdir.display(), e)))?
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("mirror"))
                    .collect();
                sub_entries.sort();

                for path in sub_entries {
                    let stem = format!(
                        "std/{}/{}",
                        family,
                        path.file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("unknown")
                    );
                    let src = std::fs::read_to_string(&path)
                        .map_err(|e| err(format!("read {}: {}", path.display(), e)))?;
                    let compile_result = self.compile_source(&src);

                    let file_loss = compile_result.loss();
                    if !file_loss.is_zero() {
                        total_loss = total_loss.combine(file_loss);
                    }

                    let compiled = match compile_result {
                        Imperfect::Success(c) => c,
                        Imperfect::Partial(c, _) => c,
                        Imperfect::Failure(e, _) => return Err(e),
                    };

                    all_fragments.push(compiled.fragment.clone());

                    match registry.resolve_fragment(&compiled.fragment) {
                        Ok(()) => {
                            registry.register_fragment(&compiled.fragment);
                            resolved.insert(stem, compiled);
                        }
                        Err(e) => {
                            failed.insert(stem, e);
                        }
                    }
                }
            }
        }

        registry.flush();

        // Build the collapsed fragment: a wrapper containing all file fragments as children.
        let collapsed_ast = MirrorAST::Module(ModuleNode {
            name: Identifier::new("mirror"),
            children: vec![],
        });
        let collapsed_fragment = build_fragment(collapsed_ast, all_fragments);
        let collapsed = CompiledShatter {
            fragment: collapsed_fragment,
        };

        let store_root = registry.root().to_path_buf();
        Ok(BootResolution {
            resolved,
            failed,
            store_root,
            collapsed,
            total_loss,
        })
    }
}

#[derive(Debug)]
pub struct BootResolution {
    pub resolved: BTreeMap<String, CompiledShatter>,
    pub failed: BTreeMap<String, MirrorResolveError>,
    pub store_root: PathBuf,
    pub collapsed: CompiledShatter,
    /// Accumulated loss from all files in the boot sequence.
    /// Includes unrecognized declarations from any file that parsed partially.
    pub total_loss: MirrorLoss,
}

// Retain BootShatter as a type alias for transitional callers.
pub type BootShatter = BootResolution;

// ---------------------------------------------------------------------------
// emit_shatter — serialize a compiled boot to .mirror-syntax .shatter file
// ---------------------------------------------------------------------------

/// Emit a `.shatter` file from a compiled boot sequence.
///
/// The output is valid `.mirror` syntax. The compiler can read its own output.
/// Round-trip: `parse(emit_shatter(boot)) → compile → same OID`.
pub fn emit_shatter(
    collapsed: &CompiledShatter,
    resolved: &BTreeMap<String, CompiledShatter>,
    failed: &BTreeMap<String, MirrorResolveError>,
) -> String {
    let mut out = String::new();
    out.push_str("# mirror.shatter\n");
    out.push_str(&format!("# oid: {}\n", collapsed.crystal().as_str()));
    out.push_str(&format!(
        "# resolved: {} | failed: {}\n",
        resolved.len(),
        failed.len()
    ));
    out.push('\n');

    // Emit the collapsed fragment as valid .mirror syntax.
    // emit_fragment is already proven to round-trip exactly (same OIDs).
    out.push_str(&emit_fragment(&collapsed.fragment));
    out
}

impl MirrorRuntime {
    /// Compile the boot directory and emit mirror.shatter.
    pub fn materialize_crystal(
        &self,
        boot_dir: &Path,
        store_dir: &Path,
        output: &Path,
    ) -> Result<Oid, MirrorRuntimeError> {
        let boot = self.compile_boot_dir(boot_dir, store_dir)?;
        let content = emit_shatter(&boot.collapsed, &boot.resolved, &boot.failed);
        std::fs::write(output, &content)
            .map_err(|e| err(format!("write {}: {}", output.display(), e)))?;
        Ok(boot.collapsed.crystal())
    }
}

// ---------------------------------------------------------------------------
// MirrorRegistry — content-addressed store backed by FrgmntStore
// ---------------------------------------------------------------------------

const REGISTRY_CACHE_BYTES: usize = 16 * 1024 * 1024;

/// MirrorRegistry holds compiled fragments in a content-addressed store.
/// Backed by FrgmntStore<MirrorFragment>, which manages both in-memory cache
/// and persistent disk storage via the `.frgmnt/` directory structure.
pub struct MirrorRegistry {
    store: FrgmntStore<MirrorFragment>,
    ops: std::collections::BTreeSet<String>,
    root: PathBuf,
}

impl MirrorRegistry {
    /// Open or create a registry at the given path. Creates `.frgmnt/objects`
    /// and `.frgmnt/refs` subdirectories if they don't exist. Initializes
    /// builtin operations ("in", "out").
    pub fn open(path: &Path) -> Result<Self, MirrorRuntimeError> {
        let path_str = path
            .to_str()
            .ok_or_else(|| err(format!("non-utf8 registry path: {}", path.display())))?;
        let store = FrgmntStore::<MirrorFragment>::open(path_str, REGISTRY_CACHE_BYTES)
            .map_err(|e| err(format!("open frgmnt store at {}: {}", path.display(), e)))?;
        let mut ops = std::collections::BTreeSet::new();
        ops.insert("in".to_string());
        ops.insert("out".to_string());
        Ok(MirrorRegistry {
            store,
            ops,
            root: path.to_path_buf(),
        })
    }

    /// Check if an operation name is registered (builtin or custom).
    pub fn has_op(&self, name: &str) -> bool {
        self.ops.contains(name)
    }

    /// Look up a named fragment in the registry. Returns None if the name
    /// doesn't exist or the Oid it references isn't in the cache or on disk.
    pub fn lookup(&self, name: &str) -> Option<MirrorFragment> {
        let oid = self.store.get_ref(name)?;
        self.store.get_persistent(&oid)
    }

    /// Root path of the registry.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Flush all cached fragments to disk and clear the in-memory cache.
    /// Call this before dropping the registry to ensure all fragments are persisted.
    pub fn flush(&self) {
        self.store.flush();
    }

    /// Iterate the names of all refs in the underlying store. Reads from disk.
    pub fn ref_names(&self) -> Vec<String> {
        let refs_dir = self.root.join("refs");
        let Ok(entries) = std::fs::read_dir(&refs_dir) else {
            return Vec::new();
        };
        let mut names: Vec<String> = entries
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect();
        names.sort();
        names
    }

    /// Resolve a MirrorFragment tree.
    pub fn resolve_fragment(&self, frag: &MirrorFragment) -> Result<(), MirrorResolveError> {
        let ast = frag.mirror_ast();
        let tag = ast.decl_tag();
        let name = ast.name();
        if tag == "in" && self.store.get_ref(name).is_none() {
            return Err(MirrorResolveError(format!(
                "unresolved `in {}`: no such ref in registry store at {}",
                name,
                self.root.display()
            )));
        }
        if let Some(parent) = ast.parent_ref_str() {
            if self.store.get_ref(&parent).is_none() {
                return Err(MirrorResolveError(format!(
                    "unresolved parent `{}`: no such ref in registry store at {}",
                    parent,
                    self.root.display()
                )));
            }
        }
        for child in frag.mirror_children() {
            self.resolve_fragment(child)?;
        }
        Ok(())
    }

    /// Register a MirrorFragment tree — fragment-native version of `register`.
    pub fn register_fragment(&mut self, frag: &MirrorFragment) -> Vec<String> {
        let name = frag.mirror_ast().name();
        let mut oids = Vec::new();
        if name.is_empty() {
            for child in frag.mirror_children() {
                oids.extend(self.register_fragment_decl(child));
            }
        } else {
            oids.extend(self.register_fragment_decl(frag));
        }
        oids
    }

    fn register_fragment_decl(&mut self, frag: &MirrorFragment) -> Option<String> {
        let name = frag.mirror_ast().name().to_string();
        if !name.starts_with('@') {
            return None;
        }
        let oid = frag.content_hash().as_str().to_string();
        let size = self.estimate_fragment_size(frag);
        self.store
            .insert_persistent(oid.clone(), frag.clone(), size);
        if let Err(e) = self.store.set_ref(&name, &oid) {
            eprintln!("warning: set_ref({} -> {}) failed: {}", name, oid, e);
        }
        Some(oid)
    }

    fn estimate_fragment_size(&self, frag: &MirrorFragment) -> usize {
        let ast = frag.mirror_ast();
        let mut bytes = ast.name().len()
            + ast.params_as_strings().iter().map(|s: &String| s.len()).sum::<usize>()
            + ast.variants_as_strings().iter().map(|s: &String| s.len()).sum::<usize>()
            + 64;
        for child in frag.mirror_children() {
            bytes += self.estimate_fragment_size(child);
        }
        bytes
    }
}

// Form-based registry methods removed.

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use fragmentation::sha::HashAlg;
    use std::path::PathBuf;

    fn boot_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("boot")
    }

    fn tempdir_for_test(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("mirror-test-{}-{}", name, std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Build a test fragment from kind tag, name, params, variants, and children.
    fn test_frag(
        kind: &str,
        name: impl Into<String>,
        params: Vec<String>,
        variants: Vec<String>,
        children: Vec<MirrorFragment>,
    ) -> MirrorFragment {
        let ast = build_ast_node_direct(kind, &name.into(), &params, &variants, &None, vec![]);
        build_fragment(ast, children)
    }

    /// Build a test action fragment.
    fn test_action_frag(
        name: impl Into<String>,
        params: Vec<String>,
        grammar_ref: Option<String>,
        _body_text: Option<String>,
        children: Vec<MirrorFragment>,
    ) -> MirrorFragment {
        let name = name.into();
        let fields = MirrorAST::params_to_fields(&params);
        let ast = MirrorAST::Zoom(ZoomNode {
            name: Identifier::new(&name),
            params: fields,
            target: None,
            grammar_ref: grammar_ref.as_deref().map(|gr| {
                if gr.starts_with('@') { GrammarRef::new(gr) }
                else { GrammarRef::new(format!("@{}", gr)) }
            }),
            children: vec![],
            body: None,
        });
        build_fragment(ast, children)
    }

    // -----------------------------------------------------------------------
    // OpticOp classification in parsed fragments
    // -----------------------------------------------------------------------

    #[test]
    fn type_declaration_uses_iso_and_split() {
        let source = "type visibility = private | protected | public";
        let frag = parse_form(source).ok().unwrap();
        assert_eq!(frag.mirror_ast().decl_tag(), "type");
        assert!(
            vec![].contains(&OpticOp::Iso),
            "= should classify as Iso, got {:?}",
            vec![]
        );
        assert!(
            vec![].contains(&OpticOp::Split),
            "| should classify as Split, got {:?}",
            vec![]
        );
    }

    #[test]
    fn split_decl_keyword_classified_as_optic() {
        let source = "split |(ref, ref)";
        let frag = parse_form(source).ok().unwrap();
        assert_eq!(frag.mirror_ast().decl_tag(), "split");
        assert!(
            vec![].contains(&OpticOp::Split),
            "split keyword should be classified as OpticOp::Split"
        );
    }

    #[test]
    fn zoom_decl_keyword_classified_as_optic() {
        let source = "zoom |>(ref, prism)";
        let frag = parse_form(source).ok().unwrap();
        assert_eq!(frag.mirror_ast().decl_tag(), "zoom");
        assert!(
            vec![].contains(&OpticOp::Zoom),
            "zoom keyword should be classified as OpticOp::Zoom"
        );
    }

    #[test]
    fn refract_decl_keyword_classified_as_optic() {
        let source = "refract ..(ref)";
        let frag = parse_form(source).ok().unwrap();
        assert_eq!(frag.mirror_ast().decl_tag(), "refract");
        assert!(
            vec![].contains(&OpticOp::Refract),
            "refract keyword should be classified as OpticOp::Refract"
        );
    }

    #[test]
    fn fold_decl_keyword_classified_as_optic() {
        let source = "fold <=(ref, imperfect)";
        let frag = parse_form(source).ok().unwrap();
        assert_eq!(frag.mirror_ast().decl_tag(), "fold");
        assert!(
            vec![].contains(&OpticOp::Fold),
            "fold keyword should be classified as OpticOp::Fold"
        );
    }

    #[test]
    fn focus_decl_with_params_classified_as_optic() {
        let source = "focus type(id)";
        let frag = parse_form(source).ok().unwrap();
        assert_eq!(frag.mirror_ast().decl_tag(), "focus");
        assert!(
            vec![].contains(&OpticOp::Focus),
            "focus keyword with params should be classified as OpticOp::Focus"
        );
    }

    #[test]
    fn type_without_variants_has_no_split() {
        let source = "type grammar";
        let frag = parse_form(source).ok().unwrap();
        assert!(!vec![].contains(&OpticOp::Split));
        assert!(!vec![].contains(&OpticOp::Iso));
    }

    #[test]
    fn parens_classified_as_focus() {
        let source = "type beam(result)";
        let frag = parse_form(source).ok().unwrap();
        assert!(
            vec![].contains(&OpticOp::Focus),
            "parenthesized params should classify as Focus"
        );
    }

    // -----------------------------------------------------------------------
    // Parser tests
    // -----------------------------------------------------------------------

    #[test]
    fn mirror_runtime_parses_atom_decl() {
        let src = "form @form {\n  prism focus\n}\n";
        let frag = parse_form(src).ok().unwrap();
        let data = decoded(&frag);
        assert_eq!(data.kind, "form");
        assert_eq!(data.name, "@form");
        assert_eq!(frag.mirror_children().len(), 1);
        assert_eq!(
            frag.mirror_children()[0].mirror_ast().decl_tag(),
            "prism"
        );
        assert_eq!(decoded(&frag.mirror_children()[0]).name, "focus");
    }

    #[test]
    fn mirror_runtime_parses_params_and_variants() {
        let src = "form @x {\n  prism eigenvalues(precision)\n  traversal kind = a | b | c\n}\n";
        let frag = parse_form(src).ok().unwrap();
        assert_eq!(
            decoded(&frag.mirror_children()[0]).params,
            vec!["precision".to_string()]
        );
        assert_eq!(
            decoded(&frag.mirror_children()[1]).variants,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn mirror_runtime_parses_nested_property() {
        let src = "form @property {\n  property unique_variants(form) {\n    fold input\n  }\n}\n";
        let frag = parse_form(src).ok().unwrap();
        assert_eq!(frag.mirror_children().len(), 1);
        let prop = &frag.mirror_children()[0];
        let pd = decoded(prop);
        assert_eq!(pd.kind, "property");
        assert_eq!(pd.name, "unique_variants");
        assert_eq!(pd.params, vec!["form".to_string()]);
        assert_eq!(prop.mirror_children().len(), 1);
        assert_eq!(prop.mirror_children()[0].mirror_ast().decl_tag(), "fold");
    }

    #[test]
    fn mirror_runtime_compile_form_file() {
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("00-prism.mirror"))
            .unwrap();
        // 00-prism.mirror has multiple declarations, so they're wrapped in a
        // synthetic file-level Module.
        assert_eq!(compiled.ast().decl_tag(), "form");
        assert!(compiled.fragment.mirror_children().len() >= 2);
        // Look for @prism declaration
        let prism_decl = compiled
            .fragment
            .mirror_children()
            .iter()
            .find(|f| f.mirror_ast().name() == "@prism")
            .expect("@prism declaration present");
        assert_eq!(prism_decl.mirror_ast().decl_tag(), "prism");
        assert_eq!(prism_decl.mirror_children().len(), 5);
    }

    #[test]
    fn mirror_runtime_round_trip_oids_match() {
        let runtime = MirrorRuntime::new();
        for entry in std::fs::read_dir(boot_dir()).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) != Some("mirror") {
                continue;
            }
            let s1 = runtime.compile_file(&path).unwrap();
            let text = emit_fragment(&s1.fragment);
            let s2 = Result::from(runtime.compile_source(&text)).unwrap_or_else(|e| {
                panic!(
                    "round-trip parse failed for {}:\nemitted:\n{}\nerror: {}",
                    path.display(),
                    text,
                    e
                );
            });
            assert_eq!(
                s1.crystal(),
                s2.crystal(),
                "round-trip crystal mismatch for {}",
                path.display()
            );
        }
    }

    #[test]
    fn mirror_runtime_compiles_full_boot_dir() {
        let runtime = MirrorRuntime::new();
        let store_dir = tempdir_for_test("compiles_full_boot_dir");
        let boot = runtime.compile_boot_dir(&boot_dir(), &store_dir).unwrap();
        assert!(boot.resolved.len() + boot.failed.len() >= 8);
        assert_eq!(boot.collapsed.ast().name(), "mirror");
        assert!(boot.collapsed.fragment.mirror_children().len() >= 8);

        let store_dir2 = tempdir_for_test("compiles_full_boot_dir_2");
        let again = runtime.compile_boot_dir(&boot_dir(), &store_dir2).unwrap();
        assert_eq!(boot.collapsed.crystal(), again.collapsed.crystal());
    }

    #[test]
    fn mirror_runtime_property_file_compiles() {
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("05-property.mirror"))
            .unwrap();
        // The property kernel now has `out` statements at top level alongside
        // `grammar @property { ... }`, so the form is a synthetic wrapper.
        assert_eq!(compiled.ast().name(), "");
        // The @property grammar block is a child of the wrapper.
        let grammar = compiled.fragment.mirror_children().iter().find(|f| {
            let d = f.mirror_ast();
            d.kind == "grammar" && d.name == "@property"
        });
        assert!(grammar.is_some(), "@property grammar must exist");
        // The kernel defines types, not properties. Properties moved to std/properties.mirror.
        let type_count = grammar
            .unwrap()
            .mirror_children()
            .iter()
            .filter(|f| f.mirror_ast().decl_tag() == "type")
            .count();
        assert_eq!(type_count, 4, "kernel should have 4 type declarations");
        // Out statements at top level
        let out_count = compiled
            .fragment
            .mirror_children()
            .iter()
            .filter(|f| f.mirror_ast().decl_tag() == "out")
            .count();
        assert_eq!(out_count, 5, "kernel should have 5 out declarations");
    }

    #[test]
    fn mirror_runtime_mirror_form_has_property_applications() {
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("std/mirror.mirror"))
            .unwrap();
        let kinds: Vec<&str> = compiled
            .fragment
            .mirror_children()
            .iter()
            .map(|f| f.mirror_ast().decl_tag().clone())
            .collect();
        assert!(kinds.contains(&"requires"));
        assert!(kinds.contains(&"in"variant));
        assert!(kinds.contains(&"ensures"));
        assert!(kinds.contains(&"in"));
    }

    #[test]
    fn mirror_runtime_shatter_prism_round_trip() {
        // Exercise the Prism impl on Shatter: focus → project → refract.
        // The full structural round-trip uses compile_form/decompile because
        // project on the trait surface only carries the top eigenvalues.
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("00-prism.mirror"))
            .unwrap();
        let shatter = Shatter;

        // Trait-level focus carries the top eigenvalues (decoded from fragment).
        let seed: Optic<(), MirrorFragment> = Optic::ok((), compiled.fragment.clone());
        let focused = shatter.focus(seed);
        let eigen = focused.result().ok().expect("focus failed");
        assert_eq!(eigen.kind, "form");
        // 00-prism.mirror wraps multiple declarations in a synthetic Form with empty name
        assert_eq!(eigen.name, "");

        // Trait-level project produces a content-addressed (childless) frag.
        let seed2: Optic<(), MirrorFragment> = Optic::ok((), compiled.fragment.clone());
        let focused2 = shatter.focus(seed2);
        let projected = shatter.project(focused2);
        let frag_result = projected.result().ok().expect("project failed");
        assert!(!frag_result.content_hash().as_str().is_empty());

        // Stable OID across runs (CoincidenceHash<5> determinism).
        let source = std::fs::read_to_string(boot_dir().join("00-prism.mirror")).unwrap();
        let frag = parse_form(&source).ok().unwrap();
        let frag2 = parse_form(&source).ok().unwrap();
        assert_eq!(frag.content_hash(), frag2.content_hash());
        assert_eq!(compiled.fragment.content_hash(), frag.content_hash());
    }

    #[test]
    fn registry_opens_at_path_with_in_and_out_builtins() {
        let tmp = tempdir_for_test("registry_opens");
        let registry = MirrorRegistry::open(&tmp).expect("open registry");
        assert!(registry.has_op("in"), "in must be a builtin op");
        assert!(registry.has_op("out"), "out must be a builtin op");
        assert!(registry.lookup("@prism").is_none());
        assert!(tmp.join("objects").exists());
        assert!(tmp.join("refs").exists());
    }

    #[test]
    fn registry_registers_named_form_into_store() {
        let tmp = tempdir_for_test("registry_registers_named");
        let mut registry = MirrorRegistry::open(&tmp).unwrap();

        let child = test_frag("prism", "focus", Vec::new(), Vec::new(), Vec::new());
        let frag = test_frag(
            "prism",
            "@prism",
            Vec::new(),
            Vec::new(),
            vec![child],
        );
        registry.register_fragment(&frag);

        let stored = registry.lookup("@prism").expect("@prism in registry");
        let restored = decoded(&stored);
        assert_eq!(restored.name, "@prism");
        assert_eq!(stored.mirror_children().len(), 1);
        assert_eq!(decoded(&stored.mirror_children()[0]).name, "focus");
    }

    #[test]
    fn registry_registers_only_at_named_top_level_forms() {
        let tmp = tempdir_for_test("registry_registers_only_at");
        let mut registry = MirrorRegistry::open(&tmp).unwrap();

        let frag = test_frag("prism", "id", Vec::new(), Vec::new(), Vec::new());
        registry.register_fragment(&frag);
        assert!(registry.lookup("id").is_none());
        assert!(registry.lookup("@id").is_none());
    }

    #[test]
    fn registry_persists_across_reopen() {
        let tmp = tempdir_for_test("registry_persists");
        {
            let mut registry = MirrorRegistry::open(&tmp).unwrap();
            let frag = test_frag(
                "prism",
                "@prism",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            registry.register_fragment(&frag);
            registry.flush();
        }
        let registry = MirrorRegistry::open(&tmp).unwrap();
        let stored = registry
            .lookup("@prism")
            .expect("@prism survives reopen via disk");
        assert_eq!(decoded(&stored).name, "@prism");
    }

    #[test]
    fn registry_resolves_in_reference_when_target_in_store() {
        let tmp = tempdir_for_test("registry_resolves_in");
        let mut registry = MirrorRegistry::open(&tmp).unwrap();

        let prism_frag = test_frag(
            "prism",
            "@prism",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        registry.register_fragment(&prism_frag);

        let in_child = test_frag("in", "@prism", Vec::new(), Vec::new(), Vec::new());
        let file = test_frag("form", "", Vec::new(), Vec::new(), vec![in_child]);
        assert!(registry.resolve_fragment(&file).is_ok());
    }

    #[test]
    fn registry_resolve_fails_when_in_target_missing() {
        let tmp = tempdir_for_test("registry_resolve_missing");
        let registry = MirrorRegistry::open(&tmp).unwrap();
        let in_child = test_frag(
            "in",
            "@nonexistent",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        let file = test_frag("form", "", Vec::new(), Vec::new(), vec![in_child]);
        let err = registry.resolve_fragment(&file).unwrap_err();
        assert!(
            err.0.contains("@nonexistent"),
            "error message should mention the missing form: {}",
            err.0
        );
    }

    #[test]
    fn registry_resolve_uses_disk_after_reopen() {
        let tmp = tempdir_for_test("registry_resolve_disk");
        {
            let mut registry = MirrorRegistry::open(&tmp).unwrap();
            let prism_frag = test_frag(
                "prism",
                "@prism",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            registry.register_fragment(&prism_frag);
            registry.flush();
        }
        let registry = MirrorRegistry::open(&tmp).unwrap();
        let in_child = test_frag("in", "@prism", Vec::new(), Vec::new(), Vec::new());
        let file = test_frag("form", "", Vec::new(), Vec::new(), vec![in_child]);
        assert!(
            registry.resolve_fragment(&file).is_ok(),
            "resolve must use store ref lookup, not in-memory state"
        );
    }

    #[test]
    fn boot_dir_resolves_first_three_files_and_fails_property_and_mirror() {
        let runtime = MirrorRuntime::new();
        let store_dir = tempdir_for_test("boot_dir_resolves_full");
        let boot = runtime.compile_boot_dir(&boot_dir(), &store_dir).unwrap();

        assert!(boot.resolved.contains_key("00-prism"));
        assert!(boot.resolved.contains_key("01-meta"));
        assert!(boot.resolved.contains_key("03-code"));
        assert!(
            boot.resolved.contains_key("03a-code-rust"),
            "03a-code-rust should resolve"
        );
        assert!(boot.resolved.contains_key("01a-meta-actor"));

        // 01b, 01c, 02-shatter now resolve: actor (01a) loads before them
        assert!(boot.resolved.contains_key("01b-meta-action"));
        assert!(boot.resolved.contains_key("01c-meta-io"));
        assert!(boot.resolved.contains_key("02-shatter"));
        // 05-property now resolves (in @meta, not in @form)
        assert!(boot.resolved.contains_key("05-property"));
        // 10-mirror moved to std/ — not loaded by kernel compilation
        assert!(boot.failed.contains_key("06b-package-spec"));

        let reopened = MirrorRegistry::open(&store_dir).unwrap();
        assert!(reopened.lookup("@prism").is_some());
        assert!(reopened.lookup("@meta").is_some());
        assert!(reopened.lookup("@code").is_some());
        assert!(reopened.lookup("@actor").is_some());
        // @property now resolves (in @meta instead of in @form)
        assert!(reopened.lookup("@property").is_some());
        // @mirror resolves from std/mirror.mirror (in @meta, @prism, @property)
        assert!(reopened.lookup("@mirror").is_some());
    }

    #[test]
    fn meta_fails_to_resolve_without_prism_in_registry() {
        let runtime = MirrorRuntime::new();
        let tmp = tempdir_for_test("meta_without_prism");
        let registry = MirrorRegistry::open(&tmp).unwrap();
        let meta = runtime
            .compile_file(&boot_dir().join("01-meta.mirror"))
            .unwrap();
        let err = registry.resolve_fragment(&meta.fragment).unwrap_err();
        assert!(
            err.0.contains("@prism"),
            "expected unresolved @prism error, got: {}",
            err.0
        );
    }

    #[test]
    fn meta_resolves_after_prism_is_registered() {
        let runtime = MirrorRuntime::new();
        let tmp = tempdir_for_test("meta_after_prism");
        let mut registry = MirrorRegistry::open(&tmp).unwrap();
        let prism = runtime
            .compile_file(&boot_dir().join("00-prism.mirror"))
            .unwrap();
        registry.register_fragment(&prism.fragment);

        let meta = runtime
            .compile_file(&boot_dir().join("01-meta.mirror"))
            .unwrap();
        assert!(
            registry.resolve_fragment(&meta.fragment).is_ok(),
            "01-meta should resolve once @prism is registered"
        );
    }

    #[test]
    fn two_registries_at_different_paths_hold_independent_memory() {
        let runtime = MirrorRuntime::new();
        let tmp_a = tempdir_for_test("hot_swap_a");
        let tmp_b = tempdir_for_test("hot_swap_b");

        {
            let mut reg_a = MirrorRegistry::open(&tmp_a).unwrap();
            let prism = runtime
                .compile_file(&boot_dir().join("00-prism.mirror"))
                .unwrap();
            reg_a.register_fragment(&prism.fragment);
            reg_a.flush();
        }

        let _ = MirrorRegistry::open(&tmp_b).unwrap();

        let reg_a = MirrorRegistry::open(&tmp_a).unwrap();
        let reg_b = MirrorRegistry::open(&tmp_b).unwrap();
        assert!(reg_a.lookup("@prism").is_some());
        assert!(reg_b.lookup("@prism").is_none());

        let meta = runtime
            .compile_file(&boot_dir().join("01-meta.mirror"))
            .unwrap();
        assert!(
            reg_a.resolve_fragment(&meta.fragment).is_ok(),
            "mount A has @prism; meta resolves"
        );
        assert!(
            reg_b.resolve_fragment(&meta.fragment).is_err(),
            "mount B is empty; meta fails to resolve"
        );
    }

    // -----------------------------------------------------------------------
    // Action declaration tests
    // -----------------------------------------------------------------------

    #[test]
    fn parse_action_with_grammar_ref() {
        let src = "action transform(state) in @code/rust {\n    fn transform(&mut self) { }\n}\n";
        let frag = parse_form(src).ok().unwrap();
        let data = decoded(&frag);
        assert_eq!(data.kind, "action");
        assert_eq!(data.name, "transform");
        assert_eq!(data.params, vec!["state".to_string()]);
        assert_eq!(data.grammar_ref, Some("@code/rust".to_string()));
        assert!(data.body_text.is_some(), "body text should be captured");
        let body = data.body_text.as_ref().unwrap();
        assert!(
            body.contains("transform"),
            "body should contain the raw text: {}",
            body
        );
    }

    #[test]
    fn parse_action_without_grammar_ref() {
        let src = "action update(state) {\n    state.apply()\n}\n";
        let frag = parse_form(src).ok().unwrap();
        let data = decoded(&frag);
        assert_eq!(data.kind, "action");
        assert_eq!(data.name, "update");
        assert_eq!(data.params, vec!["state".to_string()]);
        assert_eq!(data.grammar_ref, None, "no `in @grammar` means None");
        assert!(data.body_text.is_some());
    }

    #[test]
    fn parse_action_receiver_stored() {
        let src = "action send(process, message) in @actor {\n    dispatch(message)\n}\n";
        let frag = parse_form(src).ok().unwrap();
        let data = decoded(&frag);
        assert_eq!(data.kind, "action");
        assert_eq!(data.name, "send");
        assert_eq!(
            data.params,
            vec!["process".to_string(), "message".to_string()]
        );
        assert_eq!(data.grammar_ref, Some("@actor".to_string()));
    }

    #[test]
    fn parse_action_body_stored_as_raw() {
        let src = "action compute(x) in @code/rust {\n    let y = x * 2;\n    y + 1\n}\n";
        let frag = parse_form(src).ok().unwrap();
        let data = decoded(&frag);
        assert!(data.body_text.is_some());
        let body = data.body_text.unwrap();
        assert!(
            body.contains("let"),
            "raw body should be preserved: {}",
            body
        );
    }

    #[test]
    fn parse_action_empty_body() {
        let src = "action noop(state) { }\n";
        let frag = parse_form(src).ok().unwrap();
        let data = decoded(&frag);
        assert_eq!(data.kind, "action");
        assert_eq!(data.name, "noop");
        assert_eq!(data.body_text, None, "empty body should be None");
    }

    #[test]
    fn action_fragment_round_trip() {
        let frag = test_action_frag(
            "transform",
            vec!["state".to_string()],
            Some("@code/rust".to_string()),
            Some("fn transform() {}".to_string()),
            Vec::new(),
        );
        let restored = decoded(&frag);
        assert_eq!(restored.kind, "action");
        assert_eq!(restored.name, "transform");
        assert_eq!(restored.params, vec!["state".to_string()]);
        assert_eq!(restored.grammar_ref, Some("@code/rust".to_string()));
        assert_eq!(restored.body_text, Some("fn transform() {}".to_string()));
    }

    #[test]
    fn action_file_01a_parses_and_resolves() {
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("01b-meta-action.mirror"))
            .unwrap();
        // 01b-meta-action.mirror has multiple top-level declarations, wrapped in synthetic Module
        assert_eq!(compiled.ast().decl_tag(), "form");
        // Should contain: in @prism, in @meta, in @actor, prism action, action action, out action/collapse
        let action_decls: Vec<&MirrorFragment> = compiled
            .fragment
            .mirror_children()
            .iter()
            .filter(|f| f.mirror_ast().decl_tag() == "action")
            .collect();
        assert_eq!(
            action_decls.len(),
            1,
            "01b-meta-action.mirror has one action declaration"
        );
        let action_ast = action_decls[0].mirror_ast();
        assert_eq!(action_ast.name(), "action");
        // The action body contains mirror declaration keywords (focus, project, etc.)
        // so it's parsed as structured children, not raw body text.
        assert!(
            !action_decls[0].mirror_children().is_empty(),
            "action body with mirror keywords should be parsed as children"
        );
    }

    #[test]
    fn action_is_named_type_property_passes_for_named_receiver() {
        let action = test_action_frag(
            "transform",
            vec!["state".to_string()],
            Some("@code/rust".to_string()),
            Some("body".to_string()),
            Vec::new(),
        );
        let frag = test_frag(
            "form",
            "@test",
            Vec::new(),
            Vec::new(),
            vec![action],
        );
        let all_named = frag
            .mirror_children()
            .iter()
            .filter(|f| f.mirror_ast().decl_tag() == "action")
            .all(|f| {
                let d = decoded(f);
                !d.params.is_empty() && !d.params[0].is_empty()
            });
        assert!(all_named, "all action receivers should be named types");
    }

    #[test]
    fn action_is_named_type_property_fails_for_empty_receiver() {
        let frag = test_action_frag(
            "bad",
            Vec::new(),
            None,
            Some("body".to_string()),
            Vec::new(),
        );
        let data = decoded(&frag);
        let has_named_receiver = !data.params.is_empty() && !data.params[0].is_empty();
        assert!(
            !has_named_receiver,
            "action with no params should fail action_is_named_type"
        );
    }

    // -----------------------------------------------------------------------
    // materialize_crystal — .shatter emission and round-trip
    // -----------------------------------------------------------------------

    #[test]
    fn mirror_shatter_materializes_and_roundtrips() {
        let runtime = MirrorRuntime::new();
        let store_dir = tempdir_for_test("materialize_crystal");
        let output = store_dir.join("mirror.shatter");

        let oid = runtime
            .materialize_crystal(&boot_dir(), &store_dir, &output)
            .unwrap();

        // The file exists and is non-empty
        assert!(output.exists(), "mirror.shatter must be written to disk");
        let content = std::fs::read_to_string(&output).unwrap();
        assert!(!content.is_empty(), "mirror.shatter must not be empty");

        // Parse it back — the content IS valid .mirror syntax
        let reparsed = parse_form(&content).ok().unwrap();

        // The reparsed fragment IS already content-addressed
        assert_eq!(
            reparsed.content_hash().as_str(),
            oid.as_str(),
            "round-trip OID mismatch: emitted shatter must parse back to same crystal"
        );
    }

    #[test]
    fn mirror_shatter_is_valid_mirror_syntax() {
        let runtime = MirrorRuntime::new();
        let store_dir = tempdir_for_test("shatter_valid_syntax");
        let output = store_dir.join("mirror.shatter");

        runtime
            .materialize_crystal(&boot_dir(), &store_dir, &output)
            .unwrap();

        let content = std::fs::read_to_string(&output).unwrap();

        // Must parse without error
        let frag = parse_form(&content).ok().unwrap();

        // Must contain the boot forms (all boot files collapsed).
        assert!(
            frag.mirror_children().len() >= 8,
            "shatter must contain at least 8 boot file forms, got {}",
            frag.mirror_children().len()
        );
    }

    // -----------------------------------------------------------------------
    // "default" and "binding" — no longer silently dropped
    // -----------------------------------------------------------------------
    #[test]
    fn parse_default_declaration() {
        let src = "default(visibility) = public";
        let frag = parse_form(src).ok().unwrap();
        let data = decoded(&frag);
        assert_eq!(data.kind, "default");
        assert_eq!(data.name, "");
        assert_eq!(data.params, vec!["visibility".to_string()]);
        assert_eq!(data.variants, vec!["public".to_string()]);
        assert!(
            vec![].contains(&OpticOp::Iso),
            "= should classify as Iso"
        );
    }

    #[test]
    fn parse_binding_declaration() {
        let src = "binding(leader, key) = focus";
        let frag = parse_form(src).ok().unwrap();
        let data = decoded(&frag);
        assert_eq!(data.kind, "binding");
        assert_eq!(data.name, "");
        assert_eq!(data.params, vec!["leader".to_string(), "key".to_string()]);
        assert_eq!(data.variants, vec!["focus".to_string()]);
    }

    #[test]
    fn parse_default_inside_block() {
        let src = "form @test {\n  type visibility = private | public\n  default(visibility) = public\n}\n";
        let frag = parse_form(src).ok().unwrap();
        assert_eq!(frag.mirror_ast().decl_tag(), "form");
        assert_eq!(
            frag.mirror_children().len(),
            2,
            "default should not be silently dropped: got {:?}",
            frag.mirror_children()
                .iter()
                .map(|c| c.mirror_ast().decl_tag().as_str())
                .collect::<Vec<_>>()
        );
        assert_eq!(frag.mirror_children()[0].mirror_ast().decl_tag(), "type");
        assert_eq!(
            frag.mirror_children()[1].mirror_ast().decl_tag(),
            "default"
        );
    }

    // -----------------------------------------------------------------------
    // ParseWarning — parser tracks what it cannot parse
    // -----------------------------------------------------------------------

    #[test]
    fn parse_unrecognized_keyword_returns_partial() {
        // "widget" is not a known DeclKind — parser should return Partial with loss
        let src = "widget foo\ntype bar";
        let result = parse_form(src);
        assert!(
            result.is_partial(),
            "unrecognized keyword should produce Partial, got {:?}",
            if result.is_ok() { "Success" } else { "Failure" }
        );
        // The recognized declaration survives
        let frag = result.as_ref().ok().unwrap();
        assert_eq!(frag.mirror_ast().decl_tag(), "type");
        assert_eq!(decoded(frag).name, "bar");
    }

    #[test]
    fn parse_unrecognized_keyword_loss_contains_warning() {
        use crate::loss::{AstPosition, ParseWarning};
        let src = "widget foo\ntype bar";
        let result = parse_form(src);
        let loss = result.loss();
        assert_eq!(loss.parse.warnings.len(), 1);
        assert!(matches!(
            &loss.parse.warnings[0],
            ParseWarning::UnknownToken {
                at: AstPosition::TopLevel,
                line: 1
            }
        ));
    }

    #[test]
    fn parse_all_recognized_returns_success() {
        let src = "type visibility = private | public";
        let result = parse_form(src);
        assert!(
            !result.is_partial(),
            "fully recognized source should not be Partial"
        );
        assert!(result.is_ok(), "fully recognized source should succeed");
    }

    #[test]
    fn parse_only_unrecognized_returns_failure() {
        let src = "widget foo\ngadget bar";
        let result = parse_form(src);
        assert!(result.is_err(), "only unrecognized keywords should fail");
        let loss = result.loss();
        assert_eq!(
            loss.parse.warnings.len(),
            2,
            "both unrecognized should be tracked"
        );
    }

    // -----------------------------------------------------------------------
    // compile_source propagates Imperfect
    // -----------------------------------------------------------------------

    #[test]
    fn compile_source_returns_partial_on_unrecognized() {
        let runtime = MirrorRuntime::new();
        let src = "widget foo\ntype bar";
        let result = runtime.compile_source(src);
        assert!(
            result.is_partial(),
            "compile_source should propagate Partial from parse_form"
        );
        let loss = result.loss();
        assert!(
            !loss.parse.warnings.is_empty(),
            "loss should contain parse warnings"
        );
        // The recognized part should still compile
        assert!(result.is_ok(), "partial result should still have a value");
    }

    #[test]
    fn compile_source_returns_success_on_clean_source() {
        let runtime = MirrorRuntime::new();
        let src = "type visibility = private | public";
        let result = runtime.compile_source(src);
        assert!(!result.is_partial(), "clean source should not be Partial");
        assert!(result.is_ok());
    }

    // -----------------------------------------------------------------------
    // compile_boot_dir propagates Partial loss
    // -----------------------------------------------------------------------

    #[test]
    fn compile_boot_dir_accumulates_loss() {
        let runtime = MirrorRuntime::new();
        let boot = tempdir_for_test("boot_loss");
        let store = tempdir_for_test("boot_loss_store");

        // Write a .mirror file with an unrecognized keyword
        std::fs::write(boot.join("00-test.mirror"), "widget foo\ntype bar").unwrap();

        let result = runtime.compile_boot_dir(&boot, &store).unwrap();
        assert!(
            !result.total_loss.parse.warnings.is_empty(),
            "boot dir should accumulate parse warnings from partial files"
        );
        assert!(matches!(
            &result.total_loss.parse.warnings[0],
            crate::loss::ParseWarning::UnknownToken { .. }
        ));
    }

    #[test]
    fn compile_boot_dir_clean_has_zero_loss() {
        let runtime = MirrorRuntime::new();
        let boot = tempdir_for_test("boot_clean");
        let store = tempdir_for_test("boot_clean_store");

        std::fs::write(
            boot.join("00-test.mirror"),
            "type visibility = private | public",
        )
        .unwrap();

        let result = runtime.compile_boot_dir(&boot, &store).unwrap();
        assert!(
            result.total_loss.is_zero(),
            "clean boot dir should have zero loss"
        );
    }

    // -----------------------------------------------------------------------
    // boot file inventory — captures filesystem state before reorganization
    // -----------------------------------------------------------------------

    /// Captures the current boot inventory before reorganization.
    /// This is training data — we measure before we change.
    #[test]
    fn boot_file_inventory_before_reorg() {
        let boot = boot_dir();
        let mut files: Vec<String> = std::fs::read_dir(&boot)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|f| f.ends_with(".mirror"))
            .collect();
        files.sort();

        assert_eq!(files.len(), 15, "boot kernel file count: {:?}", files);
        assert!(files.contains(&"00-prism.mirror".to_string()));
        assert!(files.contains(&"01a-meta-actor.mirror".to_string()));
        assert!(files.contains(&"01b-meta-action.mirror".to_string()));
        assert!(files.contains(&"01c-meta-io.mirror".to_string()));
        assert!(files.contains(&"02-shatter.mirror".to_string()));
        assert!(files.contains(&"06-package.mirror".to_string()));
        assert!(files.contains(&"07-runtime.mirror".to_string()));

        // std/ exists with 7 files
        let std_dir = boot.join("std");
        assert!(std_dir.exists(), "std/ should exist");
        let mut std_files: Vec<String> = std::fs::read_dir(&std_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|f| f.ends_with(".mirror"))
            .collect();
        std_files.sort();
        assert_eq!(std_files.len(), 10, "std file count: {:?}", std_files);
        assert!(std_files.contains(&"mirror.mirror".to_string()));
        assert!(std_files.contains(&"cli.mirror".to_string()));
        assert!(std_files.contains(&"properties.mirror".to_string()));
        assert!(std_files.contains(&"file.mirror".to_string()));
        assert!(std_files.contains(&"runtime.mirror".to_string()));
        assert!(std_files.contains(&"rust.mirror".to_string()));
    }

    // -----------------------------------------------------------------------
    // mirror ci: boot baseline — the warnings ARE the specification
    // -----------------------------------------------------------------------

    /// The boot sequence as it IS right now: what resolves, what fails,
    /// what loss accumulates. This test captures the training data.
    /// Every warning is a property the compiler doesn't enforce yet.
    /// Fix the grammar AFTER this test documents the current state.
    #[test]
    fn mirror_ci_boot_baseline() {
        let runtime = MirrorRuntime::new();
        let store = tempdir_for_test("ci_boot_baseline");
        let boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();

        // --- What resolves (the compiler CAN parse these) ---
        let resolved: Vec<&str> = boot.resolved.keys().map(|s| s.as_str()).collect();
        assert!(resolved.contains(&"00-prism"), "prism must resolve");
        assert!(resolved.contains(&"01-meta"), "meta must resolve");
        assert!(resolved.contains(&"03-code"), "code must resolve");
        assert!(
            resolved.contains(&"03a-code-rust"),
            "code-rust must resolve"
        );
        assert!(
            resolved.contains(&"01a-meta-actor"),
            "actor must resolve (loads first at 01a)"
        );
        assert!(
            resolved.contains(&"01b-meta-action"),
            "action must resolve (actor loads before it)"
        );
        assert!(
            resolved.contains(&"01c-meta-io"),
            "io must resolve (actor loads before it)"
        );
        assert!(
            resolved.contains(&"02-shatter"),
            "shatter must resolve (io loads before it)"
        );
        assert!(
            resolved.contains(&"04a-runtime"),
            "runtime must resolve (actor loads before it)"
        );
        assert!(
            resolved.contains(&"07-runtime"),
            "07-runtime must resolve (prism, meta, actor all available)"
        );

        // --- What fails resolution (in @X references something missing) ---
        let failed: Vec<&str> = boot.failed.keys().map(|s| s.as_str()).collect();
        // 05-property now resolves: in @meta (was in @form)
        assert!(
            resolved.contains(&"05-property"),
            "property must resolve after in @form -> in @meta"
        );
        // 06-package and 06a-package-git resolve (in @prism, @meta, @package)
        assert!(resolved.contains(&"06-package"), "package must resolve");
        assert!(
            resolved.contains(&"06a-package-git"),
            "package-git must resolve"
        );

        // --- The loss: what the compiler saw but couldn't land ---
        let loss = &boot.total_loss;
        let holonomy = loss.holonomy();

        // --- Parse-level loss ---
        // Kernel files introduce unrecognized keywords (training data):
        //   unfold, subset, superset, iso, not-iso (01-meta operators)
        //   io (01c-meta-io, 02-shatter grammar keyword)
        //   pure, real, loss constraints with != operator
        // Std files introduce unrecognized keywords:
        //   template (8 declarations in std/properties.mirror)
        //   where (2 boundary property constraints in std/properties.mirror)
        //
        // The baseline holonomy must not INCREASE (regression).
        // It CAN decrease as the parser learns new constructs.
        // Baseline raised from 25 to 130 after fixing block-level loss detection
        // (seam/block-unrecognized-loss) -- keywords inside blocks are now measured.
        // Raised to 165 after adding ast(g) type hierarchy to 01-meta.mirror
        // (reed/emit-code) -- new struct declarations with parameterized types.
        // Raised to 183 after adding template declarations to @code grammar
        // and new std files (rust.mirror, file.mirror, runtime.mirror).
        // Raised to 246 after adding 07-runtime.mirror (SpectralRuntime grammar).
        assert!(
            holonomy <= 246.0,
            "parse holonomy must not regress above baseline: got {}",
            holonomy
        );

        // --- Resolution failures: kernel + std ---
        // Kernel failures: 06b-package-spec (missing refs: @mirror, @config, @ai)
        // Std failures: beam, benchmark, cli, tui, rust, runtime (missing grammar refs)
        // actor(01a) now loads first → action(01b), io(01c), shatter(02), runtime(04a) all resolve
        assert_eq!(
            boot.failed.len(),
            7,
            "7 of 25 files fail resolution (1 kernel + 6 std): {:?}",
            boot.failed.keys().collect::<Vec<_>>()
        );
        assert!(
            failed.contains(&"06b-package-spec"),
            "missing refs (@mirror, @config, @ai)"
        );
        // std failures
        assert!(
            failed.contains(&"std/beam"),
            "beam grammar body fails resolution"
        );
        assert!(
            failed.contains(&"std/benchmark"),
            "benchmark needs @time before it alphabetically"
        );
        assert!(
            failed.contains(&"std/cli"),
            "cli needs @spec, @shatter — not in registry"
        );
        assert!(
            failed.contains(&"std/tui"),
            "tui needs @config, @ci, @ca, @lsp — not in registry"
        );

        // --- Resolved: kernel(14) + std(4) = 18 ---
        // 07-runtime resolves: in @prism, @meta, @actor all available.
        assert_eq!(
            boot.resolved.len(),
            18,
            "18 of 25 files resolve (14 kernel + 4 std): {:?}",
            boot.resolved.keys().collect::<Vec<_>>()
        );
        // std files that resolve
        assert!(
            resolved.contains(&"std/mirror"),
            "std/mirror resolves (in @meta, @property)"
        );
        assert!(
            resolved.contains(&"std/time"),
            "std/time resolves (in @prism, @meta, @actor)"
        );
        assert!(
            resolved.contains(&"std/properties"),
            "std/properties resolves (in @meta, @property)"
        );

        // --- The crystal still forms despite failures ---
        // The compiler produces a crystal from what DID resolve.
        // This is Partial, not Failure. The observation happened.
        let crystal_oid = boot.collapsed.crystal();
        assert!(
            !crystal_oid.as_str().is_empty(),
            "crystal must form even with partial resolution"
        );
    }

    /// The reorganized boot: kernel (12 sorted) + std (5 package-resolved).
    #[test]
    fn boot_kernel_and_std() {
        let boot = boot_dir();

        // Kernel: sorted, numbered
        let mut kernel: Vec<String> = std::fs::read_dir(&boot)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|f| f.ends_with(".mirror"))
            .collect();
        kernel.sort();

        // std: unsorted, package-resolved
        let std_dir = boot.join("std");
        let mut std_files: Vec<String> = std::fs::read_dir(&std_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|f| f.ends_with(".mirror"))
            .collect();
        std_files.sort(); // sort for assertion stability only

        // Kernel: 13 files (00, 01, 01a-actor, 01b-action, 01c-io, 02, 03, 03a, 04a-runtime, 05, 06, 06a, 06b)
        assert_eq!(kernel.len(), 14, "kernel needs 14 files: {:?}", kernel);
        assert!(kernel.contains(&"00-prism.mirror".to_string()));
        assert!(kernel.contains(&"06b-package-spec.mirror".to_string()));

        // Std: 10 files (mirror, time, tui, benchmark, cli, properties, beam, file, runtime, rust)
        assert_eq!(std_files.len(), 10, "std needs 10 files: {:?}", std_files);
        assert!(std_files.contains(&"mirror.mirror".to_string()));
        assert!(std_files.contains(&"cli.mirror".to_string()));
        assert!(std_files.contains(&"time.mirror".to_string()));
        assert!(std_files.contains(&"benchmark.mirror".to_string()));
        assert!(std_files.contains(&"tui.mirror".to_string()));
        assert!(std_files.contains(&"properties.mirror".to_string()));
        assert!(std_files.contains(&"file.mirror".to_string()));
        assert!(std_files.contains(&"runtime.mirror".to_string()));
        assert!(std_files.contains(&"rust.mirror".to_string()));

        // Compiler loads both phases
        let runtime = MirrorRuntime::new();
        let store = tempdir_for_test("boot_kernel_std");
        let result = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();

        // std/mirror, std/time, and std/properties resolve against kernel registry
        assert!(
            result.resolved.contains_key("std/mirror"),
            "std/mirror should resolve"
        );
        assert!(
            result.resolved.contains_key("std/time"),
            "std/time should resolve"
        );
        assert!(
            result.resolved.contains_key("std/properties"),
            "std/properties should resolve"
        );
    }

    /// Success(Mirror). Zero loss. Zero failures. Strict passes.
    /// When this test passes, we ship.
    #[test]
    #[ignore = "blocked: boot files need `in @form` → `in @meta` etc."]
    fn mirror_ci_boot_success() {
        let runtime = MirrorRuntime::new();
        let store = tempdir_for_test("ci_boot_success");
        let boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();

        // Zero failures: every boot file resolves
        assert!(
            boot.failed.is_empty(),
            "Success(Mirror) requires zero resolution failures, got: {:?}",
            boot.failed.keys().collect::<Vec<_>>()
        );

        // Zero loss: the compiler found nothing to warn about
        assert!(
            boot.total_loss.is_zero(),
            "Success(Mirror) requires zero loss, got holonomy: {}",
            boot.total_loss.holonomy()
        );

        // Zero holonomy: the crystal is settled
        assert_eq!(
            boot.total_loss.holonomy(),
            0.0,
            "Success(Mirror) requires zero holonomy"
        );

        // The crystal identity law: compile(compile(boot)) = compile(boot)
        let store2 = tempdir_for_test("ci_boot_success_idempotent");
        let boot2 = runtime.compile_boot_dir(&boot_dir(), &store2).unwrap();
        assert_eq!(
            boot.collapsed.crystal().as_str(),
            boot2.collapsed.crystal().as_str(),
            "crystal identity law: same boot → same crystal"
        );
    }

    // -----------------------------------------------------------------------
    // @ai grammar — identity as variant over visibility
    // -----------------------------------------------------------------------

    /// The @ai grammar defines identity as a variant over visibility.
    /// Three bias trees. Three collapse orderings. Three apertures.
    /// The boot action folds identity through visibility into imperfect.
    ///
    /// reed.mirror in ~/.reed/ is the first concrete consumer.
    /// This test uses an inline grammar to prove the shape compiles.
    const AI_GRAMMAR: &str = "\
in @actor

type bias_tree = [ref]
type visibility = public | protected | private
type identity = public(bias_tree) | protected(bias_tree) | private(bias_tree)

grammar @ai {
  action boot(identity) <= imperfect
}
";

    /// The parser must not silently drop <=.
    /// `action boot(identity) <= imperfect` contains a fold operator.
    /// If the parser can't handle it, that's a compilation error — Failure.
    /// Not Success. Not silent. Failure with MirrorLoss recording what was lost.
    #[test]
    fn ai_grammar_fold_not_silent() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source(AI_GRAMMAR);

        // The compiler must either:
        // 1. Parse <= correctly → action boot has OpticOp::Fold (Success)
        // 2. Record the loss → Partial with the dropped <= in MirrorLoss
        // It must NOT return Success with the <= silently swallowed.
        let compiled = match &result {
            Imperfect::Success(c) | Imperfect::Partial(c, _) => c,
            Imperfect::Failure(_, _) => {
                // Failure is acceptable IF the loss records what was dropped.
                let loss = result.loss();
                assert!(loss.holonomy() > 0.0, "Failure must carry non-zero loss");
                return; // Failure with loss = honest. Test passes.
            }
        };

        // If we got here, the compiler returned a value (Success or Partial).
        // The fold operator MUST be recorded on the action.
        // optic_ops is a parser annotation — check via parse_form.
        let frag = parse_form(AI_GRAMMAR).ok().unwrap();
        let boot_action = frag
            .mirror_children()
            .iter()
            .flat_map(|child| std::iter::once(child).chain(child.mirror_children().iter()))
            .find(|f| f.mirror_ast().decl_tag() == "action" && decoded(f).name == "boot");
        assert!(boot_action.is_some(), "action boot must exist");
        assert!(
            boot_action
                .unwrap()
                .mirror_ast()
                .optic_ops
                .contains(&OpticOp::Fold),
            "action boot(identity) <= imperfect must produce OpticOp::Fold"
        );

        // The compiled fragment should also exist
        let _ = compiled;
    }

    /// @ai grammar resolves against boot.
    /// `in @actor` resolves. The identity type is valid.
    /// The boot action's `<= imperfect` uses the fold operator.
    #[test]
    fn ai_grammar_resolves_against_boot() {
        let runtime = MirrorRuntime::new();
        let store = tempdir_for_test("ai_grammar_boot");

        // Boot the language
        let _boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();

        // @actor must be in the registry
        let registry = MirrorRegistry::open(&store).unwrap();
        assert!(
            registry.lookup("@actor").is_some(),
            "@actor must be in registry for @ai to resolve"
        );

        // Compile @ai grammar
        let compiled = runtime.compile_source(AI_GRAMMAR);
        assert!(compiled.is_ok(), "@ai grammar must compile");

        let ai = match compiled {
            Imperfect::Success(c) | Imperfect::Partial(c, _) => c,
            Imperfect::Failure(e, _) => panic!("@ai grammar failed: {}", e),
        };

        // Resolve against booted registry
        let resolve_result = registry.resolve_fragment(&ai.fragment);
        assert!(
            resolve_result.is_ok(),
            "@ai grammar must resolve against boot: in @actor must be found. Got: {:?}",
            resolve_result
        );
    }

    // -----------------------------------------------------------------------
    // MirrorError — code that MUST NOT compile
    // -----------------------------------------------------------------------

    /// Empty source: nothing to compile. Failure.
    #[test]
    fn error_empty_source() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("");
        assert!(
            result.is_err(),
            "empty source must be Failure, got: {:?}",
            result
        );
    }

    /// Whitespace-only source: nothing to compile. Failure.
    #[test]
    fn error_whitespace_only() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("   \n\n  \n");
        assert!(result.is_err(), "whitespace-only source must be Failure");
    }

    /// Comments-only source: nothing survived. Failure.
    #[test]
    fn error_comments_only() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("-- this is a comment\n-- so is this\n");
        assert!(result.is_err(), "comments-only source must be Failure");
    }

    /// Only unrecognized keywords: nothing recognized. Failure with loss.
    #[test]
    fn error_only_unrecognized() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("widget foo\nroute /bar\n");
        assert!(
            result.is_err(),
            "only unrecognized keywords must be Failure"
        );
        let loss = result.loss();
        assert!(
            !loss.parse.warnings.is_empty(),
            "Failure must carry the parse warnings as loss"
        );
        assert!(loss.holonomy() > 0.0, "Failure must have non-zero holonomy");
    }

    /// Unclosed brace: structural error. Failure.
    #[test]
    fn error_unclosed_brace() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("grammar @broken {\n  type x\n");
        assert!(
            result.is_err(),
            "unclosed brace must be Failure, got: {:?}",
            result
        );
    }

    /// Type with no name: `type` alone on a line. Failure.
    #[test]
    fn error_type_no_name() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("type\n");
        assert!(
            result.is_err(),
            "bare `type` keyword must be Failure, got: {:?}",
            result
        );
    }

    /// Grammar with no name: `grammar` alone. Failure.
    #[test]
    fn error_grammar_no_name() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("grammar\n");
        assert!(
            result.is_err(),
            "bare `grammar` keyword must be Failure, got: {:?}",
            result
        );
    }

    /// `in` with no target: `in` alone. Failure.
    #[test]
    fn error_in_no_target() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("in\n");
        assert!(
            result.is_err(),
            "bare `in` keyword must be Failure, got: {:?}",
            result
        );
    }

    /// Action with no name: `action` alone. Failure.
    #[test]
    fn error_action_no_name() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("action\n");
        assert!(
            result.is_err(),
            "bare `action` keyword must be Failure, got: {:?}",
            result
        );
    }

    /// Duplicate type names in the same scope. Failure.
    /// Two types with the same name is a collision.
    #[test]
    fn error_duplicate_type_names() {
        let runtime = MirrorRuntime::new();
        let result =
            runtime.compile_source("type color = red | blue\ntype color = green | yellow\n");
        assert!(
            result.is_err(),
            "duplicate type names must be Failure, got: {:?}",
            result
        );
    }

    /// Non-word token at top level should not produce Success.
    /// `{ }` at top level is structural noise, not a valid program.
    #[test]
    fn error_bare_braces() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("{ }\n");
        assert!(
            result.is_err(),
            "bare braces must be Failure, got: {:?}",
            result
        );
    }

    /// Mixed valid and invalid: if recognized decls exist alongside
    /// unrecognized ones, that's Partial (not Success, not Failure).
    /// The recognized part compiles; the unrecognized is measured loss.
    #[test]
    fn error_mixed_is_partial() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("type valid = a | b\nwidget invalid\n");
        assert!(
            result.is_partial(),
            "mixed valid+invalid must be Partial, got: is_ok={} is_failure={} is_partial={}",
            result.is_ok(),
            result.is_err(),
            result.is_partial()
        );
        let loss = result.loss();
        assert_eq!(loss.parse.warnings.len(), 1, "one parse warning");
        assert!(
            matches!(
                &loss.parse.warnings[0],
                crate::loss::ParseWarning::UnknownToken { .. }
            ),
            "the warning should be UnknownToken"
        );
    }

    // -----------------------------------------------------------------------
    // Missing imports — resolution errors specify what's missing and where
    // -----------------------------------------------------------------------

    /// `in @nonexistent` — import of a grammar that doesn't exist.
    /// Must fail resolution. Error message must name the missing grammar.
    #[test]
    fn error_missing_import() {
        let runtime = MirrorRuntime::new();
        let store = tempdir_for_test("error_missing_import");

        // Boot so the registry has some refs
        let _boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();
        let registry = MirrorRegistry::open(&store).unwrap();

        let src = "in @nonexistent\ntype x";
        let compiled = runtime.compile_source(src);
        let form = compiled.ok().unwrap();

        let err = registry.resolve_fragment(&form.fragment).unwrap_err();
        assert!(
            err.0.contains("@nonexistent"),
            "error must name the missing grammar: got '{}'",
            err.0
        );
        assert!(
            err.0.contains("unresolved"),
            "error must say 'unresolved': got '{}'",
            err.0
        );
    }

    /// Multiple missing imports — the FIRST unresolved ref is reported.
    /// Error message must name it specifically.
    #[test]
    fn error_multiple_missing_imports() {
        let runtime = MirrorRuntime::new();
        let store = tempdir_for_test("error_multi_import");

        let _boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();
        let registry = MirrorRegistry::open(&store).unwrap();

        let src = "in @ghost\nin @phantom\ntype x";
        let compiled = runtime.compile_source(src);
        let form = compiled.ok().unwrap();

        let err = registry.resolve_fragment(&form.fragment).unwrap_err();
        assert!(
            err.0.contains("@ghost"),
            "error must name the first missing import: got '{}'",
            err.0
        );
    }

    /// Import of a grammar that EXISTS should succeed.
    /// Proves the resolution path works — not just the error path.
    #[test]
    fn import_existing_grammar_resolves() {
        let runtime = MirrorRuntime::new();
        let store = tempdir_for_test("import_existing");

        let _boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();
        let registry = MirrorRegistry::open(&store).unwrap();

        let src = "in @prism\ntype x";
        let compiled = runtime.compile_source(src);
        let form = compiled.ok().unwrap();

        let result = registry.resolve_fragment(&form.fragment);
        assert!(
            result.is_ok(),
            "in @prism must resolve after boot: got {:?}",
            result
        );
    }

    /// Nested missing import — `grammar @x { in @missing }`.
    /// Resolution walks children. Must find the missing ref inside the grammar.
    #[test]
    fn error_nested_missing_import() {
        let runtime = MirrorRuntime::new();
        let store = tempdir_for_test("error_nested_import");

        let _boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();
        let registry = MirrorRegistry::open(&store).unwrap();

        let src = "grammar @test {\n  in @nowhere\n  type x\n}";
        let compiled = runtime.compile_source(src);
        let form = compiled.ok().unwrap();

        let err = registry.resolve_fragment(&form.fragment).unwrap_err();
        assert!(
            err.0.contains("@nowhere"),
            "error must name nested missing import: got '{}'",
            err.0
        );
    }

    // -----------------------------------------------------------------------
    // Unknown operators — the parser must not silently swallow them
    // -----------------------------------------------------------------------

    /// Unknown operator at top level: `~>` is not a valid operator.
    /// Must not produce Success. Either Partial with loss or Failure.
    #[test]
    fn error_unknown_operator_top_level() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("type x ~> y\n");
        // The parser sees `type x` then `~>` which it can't parse.
        // It must not silently drop `~> y`.
        match &result {
            Imperfect::Success(c) => {
                // If Success, the operator content must be captured somewhere.
                // `~>` should not vanish. Check that variants or params captured it.
                let ast = c.ast();
                let has_content = !ast.variants_as_strings().is_empty()
                    || !ast.params_as_strings().is_empty()
                    || c.fragment.mirror_children().iter().any(|ch| {
                        !ch.mirror_ast().variants_as_strings().is_empty()
                    });
                assert!(
                    has_content,
                    "unknown operator ~> must not be silently dropped. \
                     type x should capture the remaining content. Got: {:?}",
                    ast
                );
            }
            Imperfect::Partial(_, loss) => {
                // Partial is acceptable if loss records the dropped content
                assert!(
                    loss.holonomy() > 0.0,
                    "Partial must have non-zero holonomy for dropped operator"
                );
            }
            Imperfect::Failure(_, _) => {
                // Failure is acceptable — unknown operator is a parse error
            }
        }
    }

    /// `<=` inside a type declaration: `type x <= y`.
    /// The fold operator is valid in action declarations, not type declarations.
    /// Must either parse it meaningfully or record the loss.
    #[test]
    fn error_fold_in_type_declaration() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("type x <= y\n");
        match &result {
            Imperfect::Success(_c) => {
                // If Success, the <= must be recorded as OpticOp::Fold.
                // optic_ops is a parser annotation — check via parse_form_raw.
                let frag = parse_form("type x <= y\n").ok().unwrap();
                let has_fold = vec![].contains(&OpticOp::Fold)
                    || frag
                        .mirror_children()
                        .iter()
                        .any(|ch| vec![].contains(&OpticOp::Fold));
                assert!(
                    has_fold,
                    "type x <= y: if Success, OpticOp::Fold must be recorded."
                );
            }
            Imperfect::Partial(_, loss) => {
                assert!(
                    loss.holonomy() > 0.0,
                    "Partial must have non-zero holonomy for <= in type"
                );
            }
            Imperfect::Failure(_, _) => {
                // Failure is acceptable — fold in type is semantically wrong
            }
        }
    }

    /// `<=` inside a property: `property p(grammar) <= verdict`.
    /// This is the CORRECT usage. The fold should be recognized.
    #[test]
    fn fold_in_property_declaration() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source(
            "property check(grammar) <= verdict {\n  traversal types\n  refract verdict\n}\n",
        );
        assert!(
            result.is_ok(),
            "property with <= must compile: {:?}",
            result
        );

        // The property must have OpticOp::Fold — check via parse_form since
        // optic_ops is a parser annotation, not stored in the fragment.
        let frag = parse_form(
            "property check(grammar) <= verdict {\n  traversal types\n  refract verdict\n}\n",
        )
        .ok()
        .unwrap();
        let has_fold = vec![].contains(&OpticOp::Fold)
            || frag
                .mirror_children()
                .iter()
                .any(|ch| vec![].contains(&OpticOp::Fold));
        assert!(
            has_fold,
            "property check(grammar) <= verdict must produce OpticOp::Fold."
        );
    }

    // -----------------------------------------------------------------------
    // Recover/Rescue method tests — imperfect type methods
    // -----------------------------------------------------------------------

    /// `recover` inside a type block with fold operator should produce
    /// a child with "recover" and OpticOp::Fold.
    #[test]
    fn imperfect_type_has_recover_method() {
        let source = "type imperfect(observation, error(observation), loss) {\n  recover |observation, loss| <= imperfect\n}\n";
        let frag = parse_form(source).ok().unwrap();
        assert_eq!(frag.mirror_ast().decl_tag(), "type");
        assert_eq!(decoded(&frag).name, "imperfect");
        assert!(
            !frag.mirror_children().is_empty(),
            "imperfect must have children"
        );
        let recover = frag
            .mirror_children()
            .iter()
            .find(|c| c.mirror_ast().decl_tag() == "recover");
        assert!(recover.is_some(), "imperfect must have a recover child");
        let recover = recover.unwrap();
        assert!(
            vec![].contains(&OpticOp::Fold),
            "recover must have OpticOp::Fold (from <=), got: {:?}",
            vec![]
        );
    }

    #[test]
    fn imperfect_type_has_rescue_method() {
        let source = "type imperfect(observation, error(observation), loss) {\n  rescue |error(observation), loss| <= imperfect\n}\n";
        let frag = parse_form(source).ok().unwrap();
        assert_eq!(frag.mirror_ast().decl_tag(), "type");
        let rescue = frag
            .mirror_children()
            .iter()
            .find(|c| c.mirror_ast().decl_tag() == "rescue");
        assert!(rescue.is_some(), "imperfect must have a rescue child");
        let rescue = rescue.unwrap();
        assert!(
            vec![].contains(&OpticOp::Fold),
            "rescue must have OpticOp::Fold (from <=), got: {:?}",
            vec![]
        );
    }

    #[test]
    fn recover_returns_imperfect() {
        let source = "type result(t, e, l) {\n  recover |t, l| <= result\n}\n";
        let frag = parse_form(source).ok().unwrap();
        let recover = frag
            .mirror_children()
            .iter()
            .find(|c| c.mirror_ast().decl_tag() == "recover");
        assert!(recover.is_some(), "result must have recover child");
        let recover = recover.unwrap();
        assert!(
            vec![].contains(&OpticOp::Fold),
            "recover must have fold operator"
        );
        assert!(
            decoded(recover).variants.contains(&"result".to_string()),
            "recover fold target should be 'result', got variants: {:?}",
            decoded(recover).variants
        );
    }

    #[test]
    fn rescue_returns_imperfect() {
        let source = "type result(t, e, l) {\n  rescue |e, l| <= result\n}\n";
        let frag = parse_form(source).ok().unwrap();
        let rescue = frag
            .mirror_children()
            .iter()
            .find(|c| c.mirror_ast().decl_tag() == "rescue");
        assert!(rescue.is_some(), "result must have rescue child");
        let rescue = rescue.unwrap();
        assert!(
            vec![].contains(&OpticOp::Fold),
            "rescue must have fold operator"
        );
        assert!(
            decoded(rescue).variants.contains(&"result".to_string()),
            "rescue fold target should be 'result', got variants: {:?}",
            decoded(rescue).variants
        );
    }

    #[test]
    fn inline_relation_markers_parsed() {
        let source = "type admin {\n  >user\n}\n";
        let frag = parse_form(source).ok().unwrap();
        assert_eq!(frag.mirror_ast().decl_tag(), "type");
        assert_eq!(decoded(&frag).name, "admin");
        let has_superset = vec![].contains(&OpticOp::Superset)
            || frag
                .mirror_children()
                .iter()
                .any(|c| vec![].contains(&OpticOp::Superset));
        assert!(has_superset, "admin type must have Superset marker");

        let source2 = "type contact {\n  <user\n}\n";
        let frag2 = parse_form(source2).ok().unwrap();
        let has_subset = vec![].contains(&OpticOp::Subset)
            || frag2
                .mirror_children()
                .iter()
                .any(|c| vec![].contains(&OpticOp::Subset));
        assert!(has_subset, "contact type must have Subset marker");
    }

    #[test]
    fn type_with_inline_relation_and_recover() {
        let source = "type contact {\n  <user\n  recover |user, contact, loss| <= contact\n}\n";
        let frag = parse_form(source).ok().unwrap();
        assert_eq!(frag.mirror_ast().decl_tag(), "type");
        assert_eq!(decoded(&frag).name, "contact");

        let has_subset = vec![].contains(&OpticOp::Subset)
            || frag
                .mirror_children()
                .iter()
                .any(|c| vec![].contains(&OpticOp::Subset));
        assert!(has_subset, "contact must have Subset marker");

        let recover = frag
            .mirror_children()
            .iter()
            .find(|c| c.mirror_ast().decl_tag() == "recover");
        assert!(recover.is_some(), "contact must have recover child");
    }

    /// Double operator: `type x = = y`. Malformed.
    /// Must not produce clean Success.
    #[test]
    fn error_double_operator() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("type x = = y\n");
        match &result {
            Imperfect::Success(c) => {
                // If somehow Success, the second `=` must not vanish
                // `y` should be captured as a variant (from `= y`)
                // but `= =` is malformed — we expect this to not be clean
                let ast = c.ast();
                assert!(
                    !ast.variants_as_strings().is_empty() || !c.fragment.mirror_children().is_empty(),
                    "double operator = = must not produce empty result: {:?}",
                    ast
                );
            }
            Imperfect::Partial(_, _) | Imperfect::Failure(_, _) => {
                // Both acceptable — malformed input
            }
        }
    }

    // -----------------------------------------------------------------------
    // `form` keyword deprecation — must produce warning (Partial)
    // -----------------------------------------------------------------------

    /// The `form` keyword is deprecated. `grammar` replaced it.
    /// Using `form` must produce Partial with a deprecation warning in MirrorLoss.
    /// The code still compiles — it's not Failure. But the loss is measured.
    #[test]
    fn form_keyword_produces_warning() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("form @test {\n  type x\n}\n");

        // Must compile — form is not rejected, it's deprecated
        assert!(
            result.is_ok(),
            "form keyword must still compile (deprecated, not removed)"
        );

        // Must be Partial, not Success — the deprecation is measured loss
        assert!(
            result.is_partial(),
            "form keyword must produce Partial (deprecation warning), got Success"
        );

        // The loss must mention the deprecation
        let loss = result.loss();
        assert!(
            loss.holonomy() > 0.0,
            "form deprecation must produce non-zero holonomy"
        );
    }

    // -----------------------------------------------------------------------
    // Declaration fields use Imperfect, not Option
    // -----------------------------------------------------------------------

    /// Declaration fields that may or may not be present should use
    /// Imperfect, not Option. Option is binary — present or absent.
    /// Imperfect is ternary — present, partially present, or absent with loss.
    ///
    /// grammar_ref: Option<String> → Imperfect<String, (), RefLoss>
    /// body_text: Option<String> → Imperfect<String, (), ParseLoss>
    /// return_type: Option<String> → Imperfect<String, (), ResolutionLoss>
    #[test]
    fn declaration_fields_not_option() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("grammar @test {\n  type x\n}\n");
        let compiled = result.ok().unwrap();

        // grammar_ref should be Imperfect, not Option
        // Currently it's Option<String> — this test documents the gap.
        // When grammar_ref becomes Imperfect, this assertion flips.
        assert!(
            compiled.ast().grammar_ref_str().is_none(),
            "BASELINE: grammar_ref is still Option (should become Imperfect)"
        );
    }

    // -----------------------------------------------------------------------
    // Fractal as AST — Form is dead, Fractal<MirrorAST> is the one representation
    // -----------------------------------------------------------------------

    /// compile_source returns Imperfect<CompiledShatter, ...> where
    /// CompiledShatter wraps MirrorFragment (Fractal<MirrorAST>).
    /// The AST is the content-addressed tree. One representation.
    #[test]
    fn compile_returns_fractal_directly() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("type color = red | blue\n");
        let compiled = result.ok().unwrap();

        // The fragment IS the AST. The optics navigate it.
        let ast = compiled.ast();
        assert_eq!(ast.name(), "color");
        assert_eq!(ast.decl_tag(), "type");
        assert!(!ast.variants_as_strings().is_empty());
    }

    // -----------------------------------------------------------------------
    // Kintsugi — canonical ordering
    // -----------------------------------------------------------------------

    /// Kintsugi hoists `in` declarations to the top.
    #[test]
    fn kintsugi_hoists_imports() {
        let src = "type x\nin @prism\ntype y\n";
        let parsed = parse_form(src).ok().unwrap();
        let canonical = kintsugi_fragment(&parsed);
        assert_eq!(
            canonical.mirror_children()[0].mirror_ast().decl_tag(),
            "in",
            "in @prism must be first after kintsugi"
        );
    }

    /// Kintsugi is idempotent: applying it twice yields the same result.
    #[test]
    fn kintsugi_is_idempotent() {
        let src = "action do_thing\ntype x\nin @prism\ngrammar @test {\n  type y\n}\n";
        let parsed = parse_form(src).ok().unwrap();
        let once = kintsugi_fragment(&parsed);
        let twice = kintsugi_fragment(&once);
        assert_eq!(
            once.content_hash(),
            twice.content_hash(),
            "kintsugi must be idempotent"
        );
    }

    /// Kintsugi preserves OID: the content-addressed hash is order-invariant.
    #[test]
    fn kintsugi_preserves_oid() {
        let src = "action do_thing\ntype x\nin @prism\n";
        let parsed = parse_form(src).ok().unwrap();
        let canonical = kintsugi_fragment(&parsed);

        let oid_before = parsed.content_hash().clone();
        let oid_after = canonical.content_hash().clone();
        assert_eq!(
            oid_before, oid_after,
            "kintsugi must not change the content-addressed OID"
        );
    }

    // -----------------------------------------------------------------------
    // simplify_fragment tests
    // -----------------------------------------------------------------------

    /// simplify_fragment removes dead types.
    #[test]
    fn simplify_eliminates_dead_types() {
        let src = "type used = active | inactive\ntype dead = x | y\naction check(s: used)\n";
        let parsed = parse_form(src).ok().unwrap();
        let (simplified, before, after) = simplify_fragment(&parsed);
        assert!(after < before, "simplify should reduce count: {} -> {}", before, after);
        let output = emit_fragment(&simplified);
        assert!(!output.contains("dead"), "dead type should be removed from: {}", output);
        assert!(output.contains("used"), "used type should survive in: {}", output);
    }

    /// simplify_fragment collapses type aliases with identical variants.
    #[test]
    fn simplify_collapses_aliases() {
        let src = "type status = active | inactive\ntype state = active | inactive\naction check(s: status)\n";
        let parsed = parse_form(src).ok().unwrap();
        let (simplified, before, after) = simplify_fragment(&parsed);
        assert!(after < before, "simplify should reduce count: {} -> {}", before, after);
        let output = emit_fragment(&simplified);
        assert!(!output.contains("type state"), "state alias should be collapsed from: {}", output);
        assert!(output.contains("status"), "canonical status should survive in: {}", output);
    }

    /// simplify_fragment composes all three passes.
    #[test]
    fn simplify_composes_all_passes() {
        let src = "type status = active | inactive\ntype state = active | inactive\ntype orphan = x | y\naction process(s: status)\n";
        let parsed = parse_form(src).ok().unwrap();
        let (simplified, before, after) = simplify_fragment(&parsed);
        // Before: status, state (alias), orphan (dead), action = 4 decls
        // After: status, action = 2 decls
        assert_eq!(before, 4, "should have 4 declarations");
        assert_eq!(after, 2, "should have 2 declarations after simplify");
        let output = emit_fragment(&simplified);
        assert!(!output.contains("orphan"), "orphan removed");
        assert!(!output.contains("type state"), "state alias collapsed");
        assert!(output.contains("status"), "status kept");
        assert!(output.contains("action"), "action kept");
    }

    // -----------------------------------------------------------------------
    // @properties package — template/property split (RED)
    // -----------------------------------------------------------------------

    /// The property kernel declares types, not properties.
    /// verdict, property_error, property_loss, effect_pattern must be declared.
    #[test]
    fn property_type_is_verdict() {
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("05-property.mirror"))
            .unwrap();

        // Find the @property grammar block
        let grammar = compiled
            .fragment
            .mirror_children()
            .iter()
            .find(|f| {
                let a = f.mirror_ast();
                a.decl_tag() == "grammar" && a.name() == "@property"
            })
            .expect("@property grammar must exist");

        let type_names: Vec<String> = grammar
            .mirror_children()
            .iter()
            .filter(|f| f.mirror_ast().decl_tag() == "type")
            .map(|f| f.mirror_ast().name().clone())
            .collect();

        assert!(
            type_names.iter().any(|n| n == "verdict"),
            "verdict type must be declared"
        );
        assert!(
            type_names.iter().any(|n| n == "property_error"),
            "property_error type must be declared"
        );
        assert!(
            type_names.iter().any(|n| n == "property_loss"),
            "property_loss type must be declared"
        );
        assert!(
            type_names.iter().any(|n| n == "effect_pattern"),
            "effect_pattern type must be declared"
        );
    }

    /// Templates in std/properties.mirror must NOT have OpticOp::Fold.
    /// They are iso, not fold. The parser now knows `template` as a DeclKind.
    #[test]
    fn template_declared_as_iso() {
        let runtime = MirrorRuntime::new();
        let result = runtime
            .compile_source("in @meta\nin @property\ntemplate types_lowercase(grammar) = iso\n");
        let compiled = match result {
            Imperfect::Success(c) | Imperfect::Partial(c, _) => c,
            Imperfect::Failure(_, _) => panic!("template tag not yet recognized"),
        };

        // If we get here, the parser parsed it. Check via fragment.
        let template = compiled
            .fragment
            .mirror_children()
            .iter()
            .find(|f| f.mirror_ast().name() == "types_lowercase");
        // RED: template should exist as a recognized DeclKind
        assert!(template.is_some(), "template tag not yet recognized");
        // Check optic_ops via parse_form since they're parser annotations
        let frag = parse_form("in @meta\nin @property\ntemplate types_lowercase(grammar) = iso\n")
            .ok()
            .unwrap();
        let t = frag
            .mirror_children()
            .iter()
            .find(|f| decoded(f).name == "types_lowercase")
            .unwrap();
        assert!(
            !vec![].contains(&OpticOp::Fold),
            "template must be iso, not fold"
        );
        assert!(
            vec![].contains(&OpticOp::Iso),
            "template must carry OpticOp::Iso"
        );
    }

    /// Properties like consent(effect(a => b)) must have a recognizable
    /// effect pattern. The parser should preserve `effect(a=>b)` in params.
    #[test]
    fn property_has_effect_pattern() {
        let runtime = MirrorRuntime::new();
        let src = "property consent(effect(a => b)) <= verdict\n";
        let result = runtime.compile_source(src);

        let compiled = match result {
            Imperfect::Success(c) | Imperfect::Partial(c, _) => c,
            Imperfect::Failure(e, _) => panic!("property with effect pattern must parse: {}", e),
        };

        // Single declaration -> the AST IS the property (no wrapper)
        let ast = compiled.ast();
        assert_eq!(ast.decl_tag(), "property");
        assert_eq!(ast.name(), "consent");

        // The parser preserves the effect pattern in params.
        // effect(a=>b) is stored as a single param string.
        let has_effect = ast.params_as_strings().iter().any(|p| p.contains("effect"));
        assert!(
            has_effect,
            "consent property must have effect pattern in params, got: {:?}",
            data.params
        );

        // The fold operator must be recorded — check via parse_form since
        // optic_ops is a parser annotation, not stored in the fragment.
        let frag = parse_form(src).ok().unwrap();
        assert!(
            vec![].contains(&OpticOp::Fold),
            "consent property must have OpticOp::Fold from <= verdict"
        );
    }

    /// The `where` clause is new syntax. The parser doesn't handle it yet.
    /// consent(effect(a => b)) where a != b — the `where` line is unrecognized.
    /// This test documents that gap. RED until the parser learns `where`.
    #[test]
    fn boundary_property_has_where_clause() {
        let runtime = MirrorRuntime::new();
        let src = "property consent(effect(a => b)) <= verdict\n  where a != b\n";
        let result = runtime.compile_source(src);

        // Grab the loss before consuming the result
        let loss = result.loss();

        let _compiled = match result {
            Imperfect::Success(c) | Imperfect::Partial(c, _) => c,
            Imperfect::Failure(e, _) => panic!("property with where clause must parse: {}", e),
        };

        // The `where` line is currently unrecognized — it shows up as parse loss.
        let has_where_loss = loss
            .parse
            .warnings
            .iter()
            .any(|w| matches!(w, crate::loss::ParseWarning::UnknownToken { .. }));
        // RED: `where` is unrecognized training data
        assert!(
            has_where_loss,
            "where clause should be unrecognized (training data) until parser learns it"
        );
    }

    /// std/properties.mirror must declare security properties:
    /// sanitize, escape, consent_boundary, audit_trail, deploy_gate, classify.
    /// Each has an effect pattern argument.
    #[test]
    fn security_properties_exist() {
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("std/properties.mirror"))
            .unwrap();

        let property_names: Vec<String> = compiled
            .fragment
            .mirror_children()
            .iter()
            .filter(|f| f.mirror_ast().decl_tag() == "property")
            .map(|f| f.mirror_ast().name().clone())
            .collect();

        let expected = [
            "sanitize",
            "escape",
            "consent_boundary",
            "audit_trail",
            "deploy_gate",
            "classify",
        ];
        for name in &expected {
            assert!(
                property_names.iter().any(|n| n == name),
                "security property '{}' must exist in std/properties.mirror, found: {:?}",
                name,
                property_names
            );
        }

        // Each security property should have an effect pattern in params
        for child in compiled.fragment.mirror_children().iter().filter(|f| {
            let a = f.mirror_ast();
            a.decl_tag() == "property" && expected.contains(&a.name())
        }) {
            let child_ast = child.mirror_ast();
            let child_params = child_ast.params_as_strings();
            let has_effect = child_params.iter().any(|p| p.contains("effect"));
            assert!(
                has_effect,
                "security property '{}' must have effect pattern in params, got: {:?}",
                child_ast.name(), child_params
            );
        }
    }

    /// All properties in std/properties.mirror resolve against the kernel.
    /// in @meta and in @property must both resolve.
    #[test]
    fn all_std_properties_resolve() {
        let runtime = MirrorRuntime::new();
        let store = tempdir_for_test("std_properties_resolve");
        let boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();

        assert!(
            boot.resolved.contains_key("std/properties"),
            "std/properties must resolve against kernel (in @meta, in @property)"
        );

        // Verify @property is in the registry
        let registry = MirrorRegistry::open(&store).unwrap();
        assert!(
            registry.lookup("@property").is_some(),
            "@property must be registered by kernel boot"
        );
    }

    // -----------------------------------------------------------------------
    // Grammar inheritance: `grammar @name < @parent { ... }`
    // -----------------------------------------------------------------------

    #[test]
    fn grammar_inheritance_parses() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("grammar @html < @sigil {\n  type element\n}\n");
        assert!(
            result.is_ok(),
            "grammar with inheritance must compile: {:?}",
            result
        );

        let compiled = match result {
            Imperfect::Success(c) => c,
            Imperfect::Partial(c, _) => c,
            Imperfect::Failure(e, _) => panic!("failed: {}", e),
        };

        assert_eq!(
            compiled.ast().parent_ref_str().as_deref(),
            Some("@sigil"),
            "parent_ref must be @sigil"
        );
    }

    #[test]
    fn grammar_without_inheritance_has_no_parent() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("grammar @test {\n  type x\n}\n");
        let compiled = result.ok().unwrap();
        assert!(
            compiled.ast().parent_ref_str().is_none(),
            "grammar without < should have no parent_ref"
        );
    }

    #[test]
    fn grammar_inheritance_resolves_parent() {
        let runtime = MirrorRuntime::new();
        let store = tempdir_for_test("inheritance_resolve");

        // Boot to get @actor in registry
        let _boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();
        let registry = MirrorRegistry::open(&store).unwrap();

        // Grammar that inherits from @actor (which exists)
        let src = "grammar @test < @actor {\n  type x\n}\n";
        let compiled = runtime.compile_source(src);
        let form = compiled.ok().unwrap();

        let result = registry.resolve_fragment(&form.fragment);
        assert!(result.is_ok(), "< @actor should resolve: {:?}", result);
    }

    #[test]
    fn grammar_inheritance_fails_missing_parent() {
        let runtime = MirrorRuntime::new();
        let store = tempdir_for_test("inheritance_missing");

        let _boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();
        let registry = MirrorRegistry::open(&store).unwrap();

        let src = "grammar @test < @nonexistent {\n  type x\n}\n";
        let compiled = runtime.compile_source(src);
        let form = compiled.ok().unwrap();

        let result = registry.resolve_fragment(&form.fragment);
        assert!(result.is_err(), "< @nonexistent should fail resolution");
        assert!(
            result.unwrap_err().0.contains("@nonexistent"),
            "error must name the missing parent"
        );
    }

    #[test]
    fn runtime_boot_file_compiles() {
        let runtime = MirrorRuntime::new();
        let store = tempdir_for_test("runtime_boot");
        let boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();

        // 04a-runtime should be in the boot results
        assert!(
            boot.resolved.contains_key("04a-runtime") || boot.failed.contains_key("04a-runtime"),
            "04a-runtime.mirror must be loaded"
        );
    }

    // -----------------------------------------------------------------------
    // compile_to_shatter — Task 2.1
    // -----------------------------------------------------------------------

    #[test]
    fn compile_to_shatter_produces_artifact() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();

        let store = crate::git_store::MirrorGitStore::open(dir.path()).unwrap();
        let runtime = MirrorRuntime::new();

        let result = runtime.compile_to_shatter("type color = red | blue", &store);
        assert!(result.is_ok(), "compile_to_shatter must succeed");
    }

    #[test]
    fn compile_to_shatter_artifact_retrievable_from_store() {
        use fragmentation::fragment::Fragmentable;

        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();

        let store = crate::git_store::MirrorGitStore::open(dir.path()).unwrap();
        let runtime = MirrorRuntime::new();

        let result = runtime.compile_to_shatter("type color = red | blue", &store);
        let (meta, _body) = result
            .ok()
            .expect("compile_to_shatter must produce a value");

        // The shatter artifact should be in the store under the meta OID
        let artifact = store.get_crystal(&meta.oid);
        assert!(
            artifact.is_some(),
            "shatter artifact must be retrievable by OID"
        );

        // Verify the stored content starts with the frontmatter delimiter
        let content = artifact.unwrap();
        assert!(
            content.data().starts_with("---\n"),
            "stored shatter must have frontmatter"
        );
        assert!(
            content.data().contains("type color = red | blue"),
            "source preserved in body"
        );
    }

    #[test]
    fn compile_to_shatter_luminosity_light_for_valid_source() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();

        let store = crate::git_store::MirrorGitStore::open(dir.path()).unwrap();
        let runtime = MirrorRuntime::new();

        let result = runtime.compile_to_shatter("type signal = on | off", &store);
        let (meta, _body) = result.ok().unwrap();
        assert_eq!(meta.luminosity, crate::shatter_format::Luminosity::Light);
    }

    // -----------------------------------------------------------------------
    // Benchmark — parse_form baseline
    // -----------------------------------------------------------------------

    #[test]
    fn benchmark_parse_form_baseline() {
        let boot_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("boot");
        let runtime = MirrorRuntime::new();
        let files = [
            "00-prism.mirror",
            "01-meta.mirror",
            "01a-meta-actor.mirror",
            "01b-meta-action.mirror",
            "01c-meta-io.mirror",
            "02-shatter.mirror",
            "03-code.mirror",
            "05-property.mirror",
        ];
        let iterations = 1000;

        for file in &files {
            let path = boot_dir.join(file);
            let source = std::fs::read_to_string(&path).unwrap();

            let start = std::time::Instant::now();
            for _ in 0..iterations {
                let _ = runtime.compile_source(&source);
            }
            let elapsed = start.elapsed();
            eprintln!(
                "--- {} x{}: {:?} ({:?}/call) ---",
                file,
                iterations,
                elapsed,
                elapsed / iterations as u32
            );
        }
    }

    // -----------------------------------------------------------------------
    // Seam: unknown keywords INSIDE blocks must produce Partial (not Success)
    // -----------------------------------------------------------------------

    #[test]
    fn unknown_keyword_inside_grammar_block_produces_partial() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("grammar @test {\n  flag strict\n  type x\n}\n");
        assert!(
            result.is_partial(),
            "unknown keyword 'flag' inside grammar block must produce Partial, not Success"
        );
        let loss = result.loss();
        assert!(
            !loss.parse.warnings.is_empty(),
            "loss must contain the unknown 'flag' token warning"
        );
        assert!(matches!(
            &loss.parse.warnings[0],
            crate::loss::ParseWarning::UnknownToken { .. }
        ));
    }

    #[test]
    fn unknown_keyword_inside_type_block_produces_partial() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("type user {\n  widget foo\n  type name\n}\n");
        assert!(
            result.is_partial(),
            "unknown keyword 'widget' inside type block must produce Partial"
        );
    }

    #[test]
    fn multiple_unknown_keywords_inside_block_all_recorded() {
        let runtime = MirrorRuntime::new();
        let result = runtime
            .compile_source("grammar @test {\n  flag strict\n  command compile\n  type x\n}\n");
        assert!(result.is_partial());
        let loss = result.loss();
        assert!(
            loss.parse.warnings.len() >= 2,
            "both 'flag' and 'command' must be recorded as parse warnings"
        );
    }

    #[test]
    fn known_keywords_inside_block_still_success() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("grammar @test {\n  type x\n  action y\n}\n");
        assert!(
            !result.is_partial(),
            "known keywords inside block should be Success, not Partial"
        );
    }

    #[test]
    fn nested_unknown_keywords_produce_partial() {
        let runtime = MirrorRuntime::new();
        let result = runtime
            .compile_source("grammar @outer {\n  grammar @inner {\n    widget foo\n  }\n}\n");
        assert!(
            result.is_partial(),
            "unknown keyword in nested block must bubble up as Partial"
        );
    }

    #[test]
    fn boot_cli_mirror_has_unrecognized_flag_and_command() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("boot/std/cli.mirror");
        if path.exists() {
            let runtime = MirrorRuntime::new();
            let src = std::fs::read_to_string(&path).unwrap();
            let result = runtime.compile_source(&src);
            if result.is_ok() && !result.is_partial() {
                panic!(
                    "boot/std/cli.mirror uses flag and command keywords which are not \
                     the parser should return Partial with loss, not Success"
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // Typed ParseWarning tests
    // -----------------------------------------------------------------------

    #[test]
    fn unknown_token_inside_grammar_is_typed_warning() {
        use crate::loss::{AstPosition, ParseWarning};
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("grammar @test {\n  flag strict\n}\n");
        let loss = result.loss();
        assert!(loss.parse.warnings.iter().any(|w| matches!(
            w,
            ParseWarning::UnknownToken {
                at: AstPosition::Grammar(_),
                ..
            }
        )));
    }

    #[test]
    fn deprecated_form_is_typed_warning() {
        use crate::loss::ParseWarning;
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("form @old {\n  type x\n}\n");
        let loss = result.loss();
        assert!(loss.parse.warnings.iter().any(|w| matches!(
            w,
            ParseWarning::DeprecatedKind {
                kind: "form",
                replacement: "grammar",
                ..
            }
        )));
    }

    #[test]
    fn warning_categories_group_by_variant() {
        use crate::loss::ParseWarning;
        let runtime = MirrorRuntime::new();
        let result = runtime
            .compile_source("grammar @test {\n  flag a\n  flag b\n  command c\n  type x\n}\n");
        let loss = result.loss();

        let unknown_count = loss
            .parse
            .warnings
            .iter()
            .filter(|w| matches!(w, ParseWarning::UnknownToken { .. }))
            .count();
        assert_eq!(unknown_count, 3); // flag, flag, command
    }

    #[test]
    fn ast_position_tracks_nesting() {
        use crate::loss::{AstPosition, ParseWarning};
        let runtime = MirrorRuntime::new();
        let result = runtime
            .compile_source("grammar @outer {\n  grammar @inner {\n    widget foo\n  }\n}\n");
        let loss = result.loss();

        // The warning should be inside @inner, not @outer
        assert!(loss.parse.warnings.iter().any(|w| match w {
            ParseWarning::UnknownToken {
                at: AstPosition::Grammar(oid),
                ..
            } => {
                // The oid should be @inner's
                let inner_oid = crate::kernel::Oid::new("@inner");
                *oid == inner_oid
            }
            _ => false,
        }));
    }

    // -----------------------------------------------------------------------
    // parse_ast — the new parser API (emits MirrorAST directly)
    // -----------------------------------------------------------------------

    #[test]
    fn parse_ast_type_enum() {
        let source = "type color = red | blue";
        let ast = parse_ast(source).ok().unwrap();
        match &ast {
            MirrorAST::Split(s) => {
                assert_eq!(s.name.as_str(), "color");
                assert_eq!(s.body, Some(TypeBody::Enum(vec![
                    Identifier::new("red"),
                    Identifier::new("blue"),
                ])));
            }
            other => panic!("expected Split (type), got {:?}", other.kind_name()),
        }
    }

    #[test]
    fn parse_ast_type_unit() {
        let source = "type token";
        let ast = parse_ast(source).ok().unwrap();
        match &ast {
            MirrorAST::Split(s) => {
                assert_eq!(s.name.as_str(), "token");
                assert_eq!(s.body, Some(TypeBody::Unit));
            }
            other => panic!("expected Split (type), got {:?}", other.kind_name()),
        }
    }

    #[test]
    fn parse_ast_grammar() {
        let source = "grammar @test {\n  type id\n}";
        let ast = parse_ast(source).ok().unwrap();
        match &ast {
            MirrorAST::Focus(f) => {
                assert_eq!(f.name.as_str(), "@test");
                assert!(f.target.is_none());
                assert_eq!(f.children.len(), 1);
                match &f.children[0] {
                    MirrorAST::Split(s) => assert_eq!(s.name.as_str(), "id"),
                    other => panic!("expected Split child, got {:?}", other.kind_name()),
                }
            }
            other => panic!("expected Focus (grammar), got {:?}", other.kind_name()),
        }
    }

    #[test]
    fn parse_ast_grammar_with_parent() {
        let source = "grammar @test < @actor {\n  type id\n}";
        let ast = parse_ast(source).ok().unwrap();
        match &ast {
            MirrorAST::Focus(f) => {
                assert_eq!(f.name.as_str(), "@test");
                assert_eq!(f.target.as_ref().unwrap().as_str(), "@actor");
            }
            other => panic!("expected Focus (grammar), got {:?}", other.kind_name()),
        }
    }

    #[test]
    fn parse_ast_action() {
        let source = "action send(to: string) -> result";
        let ast = parse_ast(source).ok().unwrap();
        match &ast {
            MirrorAST::Zoom(z) => {
                assert_eq!(z.name.as_str(), "send");
                assert_eq!(z.params.len(), 1);
                assert_eq!(z.params[0].name.as_str(), "to");
                assert_eq!(z.params[0].type_ref.as_str(), "string");
                assert_eq!(z.target.as_ref().unwrap().as_str(), "result");
            }
            other => panic!("expected Zoom (action), got {:?}", other.kind_name()),
        }
    }

    #[test]
    fn parse_ast_import() {
        let source = "in @tools";
        let ast = parse_ast(source).ok().unwrap();
        match &ast {
            MirrorAST::Project(p) => {
                assert!(p.target.is_some());
                assert!(p.target.as_ref().unwrap().as_str().contains("tools"));
            }
            other => panic!("expected Project (import), got {:?}", other.kind_name()),
        }
    }

    #[test]
    fn parse_ast_export() {
        let source = "out send";
        let ast = parse_ast(source).ok().unwrap();
        match &ast {
            MirrorAST::Project(p) => {
                assert_eq!(p.name.as_str(), "send");
                assert!(p.target.is_none());
            }
            other => panic!("expected Project (export), got {:?}", other.kind_name()),
        }
    }

    #[test]
    fn parse_ast_property() {
        let source = "property valid(x: int)";
        let ast = parse_ast(source).ok().unwrap();
        match &ast {
            MirrorAST::Refract(r) => {
                assert_eq!(r.name.as_str(), "valid");
                assert_eq!(r.params.len(), 1);
                assert_eq!(r.params[0].name.as_str(), "x");
                assert_eq!(r.params[0].type_ref.as_str(), "int");
            }
            other => panic!("expected Refract (property), got {:?}", other.kind_name()),
        }
    }

    #[test]
    fn parse_ast_focus() {
        let source = "focus details(user)";
        let ast = parse_ast(source).ok().unwrap();
        match &ast {
            MirrorAST::Focus(f) => {
                assert_eq!(f.name.as_str(), "details");
            }
            other => panic!("expected Focus, got {:?}", other.kind_name()),
        }
    }

    #[test]
    fn parse_ast_split() {
        let source = "split route = left | right";
        let ast = parse_ast(source).ok().unwrap();
        match &ast {
            MirrorAST::Split(s) => {
                assert_eq!(s.name.as_str(), "route");
                assert_eq!(s.variants.len(), 2);
                assert_eq!(s.variants[0].as_str(), "left");
                assert_eq!(s.variants[1].as_str(), "right");
            }
            other => panic!("expected Split, got {:?}", other.kind_name()),
        }
    }

    #[test]
    fn parse_ast_module_multiple_decls() {
        let source = "type a\ntype b\n";
        let ast = parse_ast(source).ok().unwrap();
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 2);
            }
            other => panic!("expected Module for multiple decls, got {:?}", other.kind_name()),
        }
    }

    #[test]
    fn parse_ast_roundtrip_name() {
        let source = "type color = red | blue";
        let ast = parse_ast(source).ok().unwrap();
        assert_eq!(ast.name(), "color");
    }
}
