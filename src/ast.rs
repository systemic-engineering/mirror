//! The mirror AST.
//!
//! Four types, one enum. Mirror is a prism under the hood.
//!
//! - `Atom` — a symbol
//! - `Ref`  — @symbol (a reference to a named form)
//! - `Body` — { children } (a block scope)
//! - `Call` — name(args) (a name applied to arguments)
//! - `Prism` — @name { body } (a named block — the `prism` keyword)
//!
//! The AST is what `ASTPrism.split` yields as parts. Each part is one
//! of these five. The tree structure IS the split structure. Parsing
//! a .mirror file refracts it through the ASTPrism; the crystal is a
//! MirrorPrism — the compiled thing.

domain_oid!(/// Content address for AST nodes.
pub AstOid);

/// A symbol. The leaf of every expression.
///
/// Atoms are identifiers, operators, keywords, type names — anything
/// that isn't prefixed with `@` or wrapped in `{}`.
///
/// Examples: `id`, `type`, `focus`, `|>`, `f64`, `loss`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Atom(pub String);

impl Atom {
    pub fn new(s: impl Into<String>) -> Self {
        Atom(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A reference to a named form. The `@` sigil.
///
/// `@prism` is `Ref(Atom("prism"))`, not `Atom("@prism")`.
/// The sigil is structural, not textual.
///
/// Examples: `@prism`, `@meta`, `@actor`, `@property`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ref(pub Atom);

impl Ref {
    pub fn new(s: impl Into<String>) -> Self {
        Ref(Atom::new(s))
    }

    pub fn atom(&self) -> &Atom {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// A block scope. The `{ }` delimiters.
///
/// Body is a newtype around `Vec<Ast>`, giving braces structural
/// meaning in the type system — you can't accidentally confuse a
/// list of call arguments with a block body.
///
/// Examples: `{ focus type(id) }`, `{ result: result, loss: loss }`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Body(pub Vec<Ast>);

impl Body {
    pub fn new(children: Vec<Ast>) -> Self {
        Body(children)
    }

    pub fn children(&self) -> &[Ast] {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// The mirror AST.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ast {
    /// A symbol.
    Atom(Atom),

    /// A reference to a named form: `@foo`
    Ref(Ref),

    /// A block scope: `{ children }`
    Body(Body),

    /// A name applied to arguments: `name(args)` or `name arg1 arg2`
    Call { name: Atom, args: Vec<Ast> },

    /// A named block: `prism @name { body }`
    Prism { name: Ref, body: Body },
}

// ---------------------------------------------------------------------------
// Emit — print an AST back as mirror source
// ---------------------------------------------------------------------------

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Ref {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        emit(self, 0, f)
    }
}

fn emit(ast: &Ast, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let pad = "  ".repeat(indent);
    match ast {
        Ast::Atom(a) => write!(f, "{}{}", pad, a),
        Ast::Ref(r) => write!(f, "{}{}", pad, r),
        Ast::Body(body) => {
            writeln!(f, "{}{{", pad)?;
            for child in body.children() {
                emit(child, indent + 1, f)?;
                writeln!(f)?;
            }
            write!(f, "{}}}", pad)
        }
        Ast::Call { name, args } => {
            write!(f, "{}{}", pad, name)?;
            if !args.is_empty() {
                // Check if the last arg is a Body — if so, print it as a block
                let (regular, block) = split_body_arg(args);
                if !regular.is_empty() {
                    write!(f, "(")?;
                    for (i, arg) in regular.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        emit(arg, 0, f)?;
                    }
                    write!(f, ")")?;
                }
                if let Some(body) = block {
                    writeln!(f, " {{")?;
                    for child in body.children() {
                        emit(child, indent + 1, f)?;
                        writeln!(f)?;
                    }
                    write!(f, "{}}}", pad)?;
                }
            }
            Ok(())
        }
        Ast::Prism { name, body } => {
            write!(f, "{}prism {} {{", pad, name)?;
            if body.is_empty() {
                write!(f, "}}")
            } else {
                writeln!(f)?;
                for child in body.children() {
                    emit(child, indent + 1, f)?;
                    writeln!(f)?;
                }
                write!(f, "{}}}", pad)
            }
        }
    }
}

/// Split the args list into regular args and an optional trailing Body.
fn split_body_arg(args: &[Ast]) -> (&[Ast], Option<&Body>) {
    if let Some(Ast::Body(body)) = args.last() {
        (&args[..args.len() - 1], Some(body))
    } else {
        (args, None)
    }
}

// ---------------------------------------------------------------------------
// ContentAddressed — content-address by display form
// ---------------------------------------------------------------------------

impl crate::kernel::ContentAddressed for Ast {
    type Oid = crate::Oid;
    fn content_oid(&self) -> crate::Oid {
        crate::Oid::hash(format!("{}", self).as_bytes())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atom_displays() {
        assert_eq!(format!("{}", Atom::new("id")), "id");
        assert_eq!(format!("{}", Atom::new("|>")), "|>");
    }

    #[test]
    fn ref_displays_with_sigil() {
        assert_eq!(format!("{}", Ref::new("prism")), "@prism");
    }

    #[test]
    fn simple_call_displays() {
        let ast = Ast::Call {
            name: Atom::new("focus"),
            args: vec![Ast::Atom(Atom::new("id"))],
        };
        assert_eq!(format!("{}", ast), "focus(id)");
    }

    #[test]
    fn nested_call_displays() {
        let ast = Ast::Call {
            name: Atom::new("focus"),
            args: vec![Ast::Call {
                name: Atom::new("type"),
                args: vec![Ast::Atom(Atom::new("id"))],
            }],
        };
        assert_eq!(format!("{}", ast), "focus(type(id))");
    }

    #[test]
    fn call_with_ref_arg() {
        let ast = Ast::Call {
            name: Atom::new("in"),
            args: vec![Ast::Ref(Ref::new("prism"))],
        };
        assert_eq!(format!("{}", ast), "in(@prism)");
    }

    #[test]
    fn call_with_body_arg() {
        let ast = Ast::Call {
            name: Atom::new("type"),
            args: vec![
                Ast::Call {
                    name: Atom::new("beam"),
                    args: vec![Ast::Atom(Atom::new("result"))],
                },
                Ast::Body(Body::new(vec![
                    Ast::Atom(Atom::new("loss")),
                    Ast::Atom(Atom::new("precision")),
                ])),
            ],
        };
        let out = format!("{}", ast);
        assert!(out.contains("type(beam(result))"));
        assert!(out.contains("loss"));
        assert!(out.contains("precision"));
    }

    #[test]
    fn prism_displays() {
        let ast = Ast::Prism {
            name: Ref::new("meta"),
            body: Body::new(vec![Ast::Call {
                name: Atom::new("focus"),
                args: vec![Ast::Ref(Ref::new(""))],
            }]),
        };
        let out = format!("{}", ast);
        assert!(out.starts_with("prism @meta {"));
        assert!(out.contains("focus(@)"));
    }

    #[test]
    fn empty_prism_displays() {
        let ast = Ast::Prism {
            name: Ref::new("empty"),
            body: Body::new(vec![]),
        };
        assert_eq!(format!("{}", ast), "prism @empty {}");
    }
}
