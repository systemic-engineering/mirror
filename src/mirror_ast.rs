//! MirrorAST — the typed AST where each variant IS an optic.
//!
//! DeclKind dissolves. MirrorData dissolves. The enum variant IS the kind.
//! No strings for things that have types.
//!
//! - `Identifier` — user-written names (e.g. `color`, `red`, `blue`)
//! - `GrammarRef` — grammar references (e.g. `@test`, `@code/rust`)
//! - `Oid` — content addresses (computed, not written)
//!
//! `String` appears NOWHERE in the AST.

use crate::declaration::{DeclKind, MirrorData};
use crate::kernel::Oid;

// ---------------------------------------------------------------------------
// Identifier — a user-written name. Not a String.
// ---------------------------------------------------------------------------

/// An identifier in mirror source. Not a String. A typed token.
///
/// Examples: `color`, `red`, `status`, `send_email`.
/// These are the names the user wrote in the `.mirror` file.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(s: impl Into<String>) -> Self {
        Identifier(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Content-address this identifier.
    pub fn to_oid(&self) -> Oid {
        Oid::hash(self.0.as_bytes())
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// ---------------------------------------------------------------------------
// GrammarRef — a grammar reference. Always starts with @.
// ---------------------------------------------------------------------------

/// A grammar reference in mirror source. Always starts with `@`.
///
/// Examples: `@test`, `@code/rust`, `@actor`.
/// The `@` prefix is structural — it distinguishes grammar refs from identifiers.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GrammarRef(String);

impl GrammarRef {
    /// Create a new grammar reference. Panics if `s` does not start with `@`.
    pub fn new(s: impl Into<String>) -> Self {
        let s = s.into();
        assert!(s.starts_with('@'), "grammar ref must start with @: {}", s);
        GrammarRef(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Content-address this grammar reference.
    pub fn to_oid(&self) -> Oid {
        Oid::hash(self.0.as_bytes())
    }
}

impl std::fmt::Display for GrammarRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// ---------------------------------------------------------------------------
// TypeBody — what a type declaration contains
// ---------------------------------------------------------------------------

/// The body of a type declaration. Enum, struct, alias, or unit.
#[derive(Clone, Debug, PartialEq)]
pub enum TypeBody {
    /// Variant names: `type color = red | blue`
    Enum(Vec<Identifier>),
    /// Named fields: `type point = { x: int, y: int }`
    Struct(Vec<Field>),
    /// Alias: `type id = string`
    Alias(Identifier),
    /// No body: `type token`
    Unit,
}

/// A named, typed field in a struct or parameter list.
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: Identifier,
    pub type_ref: Identifier,
}

// ---------------------------------------------------------------------------
// MirrorAST — the AST. Each variant IS an optic.
// ---------------------------------------------------------------------------

/// The Mirror AST. Each variant IS an optic.
///
/// DeclKind is gone — the enum variant IS the kind.
/// OpticOp is gone — the five optic variants (Focus/Project/Split/Zoom/Refract)
/// ARE the optic ops.
///
/// No `String` anywhere. `Identifier` for names. `GrammarRef` for `@references`.
#[derive(Clone, Debug, PartialEq)]
pub enum MirrorAST {
    // ----- The five optics -----
    /// `focus` — look closer. Grouping, function call.
    Focus(FocusNode),
    /// `project` — extract a view.
    Project(ProjectNode),
    /// `split` — one of many. Branching.
    Split(SplitNode),
    /// `zoom` — move between levels. Flow, transformation.
    Zoom(ZoomNode),
    /// `refract` — scatter and reconverge. Spread, range, settlement.
    Refract(RefractNode),

    // ----- Declarations -----
    /// `grammar @X { ... }`
    Grammar(GrammarNode),
    /// `type color = red | blue`
    Type(TypeNode),
    /// `action send_email(...) { ... }`
    Action(ActionNode),
    /// `property valid(...) <= verdict { ... }`
    Property(PropertyNode),

    // ----- Structural -----
    /// `in @X` — import a grammar
    Import(ImportNode),
    /// `out X` — export a name
    Export(ExportNode),

    // ----- Meta -----
    /// `abstract` wraps any node
    Abstract(Box<MirrorAST>),

    // ----- Container -----
    /// Top-level module containing multiple declarations
    Module(ModuleNode),
}

// ---------------------------------------------------------------------------
// Node structs — ONLY typed data, NO strings
// ---------------------------------------------------------------------------

/// `grammar @X < @parent { ... }`
#[derive(Clone, Debug, PartialEq)]
pub struct GrammarNode {
    pub name: GrammarRef,
    pub parent: Option<GrammarRef>,
    pub children: Vec<MirrorAST>,
}

/// `type color = red | blue` or `type point = { x: int, y: int }`
#[derive(Clone, Debug, PartialEq)]
pub struct TypeNode {
    pub name: Identifier,
    pub params: Vec<Identifier>,
    pub body: TypeBody,
    pub children: Vec<MirrorAST>,
}

/// `action send_email(to: string) -> result { ... }`
#[derive(Clone, Debug, PartialEq)]
pub struct ActionNode {
    pub name: Identifier,
    pub params: Vec<Field>,
    pub return_type: Option<Identifier>,
    pub grammar_ref: Option<GrammarRef>,
    pub body: Option<Vec<MirrorAST>>,
}

/// `property valid(x: int) <= verdict { ... }`
#[derive(Clone, Debug, PartialEq)]
pub struct PropertyNode {
    pub name: Identifier,
    pub params: Vec<Field>,
    pub fold_target: Option<Identifier>,
    pub body: Vec<MirrorAST>,
}

/// `in @X`
#[derive(Clone, Debug, PartialEq)]
pub struct ImportNode {
    pub target: GrammarRef,
}

/// `out X`
#[derive(Clone, Debug, PartialEq)]
pub struct ExportNode {
    pub name: Identifier,
}

/// `focus X`
#[derive(Clone, Debug, PartialEq)]
pub struct FocusNode {
    pub name: Identifier,
    pub target: Option<Identifier>,
    pub children: Vec<MirrorAST>,
}

/// `project X`
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectNode {
    pub name: Identifier,
    pub target: Option<Identifier>,
    pub children: Vec<MirrorAST>,
}

/// `split X`
#[derive(Clone, Debug, PartialEq)]
pub struct SplitNode {
    pub name: Identifier,
    pub variants: Vec<Identifier>,
    pub children: Vec<MirrorAST>,
}

/// `zoom X`
#[derive(Clone, Debug, PartialEq)]
pub struct ZoomNode {
    pub name: Identifier,
    pub target: Option<Identifier>,
    pub children: Vec<MirrorAST>,
}

/// `refract X`
#[derive(Clone, Debug, PartialEq)]
pub struct RefractNode {
    pub name: Identifier,
    pub target: Option<Identifier>,
    pub children: Vec<MirrorAST>,
}

/// Top-level module.
#[derive(Clone, Debug, PartialEq)]
pub struct ModuleNode {
    pub name: Identifier,
    pub children: Vec<MirrorAST>,
}

// ---------------------------------------------------------------------------
// Content addressing for MirrorAST
// ---------------------------------------------------------------------------

/// Hash helper: CoincidenceHash<3> of tagged content → kernel Oid.
fn hash_tagged(tag: &str, content: &[u8]) -> Oid {
    let mut buf = Vec::with_capacity(tag.len() + 1 + content.len());
    buf.extend_from_slice(tag.as_bytes());
    buf.push(b':');
    buf.extend_from_slice(content);
    Oid::hash(&buf)
}

impl MirrorAST {
    /// Content-address this AST node using CoincidenceHash<3>.
    /// Returns the kernel Oid (mirror's native content address).
    pub fn content_oid(&self) -> Oid {
        match self {
            MirrorAST::Grammar(g) => {
                let mut buf = Vec::new();
                buf.extend_from_slice(g.name.as_str().as_bytes());
                if let Some(ref p) = g.parent {
                    buf.extend_from_slice(b"<");
                    buf.extend_from_slice(p.as_str().as_bytes());
                }
                for child in &g.children {
                    buf.extend_from_slice(b":");
                    buf.extend_from_slice(child.content_oid().as_ref().as_bytes());
                }
                hash_tagged("grammar", &buf)
            }
            MirrorAST::Type(t) => {
                let mut buf = Vec::new();
                buf.extend_from_slice(t.name.as_str().as_bytes());
                for p in &t.params {
                    buf.extend_from_slice(b",");
                    buf.extend_from_slice(p.as_str().as_bytes());
                }
                buf.extend_from_slice(b"=");
                match &t.body {
                    TypeBody::Enum(variants) => {
                        buf.extend_from_slice(b"enum:");
                        for (i, v) in variants.iter().enumerate() {
                            if i > 0 {
                                buf.push(b'|');
                            }
                            buf.extend_from_slice(v.as_str().as_bytes());
                        }
                    }
                    TypeBody::Struct(fields) => {
                        buf.extend_from_slice(b"struct:");
                        for (i, f) in fields.iter().enumerate() {
                            if i > 0 {
                                buf.push(b',');
                            }
                            buf.extend_from_slice(f.name.as_str().as_bytes());
                            buf.push(b':');
                            buf.extend_from_slice(f.type_ref.as_str().as_bytes());
                        }
                    }
                    TypeBody::Alias(a) => {
                        buf.extend_from_slice(b"alias:");
                        buf.extend_from_slice(a.as_str().as_bytes());
                    }
                    TypeBody::Unit => {
                        buf.extend_from_slice(b"unit");
                    }
                }
                for child in &t.children {
                    buf.extend_from_slice(b":");
                    buf.extend_from_slice(child.content_oid().as_ref().as_bytes());
                }
                hash_tagged("type", &buf)
            }
            MirrorAST::Action(a) => {
                let mut buf = Vec::new();
                buf.extend_from_slice(a.name.as_str().as_bytes());
                for p in &a.params {
                    buf.extend_from_slice(b",");
                    buf.extend_from_slice(p.name.as_str().as_bytes());
                    buf.push(b':');
                    buf.extend_from_slice(p.type_ref.as_str().as_bytes());
                }
                if let Some(ref rt) = a.return_type {
                    buf.extend_from_slice(b"->");
                    buf.extend_from_slice(rt.as_str().as_bytes());
                }
                if let Some(ref gr) = a.grammar_ref {
                    buf.extend_from_slice(b"@");
                    buf.extend_from_slice(gr.as_str().as_bytes());
                }
                if let Some(ref body) = a.body {
                    for child in body {
                        buf.extend_from_slice(b":");
                        buf.extend_from_slice(child.content_oid().as_ref().as_bytes());
                    }
                }
                hash_tagged("action", &buf)
            }
            MirrorAST::Property(p) => {
                let mut buf = Vec::new();
                buf.extend_from_slice(p.name.as_str().as_bytes());
                for param in &p.params {
                    buf.extend_from_slice(b",");
                    buf.extend_from_slice(param.name.as_str().as_bytes());
                    buf.push(b':');
                    buf.extend_from_slice(param.type_ref.as_str().as_bytes());
                }
                if let Some(ref ft) = p.fold_target {
                    buf.extend_from_slice(b"<=");
                    buf.extend_from_slice(ft.as_str().as_bytes());
                }
                for child in &p.body {
                    buf.extend_from_slice(b":");
                    buf.extend_from_slice(child.content_oid().as_ref().as_bytes());
                }
                hash_tagged("property", &buf)
            }
            MirrorAST::Import(i) => {
                hash_tagged("import", i.target.as_str().as_bytes())
            }
            MirrorAST::Export(e) => {
                hash_tagged("export", e.name.as_str().as_bytes())
            }
            MirrorAST::Focus(f) => {
                let mut buf = Vec::new();
                buf.extend_from_slice(f.name.as_str().as_bytes());
                if let Some(ref t) = f.target {
                    buf.extend_from_slice(b"->");
                    buf.extend_from_slice(t.as_str().as_bytes());
                }
                for child in &f.children {
                    buf.extend_from_slice(b":");
                    buf.extend_from_slice(child.content_oid().as_ref().as_bytes());
                }
                hash_tagged("focus", &buf)
            }
            MirrorAST::Project(p) => {
                let mut buf = Vec::new();
                buf.extend_from_slice(p.name.as_str().as_bytes());
                if let Some(ref t) = p.target {
                    buf.extend_from_slice(b"->");
                    buf.extend_from_slice(t.as_str().as_bytes());
                }
                for child in &p.children {
                    buf.extend_from_slice(b":");
                    buf.extend_from_slice(child.content_oid().as_ref().as_bytes());
                }
                hash_tagged("project", &buf)
            }
            MirrorAST::Split(s) => {
                let mut buf = Vec::new();
                buf.extend_from_slice(s.name.as_str().as_bytes());
                for v in &s.variants {
                    buf.push(b'|');
                    buf.extend_from_slice(v.as_str().as_bytes());
                }
                for child in &s.children {
                    buf.extend_from_slice(b":");
                    buf.extend_from_slice(child.content_oid().as_ref().as_bytes());
                }
                hash_tagged("split", &buf)
            }
            MirrorAST::Zoom(z) => {
                let mut buf = Vec::new();
                buf.extend_from_slice(z.name.as_str().as_bytes());
                if let Some(ref t) = z.target {
                    buf.extend_from_slice(b"->");
                    buf.extend_from_slice(t.as_str().as_bytes());
                }
                for child in &z.children {
                    buf.extend_from_slice(b":");
                    buf.extend_from_slice(child.content_oid().as_ref().as_bytes());
                }
                hash_tagged("zoom", &buf)
            }
            MirrorAST::Refract(r) => {
                let mut buf = Vec::new();
                buf.extend_from_slice(r.name.as_str().as_bytes());
                if let Some(ref t) = r.target {
                    buf.extend_from_slice(b"->");
                    buf.extend_from_slice(t.as_str().as_bytes());
                }
                for child in &r.children {
                    buf.extend_from_slice(b":");
                    buf.extend_from_slice(child.content_oid().as_ref().as_bytes());
                }
                hash_tagged("refract", &buf)
            }
            MirrorAST::Abstract(inner) => {
                let inner_oid = inner.content_oid();
                hash_tagged("abstract", inner_oid.as_ref().as_bytes())
            }
            MirrorAST::Module(m) => {
                let mut buf = Vec::new();
                buf.extend_from_slice(m.name.as_str().as_bytes());
                for child in &m.children {
                    buf.extend_from_slice(b":");
                    buf.extend_from_slice(child.content_oid().as_ref().as_bytes());
                }
                hash_tagged("module", &buf)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// prism::Addressable — bridge to prism's Oid type for MerkleTree
// ---------------------------------------------------------------------------

impl prism::Addressable for MirrorAST {
    fn oid(&self) -> prism::Oid {
        // Convert kernel Oid → prism Oid (both are String newtypes)
        prism::Oid::new(self.content_oid().as_ref())
    }
}

// ---------------------------------------------------------------------------
// MerkleTree — content-addressed tree traversal
// ---------------------------------------------------------------------------

/// Empty children slice for leaf nodes.
static EMPTY_CHILDREN: &[MirrorAST] = &[];

impl prism::MerkleTree for MirrorAST {
    type Data = Self;

    fn data(&self) -> &Self {
        self
    }

    fn children(&self) -> &[Self] {
        match self {
            MirrorAST::Grammar(g) => &g.children,
            MirrorAST::Type(t) => &t.children,
            MirrorAST::Action(a) => a.body.as_deref().unwrap_or(EMPTY_CHILDREN),
            MirrorAST::Property(p) => &p.body,
            MirrorAST::Module(m) => &m.children,
            MirrorAST::Focus(f) => &f.children,
            MirrorAST::Project(p) => &p.children,
            MirrorAST::Split(s) => &s.children,
            MirrorAST::Zoom(z) => &z.children,
            MirrorAST::Refract(r) => &r.children,
            MirrorAST::Abstract(inner) => inner.children(),
            MirrorAST::Import(_) | MirrorAST::Export(_) => EMPTY_CHILDREN,
        }
    }
}

// ---------------------------------------------------------------------------
// MirrorAST → MirrorData — the parser builds MirrorAST, this converts for
// fragment storage. The canonical direction: AST first, data second.
// ---------------------------------------------------------------------------

impl MirrorAST {
    /// Convert a MirrorAST node into MirrorData for fragment construction.
    ///
    /// This is the canonical direction: the parser builds MirrorAST nodes
    /// directly, then converts to MirrorData for content-addressed storage.
    pub fn to_mirror_data(&self) -> MirrorData {
        match self {
            MirrorAST::Grammar(g) => {
                let mut data = MirrorData::new(
                    DeclKind::Grammar,
                    g.name.as_str(),
                    Vec::new(),
                    Vec::new(),
                );
                data.parent_ref = g.parent.as_ref().map(|p| p.as_str().to_string());
                data
            }
            MirrorAST::Type(t) => {
                let params: Vec<String> = t.params.iter().map(|p| p.as_str().to_string()).collect();
                let variants: Vec<String> = match &t.body {
                    TypeBody::Enum(vs) => vs.iter().map(|v| v.as_str().to_string()).collect(),
                    TypeBody::Struct(fields) => fields
                        .iter()
                        .map(|f| format!("{}:{}", f.name.as_str(), f.type_ref.as_str()))
                        .collect(),
                    TypeBody::Alias(a) => vec![a.as_str().to_string()],
                    TypeBody::Unit => Vec::new(),
                };
                MirrorData::new(DeclKind::Type, t.name.as_str(), params, variants)
            }
            MirrorAST::Action(a) => {
                let params: Vec<String> = a
                    .params
                    .iter()
                    .map(|f| {
                        if f.type_ref.as_str() == "_" {
                            f.name.as_str().to_string()
                        } else {
                            format!("{}:{}", f.name.as_str(), f.type_ref.as_str())
                        }
                    })
                    .collect();
                let mut data = MirrorData::new(DeclKind::Action, a.name.as_str(), params, Vec::new());
                data.return_type = a.return_type.as_ref().map(|rt| rt.as_str().to_string());
                data.grammar_ref = a.grammar_ref.as_ref().map(|gr| gr.as_str().to_string());
                data
            }
            MirrorAST::Property(p) => {
                let params: Vec<String> = p
                    .params
                    .iter()
                    .map(|f| {
                        if f.type_ref.as_str() == "_" {
                            f.name.as_str().to_string()
                        } else {
                            format!("{}:{}", f.name.as_str(), f.type_ref.as_str())
                        }
                    })
                    .collect();
                let mut data = MirrorData::new(DeclKind::Property, p.name.as_str(), params, Vec::new());
                // fold_target is stored as a variant for round-trip
                if let Some(ref ft) = p.fold_target {
                    data.variants.push(ft.as_str().to_string());
                }
                data
            }
            MirrorAST::Import(i) => {
                MirrorData::new(DeclKind::In, i.target.as_str(), Vec::new(), Vec::new())
            }
            MirrorAST::Export(e) => {
                MirrorData::new(DeclKind::Out, e.name.as_str(), Vec::new(), Vec::new())
            }
            MirrorAST::Focus(f) => {
                let params = f.target.as_ref().map(|t| vec![t.as_str().to_string()]).unwrap_or_default();
                MirrorData::new(DeclKind::Focus, f.name.as_str(), params, Vec::new())
            }
            MirrorAST::Project(p) => {
                let params = p.target.as_ref().map(|t| vec![t.as_str().to_string()]).unwrap_or_default();
                MirrorData::new(DeclKind::Project, p.name.as_str(), params, Vec::new())
            }
            MirrorAST::Split(s) => {
                let variants: Vec<String> = s.variants.iter().map(|v| v.as_str().to_string()).collect();
                MirrorData::new(DeclKind::Split, s.name.as_str(), Vec::new(), variants)
            }
            MirrorAST::Zoom(z) => {
                let params = z.target.as_ref().map(|t| vec![t.as_str().to_string()]).unwrap_or_default();
                MirrorData::new(DeclKind::Zoom, z.name.as_str(), params, Vec::new())
            }
            MirrorAST::Refract(r) => {
                let params = r.target.as_ref().map(|t| vec![t.as_str().to_string()]).unwrap_or_default();
                MirrorData::new(DeclKind::Refract, r.name.as_str(), params, Vec::new())
            }
            MirrorAST::Abstract(inner) => {
                let mut data = inner.to_mirror_data();
                data.is_abstract = true;
                data
            }
            MirrorAST::Module(m) => {
                MirrorData::new(DeclKind::Form, m.name.as_str(), Vec::new(), Vec::new())
            }
        }
    }

    /// Build a content-addressed MirrorFragment from this AST node.
    ///
    /// Converts to MirrorData, then wraps in a Fractal with content hash.
    /// Child AST nodes are recursively converted to child fragments.
    pub fn to_fragment(&self) -> crate::declaration::MirrorFragment {
        let mut data = self.to_mirror_data();
        let children: Vec<crate::declaration::MirrorFragment> = match self {
            MirrorAST::Grammar(g) => g.children.iter().map(|c| c.to_fragment()).collect(),
            MirrorAST::Type(t) => t.children.iter().map(|c| c.to_fragment()).collect(),
            MirrorAST::Action(a) => a.body.as_ref()
                .map(|b| b.iter().map(|c| c.to_fragment()).collect())
                .unwrap_or_default(),
            MirrorAST::Property(p) => p.body.iter().map(|c| c.to_fragment()).collect(),
            MirrorAST::Module(m) => m.children.iter().map(|c| c.to_fragment()).collect(),
            MirrorAST::Focus(f) => f.children.iter().map(|c| c.to_fragment()).collect(),
            MirrorAST::Project(p) => p.children.iter().map(|c| c.to_fragment()).collect(),
            MirrorAST::Split(s) => s.children.iter().map(|c| c.to_fragment()).collect(),
            MirrorAST::Zoom(z) => z.children.iter().map(|c| c.to_fragment()).collect(),
            MirrorAST::Refract(r) => r.children.iter().map(|c| c.to_fragment()).collect(),
            MirrorAST::Abstract(inner) => {
                // Abstract wraps another node — the inner node's fragment IS the child
                return inner.to_fragment();
            }
            MirrorAST::Import(_) | MirrorAST::Export(_) => Vec::new(),
        };
        // Preserve optic_ops from the DeclKind
        if let Some(op) = crate::declaration::OpticOp::from_decl_kind(&data.kind) {
            if !data.optic_ops.contains(&op) {
                data.optic_ops.push(op);
            }
        }
        crate::declaration::fragment_encoded(data, children)
    }
}

// ---------------------------------------------------------------------------
// MirrorAST → kind name (for debugging / display)
// ---------------------------------------------------------------------------

impl MirrorAST {
    /// The structural kind name of this AST node.
    pub fn kind_name(&self) -> &'static str {
        match self {
            MirrorAST::Focus(_) => "focus",
            MirrorAST::Project(_) => "project",
            MirrorAST::Split(_) => "split",
            MirrorAST::Zoom(_) => "zoom",
            MirrorAST::Refract(_) => "refract",
            MirrorAST::Grammar(_) => "grammar",
            MirrorAST::Type(_) => "type",
            MirrorAST::Action(_) => "action",
            MirrorAST::Property(_) => "property",
            MirrorAST::Import(_) => "import",
            MirrorAST::Export(_) => "export",
            MirrorAST::Abstract(_) => "abstract",
            MirrorAST::Module(_) => "module",
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use prism::merkle::diff;
    use prism::Addressable as _;
    use prism::MerkleTree as _;

    // -- Identifier tests --

    #[test]
    fn identifier_new_and_as_str() {
        let id = Identifier::new("color");
        assert_eq!(id.as_str(), "color");
    }

    #[test]
    fn identifier_display() {
        let id = Identifier::new("status");
        assert_eq!(format!("{}", id), "status");
    }

    #[test]
    fn identifier_equality() {
        assert_eq!(Identifier::new("x"), Identifier::new("x"));
        assert_ne!(Identifier::new("x"), Identifier::new("y"));
    }

    #[test]
    fn identifier_to_oid_deterministic() {
        let a = Identifier::new("test").to_oid();
        let b = Identifier::new("test").to_oid();
        assert_eq!(a, b);
    }

    #[test]
    fn identifier_to_oid_different_for_different_names() {
        let a = Identifier::new("foo").to_oid();
        let b = Identifier::new("bar").to_oid();
        assert_ne!(a, b);
    }

    #[test]
    fn identifier_hash_and_ord() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Identifier::new("a"));
        set.insert(Identifier::new("a"));
        set.insert(Identifier::new("b"));
        assert_eq!(set.len(), 2);

        assert!(Identifier::new("a") < Identifier::new("b"));
    }

    // -- GrammarRef tests --

    #[test]
    fn grammar_ref_new_and_as_str() {
        let gr = GrammarRef::new("@test");
        assert_eq!(gr.as_str(), "@test");
    }

    #[test]
    fn grammar_ref_display() {
        let gr = GrammarRef::new("@code/rust");
        assert_eq!(format!("{}", gr), "@code/rust");
    }

    #[test]
    #[should_panic(expected = "grammar ref must start with @")]
    fn grammar_ref_panics_without_at() {
        GrammarRef::new("test");
    }

    #[test]
    fn grammar_ref_to_oid_deterministic() {
        let a = GrammarRef::new("@test").to_oid();
        let b = GrammarRef::new("@test").to_oid();
        assert_eq!(a, b);
    }

    #[test]
    fn grammar_ref_to_oid_different_for_different_refs() {
        let a = GrammarRef::new("@test").to_oid();
        let b = GrammarRef::new("@code").to_oid();
        assert_ne!(a, b);
    }

    #[test]
    fn grammar_ref_hash_and_ord() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(GrammarRef::new("@a"));
        set.insert(GrammarRef::new("@a"));
        set.insert(GrammarRef::new("@b"));
        assert_eq!(set.len(), 2);

        assert!(GrammarRef::new("@a") < GrammarRef::new("@b"));
    }

    // -- MirrorAST construction tests --

    #[test]
    fn grammar_node_construction() {
        let node = MirrorAST::Grammar(GrammarNode {
            name: GrammarRef::new("@test"),
            parent: None,
            children: vec![],
        });
        assert_eq!(node.kind_name(), "grammar");
    }

    #[test]
    fn type_node_enum_body() {
        let node = MirrorAST::Type(TypeNode {
            name: Identifier::new("color"),
            params: vec![],
            body: TypeBody::Enum(vec![
                Identifier::new("red"),
                Identifier::new("blue"),
            ]),
            children: vec![],
        });
        assert_eq!(node.kind_name(), "type");
    }

    #[test]
    fn type_node_struct_body() {
        let node = MirrorAST::Type(TypeNode {
            name: Identifier::new("point"),
            params: vec![],
            body: TypeBody::Struct(vec![
                Field {
                    name: Identifier::new("x"),
                    type_ref: Identifier::new("int"),
                },
                Field {
                    name: Identifier::new("y"),
                    type_ref: Identifier::new("int"),
                },
            ]),
            children: vec![],
        });
        assert_eq!(node.kind_name(), "type");
    }

    #[test]
    fn type_node_alias_body() {
        let node = MirrorAST::Type(TypeNode {
            name: Identifier::new("id"),
            params: vec![],
            body: TypeBody::Alias(Identifier::new("string")),
            children: vec![],
        });
        assert_eq!(node.kind_name(), "type");
    }

    #[test]
    fn type_node_unit_body() {
        let node = MirrorAST::Type(TypeNode {
            name: Identifier::new("token"),
            params: vec![],
            body: TypeBody::Unit,
            children: vec![],
        });
        assert_eq!(node.kind_name(), "type");
    }

    #[test]
    fn action_node_construction() {
        let node = MirrorAST::Action(ActionNode {
            name: Identifier::new("send"),
            params: vec![Field {
                name: Identifier::new("to"),
                type_ref: Identifier::new("string"),
            }],
            return_type: Some(Identifier::new("result")),
            grammar_ref: Some(GrammarRef::new("@email")),
            body: None,
        });
        assert_eq!(node.kind_name(), "action");
    }

    #[test]
    fn property_node_construction() {
        let node = MirrorAST::Property(PropertyNode {
            name: Identifier::new("valid"),
            params: vec![],
            fold_target: Some(Identifier::new("verdict")),
            body: vec![],
        });
        assert_eq!(node.kind_name(), "property");
    }

    #[test]
    fn import_export_construction() {
        let imp = MirrorAST::Import(ImportNode {
            target: GrammarRef::new("@tools"),
        });
        assert_eq!(imp.kind_name(), "import");

        let exp = MirrorAST::Export(ExportNode {
            name: Identifier::new("send"),
        });
        assert_eq!(exp.kind_name(), "export");
    }

    #[test]
    fn abstract_wraps_any_node() {
        let inner = MirrorAST::Type(TypeNode {
            name: Identifier::new("token"),
            params: vec![],
            body: TypeBody::Unit,
            children: vec![],
        });
        let wrapped = MirrorAST::Abstract(Box::new(inner.clone()));
        assert_eq!(wrapped.kind_name(), "abstract");
        // Abstract delegates children to inner
        assert_eq!(wrapped.children(), inner.children());
    }

    #[test]
    fn module_with_children() {
        let child = MirrorAST::Type(TypeNode {
            name: Identifier::new("id"),
            params: vec![],
            body: TypeBody::Unit,
            children: vec![],
        });
        let module = MirrorAST::Module(ModuleNode {
            name: Identifier::new("test"),
            children: vec![child],
        });
        assert_eq!(module.children().len(), 1);
    }

    // -- Optic node tests --

    #[test]
    fn focus_node() {
        let node = MirrorAST::Focus(FocusNode {
            name: Identifier::new("details"),
            target: Some(Identifier::new("user")),
            children: vec![],
        });
        assert_eq!(node.kind_name(), "focus");
    }

    #[test]
    fn project_node() {
        let node = MirrorAST::Project(ProjectNode {
            name: Identifier::new("summary"),
            target: None,
            children: vec![],
        });
        assert_eq!(node.kind_name(), "project");
    }

    #[test]
    fn split_node() {
        let node = MirrorAST::Split(SplitNode {
            name: Identifier::new("route"),
            variants: vec![
                Identifier::new("left"),
                Identifier::new("right"),
            ],
            children: vec![],
        });
        assert_eq!(node.kind_name(), "split");
    }

    #[test]
    fn zoom_node() {
        let node = MirrorAST::Zoom(ZoomNode {
            name: Identifier::new("transform"),
            target: Some(Identifier::new("output")),
            children: vec![],
        });
        assert_eq!(node.kind_name(), "zoom");
    }

    #[test]
    fn refract_node() {
        let node = MirrorAST::Refract(RefractNode {
            name: Identifier::new("spread"),
            target: None,
            children: vec![],
        });
        assert_eq!(node.kind_name(), "refract");
    }

    // -- Addressable tests: deterministic Oids --

    #[test]
    fn same_content_same_oid() {
        let a = MirrorAST::Type(TypeNode {
            name: Identifier::new("color"),
            params: vec![],
            body: TypeBody::Enum(vec![
                Identifier::new("red"),
                Identifier::new("blue"),
            ]),
            children: vec![],
        });
        let b = MirrorAST::Type(TypeNode {
            name: Identifier::new("color"),
            params: vec![],
            body: TypeBody::Enum(vec![
                Identifier::new("red"),
                Identifier::new("blue"),
            ]),
            children: vec![],
        });
        assert_eq!(a.oid(), b.oid());
    }

    #[test]
    fn different_content_different_oid() {
        let a = MirrorAST::Type(TypeNode {
            name: Identifier::new("color"),
            params: vec![],
            body: TypeBody::Enum(vec![Identifier::new("red")]),
            children: vec![],
        });
        let b = MirrorAST::Type(TypeNode {
            name: Identifier::new("color"),
            params: vec![],
            body: TypeBody::Enum(vec![Identifier::new("blue")]),
            children: vec![],
        });
        assert_ne!(a.oid(), b.oid());
    }

    #[test]
    fn different_kind_different_oid() {
        // Same name, different kind → different Oid (tag separation)
        let a = MirrorAST::Import(ImportNode {
            target: GrammarRef::new("@test"),
        });
        let b = MirrorAST::Grammar(GrammarNode {
            name: GrammarRef::new("@test"),
            parent: None,
            children: vec![],
        });
        assert_ne!(a.oid(), b.oid());
    }

    #[test]
    fn children_affect_oid() {
        let child = MirrorAST::Type(TypeNode {
            name: Identifier::new("id"),
            params: vec![],
            body: TypeBody::Unit,
            children: vec![],
        });
        let a = MirrorAST::Grammar(GrammarNode {
            name: GrammarRef::new("@test"),
            parent: None,
            children: vec![],
        });
        let b = MirrorAST::Grammar(GrammarNode {
            name: GrammarRef::new("@test"),
            parent: None,
            children: vec![child],
        });
        assert_ne!(a.oid(), b.oid());
    }

    #[test]
    fn abstract_oid_differs_from_inner() {
        let inner = MirrorAST::Type(TypeNode {
            name: Identifier::new("x"),
            params: vec![],
            body: TypeBody::Unit,
            children: vec![],
        });
        let wrapped = MirrorAST::Abstract(Box::new(inner.clone()));
        assert_ne!(inner.oid(), wrapped.oid());
    }

    #[test]
    fn grammar_parent_affects_oid() {
        let a = MirrorAST::Grammar(GrammarNode {
            name: GrammarRef::new("@test"),
            parent: None,
            children: vec![],
        });
        let b = MirrorAST::Grammar(GrammarNode {
            name: GrammarRef::new("@test"),
            parent: Some(GrammarRef::new("@parent")),
            children: vec![],
        });
        assert_ne!(a.oid(), b.oid());
    }

    #[test]
    fn action_return_type_affects_oid() {
        let a = MirrorAST::Action(ActionNode {
            name: Identifier::new("send"),
            params: vec![],
            return_type: None,
            grammar_ref: None,
            body: None,
        });
        let b = MirrorAST::Action(ActionNode {
            name: Identifier::new("send"),
            params: vec![],
            return_type: Some(Identifier::new("result")),
            grammar_ref: None,
            body: None,
        });
        assert_ne!(a.oid(), b.oid());
    }

    #[test]
    fn property_fold_target_affects_oid() {
        let a = MirrorAST::Property(PropertyNode {
            name: Identifier::new("valid"),
            params: vec![],
            fold_target: None,
            body: vec![],
        });
        let b = MirrorAST::Property(PropertyNode {
            name: Identifier::new("valid"),
            params: vec![],
            fold_target: Some(Identifier::new("verdict")),
            body: vec![],
        });
        assert_ne!(a.oid(), b.oid());
    }

    // -- MerkleTree trait tests --

    #[test]
    fn leaf_has_no_children() {
        let leaf = MirrorAST::Import(ImportNode {
            target: GrammarRef::new("@x"),
        });
        assert!(leaf.is_leaf());
        assert_eq!(leaf.degree(), 0);
    }

    #[test]
    fn branch_has_children() {
        let child = MirrorAST::Export(ExportNode {
            name: Identifier::new("x"),
        });
        let parent = MirrorAST::Module(ModuleNode {
            name: Identifier::new("mod"),
            children: vec![child],
        });
        assert!(!parent.is_leaf());
        assert_eq!(parent.degree(), 1);
    }

    #[test]
    fn data_returns_self() {
        let node = MirrorAST::Export(ExportNode {
            name: Identifier::new("x"),
        });
        assert_eq!(node.data(), &node);
    }

    #[test]
    fn merkle_diff_identical() {
        let a = MirrorAST::Type(TypeNode {
            name: Identifier::new("x"),
            params: vec![],
            body: TypeBody::Unit,
            children: vec![],
        });
        let b = a.clone();
        assert!(diff(&a, &b).is_empty());
    }

    #[test]
    fn merkle_diff_different() {
        let a = MirrorAST::Type(TypeNode {
            name: Identifier::new("x"),
            params: vec![],
            body: TypeBody::Unit,
            children: vec![],
        });
        let b = MirrorAST::Type(TypeNode {
            name: Identifier::new("y"),
            params: vec![],
            body: TypeBody::Unit,
            children: vec![],
        });
        let d = diff(&a, &b);
        assert!(!d.is_empty());
    }

    // -- to_mirror_data tests: MirrorAST → MirrorData --

    #[test]
    fn to_data_grammar() {
        let ast = MirrorAST::Grammar(GrammarNode {
            name: GrammarRef::new("@test"),
            parent: None,
            children: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Grammar);
        assert_eq!(data.name, "@test");
        assert!(data.parent_ref.is_none());
    }

    #[test]
    fn to_data_grammar_with_parent() {
        let ast = MirrorAST::Grammar(GrammarNode {
            name: GrammarRef::new("@test"),
            parent: Some(GrammarRef::new("@actor")),
            children: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.parent_ref.as_deref(), Some("@actor"));
    }

    #[test]
    fn to_data_type_enum() {
        let ast = MirrorAST::Type(TypeNode {
            name: Identifier::new("color"),
            params: vec![],
            body: TypeBody::Enum(vec![
                Identifier::new("red"),
                Identifier::new("blue"),
            ]),
            children: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Type);
        assert_eq!(data.name, "color");
        assert_eq!(data.variants, vec!["red", "blue"]);
    }

    #[test]
    fn to_data_type_unit() {
        let ast = MirrorAST::Type(TypeNode {
            name: Identifier::new("token"),
            params: vec![],
            body: TypeBody::Unit,
            children: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Type);
        assert!(data.variants.is_empty());
    }

    #[test]
    fn to_data_type_with_params() {
        let ast = MirrorAST::Type(TypeNode {
            name: Identifier::new("list"),
            params: vec![Identifier::new("t")],
            body: TypeBody::Unit,
            children: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.params, vec!["t"]);
    }

    #[test]
    fn to_data_action() {
        let ast = MirrorAST::Action(ActionNode {
            name: Identifier::new("send"),
            params: vec![Field {
                name: Identifier::new("to"),
                type_ref: Identifier::new("string"),
            }],
            return_type: Some(Identifier::new("result")),
            grammar_ref: Some(GrammarRef::new("@email")),
            body: None,
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Action);
        assert_eq!(data.name, "send");
        assert_eq!(data.params, vec!["to:string"]);
        assert_eq!(data.return_type.as_deref(), Some("result"));
        assert_eq!(data.grammar_ref.as_deref(), Some("@email"));
    }

    #[test]
    fn to_data_action_untyped_param() {
        let ast = MirrorAST::Action(ActionNode {
            name: Identifier::new("run"),
            params: vec![Field {
                name: Identifier::new("cmd"),
                type_ref: Identifier::new("_"),
            }],
            return_type: None,
            grammar_ref: None,
            body: None,
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.params, vec!["cmd"]);
    }

    #[test]
    fn to_data_property() {
        let ast = MirrorAST::Property(PropertyNode {
            name: Identifier::new("valid"),
            params: vec![],
            fold_target: None,
            body: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Property);
        assert_eq!(data.name, "valid");
    }

    #[test]
    fn to_data_import() {
        let ast = MirrorAST::Import(ImportNode {
            target: GrammarRef::new("@tools"),
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::In);
        assert_eq!(data.name, "@tools");
    }

    #[test]
    fn to_data_export() {
        let ast = MirrorAST::Export(ExportNode {
            name: Identifier::new("send"),
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Out);
        assert_eq!(data.name, "send");
    }

    #[test]
    fn to_data_focus() {
        let ast = MirrorAST::Focus(FocusNode {
            name: Identifier::new("details"),
            target: Some(Identifier::new("user")),
            children: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Focus);
        assert_eq!(data.name, "details");
        assert_eq!(data.params, vec!["user"]);
    }

    #[test]
    fn to_data_split() {
        let ast = MirrorAST::Split(SplitNode {
            name: Identifier::new("route"),
            variants: vec![Identifier::new("left"), Identifier::new("right")],
            children: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Split);
        assert_eq!(data.name, "route");
        assert_eq!(data.variants, vec!["left", "right"]);
    }

    #[test]
    fn to_data_zoom() {
        let ast = MirrorAST::Zoom(ZoomNode {
            name: Identifier::new("transform"),
            target: None,
            children: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Zoom);
    }

    #[test]
    fn to_data_refract() {
        let ast = MirrorAST::Refract(RefractNode {
            name: Identifier::new("spread"),
            target: None,
            children: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Refract);
    }

    #[test]
    fn to_data_project() {
        let ast = MirrorAST::Project(ProjectNode {
            name: Identifier::new("summary"),
            target: None,
            children: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Project);
    }

    #[test]
    fn to_data_module() {
        let ast = MirrorAST::Module(ModuleNode {
            name: Identifier::new("@test"),
            children: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Form);
        assert_eq!(data.name, "@test");
    }

    #[test]
    fn to_data_abstract() {
        let inner = MirrorAST::Action(ActionNode {
            name: Identifier::new("retry"),
            params: vec![],
            return_type: None,
            grammar_ref: None,
            body: None,
        });
        let ast = MirrorAST::Abstract(Box::new(inner));
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Action);
        assert!(data.is_abstract);
    }

    #[test]
    fn to_data_property_with_fold_target() {
        let ast = MirrorAST::Property(PropertyNode {
            name: Identifier::new("collapse"),
            params: vec![],
            fold_target: Some(Identifier::new("target")),
            body: vec![],
        });
        let data = ast.to_mirror_data();
        assert_eq!(data.kind, DeclKind::Property);
        assert_eq!(data.variants, vec!["target"]);
    }

    // -- to_fragment round-trip --

    #[test]
    fn to_fragment_grammar() {
        let ast = MirrorAST::Grammar(GrammarNode {
            name: GrammarRef::new("@test"),
            parent: None,
            children: vec![],
        });
        let frag = ast.to_fragment();
        use crate::declaration::MirrorFragmentExt;
        assert_eq!(frag.mirror_data().kind, DeclKind::Grammar);
    }

    #[test]
    fn to_fragment_type_with_children() {
        let child = MirrorAST::Import(ImportNode {
            target: GrammarRef::new("@tools"),
        });
        let ast = MirrorAST::Type(TypeNode {
            name: Identifier::new("color"),
            params: vec![],
            body: TypeBody::Unit,
            children: vec![child],
        });
        let frag = ast.to_fragment();
        use crate::declaration::MirrorFragmentExt;
        assert_eq!(frag.mirror_children().len(), 1);
    }

    // -- Oid tests for all node types --

    #[test]
    fn import_oid_deterministic() {
        let a = MirrorAST::Import(ImportNode {
            target: GrammarRef::new("@x"),
        });
        let b = MirrorAST::Import(ImportNode {
            target: GrammarRef::new("@x"),
        });
        assert_eq!(a.oid(), b.oid());
    }

    #[test]
    fn export_oid_deterministic() {
        let a = MirrorAST::Export(ExportNode {
            name: Identifier::new("x"),
        });
        let b = MirrorAST::Export(ExportNode {
            name: Identifier::new("x"),
        });
        assert_eq!(a.oid(), b.oid());
    }

    #[test]
    fn split_variants_affect_oid() {
        let a = MirrorAST::Split(SplitNode {
            name: Identifier::new("route"),
            variants: vec![Identifier::new("left")],
            children: vec![],
        });
        let b = MirrorAST::Split(SplitNode {
            name: Identifier::new("route"),
            variants: vec![Identifier::new("right")],
            children: vec![],
        });
        assert_ne!(a.oid(), b.oid());
    }

    #[test]
    fn module_oid_deterministic() {
        let a = MirrorAST::Module(ModuleNode {
            name: Identifier::new("mod"),
            children: vec![],
        });
        let b = MirrorAST::Module(ModuleNode {
            name: Identifier::new("mod"),
            children: vec![],
        });
        assert_eq!(a.oid(), b.oid());
    }

    #[test]
    fn action_grammar_ref_affects_oid() {
        let a = MirrorAST::Action(ActionNode {
            name: Identifier::new("run"),
            params: vec![],
            return_type: None,
            grammar_ref: None,
            body: None,
        });
        let b = MirrorAST::Action(ActionNode {
            name: Identifier::new("run"),
            params: vec![],
            return_type: None,
            grammar_ref: Some(GrammarRef::new("@tools")),
            body: None,
        });
        assert_ne!(a.oid(), b.oid());
    }

    #[test]
    fn action_body_affects_oid() {
        let child = MirrorAST::Export(ExportNode {
            name: Identifier::new("x"),
        });
        let a = MirrorAST::Action(ActionNode {
            name: Identifier::new("run"),
            params: vec![],
            return_type: None,
            grammar_ref: None,
            body: None,
        });
        let b = MirrorAST::Action(ActionNode {
            name: Identifier::new("run"),
            params: vec![],
            return_type: None,
            grammar_ref: None,
            body: Some(vec![child]),
        });
        assert_ne!(a.oid(), b.oid());
    }

    #[test]
    fn type_params_affect_oid() {
        let a = MirrorAST::Type(TypeNode {
            name: Identifier::new("list"),
            params: vec![],
            body: TypeBody::Unit,
            children: vec![],
        });
        let b = MirrorAST::Type(TypeNode {
            name: Identifier::new("list"),
            params: vec![Identifier::new("t")],
            body: TypeBody::Unit,
            children: vec![],
        });
        assert_ne!(a.oid(), b.oid());
    }

    #[test]
    fn type_struct_body_oid() {
        let a = MirrorAST::Type(TypeNode {
            name: Identifier::new("point"),
            params: vec![],
            body: TypeBody::Struct(vec![
                Field { name: Identifier::new("x"), type_ref: Identifier::new("int") },
            ]),
            children: vec![],
        });
        let b = MirrorAST::Type(TypeNode {
            name: Identifier::new("point"),
            params: vec![],
            body: TypeBody::Struct(vec![
                Field { name: Identifier::new("x"), type_ref: Identifier::new("int") },
            ]),
            children: vec![],
        });
        assert_eq!(a.oid(), b.oid());

        let c = MirrorAST::Type(TypeNode {
            name: Identifier::new("point"),
            params: vec![],
            body: TypeBody::Struct(vec![
                Field { name: Identifier::new("x"), type_ref: Identifier::new("float") },
            ]),
            children: vec![],
        });
        assert_ne!(a.oid(), c.oid());
    }

    #[test]
    fn type_alias_body_oid() {
        let a = MirrorAST::Type(TypeNode {
            name: Identifier::new("id"),
            params: vec![],
            body: TypeBody::Alias(Identifier::new("string")),
            children: vec![],
        });
        let b = MirrorAST::Type(TypeNode {
            name: Identifier::new("id"),
            params: vec![],
            body: TypeBody::Alias(Identifier::new("int")),
            children: vec![],
        });
        assert_ne!(a.oid(), b.oid());
    }

    // -- MerkleTree children for all node types --

    #[test]
    fn grammar_children() {
        let child = MirrorAST::Import(ImportNode {
            target: GrammarRef::new("@x"),
        });
        let g = MirrorAST::Grammar(GrammarNode {
            name: GrammarRef::new("@test"),
            parent: None,
            children: vec![child.clone()],
        });
        assert_eq!(g.children(), &[child]);
    }

    #[test]
    fn type_children() {
        let child = MirrorAST::Property(PropertyNode {
            name: Identifier::new("valid"),
            params: vec![],
            fold_target: None,
            body: vec![],
        });
        let t = MirrorAST::Type(TypeNode {
            name: Identifier::new("x"),
            params: vec![],
            body: TypeBody::Unit,
            children: vec![child.clone()],
        });
        assert_eq!(t.children(), &[child]);
    }

    #[test]
    fn action_body_as_children() {
        let child = MirrorAST::Export(ExportNode {
            name: Identifier::new("x"),
        });
        let a = MirrorAST::Action(ActionNode {
            name: Identifier::new("run"),
            params: vec![],
            return_type: None,
            grammar_ref: None,
            body: Some(vec![child.clone()]),
        });
        assert_eq!(a.children(), &[child]);

        let no_body = MirrorAST::Action(ActionNode {
            name: Identifier::new("run"),
            params: vec![],
            return_type: None,
            grammar_ref: None,
            body: None,
        });
        assert!(no_body.children().is_empty());
    }

    #[test]
    fn property_body_as_children() {
        let child = MirrorAST::Export(ExportNode {
            name: Identifier::new("x"),
        });
        let p = MirrorAST::Property(PropertyNode {
            name: Identifier::new("valid"),
            params: vec![],
            fold_target: None,
            body: vec![child.clone()],
        });
        assert_eq!(p.children(), &[child]);
    }

    #[test]
    fn optic_nodes_children() {
        let child = MirrorAST::Export(ExportNode {
            name: Identifier::new("x"),
        });

        let focus = MirrorAST::Focus(FocusNode {
            name: Identifier::new("f"),
            target: None,
            children: vec![child.clone()],
        });
        assert_eq!(focus.children(), &[child.clone()]);

        let project = MirrorAST::Project(ProjectNode {
            name: Identifier::new("p"),
            target: None,
            children: vec![child.clone()],
        });
        assert_eq!(project.children(), &[child.clone()]);

        let split = MirrorAST::Split(SplitNode {
            name: Identifier::new("s"),
            variants: vec![],
            children: vec![child.clone()],
        });
        assert_eq!(split.children(), &[child.clone()]);

        let zoom = MirrorAST::Zoom(ZoomNode {
            name: Identifier::new("z"),
            target: None,
            children: vec![child.clone()],
        });
        assert_eq!(zoom.children(), &[child.clone()]);

        let refract = MirrorAST::Refract(RefractNode {
            name: Identifier::new("r"),
            target: None,
            children: vec![child.clone()],
        });
        assert_eq!(refract.children(), &[child]);
    }

    #[test]
    fn leaf_nodes_have_empty_children() {
        let import = MirrorAST::Import(ImportNode {
            target: GrammarRef::new("@x"),
        });
        assert!(import.children().is_empty());

        let export = MirrorAST::Export(ExportNode {
            name: Identifier::new("x"),
        });
        assert!(export.children().is_empty());
    }
}
