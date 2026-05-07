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

    // -- Sigils --

    /// A typed prefix literal: `~f'path'` or `~io/file'path'`
    /// prefix is the short form (e.g. "f") or qualified form (e.g. "io/file").
    /// value is the content between single quotes.
    Sigil { prefix: Atom, value: Atom },

    // -- Optics --

    /// Focus: observe a subgraph. Read-only projection.
    /// `focus(target) { observations }` or `focus { observations }`
    Focus { target: Option<Box<Ast>>, body: Body },

    /// Project: filter by what matters. Reduce dimensionality.
    /// `project(query) { filtered results }` or `project { filtered results }`
    Project { query: Option<Box<Ast>>, body: Body },

    /// Split: explore connectivity. Follow edges.
    /// `split(root) { connected components }` or `split { connected components }`
    Split { root: Option<Box<Ast>>, body: Body },

    /// Zoom: shift perspective without acting.
    /// `zoom(perspective) { view from there }` or `zoom { view from there }`
    Zoom { perspective: Option<Box<Ast>>, body: Body },

    /// Refract: the one write. Settle.
    /// `refract(mutation) { proof }` or `refract { proof }`
    Refract { mutation: Option<Box<Ast>>, body: Body },
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
        Ast::Sigil { prefix, value } => write!(f, "{}~{}'{}'" , pad, prefix, value),
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
        Ast::Focus { target, body } => emit_optic("focus", target.as_deref(), body, indent, &pad, f),
        Ast::Project { query, body } => emit_optic("project", query.as_deref(), body, indent, &pad, f),
        Ast::Split { root, body } => emit_optic("split", root.as_deref(), body, indent, &pad, f),
        Ast::Zoom { perspective, body } => emit_optic("zoom", perspective.as_deref(), body, indent, &pad, f),
        Ast::Refract { mutation, body } => emit_optic("refract", mutation.as_deref(), body, indent, &pad, f),
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

/// Emit an optic variant: `name(arg) { body }`, `name(arg) {}`, or `name { body }`.
fn emit_optic(
    name: &str,
    arg: Option<&Ast>,
    body: &Body,
    indent: usize,
    pad: &str,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    write!(f, "{}{}", pad, name)?;
    if let Some(a) = arg {
        write!(f, "(")?;
        emit(a, 0, f)?;
        write!(f, ")")?;
    }
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
// Transformation primitives — walk/map/fold on the AST
// ---------------------------------------------------------------------------

impl Ast {
    /// Walk: visit every node depth-first, calling `visitor` on each.
    /// This is the split operation on the AST — enumerate all parts.
    pub fn walk<F: FnMut(&Ast)>(&self, visitor: &mut F) {
        visitor(self);
        for child in self.children() {
            child.walk(visitor);
        }
    }

    /// Map: transform every node bottom-up (children first, then parent).
    /// This is the zoom operation — shift perspective at every level.
    pub fn map<F: FnMut(Ast) -> Ast>(self, f: &mut F) -> Ast {
        let mapped = match self {
            Ast::Atom(_) | Ast::Ref(_) | Ast::Sigil { .. } => self,
            Ast::Body(body) => {
                let children = body.0.into_iter().map(|c| c.map(f)).collect();
                Ast::Body(Body(children))
            }
            Ast::Call { name, args } => {
                let args = args.into_iter().map(|a| a.map(f)).collect();
                Ast::Call { name, args }
            }
            Ast::Prism { name, body } => {
                let children = body.0.into_iter().map(|c| c.map(f)).collect();
                Ast::Prism { name, body: Body(children) }
            }
            Ast::Focus { target, body } => {
                let target = target.map(|t| Box::new(t.map(f)));
                let children = body.0.into_iter().map(|c| c.map(f)).collect();
                Ast::Focus { target, body: Body(children) }
            }
            Ast::Project { query, body } => {
                let query = query.map(|q| Box::new(q.map(f)));
                let children = body.0.into_iter().map(|c| c.map(f)).collect();
                Ast::Project { query, body: Body(children) }
            }
            Ast::Split { root, body } => {
                let root = root.map(|r| Box::new(r.map(f)));
                let children = body.0.into_iter().map(|c| c.map(f)).collect();
                Ast::Split { root, body: Body(children) }
            }
            Ast::Zoom { perspective, body } => {
                let perspective = perspective.map(|p| Box::new(p.map(f)));
                let children = body.0.into_iter().map(|c| c.map(f)).collect();
                Ast::Zoom { perspective, body: Body(children) }
            }
            Ast::Refract { mutation, body } => {
                let mutation = mutation.map(|m| Box::new(m.map(f)));
                let children = body.0.into_iter().map(|c| c.map(f)).collect();
                Ast::Refract { mutation, body: Body(children) }
            }
        };
        f(mapped)
    }

    /// Fold: accumulate a value over every node depth-first.
    /// This is the fold operation — collapse structure into a single value.
    pub fn fold<A, F: FnMut(A, &Ast) -> A>(&self, init: A, f: &mut F) -> A {
        let acc = f(init, self);
        self.children().into_iter().fold(acc, |a, child| child.fold(a, f))
    }

    /// Substitute: replace all `Ref(name)` with `replacement`.
    /// This is the refract operation — the one write on the AST.
    pub fn substitute(self, name: &str, replacement: &Ast) -> Ast {
        self.map(&mut |node| {
            if let Ast::Ref(ref r) = node {
                if r.as_str() == name {
                    return replacement.clone();
                }
            }
            node
        })
    }

    /// Direct child nodes (non-recursive).
    pub fn children(&self) -> Vec<&Ast> {
        match self {
            Ast::Atom(_) | Ast::Ref(_) | Ast::Sigil { .. } => vec![],
            Ast::Body(body) => body.0.iter().collect(),
            Ast::Call { args, .. } => args.iter().collect(),
            Ast::Prism { body, .. } => body.0.iter().collect(),
            Ast::Focus { target, body } => {
                let mut v: Vec<&Ast> = target.as_deref().into_iter().collect();
                v.extend(body.0.iter());
                v
            }
            Ast::Project { query, body } => {
                let mut v: Vec<&Ast> = query.as_deref().into_iter().collect();
                v.extend(body.0.iter());
                v
            }
            Ast::Split { root, body } => {
                let mut v: Vec<&Ast> = root.as_deref().into_iter().collect();
                v.extend(body.0.iter());
                v
            }
            Ast::Zoom { perspective, body } => {
                let mut v: Vec<&Ast> = perspective.as_deref().into_iter().collect();
                v.extend(body.0.iter());
                v
            }
            Ast::Refract { mutation, body } => {
                let mut v: Vec<&Ast> = mutation.as_deref().into_iter().collect();
                v.extend(body.0.iter());
                v
            }
        }
    }

    /// Maximum nesting depth of the tree.
    pub fn depth(&self) -> usize {
        let child_depth = self.children().iter()
            .map(|c| c.depth())
            .max()
            .unwrap_or(0);
        1 + child_depth
    }

    /// Total number of nodes in the tree (recursive).
    pub fn node_count(&self) -> usize {
        let mut count = 0;
        self.walk(&mut |_| count += 1);
        count
    }

    /// Collect all `Ref` names referenced anywhere in the tree.
    pub fn referenced_names(&self) -> std::collections::HashSet<String> {
        let mut names = std::collections::HashSet::new();
        self.walk(&mut |node| {
            if let Ast::Ref(r) = node {
                names.insert(r.as_str().to_string());
            }
        });
        names
    }

    /// Check if this node is a Call with the given name.
    pub fn is_call(&self, name: &str) -> bool {
        matches!(self, Ast::Call { name: n, .. } if n.as_str() == name)
    }

    /// If this is a Call, return its name.
    pub fn call_name(&self) -> Option<&str> {
        if let Ast::Call { name, .. } = self {
            Some(name.as_str())
        } else {
            None
        }
    }

    /// If this is a Call, return the first Atom argument's value (the declaration name).
    pub fn decl_name(&self) -> Option<&str> {
        if let Ast::Call { args, .. } = self {
            if let Some(Ast::Atom(a)) = args.first() {
                return Some(a.as_str());
            }
        }
        None
    }
}

// ---------------------------------------------------------------------------
// Simplification operations — compose walk/map/fold
// ---------------------------------------------------------------------------

impl Ast {
    /// Remove top-level declarations that are never referenced by other nodes.
    ///
    /// Works on a Body of declarations. Finds all `@ref` names used inside
    /// action bodies, then removes type declarations whose name does not
    /// appear in that set.
    ///
    /// This is the Introject strategy: keep only what actions need.
    pub fn eliminate_dead(self) -> Ast {
        if let Ast::Body(body) = &self {
            // Phase 1: collect all names referenced in action bodies
            let mut referenced = std::collections::HashSet::new();
            for child in body.children() {
                if child.is_call("action") {
                    let refs = child.referenced_names();
                    referenced.extend(refs);
                }
            }
            // Phase 2: also collect refs from type bodies that are themselves referenced
            // (transitive closure: if action uses type A, and A references @B, keep B)
            let mut changed = true;
            while changed {
                changed = false;
                for child in body.children() {
                    if child.is_call("type") {
                        if let Some(name) = child.decl_name() {
                            if referenced.contains(name) {
                                let type_refs = child.referenced_names();
                                for r in type_refs {
                                    if referenced.insert(r) {
                                        changed = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Phase 3: filter — keep non-type decls, and type decls that are referenced
            let filtered: Vec<Ast> = body.0.iter().filter(|child| {
                if child.is_call("type") {
                    if let Some(name) = child.decl_name() {
                        return referenced.contains(name);
                    }
                }
                true // keep non-type declarations
            }).cloned().collect();
            Ast::Body(Body::new(filtered))
        } else {
            self
        }
    }

    /// Collapse type aliases that have identical definitions.
    ///
    /// When multiple type declarations have the same body (same args after
    /// the name), keep the first one and substitute all references to the
    /// others with the kept name.
    ///
    /// This is the Cartographer strategy: merge redundant structure.
    pub fn collapse_aliases(self) -> Ast {
        if let Ast::Body(body) = &self {
            // Phase 1: group type declarations by their body signature
            let mut signatures: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();
            let mut renames: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();

            for child in body.children() {
                if child.is_call("type") {
                    if let Some(name) = child.decl_name() {
                        // The signature is the Display form of args[1..] (everything after the name)
                        if let Ast::Call { args, .. } = child {
                            let sig: String = args.iter().skip(1)
                                .map(|a| format!("{}", a))
                                .collect::<Vec<_>>()
                                .join(",");
                            if let Some(canonical) = signatures.get(&sig) {
                                // This is a duplicate — rename to canonical
                                renames.insert(name.to_string(), canonical.clone());
                            } else {
                                signatures.insert(sig, name.to_string());
                            }
                        }
                    }
                }
            }

            if renames.is_empty() {
                return self;
            }

            // Phase 2: remove duplicate type declarations
            let filtered: Vec<Ast> = body.0.iter().filter(|child| {
                if child.is_call("type") {
                    if let Some(name) = child.decl_name() {
                        return !renames.contains_key(name);
                    }
                }
                true
            }).cloned().collect();

            // Phase 3: substitute references to removed types
            let mut result = Ast::Body(Body::new(filtered));
            for (old_name, new_name) in &renames {
                result = result.substitute(old_name, &Ast::Ref(Ref::new(new_name.as_str())));
            }
            result
        } else {
            self
        }
    }

    /// Flatten wrapper types — types with a single field that just forward
    /// to an inner type.
    ///
    /// A wrapper type looks like: `type wrapped_id { inner: id }`
    /// After flattening, all references to `@wrapped_id` become `@id`
    /// (or the inner type name), and the wrapper declaration is removed.
    ///
    /// This is the Explorer strategy: remove unnecessary indirection.
    pub fn flatten_wrappers(self) -> Ast {
        if let Ast::Body(body) = &self {
            // Phase 1: identify wrapper types (single-field struct types)
            let mut wrappers: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();

            for child in body.children() {
                if child.is_call("type") {
                    if let (Some(name), Some(inner)) = (child.decl_name(), detect_wrapper(child)) {
                        wrappers.insert(name.to_string(), inner);
                    }
                }
            }

            if wrappers.is_empty() {
                return self;
            }

            // Phase 2: remove wrapper type declarations
            let filtered: Vec<Ast> = body.0.iter().filter(|child| {
                if child.is_call("type") {
                    if let Some(name) = child.decl_name() {
                        return !wrappers.contains_key(name);
                    }
                }
                true
            }).cloned().collect();

            // Phase 3: substitute references to wrappers with their inner type
            let mut result = Ast::Body(Body::new(filtered));
            // Iteratively resolve chains: wrapped_a -> wrapped_b -> id
            let mut resolved = wrappers.clone();
            for _ in 0..10 {
                let mut changed = false;
                for (_, inner) in resolved.iter_mut() {
                    if let Some(deeper) = wrappers.get(inner.as_str()) {
                        *inner = deeper.clone();
                        changed = true;
                    }
                }
                if !changed { break; }
            }
            for (wrapper_name, inner_name) in &resolved {
                result = result.substitute(wrapper_name, &Ast::Ref(Ref::new(inner_name.as_str())));
            }
            result
        } else {
            self
        }
    }
}

/// Detect if a type Call is a wrapper — has exactly one Body child with one field.
/// Returns the inner type name if it's a wrapper.
fn detect_wrapper(ast: &Ast) -> Option<String> {
    if let Ast::Call { name, args } = ast {
        if name.as_str() != "type" { return None; }
        // Look for a Body arg with exactly one child that's a Call or has a Ref
        for arg in args.iter().skip(1) {
            if let Ast::Body(body) = arg {
                if body.len() == 1 {
                    // The single child should reference a type name
                    if let Some(child) = body.children().first() {
                        // Look for a Ref in the child
                        let mut refs = Vec::new();
                        child.walk(&mut |node| {
                            if let Ast::Ref(r) = node {
                                refs.push(r.as_str().to_string());
                            }
                        });
                        if refs.len() == 1 {
                            return Some(refs.into_iter().next().unwrap());
                        }
                    }
                }
            }
        }
    }
    None
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
            target: Some(Box::new(Ast::Ref(Ref::new("graph")))),
            body: Body::new(vec![Ast::Atom(Atom::new("nodes"))]),
        };
        let out = format!("{}", ast);
        assert!(out.contains("focus(@graph)"));
        assert!(out.contains("nodes"));
    }

    #[test]
    fn project_displays() {
        let ast = Ast::Project {
            query: Some(Box::new(Ast::Atom(Atom::new("active")))),
            body: Body::new(vec![Ast::Atom(Atom::new("filtered"))]),
        };
        let out = format!("{}", ast);
        assert!(out.contains("project(active)"));
        assert!(out.contains("filtered"));
    }

    #[test]
    fn split_displays() {
        let ast = Ast::Split {
            root: Some(Box::new(Ast::Atom(Atom::new("origin")))),
            body: Body::new(vec![Ast::Atom(Atom::new("component_a"))]),
        };
        let out = format!("{}", ast);
        assert!(out.contains("split(origin)"));
        assert!(out.contains("component_a"));
    }

    #[test]
    fn zoom_displays() {
        let ast = Ast::Zoom {
            perspective: Some(Box::new(Ast::Ref(Ref::new("user")))),
            body: Body::new(vec![Ast::Atom(Atom::new("view"))]),
        };
        let out = format!("{}", ast);
        assert!(out.contains("zoom(@user)"));
        assert!(out.contains("view"));
    }

    #[test]
    fn refract_displays() {
        let ast = Ast::Refract {
            mutation: Some(Box::new(Ast::Atom(Atom::new("settle")))),
            body: Body::new(vec![Ast::Atom(Atom::new("proof"))]),
        };
        let out = format!("{}", ast);
        assert!(out.contains("refract(settle)"));
        assert!(out.contains("proof"));
    }

    #[test]
    fn optic_empty_body_displays() {
        let ast = Ast::Focus {
            target: Some(Box::new(Ast::Atom(Atom::new("x")))),
            body: Body::new(vec![]),
        };
        let out = format!("{}", ast);
        assert_eq!(out, "focus(x) {}");
    }

    #[test]
    fn optic_content_addressed_deterministic() {
        let a = Ast::Focus {
            target: Some(Box::new(Ast::Atom(Atom::new("x")))),
            body: Body::new(vec![Ast::Atom(Atom::new("y"))]),
        };
        let b = Ast::Focus {
            target: Some(Box::new(Ast::Atom(Atom::new("x")))),
            body: Body::new(vec![Ast::Atom(Atom::new("y"))]),
        };
        use crate::kernel::ContentAddressed;
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn optic_different_variants_different_oid() {
        use crate::kernel::ContentAddressed;
        let focus = Ast::Focus {
            target: Some(Box::new(Ast::Atom(Atom::new("x")))),
            body: Body::new(vec![]),
        };
        let project = Ast::Project {
            query: Some(Box::new(Ast::Atom(Atom::new("x")))),
            body: Body::new(vec![]),
        };
        assert_ne!(focus.content_oid(), project.content_oid());
    }

    // -- Phase 6: AST transformation primitives --

    /// Helper: build a nested AST for transform tests.
    /// focus(@graph) { project(active) { @leaf } }
    fn nested_fixture() -> Ast {
        Ast::Focus {
            target: Some(Box::new(Ast::Ref(Ref::new("graph")))),
            body: Body::new(vec![
                Ast::Project {
                    query: Some(Box::new(Ast::Atom(Atom::new("active")))),
                    body: Body::new(vec![
                        Ast::Ref(Ref::new("leaf")),
                    ]),
                },
            ]),
        }
    }

    #[test]
    fn walk_visits_all_nodes() {
        let ast = nested_fixture();
        let mut count = 0;
        ast.walk(&mut |_| count += 1);
        // focus(@graph) { project(active) { @leaf } }
        // Nodes: Focus, @graph, Project, active, @leaf = 5
        assert_eq!(count, 5, "walk should visit all 5 nodes");
    }

    #[test]
    fn map_transforms_atoms() {
        let ast = Ast::Call {
            name: Atom::new("hello"),
            args: vec![
                Ast::Atom(Atom::new("world")),
                Ast::Atom(Atom::new("foo")),
            ],
        };
        let mapped = ast.map(&mut |node| {
            if let Ast::Atom(a) = &node {
                Ast::Atom(Atom::new(a.as_str().to_uppercase()))
            } else {
                node
            }
        });
        // The Call's args should be uppercased Atoms
        if let Ast::Call { args, .. } = &mapped {
            assert_eq!(args[0], Ast::Atom(Atom::new("WORLD")));
            assert_eq!(args[1], Ast::Atom(Atom::new("FOO")));
        } else {
            panic!("map should preserve Call structure");
        }
    }

    #[test]
    fn fold_counts_refs() {
        let ast = nested_fixture();
        let ref_count = ast.fold(0usize, &mut |acc, node| {
            if matches!(node, Ast::Ref(_)) { acc + 1 } else { acc }
        });
        // @graph and @leaf = 2 refs
        assert_eq!(ref_count, 2, "fold should count 2 @references");
    }

    #[test]
    fn substitute_replaces_ref() {
        let ast = nested_fixture();
        let replaced = ast.substitute("leaf", &Ast::Atom(Atom::new("crystal")));
        // The @leaf should now be Atom("crystal")
        let mut found_crystal = false;
        replaced.walk(&mut |node| {
            if let Ast::Atom(a) = node {
                if a.as_str() == "crystal" {
                    found_crystal = true;
                }
            }
        });
        assert!(found_crystal, "substitute should replace @leaf with crystal");
        // And @leaf should be gone
        let ref_count = replaced.fold(0usize, &mut |acc, node| {
            if let Ast::Ref(r) = node {
                if r.as_str() == "leaf" { acc + 1 } else { acc }
            } else {
                acc
            }
        });
        assert_eq!(ref_count, 0, "@leaf should be fully substituted");
    }

    #[test]
    fn children_of_call() {
        let ast = Ast::Call {
            name: Atom::new("type"),
            args: vec![
                Ast::Atom(Atom::new("id")),
                Ast::Ref(Ref::new("prism")),
            ],
        };
        let children = ast.children();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0], &Ast::Atom(Atom::new("id")));
        assert_eq!(children[1], &Ast::Ref(Ref::new("prism")));
    }

    #[test]
    fn children_of_focus() {
        let ast = Ast::Focus {
            target: Some(Box::new(Ast::Ref(Ref::new("graph")))),
            body: Body::new(vec![Ast::Atom(Atom::new("nodes"))]),
        };
        let children = ast.children();
        // target + body children
        assert_eq!(children.len(), 2);
        assert_eq!(children[0], &Ast::Ref(Ref::new("graph")));
        assert_eq!(children[1], &Ast::Atom(Atom::new("nodes")));
    }

    #[test]
    fn depth_of_nested() {
        // focus(@graph) { project(active) { @leaf } }
        // depth: focus(1) -> body -> project(3) -> body -> @leaf(5)
        // Actually: focus=1, children are @graph(depth=1) and project.
        // project=1, children are active(depth=1) and @leaf(depth=1).
        // So: focus -> project -> @leaf = depth 3 for the body path,
        // but the target @graph is depth 1.
        // focus depth = 1 + max(depth(@graph), depth(project(...)))
        //             = 1 + max(1, 1 + max(depth(active), depth(@leaf)))
        //             = 1 + max(1, 1 + max(1, 1))
        //             = 1 + max(1, 2)
        //             = 1 + 2 = 3
        let ast = nested_fixture();
        assert_eq!(ast.depth(), 3);
    }

    #[test]
    fn node_count_matches_walk() {
        let ast = nested_fixture();
        let nc = ast.node_count();
        let mut walk_count = 0;
        ast.walk(&mut |_| walk_count += 1);
        assert_eq!(nc, walk_count, "node_count should agree with walk count");
    }

    #[test]
    fn children_of_atom_is_empty() {
        let ast = Ast::Atom(Atom::new("leaf"));
        assert!(ast.children().is_empty());
    }

    #[test]
    fn depth_of_atom_is_one() {
        let ast = Ast::Atom(Atom::new("x"));
        assert_eq!(ast.depth(), 1);
    }

    #[test]
    fn map_is_bottom_up() {
        // map processes children first, then the parent.
        // If we wrap every Atom in a Call, the Atoms inside Calls
        // should also be wrapped (because children are mapped first).
        let ast = Ast::Body(Body::new(vec![
            Ast::Atom(Atom::new("a")),
            Ast::Atom(Atom::new("b")),
        ]));
        let mapped = ast.map(&mut |node| {
            if let Ast::Atom(ref a) = node {
                Ast::Call {
                    name: Atom::new("wrap"),
                    args: vec![node.clone()],
                }
            } else {
                node
            }
        });
        // Each Atom("a") becomes Call("wrap", [Atom("a")])
        // Then the Body contains those Calls.
        if let Ast::Body(body) = &mapped {
            assert_eq!(body.len(), 2);
            for child in body.children() {
                assert!(matches!(child, Ast::Call { name, .. } if name.as_str() == "wrap"));
            }
        } else {
            panic!("map should preserve Body at top level");
        }
    }

    #[test]
    fn substitute_is_idempotent_on_missing_ref() {
        let ast = nested_fixture();
        let original_count = ast.node_count();
        let result = ast.substitute("nonexistent", &Ast::Atom(Atom::new("x")));
        assert_eq!(result.node_count(), original_count,
            "substitute of missing ref should not change the tree");
    }

    // -- Phase 7: Simplification operations --

    /// Helper: build a Body of declarations for simplification tests.
    /// Simulates a grammar with:
    ///   type status = active | inactive
    ///   type orphan = { x: f64 }
    ///   action process(input: @status) { emit result }
    fn decl_fixture() -> Ast {
        Ast::Body(Body::new(vec![
            // type status ...
            Ast::Call {
                name: Atom::new("type"),
                args: vec![
                    Ast::Atom(Atom::new("status")),
                    Ast::Atom(Atom::new("active")),
                    Ast::Atom(Atom::new("inactive")),
                ],
            },
            // type orphan { x: f64 } — dead type, not referenced by action
            Ast::Call {
                name: Atom::new("type"),
                args: vec![
                    Ast::Atom(Atom::new("orphan")),
                    Ast::Body(Body::new(vec![
                        Ast::Atom(Atom::new("x")),
                    ])),
                ],
            },
            // action process(input: @status) { emit result }
            Ast::Call {
                name: Atom::new("action"),
                args: vec![
                    Ast::Atom(Atom::new("process")),
                    Ast::Ref(Ref::new("status")),
                    Ast::Body(Body::new(vec![
                        Ast::Atom(Atom::new("result")),
                    ])),
                ],
            },
        ]))
    }

    #[test]
    fn eliminate_dead_removes_unreferenced_types() {
        let ast = decl_fixture();
        let original_count = ast.node_count();
        let simplified = ast.eliminate_dead();
        assert!(simplified.node_count() < original_count,
            "eliminate_dead should remove the orphan type");
        // The orphan type should be gone
        let mut found_orphan = false;
        simplified.walk(&mut |node| {
            if let Ast::Atom(a) = node {
                if a.as_str() == "orphan" { found_orphan = true; }
            }
        });
        assert!(!found_orphan, "orphan type should be eliminated");
        // The status type should remain
        let mut found_status = false;
        simplified.walk(&mut |node| {
            if let Ast::Atom(a) = node {
                if a.as_str() == "status" { found_status = true; }
            }
        });
        assert!(found_status, "status type should be kept (referenced by action)");
    }

    #[test]
    fn eliminate_dead_keeps_transitively_referenced() {
        // type base = { id: id }
        // type wrapper = { inner: @base }
        // action use_it(x: @wrapper) { emit done }
        // base is transitively referenced through wrapper
        let ast = Ast::Body(Body::new(vec![
            Ast::Call {
                name: Atom::new("type"),
                args: vec![
                    Ast::Atom(Atom::new("base")),
                    Ast::Body(Body::new(vec![Ast::Atom(Atom::new("id"))])),
                ],
            },
            Ast::Call {
                name: Atom::new("type"),
                args: vec![
                    Ast::Atom(Atom::new("wrapper")),
                    Ast::Body(Body::new(vec![Ast::Ref(Ref::new("base"))])),
                ],
            },
            Ast::Call {
                name: Atom::new("action"),
                args: vec![
                    Ast::Atom(Atom::new("use_it")),
                    Ast::Ref(Ref::new("wrapper")),
                    Ast::Body(Body::new(vec![Ast::Atom(Atom::new("done"))])),
                ],
            },
        ]));
        let simplified = ast.eliminate_dead();
        // Both base and wrapper should survive
        let mut found_base = false;
        let mut found_wrapper = false;
        simplified.walk(&mut |node| {
            if let Ast::Atom(a) = node {
                if a.as_str() == "base" { found_base = true; }
                if a.as_str() == "wrapper" { found_wrapper = true; }
            }
        });
        assert!(found_base, "base should be kept (transitively referenced)");
        assert!(found_wrapper, "wrapper should be kept (directly referenced)");
    }

    #[test]
    fn collapse_aliases_merges_duplicates() {
        // type status = active | inactive
        // type state = active | inactive   <-- duplicate of status
        // action check(s: @state) { ok }
        let ast = Ast::Body(Body::new(vec![
            Ast::Call {
                name: Atom::new("type"),
                args: vec![
                    Ast::Atom(Atom::new("status")),
                    Ast::Atom(Atom::new("active")),
                    Ast::Atom(Atom::new("inactive")),
                ],
            },
            Ast::Call {
                name: Atom::new("type"),
                args: vec![
                    Ast::Atom(Atom::new("state")),
                    Ast::Atom(Atom::new("active")),
                    Ast::Atom(Atom::new("inactive")),
                ],
            },
            Ast::Call {
                name: Atom::new("action"),
                args: vec![
                    Ast::Atom(Atom::new("check")),
                    Ast::Ref(Ref::new("state")),
                    Ast::Body(Body::new(vec![Ast::Atom(Atom::new("ok"))])),
                ],
            },
        ]));
        let original_count = ast.node_count();
        let simplified = ast.collapse_aliases();
        assert!(simplified.node_count() < original_count,
            "collapse should remove the duplicate type");
        // @state should be replaced with @status in the action
        let mut found_state_ref = false;
        let mut found_status_ref = false;
        simplified.walk(&mut |node| {
            if let Ast::Ref(r) = node {
                if r.as_str() == "state" { found_state_ref = true; }
                if r.as_str() == "status" { found_status_ref = true; }
            }
        });
        assert!(!found_state_ref, "@state ref should be replaced");
        assert!(found_status_ref, "@status ref should appear (substituted)");
    }

    #[test]
    fn flatten_wrappers_inlines_single_field() {
        // type wrapped_id { inner: @id }
        // action get(x: @wrapped_id) { done }
        let ast = Ast::Body(Body::new(vec![
            Ast::Call {
                name: Atom::new("type"),
                args: vec![
                    Ast::Atom(Atom::new("wrapped_id")),
                    Ast::Body(Body::new(vec![
                        Ast::Call {
                            name: Atom::new("inner"),
                            args: vec![Ast::Ref(Ref::new("id"))],
                        },
                    ])),
                ],
            },
            Ast::Call {
                name: Atom::new("action"),
                args: vec![
                    Ast::Atom(Atom::new("get")),
                    Ast::Ref(Ref::new("wrapped_id")),
                    Ast::Body(Body::new(vec![Ast::Atom(Atom::new("done"))])),
                ],
            },
        ]));
        let original_count = ast.node_count();
        let simplified = ast.flatten_wrappers();
        assert!(simplified.node_count() < original_count,
            "flatten should remove the wrapper type");
        // @wrapped_id should be replaced with @id
        let mut found_wrapped = false;
        let mut found_id = false;
        simplified.walk(&mut |node| {
            if let Ast::Ref(r) = node {
                if r.as_str() == "wrapped_id" { found_wrapped = true; }
                if r.as_str() == "id" { found_id = true; }
            }
        });
        assert!(!found_wrapped, "@wrapped_id ref should be inlined");
        assert!(found_id, "@id ref should appear (substituted)");
    }

    #[test]
    fn simplification_pipeline_composes() {
        // Build a fixture with all three anti-patterns:
        // - dead type (orphan)
        // - duplicate alias (state = status)
        // - wrapper type (wrapped_id -> id)
        let ast = Ast::Body(Body::new(vec![
            Ast::Call {
                name: Atom::new("type"),
                args: vec![
                    Ast::Atom(Atom::new("status")),
                    Ast::Atom(Atom::new("active")),
                ],
            },
            Ast::Call {
                name: Atom::new("type"),
                args: vec![
                    Ast::Atom(Atom::new("state")),
                    Ast::Atom(Atom::new("active")),
                ],
            },
            Ast::Call {
                name: Atom::new("type"),
                args: vec![
                    Ast::Atom(Atom::new("orphan")),
                    Ast::Body(Body::new(vec![Ast::Atom(Atom::new("unused"))])),
                ],
            },
            Ast::Call {
                name: Atom::new("type"),
                args: vec![
                    Ast::Atom(Atom::new("wrapped_id")),
                    Ast::Body(Body::new(vec![
                        Ast::Call {
                            name: Atom::new("inner"),
                            args: vec![Ast::Ref(Ref::new("id"))],
                        },
                    ])),
                ],
            },
            Ast::Call {
                name: Atom::new("action"),
                args: vec![
                    Ast::Atom(Atom::new("process")),
                    Ast::Ref(Ref::new("state")),
                    Ast::Ref(Ref::new("wrapped_id")),
                    Ast::Body(Body::new(vec![Ast::Atom(Atom::new("done"))])),
                ],
            },
        ]));
        let original_count = ast.node_count();
        // Order: collapse aliases first (canonical name survives),
        // flatten wrappers (inline indirection),
        // eliminate dead last (remove what's no longer needed).
        let simplified = ast
            .collapse_aliases()
            .flatten_wrappers()
            .eliminate_dead();
        assert!(simplified.node_count() < original_count,
            "pipeline should reduce node count: {} -> {}",
            original_count, simplified.node_count());
        assert!(simplified.depth() <= 4,
            "pipeline should keep depth bounded");
    }

    #[test]
    fn kintsugi_simplification_reduces_complexity() {
        // The kintsugi test: a complex grammar simplified through the pipeline
        // should have fewer nodes and shallower depth.
        let complex = Ast::Body(Body::new(vec![
            // redundant aliases
            Ast::Call { name: Atom::new("type"), args: vec![
                Ast::Atom(Atom::new("status")), Ast::Atom(Atom::new("active")),
            ]},
            Ast::Call { name: Atom::new("type"), args: vec![
                Ast::Atom(Atom::new("state")), Ast::Atom(Atom::new("active")),
            ]},
            Ast::Call { name: Atom::new("type"), args: vec![
                Ast::Atom(Atom::new("condition")), Ast::Atom(Atom::new("active")),
            ]},
            // dead types
            Ast::Call { name: Atom::new("type"), args: vec![
                Ast::Atom(Atom::new("orphan_a")),
                Ast::Body(Body::new(vec![Ast::Atom(Atom::new("x"))])),
            ]},
            Ast::Call { name: Atom::new("type"), args: vec![
                Ast::Atom(Atom::new("orphan_b")),
                Ast::Body(Body::new(vec![Ast::Atom(Atom::new("y"))])),
            ]},
            // action referencing status
            Ast::Call { name: Atom::new("action"), args: vec![
                Ast::Atom(Atom::new("process")),
                Ast::Ref(Ref::new("status")),
                Ast::Body(Body::new(vec![Ast::Atom(Atom::new("ok"))])),
            ]},
        ]));
        let complex_nodes = complex.node_count();
        let simplified = complex
            .collapse_aliases()
            .flatten_wrappers()
            .eliminate_dead();
        let simplified_nodes = simplified.node_count();
        assert!(simplified_nodes < complex_nodes,
            "kintsugi: {} -> {} nodes", complex_nodes, simplified_nodes);
    }

    // -- Phase 5: Serde serialization (shatter feature) --

    #[cfg(feature = "shatter")]
    mod shatter_tests {
        use super::*;

        fn sample_focus() -> Ast {
            Ast::Focus {
                target: Some(Box::new(Ast::Atom(Atom::new("eigenboard")))),
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
                        target: Some(Box::new(Ast::Ref(Ref::new("graph")))),
                        body: Body::new(vec![Ast::Atom(Atom::new("nodes"))]),
                    },
                    Ast::Project {
                        query: Some(Box::new(Ast::Atom(Atom::new("active")))),
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
                    target: Some(Box::new(Ast::Atom(Atom::new("t")))),
                    body: Body::new(vec![]),
                },
                Ast::Project {
                    query: Some(Box::new(Ast::Atom(Atom::new("q")))),
                    body: Body::new(vec![]),
                },
                Ast::Split {
                    root: Some(Box::new(Ast::Atom(Atom::new("r")))),
                    body: Body::new(vec![]),
                },
                Ast::Zoom {
                    perspective: Some(Box::new(Ast::Atom(Atom::new("p")))),
                    body: Body::new(vec![]),
                },
                Ast::Refract {
                    mutation: Some(Box::new(Ast::Atom(Atom::new("m")))),
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
                        target: Some(Box::new(Ast::Atom(Atom::new("fiedler_vector")))),
                        body: Body::new(vec![
                            Ast::Atom(Atom::new("algebraic_connectivity")),
                            Ast::Atom(Atom::new("spectral_gap")),
                            Ast::Atom(Atom::new("laplacian")),
                        ]),
                    },
                    Ast::Project {
                        query: Some(Box::new(Ast::Atom(Atom::new("settlement")))),
                        body: Body::new(vec![
                            Ast::Atom(Atom::new("convergence")),
                            Ast::Atom(Atom::new("holonomy")),
                        ]),
                    },
                    Ast::Refract {
                        mutation: Some(Box::new(Ast::Atom(Atom::new("crystal")))),
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
