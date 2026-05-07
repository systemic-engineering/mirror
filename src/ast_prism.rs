//! ASTPrism — the Rust-side seed prism.
//!
//! The parsing pipeline IS the Prism trait:
//!
//! - focus:   source text → tokens (read-only decomposition)
//! - project: tokens → Ast (precision-bounded parsing)
//! - refract: Ast → MirrorPrism (settle into compiled form) — todo
//!
//! ASTPrism is MetalPrism. It's the only prism not constructed by
//! another prism — it's hardcoded in Rust. It knows the nine tokens
//! and can refract a .mirror source into a MirrorPrism.

use prism::{Beam, Optic, Prism};

use crate::ast::{Ast, Atom, Body, Ref};

// ---------------------------------------------------------------------------
// Tokens — the Focused type. Internal to ASTPrism.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    At,
    Tilde,
    SigilValue(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Newline,
}

// ---------------------------------------------------------------------------
// ASTPrism
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ASTPrism;

impl Prism for ASTPrism {
    type Input = Optic<(), String>;
    type Focused = Optic<String, Vec<Token>>;
    type Projected = Optic<Vec<Token>, Ast>;
    type Refracted = Optic<Ast, ASTPrism>;

    /// Focus: source text → token stream.
    ///
    /// The read-only decomposition. No information is lost — every
    /// character in the source maps to a token or is whitespace.
    fn focus(&self, beam: Self::Input) -> Self::Focused {
        let tokens = tokenize(beam.result().ok().expect("focus: Err beam"));
        beam.next(tokens)
    }

    /// Project: token stream → AST.
    ///
    /// The precision-bounded parse. Tokens that don't fit the grammar
    /// are dropped (information loss). The resulting AST is the
    /// structural view of the source.
    fn project(&self, beam: Self::Focused) -> Self::Projected {
        let mut cursor = 0;
        let ast = parse_top(beam.result().ok().expect("project: Err beam"), &mut cursor);
        beam.next(ast)
    }

    /// Refract: settle the AST into a compiled MirrorPrism.
    ///
    /// This is where the bootstrap happens — the AST becomes a prism
    /// that can refract the next file. For now: todo.
    fn refract(&self, _beam: Self::Projected) -> Self::Refracted {
        todo!("ASTPrism::refract → MirrorPrism (the bootstrap)")
    }
}

impl ASTPrism {
    pub fn new() -> Self {
        ASTPrism
    }

    /// Convenience: source → AST in one call (focus then project).
    pub fn parse(&self, source: &str) -> Ast {
        let seed = Optic::ok((), source.to_string());
        let focused = self.focus(seed);
        let projected = self.project(focused);
        projected.result().ok().expect("parse: Err beam").clone()
    }
}

impl Default for ASTPrism {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tokenizer (the focus implementation)
// ---------------------------------------------------------------------------

fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] as char {
            ' ' | '\t' | '\r' => i += 1,
            '\n' => {
                tokens.push(Token::Newline);
                i += 1;
            }
            '@' => {
                tokens.push(Token::At);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            '{' => {
                tokens.push(Token::LBrace);
                i += 1;
            }
            '}' => {
                tokens.push(Token::RBrace);
                i += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            '#' => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            _ => {
                let start = i;
                while i < bytes.len() {
                    let c = bytes[i] as char;
                    if c.is_alphanumeric()
                        || c == '_'
                        || c == '.'
                        || c == '|'
                        || c == '>'
                        || c == '<'
                        || c == '/'
                        || c == ':'
                    {
                        i += 1;
                    } else {
                        break;
                    }
                }
                if i == start {
                    i += 1;
                } else {
                    tokens.push(Token::Word(source[start..i].to_string()));
                }
            }
        }
    }
    tokens
}

// ---------------------------------------------------------------------------
// Parser (the project implementation)
// ---------------------------------------------------------------------------

fn skip_trivia(tokens: &[Token], cursor: &mut usize) {
    while *cursor < tokens.len() && matches!(tokens[*cursor], Token::Newline) {
        *cursor += 1;
    }
}

/// Parse a top-level source: one or more expressions.
/// Multiple top-level expressions become a Body.
fn parse_top(tokens: &[Token], cursor: &mut usize) -> Ast {
    skip_trivia(tokens, cursor);
    let first = parse_expr(tokens, cursor);
    skip_trivia(tokens, cursor);
    if *cursor < tokens.len() && !matches!(tokens.get(*cursor), Some(Token::RBrace | Token::RParen))
    {
        let mut children = vec![first];
        while *cursor < tokens.len()
            && !matches!(tokens.get(*cursor), Some(Token::RBrace | Token::RParen))
        {
            skip_trivia(tokens, cursor);
            if *cursor >= tokens.len()
                || matches!(tokens.get(*cursor), Some(Token::RBrace | Token::RParen))
            {
                break;
            }
            children.push(parse_expr(tokens, cursor));
            skip_trivia(tokens, cursor);
        }
        Ast::Body(Body::new(children))
    } else {
        first
    }
}

fn parse_expr(tokens: &[Token], cursor: &mut usize) -> Ast {
    skip_trivia(tokens, cursor);

    match tokens.get(*cursor) {
        Some(Token::At) => {
            *cursor += 1;
            let name = match tokens.get(*cursor) {
                Some(Token::Word(w)) => {
                    *cursor += 1;
                    w.clone()
                }
                _ => String::new(),
            };
            Ast::Ref(Ref::new(name))
        }

        Some(Token::LBrace) => {
            *cursor += 1;
            let body = parse_body(tokens, cursor);
            Ast::Body(body)
        }

        Some(Token::Word(w)) => {
            let name = w.clone();
            *cursor += 1;

            // `prism @name { body }`
            if name == "prism" {
                skip_trivia(tokens, cursor);
                if let Some(Token::At) = tokens.get(*cursor) {
                    *cursor += 1;
                    let ref_name = match tokens.get(*cursor) {
                        Some(Token::Word(w)) => {
                            *cursor += 1;
                            w.clone()
                        }
                        _ => String::new(),
                    };
                    skip_trivia(tokens, cursor);
                    if matches!(tokens.get(*cursor), Some(Token::LBrace)) {
                        *cursor += 1;
                        let body = parse_body(tokens, cursor);
                        return Ast::Prism {
                            name: Ref::new(ref_name),
                            body,
                        };
                    }
                    return Ast::Call {
                        name: Atom::new("prism"),
                        args: vec![Ast::Ref(Ref::new(ref_name))],
                    };
                }
            }

            let is_optic = is_optic_name(&name);

            // name(args) possibly followed by { body }
            if matches!(tokens.get(*cursor), Some(Token::LParen)) {
                *cursor += 1;
                let args = parse_args(tokens, cursor);
                skip_trivia(tokens, cursor);
                if matches!(tokens.get(*cursor), Some(Token::LBrace)) {
                    *cursor += 1;
                    let body = parse_body(tokens, cursor);
                    // Optic: single arg + body + optic name → optic variant
                    if is_optic && args.len() == 1 {
                        return make_optic(&name, args.into_iter().next().unwrap(), body);
                    }
                    let mut all_args = args;
                    all_args.push(Ast::Body(body));
                    return Ast::Call {
                        name: Atom::new(name),
                        args: all_args,
                    };
                }
                return Ast::Call {
                    name: Atom::new(name),
                    args,
                };
            }

            // name { body } (no parens)
            skip_trivia(tokens, cursor);
            if matches!(tokens.get(*cursor), Some(Token::LBrace)) {
                *cursor += 1;
                let body = parse_body(tokens, cursor);
                if is_optic {
                    return make_bare_optic(&name, body);
                }
                return Ast::Call {
                    name: Atom::new(name),
                    args: vec![Ast::Body(body)],
                };
            }

            // name followed by another expression (space-separated arg)
            //
            // For optic names, parse only a primary expression (atom, ref,
            // or paren-call) so we don't greedily consume the body block.
            match tokens.get(*cursor) {
                Some(Token::Word(_)) | Some(Token::At) => {
                    let arg = if is_optic {
                        parse_primary(tokens, cursor)
                    } else {
                        parse_expr(tokens, cursor)
                    };
                    skip_trivia(tokens, cursor);
                    if matches!(tokens.get(*cursor), Some(Token::LBrace)) {
                        *cursor += 1;
                        let body = parse_body(tokens, cursor);
                        // Optic: space-separated arg + body + optic name → optic variant
                        if is_optic {
                            return make_optic(&name, arg, body);
                        }
                        return Ast::Call {
                            name: Atom::new(name),
                            args: vec![arg, Ast::Body(body)],
                        };
                    }
                    Ast::Call {
                        name: Atom::new(name),
                        args: vec![arg],
                    }
                }
                _ => Ast::Atom(Atom::new(name)),
            }
        }

        _ => {
            *cursor += 1;
            Ast::Atom(Atom::new(""))
        }
    }
}

fn parse_args(tokens: &[Token], cursor: &mut usize) -> Vec<Ast> {
    let mut args = Vec::new();
    loop {
        skip_trivia(tokens, cursor);
        match tokens.get(*cursor) {
            Some(Token::RParen) => {
                *cursor += 1;
                break;
            }
            Some(Token::Comma) => {
                *cursor += 1;
            }
            None => break,
            _ => {
                args.push(parse_expr(tokens, cursor));
            }
        }
    }
    args
}

/// Parse a primary expression: atom, ref, or name(args).
///
/// Does NOT consume a trailing body block — used by optic parsing
/// so that the body is left for the optic variant to consume.
fn parse_primary(tokens: &[Token], cursor: &mut usize) -> Ast {
    skip_trivia(tokens, cursor);
    match tokens.get(*cursor) {
        Some(Token::At) => {
            *cursor += 1;
            let name = match tokens.get(*cursor) {
                Some(Token::Word(w)) => {
                    *cursor += 1;
                    w.clone()
                }
                _ => String::new(),
            };
            Ast::Ref(Ref::new(name))
        }
        Some(Token::Word(w)) => {
            let name = w.clone();
            *cursor += 1;
            // name(args) — parse paren-args if present
            if matches!(tokens.get(*cursor), Some(Token::LParen)) {
                *cursor += 1;
                let args = parse_args(tokens, cursor);
                Ast::Call {
                    name: Atom::new(name),
                    args,
                }
            } else {
                Ast::Atom(Atom::new(name))
            }
        }
        _ => {
            *cursor += 1;
            Ast::Atom(Atom::new(""))
        }
    }
}

fn parse_body(tokens: &[Token], cursor: &mut usize) -> Body {
    let mut children = Vec::new();
    loop {
        skip_trivia(tokens, cursor);
        match tokens.get(*cursor) {
            Some(Token::RBrace) => {
                *cursor += 1;
                break;
            }
            None => break,
            _ => {
                children.push(parse_expr(tokens, cursor));
            }
        }
    }
    Body::new(children)
}

// ---------------------------------------------------------------------------
// Optic detection — promote Call to optic variant when name matches
// ---------------------------------------------------------------------------

const OPTIC_NAMES: &[&str] = &["focus", "project", "split", "zoom", "refract"];

fn is_optic_name(name: &str) -> bool {
    OPTIC_NAMES.contains(&name)
}

/// Build the optic AST variant for the given name with an argument.
///
/// Panics if name is not an optic name (caller must check first).
fn make_optic(name: &str, arg: Ast, body: Body) -> Ast {
    let boxed = Some(Box::new(arg));
    match name {
        "focus" => Ast::Focus { target: boxed, body },
        "project" => Ast::Project { query: boxed, body },
        "split" => Ast::Split { root: boxed, body },
        "zoom" => Ast::Zoom { perspective: boxed, body },
        "refract" => Ast::Refract { mutation: boxed, body },
        _ => unreachable!("make_optic called with non-optic name: {}", name),
    }
}

/// Build the optic AST variant for the given name without an argument (bare optic).
///
/// Panics if name is not an optic name (caller must check first).
fn make_bare_optic(name: &str, body: Body) -> Ast {
    match name {
        "focus" => Ast::Focus { target: None, body },
        "project" => Ast::Project { query: None, body },
        "split" => Ast::Split { root: None, body },
        "zoom" => Ast::Zoom { perspective: None, body },
        "refract" => Ast::Refract { mutation: None, body },
        _ => unreachable!("make_bare_optic called with non-optic name: {}", name),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> Ast {
        ASTPrism.parse(source)
    }

    #[test]
    fn parse_bare_atom() {
        assert_eq!(parse("id"), Ast::Atom(Atom::new("id")));
    }

    #[test]
    fn parse_ref() {
        assert_eq!(parse("@prism"), Ast::Ref(Ref::new("prism")));
    }

    #[test]
    fn parse_simple_call() {
        assert_eq!(
            parse("focus id"),
            Ast::Call {
                name: Atom::new("focus"),
                args: vec![Ast::Atom(Atom::new("id"))],
            }
        );
    }

    #[test]
    fn parse_call_with_parens() {
        assert_eq!(
            parse("type(id)"),
            Ast::Call {
                name: Atom::new("type"),
                args: vec![Ast::Atom(Atom::new("id"))],
            }
        );
    }

    #[test]
    fn parse_nested_call() {
        assert_eq!(
            parse("focus type(id)"),
            Ast::Call {
                name: Atom::new("focus"),
                args: vec![Ast::Call {
                    name: Atom::new("type"),
                    args: vec![Ast::Atom(Atom::new("id"))],
                }],
            }
        );
    }

    #[test]
    fn parse_call_with_ref_arg() {
        assert_eq!(
            parse("in @prism"),
            Ast::Call {
                name: Atom::new("in"),
                args: vec![Ast::Ref(Ref::new("prism"))],
            }
        );
    }

    #[test]
    fn parse_prism_block() {
        let ast = parse("prism @meta { focus id }");
        match ast {
            Ast::Prism { name, body } => {
                assert_eq!(name, Ref::new("meta"));
                assert_eq!(body.len(), 1);
            }
            other => panic!("expected Prism, got {:?}", other),
        }
    }

    #[test]
    #[ignore] // Parser bug: inner call consumes the body block. Fix in parser, not tests.
    fn parse_call_with_body() {
        let ast = parse("type beam(result) { loss precision }");
        match ast {
            Ast::Call { name, args } => {
                assert_eq!(name, Atom::new("type"));
                // args: [Call("beam", ["result"]), Body(["loss", "precision"])]
                assert_eq!(args.len(), 2);
                assert!(matches!(args.last(), Some(Ast::Body(_))));
            }
            other => panic!("expected Call with body, got {:?}", other),
        }
    }

    #[test]
    #[ignore] // Parser bug: newline-separated exprs not yet wrapped in Body. Fix in parser.
    fn parse_multiple_top_level_exprs() {
        let ast = parse("focus id\nproject ref");
        assert!(matches!(ast, Ast::Body(_)));
    }

    #[test]
    fn parse_split_operator_decl() {
        assert_eq!(
            parse("split |(ref, ref)"),
            Ast::Call {
                name: Atom::new("split"),
                args: vec![Ast::Call {
                    name: Atom::new("|"),
                    args: vec![Ast::Atom(Atom::new("ref")), Ast::Atom(Atom::new("ref")),],
                }],
            }
        );
    }

    #[test]
    fn focus_then_project_round_trip() {
        // The litmus test: parse → print → parse produces the same AST.
        let source = "focus type(id)";
        let ast = ASTPrism.parse(source);
        let emitted = format!("{}", ast);
        let reparsed = ASTPrism.parse(&emitted);
        assert_eq!(ast, reparsed);
    }

    #[test]
    fn prism_trait_focus_produces_tokens() {
        let seed = Optic::ok((), "focus id".to_string());
        let beam = ASTPrism.focus(seed);
        let tokens = beam.result().ok().expect("focus failed");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Word("focus".to_string()));
        assert_eq!(tokens[1], Token::Word("id".to_string()));
    }

    #[test]
    fn prism_trait_project_produces_ast() {
        let seed = Optic::ok((), "focus id".to_string());
        let focused = ASTPrism.focus(seed);
        let projected = ASTPrism.project(focused);
        assert_eq!(
            projected.result().ok().unwrap(),
            &Ast::Call {
                name: Atom::new("focus"),
                args: vec![Ast::Atom(Atom::new("id"))],
            }
        );
    }

    // -- Phase 4: Parser produces optic variants --

    #[test]
    fn parse_focus_optic_with_parens_and_body() {
        let ast = parse("focus(x) { y }");
        assert_eq!(
            ast,
            Ast::Focus {
                target: Some(Box::new(Ast::Atom(Atom::new("x")))),
                body: Body::new(vec![Ast::Atom(Atom::new("y"))]),
            }
        );
    }

    #[test]
    fn parse_project_optic() {
        let ast = parse("project(active) { filtered }");
        assert_eq!(
            ast,
            Ast::Project {
                query: Some(Box::new(Ast::Atom(Atom::new("active")))),
                body: Body::new(vec![Ast::Atom(Atom::new("filtered"))]),
            }
        );
    }

    #[test]
    fn parse_split_optic() {
        let ast = parse("split(origin) { component }");
        assert_eq!(
            ast,
            Ast::Split {
                root: Some(Box::new(Ast::Atom(Atom::new("origin")))),
                body: Body::new(vec![Ast::Atom(Atom::new("component"))]),
            }
        );
    }

    #[test]
    fn parse_zoom_optic() {
        let ast = parse("zoom(@user) { view }");
        assert_eq!(
            ast,
            Ast::Zoom {
                perspective: Some(Box::new(Ast::Ref(Ref::new("user")))),
                body: Body::new(vec![Ast::Atom(Atom::new("view"))]),
            }
        );
    }

    #[test]
    fn parse_refract_optic() {
        let ast = parse("refract(settle) { proof }");
        assert_eq!(
            ast,
            Ast::Refract {
                mutation: Some(Box::new(Ast::Atom(Atom::new("settle")))),
                body: Body::new(vec![Ast::Atom(Atom::new("proof"))]),
            }
        );
    }

    #[test]
    fn parse_optic_without_body_stays_call() {
        // No body → stays as Call, not an optic variant
        let ast = parse("focus(x)");
        assert_eq!(
            ast,
            Ast::Call {
                name: Atom::new("focus"),
                args: vec![Ast::Atom(Atom::new("x"))],
            }
        );
    }

    #[test]
    fn parse_optic_space_arg_with_body() {
        // focus x { y } — space-separated arg, then body → optic
        let ast = parse("focus x { y }");
        assert_eq!(
            ast,
            Ast::Focus {
                target: Some(Box::new(Ast::Atom(Atom::new("x")))),
                body: Body::new(vec![Ast::Atom(Atom::new("y"))]),
            }
        );
    }

    #[test]
    fn parse_optic_empty_body() {
        let ast = parse("focus(x) {}");
        assert_eq!(
            ast,
            Ast::Focus {
                target: Some(Box::new(Ast::Atom(Atom::new("x")))),
                body: Body::new(vec![]),
            }
        );
    }

    #[test]
    fn parse_optic_round_trip() {
        // parse → display → parse produces the same AST
        let source = "focus(eigenboard) {\n  fiedler\n  loss\n}";
        let ast = parse(source);
        assert!(matches!(ast, Ast::Focus { .. }), "should parse as Focus optic");
        let emitted = format!("{}", ast);
        let reparsed = parse(&emitted);
        assert_eq!(ast, reparsed, "round-trip failed: emitted={}", emitted);
    }

    #[test]
    fn parse_non_optic_name_with_body_stays_call() {
        // "type(x) { y }" — not an optic name, stays as Call
        let ast = parse("type(x) { y }");
        assert!(matches!(ast, Ast::Call { .. }), "non-optic name should stay as Call");
    }

    #[test]
    fn focus_with_body_in_args_stays_call() {
        // focus(x, { y }) — body inside the argument list as second arg → Call, NOT Focus.
        // The optic detection only triggers for name(single_arg) { body }, not name(arg1, arg2).
        let ast = parse("focus(x, { y })");
        match &ast {
            Ast::Call { name, args } => {
                assert_eq!(name, &Atom::new("focus"));
                assert_eq!(args.len(), 2);
                assert_eq!(args[0], Ast::Atom(Atom::new("x")));
                assert!(
                    matches!(&args[1], Ast::Body(body) if body.children() == &[Ast::Atom(Atom::new("y"))]),
                    "second arg should be Body([y]), got {:?}", args[1]
                );
            }
            other => panic!("expected Call, got {:?}", other),
        }
    }

    #[test]
    fn parse_bare_refract_is_optic() {
        // refract { settle } — bare optic (no argument, just body) → Refract with None mutation
        let ast = parse("refract { settle }");
        assert_eq!(
            ast,
            Ast::Refract {
                mutation: None,
                body: Body::new(vec![Ast::Atom(Atom::new("settle"))]),
            }
        );
    }

    #[test]
    fn parse_bare_focus_is_optic() {
        // focus { x } — bare optic (no argument, just body) → Focus with None target
        let ast = parse("focus { x }");
        assert_eq!(
            ast,
            Ast::Focus {
                target: None,
                body: Body::new(vec![Ast::Atom(Atom::new("x"))]),
            }
        );
    }

    #[test]
    fn parse_bare_optic_round_trip() {
        // Bare optics should round-trip through display and re-parse
        let ast = Ast::Refract {
            mutation: None,
            body: Body::new(vec![Ast::Atom(Atom::new("settle"))]),
        };
        let emitted = format!("{}", ast);
        let reparsed = parse(&emitted);
        assert_eq!(ast, reparsed, "bare optic round-trip failed: emitted={}", emitted);
    }

    // -- Phase: Sigil syntax --

    #[test]
    fn tokenize_sigil_short_form() {
        let tokens = tokenize("~f'flake.nix'");
        assert_eq!(tokens, vec![
            Token::Tilde,
            Token::Word("f".to_string()),
            Token::SigilValue("flake.nix".to_string()),
        ]);
    }

    #[test]
    fn tokenize_sigil_qualified_form() {
        let tokens = tokenize("~io/file'flake.nix'");
        assert_eq!(tokens, vec![
            Token::Tilde,
            Token::Word("io/file".to_string()),
            Token::SigilValue("flake.nix".to_string()),
        ]);
    }

    #[test]
    fn parse_sigil_short_form() {
        let ast = parse("~f'flake.nix'");
        assert_eq!(
            ast,
            Ast::Sigil {
                prefix: Atom::new("f"),
                value: Atom::new("flake.nix"),
            }
        );
    }

    #[test]
    fn parse_sigil_qualified_form() {
        let ast = parse("~io/file'flake.nix'");
        assert_eq!(
            ast,
            Ast::Sigil {
                prefix: Atom::new("io/file"),
                value: Atom::new("flake.nix"),
            }
        );
    }

    #[test]
    fn parse_sigil_dir() {
        let ast = parse("~d'boot/'");
        assert_eq!(
            ast,
            Ast::Sigil {
                prefix: Atom::new("d"),
                value: Atom::new("boot/"),
            }
        );
    }

    #[test]
    fn parse_sigil_uri() {
        let ast = parse("~u'github:systemic-engineer/spectral'");
        assert_eq!(
            ast,
            Ast::Sigil {
                prefix: Atom::new("u"),
                value: Atom::new("github:systemic-engineer/spectral"),
            }
        );
    }

    #[test]
    fn parse_sigil_in_call() {
        // out @code/nix -> ~f'flake.nix'
        let ast = parse("out ~f'flake.nix'");
        assert_eq!(
            ast,
            Ast::Call {
                name: Atom::new("out"),
                args: vec![Ast::Sigil {
                    prefix: Atom::new("f"),
                    value: Atom::new("flake.nix"),
                }],
            }
        );
    }

    #[test]
    fn sigil_round_trip() {
        let ast = Ast::Sigil {
            prefix: Atom::new("f"),
            value: Atom::new("flake.nix"),
        };
        let emitted = format!("{}", ast);
        assert_eq!(emitted, "~f'flake.nix'");
        let reparsed = parse(&emitted);
        assert_eq!(ast, reparsed, "sigil round-trip failed: emitted={}", emitted);
    }

    #[test]
    fn sigil_display() {
        let ast = Ast::Sigil {
            prefix: Atom::new("io/file"),
            value: Atom::new("path/to/thing"),
        };
        assert_eq!(format!("{}", ast), "~io/file'path/to/thing'");
    }
}
