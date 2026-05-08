//! MirrorAST — the typed AST where each variant IS an optic.
//!
//! Seven variants. Five operations + Abstract + Module.
//! Grammar, Type, Action, Property, Import, Export are GONE.
//! They collapse into Focus, Split, Zoom, Refract, Project.
//!
//! - `Identifier` — user-written names (e.g. `color`, `red`, `blue`)
//! - `GrammarRef` — grammar references (e.g. `@test`, `@code/rust`)
//! - `Oid` — content addresses (computed, not written)
//!
//! `String` appears NOWHERE in the AST.

use crate::kernel::Oid;
use fragmentation::encoding::{Decode, Encode};

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
// MirrorAST — the AST. Seven variants. Five operations.
// ---------------------------------------------------------------------------

/// The Mirror AST. Seven variants. Five operations + Abstract + Module.
///
/// Grammar → Focus (focus on a namespace)
/// Type → Split (split into variants)
/// Action → Zoom (transform, cross levels)
/// Property → Refract (scatter, verify, settle)
/// Import/Export → Project (extract view)
///
/// No `String` anywhere. `Identifier` for names. `GrammarRef` for `@references`.
#[derive(Clone, Debug, PartialEq)]
pub enum MirrorAST {
    /// `focus` — look closer. Grammar, namespace, grouping.
    /// Was: Focus + Grammar
    Focus(FocusNode),
    /// `project` — extract a view. Import, export.
    /// Was: Project + Import + Export
    Project(ProjectNode),
    /// `split` — one of many. Type with variants.
    /// Was: Split + Type
    Split(SplitNode),
    /// `zoom` — move between levels. Action, io, transformation.
    /// Was: Zoom + Action
    Zoom(ZoomNode),
    /// `refract` — scatter and reconverge. Property, settlement.
    /// Was: Refract + Property
    Refract(RefractNode),
    /// `abstract` wraps any node
    Abstract(Box<MirrorAST>),
    /// Top-level module containing multiple declarations
    Module(ModuleNode),
}

// ---------------------------------------------------------------------------
// Node structs — carry what the deleted variants carried
// ---------------------------------------------------------------------------

/// `grammar @X < @parent { ... }` or `focus X` or `form @X { ... }`
///
/// A Focus that's a grammar has `grammar_ref` (the @name) and optionally `parent`.
/// A Focus that's just grouping has only `name`.
#[derive(Clone, Debug, PartialEq)]
pub struct FocusNode {
    pub name: Identifier,
    pub target: Option<GrammarRef>,  // parent grammar ref (was: parent in GrammarNode)
    pub children: Vec<MirrorAST>,
}

/// `in @X` or `out X` or `project X`
///
/// A Project with `target` is an import. A Project without is an export/view.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectNode {
    pub name: Identifier,
    pub target: Option<GrammarRef>,  // import target (was: target in ImportNode)
    pub children: Vec<MirrorAST>,
}

/// `type color = red | blue` or `split X`
///
/// A Split with `params` and `body` is a type declaration.
/// A Split with just `variants` is a simple split.
#[derive(Clone, Debug, PartialEq)]
pub struct SplitNode {
    pub name: Identifier,
    pub variants: Vec<Identifier>,
    pub params: Vec<Identifier>,     // type params
    pub body: Option<TypeBody>,      // type body
    pub children: Vec<MirrorAST>,
}

/// `action send_email(to: string) -> result { ... }` or `zoom X`
///
/// A Zoom with `params` and `body` is an action/template/io.
/// A Zoom with just `target` is a simple transformation.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoomNode {
    pub name: Identifier,
    pub target: Option<Identifier>,  // return type
    pub params: Vec<Field>,          // action params
    pub grammar_ref: Option<GrammarRef>,
    pub children: Vec<MirrorAST>,
    pub body: Option<Vec<MirrorAST>>,
}

/// `property valid(x: int) <= verdict { ... }` or `refract X`
///
/// A Refract with `params` is a property/invariant/requires.
/// A Refract with just `target` is a simple refraction.
#[derive(Clone, Debug, PartialEq)]
pub struct RefractNode {
    pub name: Identifier,
    pub target: Option<Identifier>,  // fold target
    pub params: Vec<Field>,          // property params
    pub children: Vec<MirrorAST>,
}

/// Top-level module.
#[derive(Clone, Debug, PartialEq)]
pub struct ModuleNode {
    pub name: Identifier,
    pub children: Vec<MirrorAST>,
}

// ---------------------------------------------------------------------------
// Encode/Decode for MirrorAST — deterministic serialization
// ---------------------------------------------------------------------------

impl Encode for MirrorAST {
    fn encode(&self) -> Vec<u8> {
        match self {
            MirrorAST::Focus(f) => {
                let mut s = format!("focus:{}", f.name.as_str());
                if let Some(ref t) = f.target {
                    s.push_str(&format!(":{}", t.as_str()));
                }
                s.into_bytes()
            }
            MirrorAST::Project(p) => {
                let mut s = format!("project:{}", p.name.as_str());
                if let Some(ref t) = p.target {
                    s.push_str(&format!(":{}", t.as_str()));
                }
                s.into_bytes()
            }
            MirrorAST::Split(sp) => {
                let mut s = format!("split:{}", sp.name.as_str());
                if !sp.params.is_empty() {
                    let ps: Vec<&str> = sp.params.iter().map(|p| p.as_str()).collect();
                    s.push_str(&format!("({})", ps.join(",")));
                }
                if !sp.variants.is_empty() {
                    let vs: Vec<&str> = sp.variants.iter().map(|v| v.as_str()).collect();
                    s.push_str(&format!("={}", vs.join("|")));
                }
                if let Some(ref body) = sp.body {
                    match body {
                        TypeBody::Enum(vs) => {
                            let vs: Vec<&str> = vs.iter().map(|v| v.as_str()).collect();
                            s.push_str(&format!(":enum:{}", vs.join("|")));
                        }
                        TypeBody::Struct(fs) => {
                            let fs: Vec<String> = fs.iter().map(|f| format!("{}:{}", f.name.as_str(), f.type_ref.as_str())).collect();
                            s.push_str(&format!(":struct:{}", fs.join(",")));
                        }
                        TypeBody::Alias(a) => {
                            s.push_str(&format!(":alias:{}", a.as_str()));
                        }
                        TypeBody::Unit => {
                            s.push_str(":unit");
                        }
                    }
                }
                s.into_bytes()
            }
            MirrorAST::Zoom(z) => {
                let mut s = format!("zoom:{}", z.name.as_str());
                if !z.params.is_empty() {
                    let ps: Vec<String> = z.params.iter().map(|f| format!("{}:{}", f.name.as_str(), f.type_ref.as_str())).collect();
                    s.push_str(&format!("({})", ps.join(",")));
                }
                if let Some(ref t) = z.target {
                    s.push_str(&format!("->{}", t.as_str()));
                }
                if let Some(ref gr) = z.grammar_ref {
                    s.push_str(&format!("@{}", gr.as_str()));
                }
                s.into_bytes()
            }
            MirrorAST::Refract(r) => {
                let mut s = format!("refract:{}", r.name.as_str());
                if !r.params.is_empty() {
                    let ps: Vec<String> = r.params.iter().map(|f| format!("{}:{}", f.name.as_str(), f.type_ref.as_str())).collect();
                    s.push_str(&format!("({})", ps.join(",")));
                }
                if let Some(ref t) = r.target {
                    s.push_str(&format!("<={}", t.as_str()));
                }
                s.into_bytes()
            }
            MirrorAST::Abstract(inner) => {
                let mut s = b"abstract:".to_vec();
                s.extend_from_slice(&inner.encode());
                s
            }
            MirrorAST::Module(m) => {
                format!("module:{}", m.name.as_str()).into_bytes()
            }
        }
    }
}

impl Decode for MirrorAST {
    type Error = String;

    fn decode(data: &[u8]) -> Result<Self, Self::Error> {
        let s = String::from_utf8_lossy(data);
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        let tag = parts[0];
        let rest = if parts.len() > 1 { parts[1] } else { "" };

        Ok(match tag {
            "focus" => {
                let name_parts: Vec<&str> = rest.splitn(2, ':').collect();
                let name = name_parts[0];
                let target = name_parts.get(1).and_then(|t| {
                    if t.starts_with('@') { Some(GrammarRef::new(*t)) } else { None }
                });
                MirrorAST::Focus(FocusNode {
                    name: Identifier::new(name),
                    target,
                    children: vec![],
                })
            }
            "project" => {
                let name_parts: Vec<&str> = rest.splitn(2, ':').collect();
                let name = name_parts[0];
                let target = name_parts.get(1).and_then(|t| {
                    if t.starts_with('@') { Some(GrammarRef::new(*t)) } else { None }
                });
                MirrorAST::Project(ProjectNode {
                    name: Identifier::new(name),
                    target,
                    children: vec![],
                })
            }
            "split" => {
                let name = rest.split(['(', '=', ':']).next().unwrap_or(rest);
                MirrorAST::Split(SplitNode {
                    name: Identifier::new(name),
                    variants: vec![],
                    params: vec![],
                    body: None,
                    children: vec![],
                })
            }
            "zoom" => {
                let name = rest.split(['(', '-', '@']).next().unwrap_or(rest);
                MirrorAST::Zoom(ZoomNode {
                    name: Identifier::new(name),
                    target: None,
                    params: vec![],
                    grammar_ref: None,
                    children: vec![],
                    body: None,
                })
            }
            "refract" => {
                let name = rest.split(['(', '<']).next().unwrap_or(rest);
                MirrorAST::Refract(RefractNode {
                    name: Identifier::new(name),
                    target: None,
                    params: vec![],
                    children: vec![],
                })
            }
            "abstract" => {
                let inner = MirrorAST::decode(rest.as_bytes())?;
                MirrorAST::Abstract(Box::new(inner))
            }
            "module" => {
                MirrorAST::Module(ModuleNode {
                    name: Identifier::new(rest),
                    children: vec![],
                })
            }
            _ => {
                // Fallback: treat as focus
                MirrorAST::Focus(FocusNode {
                    name: Identifier::new(&*s),
                    target: None,
                    children: vec![],
                })
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Content addressing for MirrorAST
// ---------------------------------------------------------------------------

/// Hash helper: CoincidenceHash<3> of tagged content -> kernel Oid.
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
            MirrorAST::Focus(f) => {
                let mut buf = Vec::new();
                buf.extend_from_slice(f.name.as_str().as_bytes());
                if let Some(ref t) = f.target {
                    buf.extend_from_slice(b"<");
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
                for p in &s.params {
                    buf.extend_from_slice(b",");
                    buf.extend_from_slice(p.as_str().as_bytes());
                }
                if !s.variants.is_empty() {
                    for v in &s.variants {
                        buf.push(b'|');
                        buf.extend_from_slice(v.as_str().as_bytes());
                    }
                }
                if let Some(ref body) = s.body {
                    buf.extend_from_slice(b"=");
                    match body {
                        TypeBody::Enum(variants) => {
                            buf.extend_from_slice(b"enum:");
                            for (i, v) in variants.iter().enumerate() {
                                if i > 0 { buf.push(b'|'); }
                                buf.extend_from_slice(v.as_str().as_bytes());
                            }
                        }
                        TypeBody::Struct(fields) => {
                            buf.extend_from_slice(b"struct:");
                            for (i, f) in fields.iter().enumerate() {
                                if i > 0 { buf.push(b','); }
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
                for p in &z.params {
                    buf.extend_from_slice(b",");
                    buf.extend_from_slice(p.name.as_str().as_bytes());
                    buf.push(b':');
                    buf.extend_from_slice(p.type_ref.as_str().as_bytes());
                }
                if let Some(ref t) = z.target {
                    buf.extend_from_slice(b"->");
                    buf.extend_from_slice(t.as_str().as_bytes());
                }
                if let Some(ref gr) = z.grammar_ref {
                    buf.extend_from_slice(b"@");
                    buf.extend_from_slice(gr.as_str().as_bytes());
                }
                if let Some(ref body) = z.body {
                    for child in body {
                        buf.extend_from_slice(b":");
                        buf.extend_from_slice(child.content_oid().as_ref().as_bytes());
                    }
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
                for param in &r.params {
                    buf.extend_from_slice(b",");
                    buf.extend_from_slice(param.name.as_str().as_bytes());
                    buf.push(b':');
                    buf.extend_from_slice(param.type_ref.as_str().as_bytes());
                }
                if let Some(ref t) = r.target {
                    buf.extend_from_slice(b"<=");
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
        // Convert kernel Oid -> prism Oid (both are String newtypes)
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
            MirrorAST::Focus(f) => &f.children,
            MirrorAST::Project(p) => &p.children,
            MirrorAST::Split(s) => &s.children,
            MirrorAST::Zoom(z) => {
                if !z.children.is_empty() {
                    &z.children
                } else {
                    z.body.as_deref().unwrap_or(EMPTY_CHILDREN)
                }
            }
            MirrorAST::Refract(r) => &r.children,
            MirrorAST::Abstract(inner) => inner.children(),
            MirrorAST::Module(m) => &m.children,
        }
    }
}

// ---------------------------------------------------------------------------
// MirrorAST -> kind name (for debugging / display)
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
            MirrorAST::Abstract(_) => "abstract",
            MirrorAST::Module(_) => "module",
        }
    }

    /// Is this node a grammar focus (name starts with @)?
    pub fn is_grammar(&self) -> bool {
        match self {
            MirrorAST::Focus(f) => f.name.as_str().starts_with('@'),
            _ => false,
        }
    }

    /// Get the name of this node.
    pub fn name(&self) -> &str {
        match self {
            MirrorAST::Focus(f) => f.name.as_str(),
            MirrorAST::Project(p) => p.name.as_str(),
            MirrorAST::Split(s) => s.name.as_str(),
            MirrorAST::Zoom(z) => z.name.as_str(),
            MirrorAST::Refract(r) => r.name.as_str(),
            MirrorAST::Abstract(inner) => inner.name(),
            MirrorAST::Module(m) => m.name.as_str(),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers — param parsing
// ---------------------------------------------------------------------------

impl MirrorAST {
    /// Rejoin tokenizer-split params: ["to", ":", "string"] -> ["to:string"].
    ///
    /// The mirror tokenizer splits `:` into its own Word token, so typed
    /// params like `(to: string)` become three separate param entries.
    /// This function reassembles them before constructing typed Fields.
    pub fn rejoin_params(raw: &[String]) -> Vec<String> {
        let mut result = Vec::new();
        let mut i = 0;
        while i < raw.len() {
            if i + 2 < raw.len() && raw[i + 1] == ":" {
                // name : type -> "name:type"
                result.push(format!("{}:{}", raw[i], raw[i + 2]));
                i += 3;
            } else if i + 1 < raw.len() && raw[i].ends_with(':') {
                // "name:" "type" -> "name:type"
                result.push(format!("{}{}", raw[i], raw[i + 1]));
                i += 2;
            } else if i + 1 < raw.len() && raw[i + 1].starts_with(':') {
                // "name" ":type" -> "name:type"
                result.push(format!("{}{}", raw[i], raw[i + 1]));
                i += 2;
            } else {
                result.push(raw[i].clone());
                i += 1;
            }
        }
        result
    }

    /// Parse a string param list into typed Fields.
    pub fn params_to_fields(raw: &[String]) -> Vec<Field> {
        let joined = Self::rejoin_params(raw);
        joined
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
            .collect()
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
    fn focus_grammar_construction() {
        // grammar @test -> Focus with @ name
        let node = MirrorAST::Focus(FocusNode {
            name: Identifier::new("@test"),
            target: None,
            children: vec![],
        });
        assert_eq!(node.kind_name(), "focus");
        assert!(node.is_grammar());
    }

    #[test]
    fn split_enum_body() {
        // type color = red | blue -> Split with TypeBody::Enum
        let node = MirrorAST::Split(SplitNode {
            name: Identifier::new("color"),
            variants: vec![],
            params: vec![],
            body: Some(TypeBody::Enum(vec![Identifier::new("red"), Identifier::new("blue")])),
            children: vec![],
        });
        assert_eq!(node.kind_name(), "split");
    }

    #[test]
    fn zoom_action_construction() {
        // action send(to: string) -> result
        let node = MirrorAST::Zoom(ZoomNode {
            name: Identifier::new("send"),
            params: vec![Field {
                name: Identifier::new("to"),
                type_ref: Identifier::new("string"),
            }],
            target: Some(Identifier::new("result")),
            grammar_ref: Some(GrammarRef::new("@email")),
            children: vec![],
            body: None,
        });
        assert_eq!(node.kind_name(), "zoom");
    }

    #[test]
    fn refract_property_construction() {
        // property valid <= verdict
        let node = MirrorAST::Refract(RefractNode {
            name: Identifier::new("valid"),
            params: vec![],
            target: Some(Identifier::new("verdict")),
            children: vec![],
        });
        assert_eq!(node.kind_name(), "refract");
    }

    #[test]
    fn project_import_construction() {
        // in @tools -> Project with target
        let imp = MirrorAST::Project(ProjectNode {
            name: Identifier::new("@tools"),
            target: Some(GrammarRef::new("@tools")),
            children: vec![],
        });
        assert_eq!(imp.kind_name(), "project");

        // out send -> Project without target
        let exp = MirrorAST::Project(ProjectNode {
            name: Identifier::new("send"),
            target: None,
            children: vec![],
        });
        assert_eq!(exp.kind_name(), "project");
    }

    #[test]
    fn abstract_wraps_any_node() {
        let inner = MirrorAST::Split(SplitNode {
            name: Identifier::new("token"),
            variants: vec![],
            params: vec![],
            body: Some(TypeBody::Unit),
            children: vec![],
        });
        let wrapped = MirrorAST::Abstract(Box::new(inner.clone()));
        assert_eq!(wrapped.kind_name(), "abstract");
        assert_eq!(wrapped.children(), inner.children());
    }

    #[test]
    fn module_with_children() {
        let child = MirrorAST::Split(SplitNode {
            name: Identifier::new("id"),
            variants: vec![],
            params: vec![],
            body: Some(TypeBody::Unit),
            children: vec![],
        });
        let module = MirrorAST::Module(ModuleNode {
            name: Identifier::new("test"),
            children: vec![child],
        });
        assert_eq!(module.children().len(), 1);
    }

    // -- Addressable tests: deterministic Oids --

    #[test]
    fn same_content_same_oid() {
        let a = MirrorAST::Split(SplitNode {
            name: Identifier::new("color"),
            variants: vec![],
            params: vec![],
            body: Some(TypeBody::Enum(vec![Identifier::new("red"), Identifier::new("blue")])),
            children: vec![],
        });
        let b = a.clone();
        assert_eq!(a.oid(), b.oid());
    }

    #[test]
    fn different_content_different_oid() {
        let a = MirrorAST::Split(SplitNode {
            name: Identifier::new("color"),
            variants: vec![],
            params: vec![],
            body: Some(TypeBody::Enum(vec![Identifier::new("red")])),
            children: vec![],
        });
        let b = MirrorAST::Split(SplitNode {
            name: Identifier::new("color"),
            variants: vec![],
            params: vec![],
            body: Some(TypeBody::Enum(vec![Identifier::new("blue")])),
            children: vec![],
        });
        assert_ne!(a.oid(), b.oid());
    }

    #[test]
    fn abstract_oid_differs_from_inner() {
        let inner = MirrorAST::Split(SplitNode {
            name: Identifier::new("x"),
            variants: vec![],
            params: vec![],
            body: Some(TypeBody::Unit),
            children: vec![],
        });
        let wrapped = MirrorAST::Abstract(Box::new(inner.clone()));
        assert_ne!(inner.oid(), wrapped.oid());
    }

    // -- MerkleTree tests --

    #[test]
    fn leaf_has_no_children() {
        let leaf = MirrorAST::Project(ProjectNode {
            name: Identifier::new("@x"),
            target: Some(GrammarRef::new("@x")),
            children: vec![],
        });
        assert!(leaf.is_leaf());
        assert_eq!(leaf.degree(), 0);
    }

    #[test]
    fn branch_has_children() {
        let child = MirrorAST::Project(ProjectNode {
            name: Identifier::new("x"),
            target: None,
            children: vec![],
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
        let node = MirrorAST::Project(ProjectNode {
            name: Identifier::new("x"),
            target: None,
            children: vec![],
        });
        assert_eq!(node.data(), &node);
    }

    #[test]
    fn merkle_diff_identical() {
        let a = MirrorAST::Split(SplitNode {
            name: Identifier::new("x"),
            variants: vec![],
            params: vec![],
            body: Some(TypeBody::Unit),
            children: vec![],
        });
        let b = a.clone();
        assert!(diff(&a, &b).is_empty());
    }

    #[test]
    fn merkle_diff_different() {
        let a = MirrorAST::Split(SplitNode {
            name: Identifier::new("x"),
            variants: vec![],
            params: vec![],
            body: Some(TypeBody::Unit),
            children: vec![],
        });
        let b = MirrorAST::Split(SplitNode {
            name: Identifier::new("y"),
            variants: vec![],
            params: vec![],
            body: Some(TypeBody::Unit),
            children: vec![],
        });
        let d = diff(&a, &b);
        assert!(!d.is_empty());
    }

    // -- Encode/Decode roundtrip --

    #[test]
    fn encode_decode_focus() {
        let ast = MirrorAST::Focus(FocusNode {
            name: Identifier::new("@test"),
            target: None,
            children: vec![],
        });
        let encoded = ast.encode();
        let decoded = MirrorAST::decode(&encoded).unwrap();
        assert_eq!(decoded.kind_name(), "focus");
        assert_eq!(decoded.name(), "@test");
    }

    #[test]
    fn encode_decode_module() {
        let ast = MirrorAST::Module(ModuleNode {
            name: Identifier::new("test"),
            children: vec![],
        });
        let encoded = ast.encode();
        let decoded = MirrorAST::decode(&encoded).unwrap();
        assert_eq!(decoded.kind_name(), "module");
        assert_eq!(decoded.name(), "test");
    }
}
