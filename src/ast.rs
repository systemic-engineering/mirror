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
#[cfg_attr(feature = "shatter", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "shatter", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "shatter", derive(serde::Serialize, serde::Deserialize))]
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

/// The mirror AST — each node is either syntax or an optic.
///
/// The five syntax variants (Atom, Ref, Body, Call, Prism) represent
/// structural elements. The five optic variants (Focus, Project, Split,
/// Zoom, Refract) represent the Prism trait operations — composition
/// of optics IS the Dirac operator. Serialization of the AST IS the
/// .shatter artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "shatter", derive(serde::Serialize, serde::Deserialize))]
pub enum Ast {
    // -- Syntax --

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

    // -- Optics --

    /// Focus: observe a subgraph. Read-only projection.
    /// `focus(target) { observations }`
    Focus { target: Box<Ast>, body: Body },

    /// Project: filter by what matters. Reduce dimensionality.
    /// `project(query) { filtered results }`
    Project { query: Box<Ast>, body: Body },

    /// Split: explore connectivity. Follow edges.
    /// `split(root) { connected components }`
    Split { root: Box<Ast>, body: Body },

    /// Zoom: shift perspective without acting.
    /// `zoom(perspective) { view from there }`
    Zoom { perspective: Box<Ast>, body: Body },

    /// Refract: the one write. Settle.
    /// `refract(mutation) { proof }`
    Refract { mutation: Box<Ast>, body: Body },
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

        // -- Optics --
        Ast::Focus { target, body } => emit_optic("focus", target, body, indent, &pad, f),
        Ast::Project { query, body } => emit_optic("project", query, body, indent, &pad, f),
        Ast::Split { root, body } => emit_optic("split", root, body, indent, &pad, f),
        Ast::Zoom { perspective, body } => emit_optic("zoom", perspective, body, indent, &pad, f),
        Ast::Refract { mutation, body } => emit_optic("refract", mutation, body, indent, &pad, f),
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

/// Emit an optic variant: `name(arg) { body }` or `name(arg) {}`.
fn emit_optic(
    name: &str,
    arg: &Ast,
    body: &Body,
    indent: usize,
    pad: &str,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    write!(f, "{}{}(", pad, name)?;
    emit(arg, 0, f)?;
    write!(f, ")")?;
    if body.is_empty() {
        write!(f, " {{}}")
    } else {
        writeln!(f, " {{")?;
        for child in body.children() {
            emit(child, indent + 1, f)?;
            writeln!(f)?;
        }
        write!(f, "{}}}", pad)
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

    // -- Optic variant tests --

    #[test]
    fn focus_displays() {
        let ast = Ast::Focus {
            target: Box::new(Ast::Ref(Ref::new("graph"))),
            body: Body::new(vec![Ast::Atom(Atom::new("nodes"))]),
        };
        let out = format!("{}", ast);
        assert!(out.contains("focus(@graph)"));
        assert!(out.contains("nodes"));
    }

    #[test]
    fn project_displays() {
        let ast = Ast::Project {
            query: Box::new(Ast::Atom(Atom::new("active"))),
            body: Body::new(vec![Ast::Atom(Atom::new("filtered"))]),
        };
        let out = format!("{}", ast);
        assert!(out.contains("project(active)"));
        assert!(out.contains("filtered"));
    }

    #[test]
    fn split_displays() {
        let ast = Ast::Split {
            root: Box::new(Ast::Atom(Atom::new("origin"))),
            body: Body::new(vec![Ast::Atom(Atom::new("component_a"))]),
        };
        let out = format!("{}", ast);
        assert!(out.contains("split(origin)"));
        assert!(out.contains("component_a"));
    }

    #[test]
    fn zoom_displays() {
        let ast = Ast::Zoom {
            perspective: Box::new(Ast::Ref(Ref::new("user"))),
            body: Body::new(vec![Ast::Atom(Atom::new("view"))]),
        };
        let out = format!("{}", ast);
        assert!(out.contains("zoom(@user)"));
        assert!(out.contains("view"));
    }

    #[test]
    fn refract_displays() {
        let ast = Ast::Refract {
            mutation: Box::new(Ast::Atom(Atom::new("settle"))),
            body: Body::new(vec![Ast::Atom(Atom::new("proof"))]),
        };
        let out = format!("{}", ast);
        assert!(out.contains("refract(settle)"));
        assert!(out.contains("proof"));
    }

    #[test]
    fn optic_empty_body_displays() {
        let ast = Ast::Focus {
            target: Box::new(Ast::Atom(Atom::new("x"))),
            body: Body::new(vec![]),
        };
        let out = format!("{}", ast);
        assert_eq!(out, "focus(x) {}");
    }

    #[test]
    fn optic_content_addressed_deterministic() {
        let a = Ast::Focus {
            target: Box::new(Ast::Atom(Atom::new("x"))),
            body: Body::new(vec![Ast::Atom(Atom::new("y"))]),
        };
        let b = Ast::Focus {
            target: Box::new(Ast::Atom(Atom::new("x"))),
            body: Body::new(vec![Ast::Atom(Atom::new("y"))]),
        };
        use crate::kernel::ContentAddressed;
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn optic_different_variants_different_oid() {
        use crate::kernel::ContentAddressed;
        let focus = Ast::Focus {
            target: Box::new(Ast::Atom(Atom::new("x"))),
            body: Body::new(vec![]),
        };
        let project = Ast::Project {
            query: Box::new(Ast::Atom(Atom::new("x"))),
            body: Body::new(vec![]),
        };
        assert_ne!(focus.content_oid(), project.content_oid());
    }

    // -- Phase 5: Serde serialization (shatter feature) --

    #[cfg(feature = "shatter")]
    mod shatter_tests {
        use super::*;

        fn sample_focus() -> Ast {
            Ast::Focus {
                target: Box::new(Ast::Atom(Atom::new("eigenboard"))),
                body: Body::new(vec![
                    Ast::Atom(Atom::new("fiedler")),
                    Ast::Atom(Atom::new("loss")),
                ]),
            }
        }

        fn sample_nested() -> Ast {
            Ast::Prism {
                name: Ref::new("meta"),
                body: Body::new(vec![
                    Ast::Focus {
                        target: Box::new(Ast::Ref(Ref::new("graph"))),
                        body: Body::new(vec![Ast::Atom(Atom::new("nodes"))]),
                    },
                    Ast::Project {
                        query: Box::new(Ast::Atom(Atom::new("active"))),
                        body: Body::new(vec![]),
                    },
                ]),
            }
        }

        #[test]
        fn bincode_round_trip_focus() {
            let ast = sample_focus();
            let bytes = bincode::serialize(&ast).expect("serialize");
            let back: Ast = bincode::deserialize(&bytes).expect("deserialize");
            assert_eq!(ast, back);
        }

        #[test]
        fn bincode_round_trip_nested() {
            let ast = sample_nested();
            let bytes = bincode::serialize(&ast).expect("serialize");
            let back: Ast = bincode::deserialize(&bytes).expect("deserialize");
            assert_eq!(ast, back);
        }

        #[test]
        fn bincode_round_trip_all_variants() {
            let asts = vec![
                Ast::Atom(Atom::new("x")),
                Ast::Ref(Ref::new("y")),
                Ast::Body(Body::new(vec![Ast::Atom(Atom::new("z"))])),
                Ast::Call {
                    name: Atom::new("f"),
                    args: vec![Ast::Atom(Atom::new("a"))],
                },
                Ast::Prism {
                    name: Ref::new("p"),
                    body: Body::new(vec![]),
                },
                Ast::Focus {
                    target: Box::new(Ast::Atom(Atom::new("t"))),
                    body: Body::new(vec![]),
                },
                Ast::Project {
                    query: Box::new(Ast::Atom(Atom::new("q"))),
                    body: Body::new(vec![]),
                },
                Ast::Split {
                    root: Box::new(Ast::Atom(Atom::new("r"))),
                    body: Body::new(vec![]),
                },
                Ast::Zoom {
                    perspective: Box::new(Ast::Atom(Atom::new("p"))),
                    body: Body::new(vec![]),
                },
                Ast::Refract {
                    mutation: Box::new(Ast::Atom(Atom::new("m"))),
                    body: Body::new(vec![]),
                },
            ];
            for ast in asts {
                let bytes = bincode::serialize(&ast).expect("serialize");
                let back: Ast = bincode::deserialize(&bytes).expect("deserialize");
                assert_eq!(ast, back, "round-trip failed for {:?}", back);
            }
        }

        #[test]
        fn bincode_compact_for_deep_trees() {
            // Bincode has per-field overhead (enum tags, length prefixes)
            // that exceeds text for tiny ASTs. For deeper/wider trees the
            // binary format wins because it avoids indentation and keywords.
            let ast = Ast::Prism {
                name: Ref::new("eigenboard"),
                body: Body::new(vec![
                    Ast::Focus {
                        target: Box::new(Ast::Atom(Atom::new("fiedler_vector"))),
                        body: Body::new(vec![
                            Ast::Atom(Atom::new("algebraic_connectivity")),
                            Ast::Atom(Atom::new("spectral_gap")),
                            Ast::Atom(Atom::new("laplacian")),
                        ]),
                    },
                    Ast::Project {
                        query: Box::new(Ast::Atom(Atom::new("settlement"))),
                        body: Body::new(vec![
                            Ast::Atom(Atom::new("convergence")),
                            Ast::Atom(Atom::new("holonomy")),
                        ]),
                    },
                    Ast::Refract {
                        mutation: Box::new(Ast::Atom(Atom::new("crystal"))),
                        body: Body::new(vec![
                            Ast::Atom(Atom::new("proof")),
                        ]),
                    },
                ]),
            };
            let text = format!("{}", ast);
            let bytes = bincode::serialize(&ast).expect("serialize");
            // The binary form should be reasonably compact — not orders of
            // magnitude larger than text for real-world ASTs.
            assert!(
                bytes.len() < text.len() * 2,
                "bincode ({} bytes) should be < 2x text ({} bytes)",
                bytes.len(),
                text.len()
            );
        }

        #[test]
        fn bincode_content_oid_preserved() {
            use crate::kernel::ContentAddressed;
            let ast = sample_focus();
            let bytes = bincode::serialize(&ast).expect("serialize");
            let back: Ast = bincode::deserialize(&bytes).expect("deserialize");
            assert_eq!(
                ast.content_oid(),
                back.content_oid(),
                "content_oid must survive serialization"
            );
        }
    }
}
