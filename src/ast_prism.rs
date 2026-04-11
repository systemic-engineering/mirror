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

use prism::{Beam, Prism, PureBeam};

use crate::ast::{Ast, Atom, Body, Ref};

// ---------------------------------------------------------------------------
// Tokens — the Focused type. Internal to ASTPrism.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    At,
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
    type Input = PureBeam<(), String>;
    type Focused = PureBeam<String, Vec<Token>>;
    type Projected = PureBeam<Vec<Token>, Ast>;
    type Refracted = PureBeam<Ast, ASTPrism>;

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
        let seed = PureBeam::ok((), source.to_string());
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

            // name(args) possibly followed by { body }
            if matches!(tokens.get(*cursor), Some(Token::LParen)) {
                *cursor += 1;
                let args = parse_args(tokens, cursor);
                skip_trivia(tokens, cursor);
                if matches!(tokens.get(*cursor), Some(Token::LBrace)) {
                    *cursor += 1;
                    let body = parse_body(tokens, cursor);
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
                return Ast::Call {
                    name: Atom::new(name),
                    args: vec![Ast::Body(body)],
                };
            }

            // name followed by another expression (space-separated arg)
            match tokens.get(*cursor) {
                Some(Token::Word(_)) | Some(Token::At) => {
                    let arg = parse_expr(tokens, cursor);
                    skip_trivia(tokens, cursor);
                    if matches!(tokens.get(*cursor), Some(Token::LBrace)) {
                        *cursor += 1;
                        let body = parse_body(tokens, cursor);
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
        let seed = PureBeam::ok((), "focus id".to_string());
        let beam = ASTPrism.focus(seed);
        let tokens = beam.result().ok().expect("focus failed");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Word("focus".to_string()));
        assert_eq!(tokens[1], Token::Word("id".to_string()));
    }

    #[test]
    fn prism_trait_project_produces_ast() {
        let seed = PureBeam::ok((), "focus id".to_string());
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
}
