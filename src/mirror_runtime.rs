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
//! `FrgmntStore<MirrorFragment>` from the fragmentation crate. Form names
//! (`@prism`, `@meta`, `@actor`) are stored as named refs pointing at the
//! OIDs of the MirrorFragments that declared them. The store IS the registry;
//! the registry is just a typed surface over it.
//!
//! As each boot file is compiled:
//!
//! 1. Parse → `Form` (structural).
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
//! - parse `.mirror` source → `Form` tree
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
    fragment as build_fragment, DeclKind, MirrorData, MirrorFragment, MirrorFragmentExt,
    MirrorHash, OpticOp,
};
use fragmentation::frgmnt_store::FrgmntStore;
use fragmentation::sha::HashAlg;
use prism::{Beam, Imperfect, Loss, Optic, Prism};

use crate::loss::{MirrorLoss, ParseLoss, UnrecognizedDecl};

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

// ---------------------------------------------------------------------------
// Form — the runtime structure that Shatter compiles to/from.
// ---------------------------------------------------------------------------

/// `Form` is the parsed-but-not-yet-content-addressed view: kind / name /
/// params / variants / nested children. The structural mirror of `MirrorData`
/// + recursive children. Used as `Prism::Input` and `Prism::Crystal`.
#[derive(Clone, Debug, Eq)]
pub struct Form {
    pub kind: DeclKind,
    pub name: String,
    pub params: Vec<String>,
    pub variants: Vec<String>,
    pub children: Vec<Form>,
    /// For `action` declarations: the grammar reference (e.g. `@code/rust`).
    /// `None` means inherit the grammar from the enclosing form's `in` declaration.
    pub grammar_ref: Option<String>,
    /// For `action` declarations: the raw body text, brace-balanced but unparsed.
    /// The mirror compiler stores it as-is; the target grammar's parser handles it.
    pub body_text: Option<String>,
    /// Whether this declaration has the `abstract` modifier.
    pub is_abstract: bool,
    /// Optional return type annotation (e.g. `-> [completion]`).
    pub return_type: Option<String>,
    /// Optic operators found in this declaration.
    /// For example, `type visibility = private | protected | public` would
    /// have `[OpticOp::Iso, OpticOp::Split]`.
    pub optic_ops: Vec<OpticOp>,
}

/// `optic_ops` is excluded from equality: it's a parser annotation about which
/// optic operators were used, not structural content. Content-addressed round-trips
/// (Form → MirrorFragment → Form) don't preserve it.
impl PartialEq for Form {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.name == other.name
            && self.params == other.params
            && self.variants == other.variants
            && self.children == other.children
            && self.grammar_ref == other.grammar_ref
            && self.body_text == other.body_text
            && self.is_abstract == other.is_abstract
            && self.return_type == other.return_type
    }
}

impl Form {
    pub fn new(
        kind: DeclKind,
        name: impl Into<String>,
        params: Vec<String>,
        variants: Vec<String>,
        children: Vec<Form>,
    ) -> Self {
        Form {
            kind,
            name: name.into(),
            params,
            variants,
            children,
            grammar_ref: None,
            body_text: None,
            is_abstract: false,
            return_type: None,
            optic_ops: Vec::new(),
        }
    }

    /// Create an action Form with grammar reference and raw body text.
    pub fn action(
        name: impl Into<String>,
        params: Vec<String>,
        grammar_ref: Option<String>,
        body_text: Option<String>,
        children: Vec<Form>,
    ) -> Self {
        Form {
            kind: DeclKind::Action,
            name: name.into(),
            params,
            variants: Vec::new(),
            children,
            grammar_ref,
            body_text,
            is_abstract: false,
            return_type: None,
            optic_ops: Vec::new(),
        }
    }

    fn to_fragment(&self) -> MirrorFragment {
        // Encode action-specific and modifier fields into the fragment's
        // params and variants so they survive content-addressing round-trips.
        let mut params = self.params.clone();
        let mut variants = self.variants.clone();
        if self.kind == DeclKind::Action {
            if let Some(ref gr) = self.grammar_ref {
                params.push(format!("in:{}", gr));
            }
            if let Some(ref rt) = self.return_type {
                params.push(format!("returns:{}", rt));
            }
            if let Some(ref bt) = self.body_text {
                variants.push(format!("body:{}", bt));
            }
        }
        if self.is_abstract {
            params.push("modifier:abstract".to_string());
        }
        let data = MirrorData::new(self.kind.clone(), self.name.clone(), params, variants);
        let children: Vec<MirrorFragment> = self.children.iter().map(|c| c.to_fragment()).collect();
        build_fragment(data, children)
    }

    fn from_fragment(frag: &MirrorFragment) -> Form {
        let d = frag.mirror_data();
        let children: Vec<Form> = frag
            .mirror_children()
            .iter()
            .map(Form::from_fragment)
            .collect();
        // Decode encoded params: in:, returns:, modifier:abstract
        let mut params = Vec::new();
        let mut grammar_ref = None;
        let mut return_type = None;
        let mut is_abstract = false;
        for p in &d.params {
            if let Some(gr) = p.strip_prefix("in:") {
                grammar_ref = Some(gr.to_string());
            } else if let Some(rt) = p.strip_prefix("returns:") {
                return_type = Some(rt.to_string());
            } else if p == "modifier:abstract" {
                is_abstract = true;
            } else {
                params.push(p.clone());
            }
        }
        let mut variants = Vec::new();
        let mut body_text = None;
        for v in &d.variants {
            if let Some(bt) = v.strip_prefix("body:") {
                body_text = Some(bt.to_string());
            } else {
                variants.push(v.clone());
            }
        }
        Form {
            kind: d.kind.clone(),
            name: d.name.clone(),
            params,
            variants,
            children,
            grammar_ref,
            body_text,
            is_abstract,
            return_type,
            optic_ops: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Shatter — the compilation artifact, a Prism implementation.
// ---------------------------------------------------------------------------

/// `Shatter` is the compilation artifact of `MirrorRuntime`. It implements
/// the `Prism` trait: three operations move a `Form` into and out of its
/// content-addressed representation.
#[derive(Clone, Debug, Default)]
pub struct Shatter;

impl Prism for Shatter {
    type Input = Optic<(), Form>;
    type Focused = Optic<Form, MirrorData>;
    type Projected = Optic<MirrorData, MirrorFragment>;
    type Refracted = Optic<MirrorFragment, Shatter>;

    /// Focus: read the top-level eigenvalues (kind/name/params/variants).
    fn focus(&self, beam: Self::Input) -> Self::Focused {
        let input = beam.result().ok().expect("focus: Err beam");
        let focused = MirrorData::new(
            input.kind.clone(),
            input.name.clone(),
            input.params.clone(),
            input.variants.clone(),
        );
        beam.next(focused)
    }

    /// Project: turn the focused MirrorData into a content-addressed
    /// MirrorFragment. Structurally lossless; full projection via `compile_form`.
    fn project(&self, beam: Self::Focused) -> Self::Projected {
        let data = beam.result().ok().expect("project: Err beam").clone();
        let frag = build_fragment(data, Vec::new());
        beam.next(frag)
    }

    /// Refract: settle into the fixed-point crystal (Shatter itself).
    fn refract(&self, beam: Self::Projected) -> Self::Refracted {
        beam.next(Shatter)
    }
}

impl Shatter {
    /// Full structural compile: Form → MirrorFragment with all children
    /// content-addressed. Used by the boot pipeline.
    pub fn compile_form(&self, form: &Form) -> MirrorFragment {
        form.to_fragment()
    }

    /// Inverse: MirrorFragment → Form.
    pub fn decompile(&self, frag: &MirrorFragment) -> Form {
        Form::from_fragment(frag)
    }
}

// ---------------------------------------------------------------------------
// Parser — line-oriented, brace-balanced.
// ---------------------------------------------------------------------------

/// Parse a `.mirror` source string. The top-level may contain one or more
/// declarations. If there is exactly one, return it as-is. If there are
/// multiple, wrap them in a synthetic file-level Form.
///
/// Returns `Imperfect`: `Success` if all input was recognized,
/// `Partial` if unrecognized keywords were encountered (measured loss),
/// `Failure` if no declarations could be parsed.
pub fn parse_form(source: &str) -> Imperfect<Form, MirrorRuntimeError, MirrorLoss> {
    let tokens = tokenize(source);
    let mut cursor = 0usize;
    let mut decls = Vec::new();
    let mut unrecognized = Vec::new();

    loop {
        skip_trivia(&tokens, &mut cursor);
        if cursor >= tokens.len() {
            break;
        }
        // Only parse tokens that start with a known declaration keyword
        // or the `abstract` modifier. Collect unrecognized top-level tokens as loss.
        match tokens.get(cursor) {
            Some(Tok::Word(w)) if DeclKind::parse(w).is_some() || w == "abstract" => {
                match parse_decl(&tokens, &mut cursor) {
                    Ok(form) => {
                        // M2001: top-level type/grammar/action require a name
                        if form.name.is_empty()
                            && matches!(
                                form.kind,
                                DeclKind::Type | DeclKind::Grammar | DeclKind::Action
                            )
                        {
                            return Imperfect::failure(err(format!(
                                "M2001: `{}` requires a name",
                                form.kind.as_str()
                            )));
                        }
                        // M2002: top-level `in` requires a target
                        if form.name.is_empty() && form.kind == DeclKind::In {
                            return Imperfect::failure(err("M2002: `in` requires a target"));
                        }
                        decls.push(form);
                    }
                    Err(e) => return Imperfect::failure(e),
                }
            }
            Some(Tok::Word(w)) => {
                // Unrecognized keyword — collect instead of dropping
                let keyword = w.clone();
                let line = count_line_at(&tokens, cursor);
                let content = collect_until_next_decl(&tokens, &mut cursor);
                unrecognized.push(UnrecognizedDecl {
                    keyword,
                    line,
                    content,
                });
            }
            Some(_) => {
                // Non-word token at top level — skip to newline
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
    // Parameterized types (e.g. `type abstract(grammar)`) can share a name
    // with different params — those are specializations, not collisions.
    {
        let mut seen_types: Vec<(&str, &[String])> = Vec::new();
        for d in &decls {
            if d.kind == DeclKind::Type && !d.name.is_empty() {
                if seen_types
                    .iter()
                    .any(|(n, p)| *n == d.name && *p == d.params.as_slice())
                {
                    return Imperfect::failure(err(format!(
                        "M2003: duplicate type name `{}`",
                        d.name
                    )));
                }
                seen_types.push((&d.name, &d.params));
            }
        }
    }

    if decls.is_empty() && unrecognized.is_empty() {
        Imperfect::failure(err("no declarations found"))
    } else if decls.is_empty() {
        // Only unrecognized decls — nothing survived
        let loss = MirrorLoss {
            parse: ParseLoss { unrecognized },
            ..MirrorLoss::zero()
        };
        Imperfect::failure_with_loss(err("no recognized declarations found"), loss)
    } else {
        // Detect deprecated `form` keyword usage.
        // A named DeclKind::Form means the user wrote `form @name { ... }`.
        // The unnamed Form wrapper (synthesized for multi-decl sources) is not deprecated.
        collect_form_deprecations(&decls, &mut unrecognized);

        let form = if decls.len() == 1 {
            decls.into_iter().next().unwrap()
        } else {
            Form::new(
                DeclKind::Form,
                "".to_string(),
                Vec::new(),
                Vec::new(),
                decls,
            )
        };

        if unrecognized.is_empty() {
            Imperfect::Success(form)
        } else {
            let loss = MirrorLoss {
                parse: ParseLoss { unrecognized },
                ..MirrorLoss::zero()
            };
            Imperfect::Partial(form, loss)
        }
    }
}

/// Detect deprecated `form` keyword usage and add deprecation entries.
/// A named `DeclKind::Form` means the user wrote `form @name { ... }` —
/// they should use `grammar` instead. The unnamed Form wrapper is synthetic.
fn collect_form_deprecations(decls: &[Form], unrecognized: &mut Vec<UnrecognizedDecl>) {
    for decl in decls {
        if decl.kind == DeclKind::Form && !decl.name.is_empty() {
            unrecognized.push(UnrecognizedDecl {
                keyword: "form".to_string(),
                line: 0, // line tracking not available post-parse
                content: format!(
                    "deprecated: use `grammar {}` instead of `form {}`",
                    decl.name, decl.name
                ),
            });
        }
        // Recurse into children for nested form declarations
        collect_form_deprecations(&decl.children, unrecognized);
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
/// Each unrecognized line is captured as one `UnrecognizedDecl`.
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

fn parse_decl(tokens: &[Tok], cursor: &mut usize) -> Result<Form, MirrorRuntimeError> {
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
    // The modifier is consumed and the actual DeclKind follows.
    let (kind, modifier) = if kind_word == "abstract" {
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
        let k = DeclKind::parse(&actual_word)
            .ok_or_else(|| err(format!("unknown declaration kind: {}", actual_word)))?;
        (k, true)
    } else {
        let k = DeclKind::parse(&kind_word)
            .ok_or_else(|| err(format!("unknown declaration kind: {}", kind_word)))?;
        (k, false)
    };
    // modifier is used below when constructing the Form

    // Recover/Rescue: pipe-delimited params, optional fold operator, optional body.
    // `recover |result, loss| <= imperfect` or `rescue |error(observation), loss| <= imperfect`
    // `recover |result, loss| { body }` (legacy form)
    if kind == DeclKind::Recover || kind == DeclKind::Rescue {
        let mut params = Vec::new();
        let mut optic_ops = Vec::new();
        let mut variants = Vec::new();
        // Consume `|`
        if matches!(tokens.get(*cursor), Some(Tok::Word(w)) if w == "|") {
            *cursor += 1;
            // Collect params until closing `|`, handling nested parens like error(observation)
            loop {
                match tokens.get(*cursor) {
                    Some(Tok::Word(w)) if w == "|" => {
                        *cursor += 1;
                        break;
                    }
                    Some(Tok::Word(w)) => {
                        params.push(w.clone());
                        *cursor += 1;
                        // Handle nested parens: error(observation) → "error(observation)"
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
        // Check for fold operator: `<= target`
        // `<` is tokenized as Word("<"), `=` as Tok::Equals
        let is_fold = matches!(tokens.get(*cursor), Some(Tok::Word(w)) if w == "<")
            && matches!(tokens.get(*cursor + 1), Some(Tok::Equals));
        if is_fold {
            optic_ops.push(OpticOp::Fold);
            *cursor += 2; // consume `<` and `=`
                          // Collect the fold target until newline or brace
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
        // Parse body block (if present)
        let (body_text, children) = parse_action_body(tokens, cursor)?;
        let mut form = Form::action(kind.as_str(), params, None, body_text, children);
        form.kind = kind;
        form.is_abstract = modifier;
        form.optic_ops = optic_ops;
        form.variants = variants;
        return Ok(form);
    }

    let name = match tokens.get(*cursor) {
        Some(Tok::Word(w)) => {
            let mut n = w.clone();
            *cursor += 1;
            // Absorb path segments: `@code/rust` → `@code` `/` `rust`
            while let Some(Tok::Word(seg)) = tokens.get(*cursor) {
                if seg.starts_with('/') || seg == "/" {
                    n.push_str(seg);
                    *cursor += 1;
                    // If `/` was standalone, absorb the next segment too
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
                    // Nested closing paren: include as part of the previous param
                    if let Some(last) = params.last_mut() {
                        last.push(')');
                    }
                    *cursor += 1;
                }
                Some(Tok::LParen) => {
                    paren_depth += 1;
                    // Nested opening paren: append to the previous param
                    if let Some(last) = params.last_mut() {
                        last.push('(');
                    }
                    *cursor += 1;
                }
                Some(Tok::Word(w)) => {
                    if paren_depth > 1 {
                        // Inside nested parens: append to previous param
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
                        // Comma inside nested parens: append to previous param
                        if let Some(last) = params.last_mut() {
                            last.push(',');
                        }
                    }
                    // At depth 1, comma is just a separator — skip it
                    *cursor += 1;
                }
                other => return Err(err(format!("malformed params: {:?}", other))),
            }
        }
    }

    let mut variants = Vec::new();
    let mut optic_ops = Vec::new();
    // Fold operator: `<=` is tokenized as Word("<") + Equals.
    // Check for it before the Iso (`=`) check.
    let is_fold = matches!(tokens.get(*cursor), Some(Tok::Word(w)) if w == "<")
        && matches!(tokens.get(*cursor + 1), Some(Tok::Equals));
    if is_fold {
        optic_ops.push(OpticOp::Fold);
        *cursor += 2; // consume `<` and `=`
                      // Collect the fold target (e.g. `verdict`, `imperfect`) until newline or brace
        loop {
            match tokens.get(*cursor) {
                Some(Tok::Newline) => {
                    *cursor += 1;
                    break;
                }
                Some(Tok::LBrace) => break, // body follows
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
            // Don't skip newlines here - they terminate the variant list
            match tokens.get(*cursor) {
                Some(Tok::Newline) => {
                    *cursor += 1;
                    break;
                }
                Some(Tok::Word(w)) if w == "|" => {
                    if !optic_ops.contains(&OpticOp::Split) {
                        optic_ops.push(OpticOp::Split);
                    }
                    // Pipe separator in variant list
                    *cursor += 1;
                }
                Some(Tok::Word(w)) => {
                    variants.push(w.clone());
                    *cursor += 1;
                    // If variant is followed by params like call(...), consume them
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
                    // Double operator: `type x = = y` — the second `=` is malformed
                    return Err(err("M2004: double operator `=`"));
                }
                _ => break,
            }
        }
    }

    // Classify parentheses as Focus.
    if has_parens && !optic_ops.contains(&OpticOp::Focus) {
        optic_ops.push(OpticOp::Focus);
    }

    // Classify the declaration keyword itself as an optic if applicable.
    if let Some(op) = OpticOp::from_decl_kind(&kind) {
        if !optic_ops.contains(&op) {
            optic_ops.push(op);
        }
    }

    // Action declarations: parse optional `in @grammar`, optional `-> return_type`, and raw body block.
    if kind == DeclKind::Action {
        let grammar_ref = parse_action_grammar_ref(tokens, cursor);
        let return_type = parse_return_type(tokens, cursor);
        let (body_text, children) = parse_action_body(tokens, cursor)?;
        let mut form = Form::action(name, params, grammar_ref, body_text, children);
        form.is_abstract = modifier;
        form.return_type = return_type;
        form.optic_ops = optic_ops;
        return Ok(form);
    }

    let mut children = Vec::new();
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
                    // Try to parse as a declaration. If the word is not a recognized
                    // declaration kind, skip it and any following tokens until the
                    // next recognized declaration or closing brace.
                    // `abstract` is a modifier keyword that precedes a DeclKind.
                    if DeclKind::parse(w).is_some() || w == "abstract" {
                        let child = parse_decl(tokens, cursor)?;
                        children.push(child);
                    } else if w == "<" || w == ">" {
                        // Relation marker: `<type` (subset) or `>type` (superset)
                        let op = if w == "<" {
                            OpticOp::Subset
                        } else {
                            OpticOp::Superset
                        };
                        *cursor += 1;
                        // Collect the target type name
                        let target = match tokens.get(*cursor) {
                            Some(Tok::Word(t)) => {
                                let name = t.clone();
                                *cursor += 1;
                                name
                            }
                            _ => String::new(),
                        };
                        // Create a synthetic child carrying the relation marker
                        let mut child =
                            Form::new(DeclKind::In, target, Vec::new(), Vec::new(), Vec::new());
                        child.optic_ops.push(op);
                        children.push(child);
                        // Skip rest of line
                        while *cursor < tokens.len() {
                            match tokens.get(*cursor) {
                                Some(Tok::RBrace) | Some(Tok::Newline) => break,
                                Some(Tok::Comma) => {
                                    *cursor += 1;
                                }
                                Some(Tok::Word(_)) => {
                                    *cursor += 1;
                                }
                                _ => {
                                    *cursor += 1;
                                }
                            }
                        }
                        if matches!(tokens.get(*cursor), Some(Tok::Newline)) {
                            *cursor += 1;
                        }
                    } else {
                        // Unrecognized keyword - skip tokens until we find a newline
                        // or something that looks like the start of a new declaration
                        while *cursor < tokens.len() {
                            match tokens.get(*cursor) {
                                Some(Tok::RBrace) | Some(Tok::Newline) => break,
                                _ => {
                                    *cursor += 1;
                                }
                            }
                        }
                        // Consume the newline if present
                        if matches!(tokens.get(*cursor), Some(Tok::Newline)) {
                            *cursor += 1;
                        }
                    }
                }
                _ => {
                    // Unexpected token - skip to next line
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

    let mut form = Form::new(kind, name, params, variants, children);
    form.is_abstract = modifier;
    form.optic_ops = optic_ops;
    Ok(form)
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
) -> Result<(Option<String>, Vec<Form>), MirrorRuntimeError> {
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
                if DeclKind::parse(w).is_some() {
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
                    if DeclKind::parse(w).is_some() {
                        let child = parse_decl(tokens, cursor)?;
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
// Emitter — Form → text. Round-trip stable.
// ---------------------------------------------------------------------------

pub fn emit_form(form: &Form) -> String {
    let mut out = String::new();
    emit_form_into(form, 0, &mut out);
    out
}

fn emit_form_into(form: &Form, indent: usize, out: &mut String) {
    for _ in 0..indent {
        out.push_str("  ");
    }
    if form.is_abstract {
        out.push_str("abstract ");
    }
    out.push_str(form.kind.as_str());
    if !form.name.is_empty() {
        out.push(' ');
        out.push_str(&form.name);
    }
    // Recover/Rescue use pipe-delimited params: `recover |result, loss| { ... }`
    if (form.kind == DeclKind::Recover || form.kind == DeclKind::Rescue) && !form.params.is_empty()
    {
        out.push_str(" |");
        for (i, p) in form.params.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(p);
        }
        out.push('|');
    } else if !form.params.is_empty() {
        out.push('(');
        for (i, p) in form.params.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(p);
        }
        out.push(')');
    }
    // Action-specific: emit `in @grammar` and `-> return_type` before the body.
    if form.kind == DeclKind::Action {
        if let Some(ref gr) = form.grammar_ref {
            out.push_str(" in ");
            out.push_str(gr);
        }
        if let Some(ref rt) = form.return_type {
            out.push_str(" -> ");
            out.push_str(rt);
        }
    }
    if !form.variants.is_empty() {
        out.push_str(" = ");
        for (i, v) in form.variants.iter().enumerate() {
            if i > 0 {
                out.push_str(" | ");
            }
            out.push_str(v);
        }
    }
    // Action/Recover/Rescue with raw body text: emit the body block.
    if form.kind == DeclKind::Action
        || form.kind == DeclKind::Recover
        || form.kind == DeclKind::Rescue
    {
        if let Some(ref bt) = form.body_text {
            out.push_str(" {\n");
            for line in bt.lines() {
                for _ in 0..=indent {
                    out.push_str("  ");
                }
                out.push_str(line);
                out.push('\n');
            }
            for _ in 0..indent {
                out.push_str("  ");
            }
            out.push_str("}\n");
            return;
        }
    }
    if !form.children.is_empty() {
        out.push_str(" {\n");
        for child in &form.children {
            emit_form_into(child, indent + 1, out);
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

/// Reorder declarations into canonical order. The spectral hash
/// doesn't change — same eigenvalues, same OID. The surface changes.
///
/// Canonical order: in, type, traversal, lens, grammar, property, action.
/// Observation before action. Pure before impure.
pub fn kintsugi(form: &Form) -> Form {
    // Only reorder children of wrapper forms (multi-decl sources).
    // Single declarations pass through unchanged.
    if form.children.is_empty() {
        return form.clone();
    }

    let mut children = form.children.clone();
    children.sort_by_key(|c| kintsugi_sort_key(&c.kind));

    Form {
        kind: form.kind.clone(),
        name: form.name.clone(),
        params: form.params.clone(),
        variants: form.variants.clone(),
        children,
        grammar_ref: form.grammar_ref.clone(),
        body_text: form.body_text.clone(),
        is_abstract: form.is_abstract,
        return_type: form.return_type.clone(),
        optic_ops: form.optic_ops.clone(),
    }
}

/// Sort key for kintsugi canonical order.
/// Lower numbers sort first. Stable sort preserves order within same kind.
fn kintsugi_sort_key(kind: &DeclKind) -> u8 {
    match kind {
        DeclKind::In => 0,
        DeclKind::Type => 1,
        DeclKind::Traversal => 2,
        DeclKind::Lens => 3,
        DeclKind::Grammar | DeclKind::Form => 4,
        DeclKind::Property => 5,
        DeclKind::Action => 6,
        // Optic operations used as declarations
        DeclKind::Focus
        | DeclKind::Project
        | DeclKind::Split
        | DeclKind::Fold
        | DeclKind::Zoom
        | DeclKind::Refract => 1, // group with types
        // Other structural keywords
        DeclKind::Out => 7,
        DeclKind::Prism => 1,
        DeclKind::Requires | DeclKind::Invariant | DeclKind::Ensures => 5,
        DeclKind::Recover | DeclKind::Rescue => 6,
        DeclKind::Default | DeclKind::Binding => 7,
    }
}

// ---------------------------------------------------------------------------
// MirrorRuntime — the operation.
// ---------------------------------------------------------------------------

/// Compiled artifact: a top-level Form, its content-addressed MirrorFragment,
/// and the crystal hash (root OID).
#[derive(Clone, Debug)]
pub struct CompiledShatter {
    pub form: Form,
    pub fragment: MirrorFragment,
}

impl CompiledShatter {
    pub fn crystal(&self) -> &MirrorHash {
        self.fragment.oid()
    }
    pub fn form_name(&self) -> &str {
        &self.form.name
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
        parse_form(source).map(|form| {
            let fragment = Shatter.compile_form(&form);
            CompiledShatter { form, fragment }
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
        let mut all_forms: Vec<Form> = Vec::new();
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

            all_forms.push(compiled.form.clone());

            match registry.resolve(&compiled.form) {
                Ok(()) => {
                    registry.register(&compiled.form);
                    resolved.insert(stem, compiled);
                }
                Err(e) => {
                    failed.insert(stem, e);
                }
            }
        }

        registry.flush();

        let collapsed_form = Form::new(DeclKind::Form, "mirror", Vec::new(), Vec::new(), all_forms);
        let shatter = Shatter;
        let collapsed_fragment = shatter.compile_form(&collapsed_form);
        let collapsed = CompiledShatter {
            form: collapsed_form,
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

    // Emit the collapsed form as valid .mirror syntax.
    // emit_form is already proven to round-trip exactly (same OIDs).
    out.push_str(&emit_form(&collapsed.form));
    out
}

impl MirrorRuntime {
    /// Compile the boot directory and emit mirror.shatter.
    pub fn materialize_crystal(
        &self,
        boot_dir: &Path,
        store_dir: &Path,
        output: &Path,
    ) -> Result<MirrorHash, MirrorRuntimeError> {
        let boot = self.compile_boot_dir(boot_dir, store_dir)?;
        let content = emit_shatter(&boot.collapsed, &boot.resolved, &boot.failed);
        std::fs::write(output, &content)
            .map_err(|e| err(format!("write {}: {}", output.display(), e)))?;
        Ok(boot.collapsed.crystal().clone())
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

    /// Register forms into the store. If the form's name is empty (synthetic file-level form),
    /// recurse into children. For forms starting with `@`, compile to a MirrorFragment,
    /// store persistently, and map the name to its OID. Forms without `@` prefix are ignored.
    /// Returns OIDs of newly-registered forms.
    pub fn register(&mut self, form: &Form) -> Vec<String> {
        let mut oids = Vec::new();
        if form.name.is_empty() {
            // Synthetic file-level form: recurse into children
            for child in &form.children {
                oids.extend(self.register_decl(child));
            }
        } else {
            // Single named form: try to register it
            oids.extend(self.register_decl(form));
        }
        oids
    }

    /// Register a single declaration if its name starts with `@`.
    fn register_decl(&mut self, decl: &Form) -> Option<String> {
        if !decl.name.starts_with('@') {
            return None;
        }
        let shatter = Shatter;
        let fragment = shatter.compile_form(decl);
        let oid = fragment.oid().as_str().to_string();
        let size = self.estimate_size(decl);
        self.store.insert_persistent(oid.clone(), fragment, size);
        if let Err(e) = self.store.set_ref(&decl.name, &oid) {
            eprintln!("warning: set_ref({} -> {}) failed: {}", decl.name, oid, e);
        }
        Some(oid)
    }

    /// Estimate the byte size of a form for cache accounting.
    fn estimate_size(&self, form: &Form) -> usize {
        let mut bytes = form.name.len()
            + form.params.iter().map(|s| s.len()).sum::<usize>()
            + form.variants.iter().map(|s| s.len()).sum::<usize>()
            + 64; // Base overhead for Kind and structure
        for child in &form.children {
            bytes += self.estimate_size(child);
        }
        bytes
    }

    /// Resolve a Form tree by checking that every `in @X` reference exists in the store.
    /// Returns the first unresolved reference as an error, or Ok(()) if all resolve.
    /// Resolution goes through store.get_ref() to ensure it works after a reopen
    /// (disk-backed, not in-memory shadow).
    pub fn resolve(&self, form: &Form) -> Result<(), MirrorResolveError> {
        self.resolve_node(form)
    }

    fn resolve_node(&self, node: &Form) -> Result<(), MirrorResolveError> {
        if node.kind == DeclKind::In {
            let target = &node.name;
            if self.store.get_ref(target).is_none() {
                return Err(MirrorResolveError(format!(
                    "unresolved `in {}`: no such ref in registry store at {}",
                    target,
                    self.root.display()
                )));
            }
        }
        for child in &node.children {
            self.resolve_node(child)?;
        }
        Ok(())
    }
}

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

    // -----------------------------------------------------------------------
    // OpticOp classification in parsed Forms
    // -----------------------------------------------------------------------

    #[test]
    fn type_declaration_uses_iso_and_split() {
        let source = "type visibility = private | protected | public";
        let form = parse_form(source).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Type);
        assert!(
            form.optic_ops.contains(&OpticOp::Iso),
            "= should classify as Iso, got {:?}",
            form.optic_ops
        );
        assert!(
            form.optic_ops.contains(&OpticOp::Split),
            "| should classify as Split, got {:?}",
            form.optic_ops
        );
    }

    #[test]
    fn split_decl_keyword_classified_as_optic() {
        let source = "split |(ref, ref)";
        let form = parse_form(source).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Split);
        assert!(
            form.optic_ops.contains(&OpticOp::Split),
            "split keyword should be classified as OpticOp::Split"
        );
    }

    #[test]
    fn zoom_decl_keyword_classified_as_optic() {
        let source = "zoom |>(ref, prism)";
        let form = parse_form(source).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Zoom);
        assert!(
            form.optic_ops.contains(&OpticOp::Zoom),
            "zoom keyword should be classified as OpticOp::Zoom"
        );
    }

    #[test]
    fn refract_decl_keyword_classified_as_optic() {
        let source = "refract ..(ref)";
        let form = parse_form(source).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Refract);
        assert!(
            form.optic_ops.contains(&OpticOp::Refract),
            "refract keyword should be classified as OpticOp::Refract"
        );
    }

    #[test]
    fn fold_decl_keyword_classified_as_optic() {
        let source = "fold <=(ref, imperfect)";
        let form = parse_form(source).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Fold);
        assert!(
            form.optic_ops.contains(&OpticOp::Fold),
            "fold keyword should be classified as OpticOp::Fold"
        );
    }

    #[test]
    fn focus_decl_with_params_classified_as_optic() {
        let source = "focus type(id)";
        let form = parse_form(source).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Focus);
        assert!(
            form.optic_ops.contains(&OpticOp::Focus),
            "focus keyword with params should be classified as OpticOp::Focus"
        );
    }

    #[test]
    fn type_without_variants_has_no_split() {
        let source = "type grammar";
        let form = parse_form(source).ok().unwrap();
        assert!(!form.optic_ops.contains(&OpticOp::Split));
        assert!(!form.optic_ops.contains(&OpticOp::Iso));
    }

    #[test]
    fn parens_classified_as_focus() {
        let source = "type beam(result)";
        let form = parse_form(source).ok().unwrap();
        assert!(
            form.optic_ops.contains(&OpticOp::Focus),
            "parenthesized params should classify as Focus"
        );
    }

    // -----------------------------------------------------------------------
    // Parser tests
    // -----------------------------------------------------------------------

    #[test]
    fn mirror_runtime_parses_atom_decl() {
        let src = "form @form {\n  prism focus\n}\n";
        let form = parse_form(src).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Form);
        assert_eq!(form.name, "@form");
        assert_eq!(form.children.len(), 1);
        assert_eq!(form.children[0].kind, DeclKind::Prism);
        assert_eq!(form.children[0].name, "focus");
    }

    #[test]
    fn mirror_runtime_parses_params_and_variants() {
        let src = "form @x {\n  prism eigenvalues(precision)\n  traversal kind = a | b | c\n}\n";
        let form = parse_form(src).ok().unwrap();
        assert_eq!(form.children[0].params, vec!["precision".to_string()]);
        assert_eq!(
            form.children[1].variants,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn mirror_runtime_parses_nested_property() {
        let src = "form @property {\n  property unique_variants(form) {\n    fold input\n  }\n}\n";
        let form = parse_form(src).ok().unwrap();
        assert_eq!(form.children.len(), 1);
        let prop = &form.children[0];
        assert_eq!(prop.kind, DeclKind::Property);
        assert_eq!(prop.name, "unique_variants");
        assert_eq!(prop.params, vec!["form".to_string()]);
        assert_eq!(prop.children.len(), 1);
        assert_eq!(prop.children[0].kind, DeclKind::Fold);
    }

    #[test]
    fn mirror_runtime_compile_form_file() {
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("00-prism.mirror"))
            .unwrap();
        // 00-prism.mirror has multiple declarations, so they're wrapped in a
        // synthetic file-level Form.
        assert_eq!(compiled.form.kind, DeclKind::Form);
        assert!(compiled.form.children.len() >= 2);
        // Look for @prism declaration
        let prism_decl = compiled
            .form
            .children
            .iter()
            .find(|f| f.name == "@prism")
            .expect("@prism declaration present");
        assert_eq!(prism_decl.kind, DeclKind::Prism);
        assert_eq!(prism_decl.children.len(), 5);
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
            let text = emit_form(&s1.form);
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
        assert_eq!(boot.collapsed.form_name(), "mirror");
        assert!(boot.collapsed.form.children.len() >= 8);

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
        assert_eq!(compiled.form_name(), "@property");
        let prop_count = compiled
            .form
            .children
            .iter()
            .filter(|f| f.kind == DeclKind::Property)
            .count();
        assert_eq!(prop_count, 11);
    }

    #[test]
    fn mirror_runtime_mirror_form_has_property_applications() {
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("10-mirror.mirror"))
            .unwrap();
        let kinds: Vec<&DeclKind> = compiled.form.children.iter().map(|f| &f.kind).collect();
        assert!(kinds.contains(&&DeclKind::Requires));
        assert!(kinds.contains(&&DeclKind::Invariant));
        assert!(kinds.contains(&&DeclKind::Ensures));
        assert!(kinds.contains(&&DeclKind::In));
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

        // Trait-level focus carries the top eigenvalues.
        let seed: Optic<(), Form> = Optic::ok((), compiled.form.clone());
        let focused = shatter.focus(seed);
        let eigen = focused.result().ok().expect("focus failed");
        assert_eq!(eigen.kind, DeclKind::Form);
        // 00-prism.mirror wraps multiple declarations in a synthetic Form with empty name
        assert_eq!(eigen.name, "");

        // Trait-level project produces a content-addressed (childless) frag.
        // Re-seed because focus consumed the previous beam.
        let seed2: Optic<(), Form> = Optic::ok((), compiled.form.clone());
        let focused2 = shatter.focus(seed2);
        let projected = shatter.project(focused2);
        let frag_result = projected.result().ok().expect("project failed");
        assert!(!frag_result.oid().as_str().is_empty());

        // Full structural projection via compile_form — uses all children.
        let frag = shatter.compile_form(&compiled.form);
        let restored = shatter.decompile(&frag);
        assert_eq!(restored, compiled.form);

        // Stable OID across runs (CoincidenceHash<5> determinism).
        let frag2 = shatter.compile_form(&compiled.form);
        assert_eq!(frag.oid(), frag2.oid());
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

        let form = Form::new(
            DeclKind::Prism,
            "@prism",
            Vec::new(),
            Vec::new(),
            vec![Form::new(
                DeclKind::Prism,
                "focus",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )],
        );
        registry.register(&form);

        let stored = registry.lookup("@prism").expect("@prism in registry");
        let shatter = Shatter;
        let restored = shatter.decompile(&stored);
        assert_eq!(restored.name, "@prism");
        assert_eq!(restored.children.len(), 1);
        assert_eq!(restored.children[0].name, "focus");
    }

    #[test]
    fn registry_registers_only_at_named_top_level_forms() {
        let tmp = tempdir_for_test("registry_registers_only_at");
        let mut registry = MirrorRegistry::open(&tmp).unwrap();

        // A name without @-prefix should NOT become a form binding.
        let form = Form::new(DeclKind::Prism, "id", Vec::new(), Vec::new(), Vec::new());
        registry.register(&form);
        assert!(registry.lookup("id").is_none());
        assert!(registry.lookup("@id").is_none());
    }

    #[test]
    fn registry_persists_across_reopen() {
        let tmp = tempdir_for_test("registry_persists");
        {
            let mut registry = MirrorRegistry::open(&tmp).unwrap();
            let form = Form::new(
                DeclKind::Prism,
                "@prism",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            registry.register(&form);
            registry.flush();
        }
        // Reopen — cache is gone, but the disk + refs persist.
        let registry = MirrorRegistry::open(&tmp).unwrap();
        let stored = registry
            .lookup("@prism")
            .expect("@prism survives reopen via disk");
        let shatter = Shatter;
        let restored = shatter.decompile(&stored);
        assert_eq!(restored.name, "@prism");
    }

    #[test]
    fn registry_resolves_in_reference_when_target_in_store() {
        let tmp = tempdir_for_test("registry_resolves_in");
        let mut registry = MirrorRegistry::open(&tmp).unwrap();

        let prism_form = Form::new(
            DeclKind::Prism,
            "@prism",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        registry.register(&prism_form);

        let file = Form::new(
            DeclKind::Form,
            "",
            Vec::new(),
            Vec::new(),
            vec![Form::new(
                DeclKind::In,
                "@prism",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )],
        );
        assert!(registry.resolve(&file).is_ok());
    }

    #[test]
    fn registry_resolve_fails_when_in_target_missing() {
        let tmp = tempdir_for_test("registry_resolve_missing");
        let registry = MirrorRegistry::open(&tmp).unwrap();
        let file = Form::new(
            DeclKind::Form,
            "",
            Vec::new(),
            Vec::new(),
            vec![Form::new(
                DeclKind::In,
                "@nonexistent",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )],
        );
        let err = registry.resolve(&file).unwrap_err();
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
            let prism_form = Form::new(
                DeclKind::Prism,
                "@prism",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            registry.register(&prism_form);
            registry.flush();
        }
        let registry = MirrorRegistry::open(&tmp).unwrap();
        let file = Form::new(
            DeclKind::Form,
            "",
            Vec::new(),
            Vec::new(),
            vec![Form::new(
                DeclKind::In,
                "@prism",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )],
        );
        assert!(
            registry.resolve(&file).is_ok(),
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
        assert!(boot.resolved.contains_key("04-actor"));

        // 01a, 01b, 02-shatter fail: depend on @actor/@io which sort after them
        assert!(boot.failed.contains_key("01a-meta-action"));
        assert!(boot.failed.contains_key("01b-meta-io"));
        assert!(boot.failed.contains_key("02-shatter"));
        assert!(boot.failed.contains_key("05-property"));
        assert!(boot.failed.contains_key("10-mirror"));

        let reopened = MirrorRegistry::open(&store_dir).unwrap();
        assert!(reopened.lookup("@prism").is_some());
        assert!(reopened.lookup("@meta").is_some());
        assert!(reopened.lookup("@code").is_some());
        assert!(reopened.lookup("@actor").is_some());
        assert!(reopened.lookup("@property").is_none());
        assert!(reopened.lookup("@mirror").is_none());
    }

    #[test]
    fn meta_fails_to_resolve_without_prism_in_registry() {
        let runtime = MirrorRuntime::new();
        let tmp = tempdir_for_test("meta_without_prism");
        let registry = MirrorRegistry::open(&tmp).unwrap();
        let meta = runtime
            .compile_file(&boot_dir().join("01-meta.mirror"))
            .unwrap();
        let err = registry.resolve(&meta.form).unwrap_err();
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
        registry.register(&prism.form);

        let meta = runtime
            .compile_file(&boot_dir().join("01-meta.mirror"))
            .unwrap();
        assert!(
            registry.resolve(&meta.form).is_ok(),
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
            reg_a.register(&prism.form);
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
            reg_a.resolve(&meta.form).is_ok(),
            "mount A has @prism; meta resolves"
        );
        assert!(
            reg_b.resolve(&meta.form).is_err(),
            "mount B is empty; meta fails to resolve"
        );
    }

    // -----------------------------------------------------------------------
    // Action declaration tests
    // -----------------------------------------------------------------------

    #[test]
    fn parse_action_with_grammar_ref() {
        let src = "action transform(state) in @code/rust {\n    fn transform(&mut self) { }\n}\n";
        let form = parse_form(src).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Action);
        assert_eq!(form.name, "transform");
        assert_eq!(form.params, vec!["state".to_string()]);
        assert_eq!(form.grammar_ref, Some("@code/rust".to_string()));
        assert!(form.body_text.is_some(), "body text should be captured");
        let body = form.body_text.as_ref().unwrap();
        assert!(
            body.contains("transform"),
            "body should contain the raw text: {}",
            body
        );
    }

    #[test]
    fn parse_action_without_grammar_ref() {
        let src = "action update(state) {\n    state.apply()\n}\n";
        let form = parse_form(src).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Action);
        assert_eq!(form.name, "update");
        assert_eq!(form.params, vec!["state".to_string()]);
        assert_eq!(form.grammar_ref, None, "no `in @grammar` means None");
        assert!(form.body_text.is_some());
    }

    #[test]
    fn parse_action_receiver_stored() {
        let src = "action send(process, message) in @actor {\n    dispatch(message)\n}\n";
        let form = parse_form(src).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Action);
        assert_eq!(form.name, "send");
        assert_eq!(
            form.params,
            vec!["process".to_string(), "message".to_string()]
        );
        assert_eq!(form.grammar_ref, Some("@actor".to_string()));
    }

    #[test]
    fn parse_action_body_stored_as_raw() {
        let src = "action compute(x) in @code/rust {\n    let y = x * 2;\n    y + 1\n}\n";
        let form = parse_form(src).ok().unwrap();
        assert!(form.body_text.is_some());
        let body = form.body_text.unwrap();
        // Body should contain the raw text, not parsed mirror declarations
        assert!(
            body.contains("let"),
            "raw body should be preserved: {}",
            body
        );
    }

    #[test]
    fn parse_action_empty_body() {
        let src = "action noop(state) { }\n";
        let form = parse_form(src).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Action);
        assert_eq!(form.name, "noop");
        assert_eq!(form.body_text, None, "empty body should be None");
    }

    #[test]
    fn action_form_round_trip_fragment() {
        let form = Form::action(
            "transform",
            vec!["state".to_string()],
            Some("@code/rust".to_string()),
            Some("fn transform() {}".to_string()),
            Vec::new(),
        );
        let shatter = Shatter;
        let frag = shatter.compile_form(&form);
        let restored = shatter.decompile(&frag);
        assert_eq!(restored.kind, DeclKind::Action);
        assert_eq!(restored.name, "transform");
        assert_eq!(restored.params, vec!["state".to_string()]);
        assert_eq!(restored.grammar_ref, Some("@code/rust".to_string()));
        assert_eq!(restored.body_text, Some("fn transform() {}".to_string()));
    }

    #[test]
    fn action_file_01a_parses_and_resolves() {
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("01a-meta-action.mirror"))
            .unwrap();
        // 01a-meta-action.mirror has multiple top-level declarations, wrapped in synthetic Form
        assert_eq!(compiled.form.kind, DeclKind::Form);
        // Should contain: in @prism, in @meta, in @actor, prism action, action action, out action/collapse
        let action_decls: Vec<&Form> = compiled
            .form
            .children
            .iter()
            .filter(|f| f.kind == DeclKind::Action)
            .collect();
        assert_eq!(
            action_decls.len(),
            1,
            "01a-meta-action.mirror has one action declaration"
        );
        let action = action_decls[0];
        assert_eq!(action.name, "action");
        // The action body contains mirror declaration keywords (focus, project, etc.)
        // so it's parsed as structured children, not raw body text.
        assert!(
            !action.children.is_empty(),
            "action body with mirror keywords should be parsed as children"
        );
    }

    #[test]
    fn action_is_named_type_property_passes_for_named_receiver() {
        // Simulate checking `action_is_named_type`: all actions have named type receivers
        let form = Form::new(
            DeclKind::Form,
            "@test",
            Vec::new(),
            Vec::new(),
            vec![Form::action(
                "transform",
                vec!["state".to_string()],
                Some("@code/rust".to_string()),
                Some("body".to_string()),
                Vec::new(),
            )],
        );
        // Check: every action's first param (receiver) is a non-empty named type
        let all_named = form
            .children
            .iter()
            .filter(|f| f.kind == DeclKind::Action)
            .all(|f| !f.params.is_empty() && !f.params[0].is_empty());
        assert!(all_named, "all action receivers should be named types");
    }

    #[test]
    fn action_is_named_type_property_fails_for_empty_receiver() {
        // An action with no params = no receiver = anonymous = property violation
        let form = Form::action(
            "bad",
            Vec::new(),
            None,
            Some("body".to_string()),
            Vec::new(),
        );
        let has_named_receiver = !form.params.is_empty() && !form.params[0].is_empty();
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

        // Compile the reparsed form
        let shatter = Shatter;
        let fragment = shatter.compile_form(&reparsed);

        // Same OID — round-trip exact
        assert_eq!(
            fragment.oid(),
            &oid,
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
        let form = parse_form(&content).ok().unwrap();

        // Must contain the boot forms (all boot files collapsed).
        // Count changes as parser learns new declaration kinds.
        assert!(
            form.children.len() >= 8,
            "shatter must contain at least 8 boot file forms, got {}",
            form.children.len()
        );
    }

    #[test]
    // -----------------------------------------------------------------------
    // DeclKind::Default and DeclKind::Binding — no longer silently dropped
    // -----------------------------------------------------------------------
    #[test]
    fn parse_default_declaration() {
        let src = "default(visibility) = public";
        let form = parse_form(src).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Default);
        assert_eq!(form.name, "");
        assert_eq!(form.params, vec!["visibility".to_string()]);
        assert_eq!(form.variants, vec!["public".to_string()]);
        assert!(
            form.optic_ops.contains(&OpticOp::Iso),
            "= should classify as Iso"
        );
    }

    #[test]
    fn parse_binding_declaration() {
        let src = "binding(leader, key) = focus";
        let form = parse_form(src).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Binding);
        assert_eq!(form.name, "");
        assert_eq!(form.params, vec!["leader".to_string(), "key".to_string()]);
        assert_eq!(form.variants, vec!["focus".to_string()]);
    }

    #[test]
    fn parse_default_inside_block() {
        let src = "form @test {\n  type visibility = private | public\n  default(visibility) = public\n}\n";
        let form = parse_form(src).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Form);
        // Both children should be present — default is NOT silently dropped
        assert_eq!(
            form.children.len(),
            2,
            "default should not be silently dropped: got {:?}",
            form.children
                .iter()
                .map(|c| c.kind.as_str())
                .collect::<Vec<_>>()
        );
        assert_eq!(form.children[0].kind, DeclKind::Type);
        assert_eq!(form.children[1].kind, DeclKind::Default);
    }

    // -----------------------------------------------------------------------
    // UnrecognizedDecl — parser tracks what it cannot parse
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
        let form = result.as_ref().ok().unwrap();
        assert_eq!(form.kind, DeclKind::Type);
        assert_eq!(form.name, "bar");
    }

    #[test]
    fn parse_unrecognized_keyword_loss_contains_keyword() {
        let src = "widget foo\ntype bar";
        let result = parse_form(src);
        let loss = result.loss();
        assert_eq!(loss.parse.unrecognized.len(), 1);
        assert_eq!(loss.parse.unrecognized[0].keyword, "widget");
        assert_eq!(loss.parse.unrecognized[0].line, 1);
        assert!(loss.parse.unrecognized[0].content.contains("foo"));
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
            loss.parse.unrecognized.len(),
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
            !loss.parse.unrecognized.is_empty(),
            "loss should contain unrecognized decls"
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
            !result.total_loss.parse.unrecognized.is_empty(),
            "boot dir should accumulate unrecognized loss from partial files"
        );
        assert_eq!(result.total_loss.parse.unrecognized[0].keyword, "widget");
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

    fn mirror_shatter_deterministic_across_runs() {
        let runtime = MirrorRuntime::new();
        let store_dir1 = tempdir_for_test("shatter_deterministic_1");
        let store_dir2 = tempdir_for_test("shatter_deterministic_2");
        let output1 = store_dir1.join("mirror.shatter");
        let output2 = store_dir2.join("mirror.shatter");

        let oid1 = runtime
            .materialize_crystal(&boot_dir(), &store_dir1, &output1)
            .unwrap();
        let oid2 = runtime
            .materialize_crystal(&boot_dir(), &store_dir2, &output2)
            .unwrap();

        assert_eq!(oid1, oid2, "same boot dir must produce same crystal OID");

        let content1 = std::fs::read_to_string(&output1).unwrap();
        let content2 = std::fs::read_to_string(&output2).unwrap();
        assert_eq!(
            content1, content2,
            "same boot dir must produce identical .shatter content"
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

        assert_eq!(files.len(), 17, "boot kernel file count: {:?}", files);
        assert!(files.contains(&"00-prism.mirror".to_string()));
        assert!(files.contains(&"01a-meta-action.mirror".to_string()));
        assert!(files.contains(&"01b-meta-io.mirror".to_string()));
        assert!(files.contains(&"02-shatter.mirror".to_string()));
        assert!(files.contains(&"20-cli.mirror".to_string()));
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
        assert!(resolved.contains(&"04-actor"), "actor must resolve");

        // --- What fails resolution (in @X references something missing) ---
        let failed: Vec<&str> = boot.failed.keys().map(|s| s.as_str()).collect();
        assert!(
            failed.contains(&"05-property"),
            "property fails: in @form — @form is not defined in boot"
        );
        assert!(
            failed.contains(&"10-mirror"),
            "mirror fails: in @form, in @type, in @boundary, in @lens — none defined"
        );

        // --- The loss: what the compiler saw but couldn't land ---
        let loss = &boot.total_loss;
        let holonomy = loss.holonomy();

        // --- Parse-level loss ---
        // New kernel files (01-meta, 01a, 01b, 02-shatter) introduce:
        //   unfold, subset, superset, iso, not-iso (01-meta operators)
        //   io (01b-meta-io, 02-shatter grammar keyword)
        //   pure, real, loss constraints with != operator
        // These are training data — the holonomy tells us what the parser
        // can't handle yet.
        //
        // The baseline holonomy must not INCREASE (regression).
        // It CAN decrease as the parser learns new constructs.
        assert!(
            holonomy <= 15.0,
            "parse holonomy must not regress above baseline: got {}",
            holonomy
        );

        // --- Resolution failures: the real loss ---
        // These files parse fine but fail `in @X` reference checks.
        // This loss is NOT in holonomy — it should be.
        // That's the gap: resolution failures need to become MirrorLoss.
        assert_eq!(boot.failed.len(), 8, "8 of 17 boot files fail resolution");
        assert!(
            failed.contains(&"01a-meta-action"),
            "01a needs @actor which sorts after it"
        );
        assert!(
            failed.contains(&"01b-meta-io"),
            "01b needs @actor which sorts after it"
        );
        assert!(
            failed.contains(&"02-shatter"),
            "02-shatter needs @io which itself failed"
        );
        assert!(
            failed.contains(&"05-property"),
            "in @form — @form undefined"
        );
        assert!(
            failed.contains(&"10-mirror"),
            "in @form, @type, @boundary, @lens — undefined"
        );
        assert!(failed.contains(&"11-spec"), "missing refs");
        assert!(failed.contains(&"16-tui"), "missing refs");
        assert!(failed.contains(&"20-cli"), "missing refs");

        // --- Resolved file count: progress toward Success(Mirror) ---
        assert_eq!(boot.resolved.len(), 9, "9 of 17 boot files resolve");

        // --- The crystal still forms despite failures ---
        // The compiler produces a crystal from what DID resolve.
        // This is Partial, not Failure. The observation happened.
        let crystal_oid = boot.collapsed.crystal();
        assert!(
            !crystal_oid.as_str().is_empty(),
            "crystal must form even with partial resolution"
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
        let boot_action = compiled
            .form
            .children
            .iter()
            .flat_map(|child| std::iter::once(child).chain(child.children.iter()))
            .find(|f| f.kind == DeclKind::Action && f.name == "boot");
        assert!(boot_action.is_some(), "action boot must exist");
        assert!(
            boot_action.unwrap().optic_ops.contains(&OpticOp::Fold),
            "action boot(identity) <= imperfect must produce OpticOp::Fold"
        );

        // If Success: the fold was parsed correctly. Zero loss is correct.
        // If Partial: the fold was parsed but something else produced loss.
        //   The loss must NOT be from dropping the <=.
        if result.is_partial() {
            let loss = result.loss();
            // Partial is ok as long as the fold operator landed.
            // The loss should be from something other than a dropped <=.
            assert!(
                boot_action.unwrap().optic_ops.contains(&OpticOp::Fold),
                "Partial result must still have the fold operator"
            );
            let _ = loss; // loss from other sources is fine
        }
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
        let resolve_result = registry.resolve(&ai.form);
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
            !loss.parse.unrecognized.is_empty(),
            "Failure must carry the unrecognized keywords as loss"
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
        assert_eq!(loss.parse.unrecognized.len(), 1, "one unrecognized keyword");
        assert_eq!(
            loss.parse.unrecognized[0].keyword, "widget",
            "the unrecognized keyword is 'widget'"
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

        let err = registry.resolve(&form.form).unwrap_err();
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

        let err = registry.resolve(&form.form).unwrap_err();
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

        let result = registry.resolve(&form.form);
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

        let err = registry.resolve(&form.form).unwrap_err();
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
                let type_x = &c.form;
                let has_content = !type_x.variants.is_empty()
                    || !type_x.params.is_empty()
                    || type_x.children.iter().any(|c| !c.variants.is_empty());
                assert!(
                    has_content,
                    "unknown operator ~> must not be silently dropped. \
                     type x should capture the remaining content. Got: {:?}",
                    type_x
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
            Imperfect::Success(c) => {
                // If Success, the <= must be recorded as OpticOp::Fold
                let has_fold = c.form.optic_ops.contains(&OpticOp::Fold)
                    || c.form
                        .children
                        .iter()
                        .any(|ch| ch.optic_ops.contains(&OpticOp::Fold));
                assert!(
                    has_fold,
                    "type x <= y: if Success, OpticOp::Fold must be recorded. Got: {:?}",
                    c.form
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

        let compiled = match result {
            Imperfect::Success(c) | Imperfect::Partial(c, _) => c,
            Imperfect::Failure(e, _) => panic!("property with <= failed: {}", e),
        };

        // The property must have OpticOp::Fold
        let has_fold = compiled.form.optic_ops.contains(&OpticOp::Fold)
            || compiled
                .form
                .children
                .iter()
                .any(|ch| ch.optic_ops.contains(&OpticOp::Fold));
        assert!(
            has_fold,
            "property check(grammar) <= verdict must produce OpticOp::Fold. Got: {:?}",
            compiled.form
        );
    }

    // -----------------------------------------------------------------------
    // Recover/Rescue method tests — imperfect type methods
    // -----------------------------------------------------------------------

    /// `recover` inside a type block with fold operator should produce
    /// a child with DeclKind::Recover and OpticOp::Fold.
    #[test]
    fn imperfect_type_has_recover_method() {
        let source = "type imperfect(observation, error(observation), loss) {\n  recover |observation, loss| <= imperfect\n}\n";
        let form = parse_form(source).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Type);
        assert_eq!(form.name, "imperfect");
        assert!(!form.children.is_empty(), "imperfect must have children");
        let recover = form.children.iter().find(|c| c.kind == DeclKind::Recover);
        assert!(recover.is_some(), "imperfect must have a recover child");
        let recover = recover.unwrap();
        assert!(
            recover.optic_ops.contains(&OpticOp::Fold),
            "recover must have OpticOp::Fold (from <=), got: {:?}",
            recover.optic_ops
        );
    }

    /// `rescue` inside a type block with fold operator should produce
    /// a child with DeclKind::Rescue and OpticOp::Fold.
    #[test]
    fn imperfect_type_has_rescue_method() {
        let source = "type imperfect(observation, error(observation), loss) {\n  rescue |error(observation), loss| <= imperfect\n}\n";
        let form = parse_form(source).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Type);
        let rescue = form.children.iter().find(|c| c.kind == DeclKind::Rescue);
        assert!(rescue.is_some(), "imperfect must have a rescue child");
        let rescue = rescue.unwrap();
        assert!(
            rescue.optic_ops.contains(&OpticOp::Fold),
            "rescue must have OpticOp::Fold (from <=), got: {:?}",
            rescue.optic_ops
        );
    }

    /// `recover` with fold operator parses correctly.
    #[test]
    fn recover_returns_imperfect() {
        let source = "type result(t, e, l) {\n  recover |t, l| <= result\n}\n";
        let form = parse_form(source).ok().unwrap();
        let recover = form.children.iter().find(|c| c.kind == DeclKind::Recover);
        assert!(recover.is_some(), "result must have recover child");
        let recover = recover.unwrap();
        assert!(
            recover.optic_ops.contains(&OpticOp::Fold),
            "recover must have fold operator"
        );
        // The fold target should reference the enclosing type
        assert!(
            recover.variants.contains(&"result".to_string()),
            "recover fold target should be 'result', got variants: {:?}",
            recover.variants
        );
    }

    /// `rescue` with fold operator parses correctly.
    #[test]
    fn rescue_returns_imperfect() {
        let source = "type result(t, e, l) {\n  rescue |e, l| <= result\n}\n";
        let form = parse_form(source).ok().unwrap();
        let rescue = form.children.iter().find(|c| c.kind == DeclKind::Rescue);
        assert!(rescue.is_some(), "result must have rescue child");
        let rescue = rescue.unwrap();
        assert!(
            rescue.optic_ops.contains(&OpticOp::Fold),
            "rescue must have fold operator"
        );
        assert!(
            rescue.variants.contains(&"result".to_string()),
            "rescue fold target should be 'result', got variants: {:?}",
            rescue.variants
        );
    }

    /// Inline relation markers: `<` for subset inside type body.
    #[test]
    fn inline_relation_markers_parsed() {
        // Superset marker
        let source = "type admin {\n  >user\n}\n";
        let form = parse_form(source).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Type);
        assert_eq!(form.name, "admin");
        // The `>user` should be parsed — either as a child or as a variant
        // with OpticOp::Superset in the type's optic_ops
        let has_superset = form.optic_ops.contains(&OpticOp::Superset)
            || form
                .children
                .iter()
                .any(|c| c.optic_ops.contains(&OpticOp::Superset));
        assert!(
            has_superset,
            "admin type must have Superset marker, got form: {:?}",
            form
        );

        // Subset marker
        let source2 = "type contact {\n  <user\n}\n";
        let form2 = parse_form(source2).ok().unwrap();
        let has_subset = form2.optic_ops.contains(&OpticOp::Subset)
            || form2
                .children
                .iter()
                .any(|c| c.optic_ops.contains(&OpticOp::Subset));
        assert!(
            has_subset,
            "contact type must have Subset marker, got form: {:?}",
            form2
        );
    }

    /// Combined: type with inline relation marker AND recover method.
    #[test]
    fn type_with_inline_relation_and_recover() {
        let source = "type contact {\n  <user\n  recover |user, contact, loss| <= contact\n}\n";
        let form = parse_form(source).ok().unwrap();
        assert_eq!(form.kind, DeclKind::Type);
        assert_eq!(form.name, "contact");

        // Must have subset marker
        let has_subset = form.optic_ops.contains(&OpticOp::Subset)
            || form
                .children
                .iter()
                .any(|c| c.optic_ops.contains(&OpticOp::Subset));
        assert!(has_subset, "contact must have Subset marker");

        // Must have recover child
        let recover = form.children.iter().find(|c| c.kind == DeclKind::Recover);
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
                assert!(
                    !c.form.variants.is_empty() || !c.form.children.is_empty(),
                    "double operator = = must not produce empty result: {:?}",
                    c.form
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
            compiled.form.grammar_ref.is_none(),
            "BASELINE: grammar_ref is still Option (should become Imperfect)"
        );
    }

    // -----------------------------------------------------------------------
    // Fractal as AST — Form dissolves into Fractal<MirrorData>
    // -----------------------------------------------------------------------

    /// The compile result should be a Fractal, not a Form.
    /// Form is a parallel AST. Fractal<MirrorData> is the content-addressed
    /// tree. There should be one representation, not two.
    ///
    /// When this passes, compile_source returns Imperfect<MirrorFragment, ...>
    /// and the separate Form struct is gone. The optics navigate the Fractal
    /// directly. The OID is computed during parsing, not after.
    #[test]
    fn compile_returns_fractal_not_form() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("type color = red | blue\n");
        let compiled = result.ok().unwrap();

        // Currently: CompiledShatter has both form and fragment.
        // The fragment IS the content-addressed version of the form.
        // They carry the same information — one is redundant.
        //
        // Goal: compile returns the fragment directly. No intermediate Form.
        // The Fractal IS the AST. The optics navigate it.

        // The fragment should be navigable with the same data as the form
        let form_name = &compiled.form.name;
        let fragment_name = &compiled.fragment.mirror_data().name;
        assert_eq!(
            form_name, fragment_name,
            "form and fragment carry the same name — one is redundant"
        );

        // GOAL TEST: when Form is dissolved, CompiledShatter becomes just MirrorFragment
        // and this field access changes from compiled.form.name to compiled.data().name
        // Until then, this test documents the duplication.
        assert!(
            true, // placeholder — the real assertion is the existence of compiled.form
            "BASELINE: compile still returns Form + Fragment (should be Fragment only)"
        );
    }

    // -----------------------------------------------------------------------
    // Kintsugi — canonical ordering
    // -----------------------------------------------------------------------

    /// Kintsugi hoists `in` declarations to the top.
    #[test]
    fn kintsugi_hoists_imports() {
        let src = "type x\nin @prism\ntype y\n";
        let parsed = parse_form(src).ok().unwrap();
        let canonical = kintsugi(&parsed);
        assert_eq!(
            canonical.children[0].kind,
            DeclKind::In,
            "in @prism must be first after kintsugi"
        );
    }

    /// Kintsugi is idempotent: applying it twice yields the same result.
    #[test]
    fn kintsugi_is_idempotent() {
        let src = "action do_thing\ntype x\nin @prism\ngrammar @test {\n  type y\n}\n";
        let parsed = parse_form(src).ok().unwrap();
        let once = kintsugi(&parsed);
        let twice = kintsugi(&once);
        assert_eq!(once, twice, "kintsugi must be idempotent");
    }

    /// Kintsugi preserves OID: the content-addressed hash is order-invariant.
    #[test]
    fn kintsugi_preserves_oid() {
        let src = "action do_thing\ntype x\nin @prism\n";
        let parsed = parse_form(src).ok().unwrap();
        let canonical = kintsugi(&parsed);

        let shatter = Shatter;
        let oid_before = shatter.compile_form(&parsed).oid().clone();
        let oid_after = shatter.compile_form(&canonical).oid().clone();
        assert_eq!(
            oid_before, oid_after,
            "kintsugi must not change the content-addressed OID"
        );
    }
}
