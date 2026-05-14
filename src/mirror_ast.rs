// FROZEN -- see AGENTS.md. Do not modify without explicit approval.
// This file is Rust substrate. All extensions happen through .mirror grammars.
// If you're adding code here, you're probably wrong. Write a grammar instead.

//! MirrorAST -- the typed AST where each variant IS an optic.
//!
//! Seven variants. Five operations + In + Out.
//! Grammar, Type, Action, Property, Import, Export are GONE.
//! They collapse into Focus, Split, Zoom, Refract, Project, In, Out.
//!
//! - `Identifier` -- user-written names (e.g. `color`, `red`, `blue`)
//! - `GrammarOid` -- grammar identity (stores without @, renders with @)
//! - `GrammarRef` -- grammar reference with optional identifier (e.g. `@code/rust`)
//! - `Oid` -- content addresses (computed, not written)
//!
//! `String` appears NOWHERE in the AST.

use crate::kernel::Oid;
use fragmentation::encoding::{Decode, Encode};

// ---------------------------------------------------------------------------
// Identifier -- a user-written name. Not a String.
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
// GrammarOid -- grammar identity. Stores without @, renders with @.
// ---------------------------------------------------------------------------

/// A grammar identity. Stores the name without `@`, renders with `@`.
///
/// Examples: `GrammarOid::new("code")` -> displays as `@code`.
/// `GrammarOid::new("@test")` strips the @ -> stores `test`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GrammarOid(String);

impl GrammarOid {
    pub fn new(s: impl Into<String>) -> Self {
        let s = s.into();
        GrammarOid(s.strip_prefix('@').unwrap_or(&s).to_string())
    }
    pub fn name(&self) -> &str {
        &self.0
    }
    pub fn to_oid(&self) -> Oid {
        Oid::hash(self.0.as_bytes())
    }
}

impl std::fmt::Display for GrammarOid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// GrammarRef -- a grammar reference with optional identifier.
// ---------------------------------------------------------------------------

/// A grammar reference in mirror source. Always renders with `@`.
///
/// Examples: `@test`, `@code/rust`, `@actor`.
/// `GrammarRef { grammar: GrammarOid("code"), identifier: Some("rust") }` -> `@code/rust`
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GrammarRef {
    pub grammar: GrammarOid,
    pub identifier: Option<String>,
}

impl GrammarRef {
    pub fn new(grammar: impl Into<String>, identifier: Option<String>) -> Self {
        GrammarRef {
            grammar: GrammarOid::new(grammar),
            identifier,
        }
    }

    /// Parse "@code/rust" into GrammarRef { grammar: "code", identifier: Some("rust") }
    pub fn parse(s: &str) -> Self {
        let s = s.strip_prefix('@').unwrap_or(s);
        if let Some((g, id)) = s.split_once('/') {
            GrammarRef {
                grammar: GrammarOid::new(g),
                identifier: Some(id.to_string()),
            }
        } else {
            GrammarRef {
                grammar: GrammarOid::new(s),
                identifier: None,
            }
        }
    }

    /// The grammar name without @.
    pub fn grammar_name(&self) -> &str {
        self.grammar.name()
    }

    /// Content-address this grammar reference.
    pub fn to_oid(&self) -> Oid {
        // Hash the full display form for uniqueness
        Oid::hash(self.to_string().as_bytes())
    }

    /// Legacy as_str() -- returns the full @grammar/identifier string.
    /// Used by existing code that expects a string representation.
    pub fn as_str_owned(&self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for GrammarRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grammar)?;
        if let Some(id) = &self.identifier {
            write!(f, "/{}", id)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// TypeBody -- what a type declaration contains
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
// MirrorAST -- the AST. Seven variants. Five operations + In + Out.
// ---------------------------------------------------------------------------

/// The Mirror AST. Seven variants. Five operations + the doors.
///
/// Grammar -> Focus (focus on a namespace)
/// Type -> Split (split into variants)
/// Action -> Zoom (transform, cross levels)
/// Property -> Refract (scatter, verify, settle)
/// Import -> In (the door in)
/// Export -> Out (the door out)
/// View/Filter -> Project (extract view)
///
/// No `String` anywhere. `Identifier` for names. `GrammarRef` for `@references`.
///
/// Module and Abstract are GONE. Focus replaces Module.
/// `\` (the intent hole) is a body value on ZoomNode, not a wrapper.
#[derive(Clone, Debug, PartialEq)]
pub enum MirrorAST {
    /// `focus` -- look closer. Grammar, namespace, grouping.
    /// Was: Focus + Grammar + Module
    Focus(FocusNode),
    /// `project` -- extract a view. Filter, query.
    /// Was: Project
    Project(ProjectNode),
    /// `split` -- one of many. Type with variants.
    /// Was: Split + Type
    Split(SplitNode),
    /// `zoom` -- move between levels. Action, io, transformation.
    /// Was: Zoom + Action
    Zoom(ZoomNode),
    /// `refract` -- scatter and reconverge. Property, settlement.
    /// Was: Refract + Property
    Refract(RefractNode),
    /// `in` -- the door in. Grammar import.
    /// Was: Project with target (import)
    In(GrammarRef),
    /// `out` -- the door out. Grammar export.
    /// Was: Project without target (export)
    Out(GrammarRef),
}

// ---------------------------------------------------------------------------
// Node structs -- carry what the deleted variants carried
// ---------------------------------------------------------------------------

/// `grammar @X < @parent { ... }` or `focus X` or `form @X { ... }`
///
/// A Focus that's a grammar has `grammar_ref` (the @name) and optionally `parent`.
/// A Focus that's just grouping has only `name`.
/// Replaces ModuleNode -- top-level modules are now Focus with name "root".
#[derive(Clone, Debug, PartialEq)]
pub struct FocusNode {
    pub name: Identifier,
    pub target: Option<GrammarRef>, // parent grammar ref (was: parent in GrammarNode)
    pub children: Vec<MirrorAST>,
}

/// `project X` -- extract a view.
///
/// A Project filters or queries. No import/export semantics (those are In/Out).
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectNode {
    pub name: Identifier,
    pub target: Option<GrammarRef>, // optional target ref
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
/// `is_abstract` replaces the old Abstract wrapper -- `\` is a flag, not a wrapper.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoomNode {
    pub name: Identifier,
    pub target: Option<Identifier>,  // return type
    pub params: Vec<Field>,          // action params
    pub grammar_ref: Option<GrammarRef>,
    pub children: Vec<MirrorAST>,
    pub body: Option<Vec<MirrorAST>>,
    pub is_abstract: bool,           // true = \ (intent hole), was Abstract wrapper
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

// ---------------------------------------------------------------------------
// Encode/Decode for MirrorAST -- deterministic serialization
// ---------------------------------------------------------------------------

impl Encode for MirrorAST {
    fn encode(&self) -> Vec<u8> {
        match self {
            MirrorAST::Focus(f) => {
                let mut s = format!("focus:{}", f.name.as_str());
                if let Some(ref t) = f.target {
                    s.push_str(&format!(":{}", t));
                }
                s.into_bytes()
            }
            MirrorAST::Project(p) => {
                let mut s = format!("project:{}", p.name.as_str());
                if let Some(ref t) = p.target {
                    s.push_str(&format!(":{}", t));
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
                    s.push_str(&format!("@{}", gr));
                }
                if z.is_abstract {
                    s.push_str(":abstract");
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
            MirrorAST::In(gr) => {
                format!("in:{}", gr).into_bytes()
            }
            MirrorAST::Out(gr) => {
                format!("out:{}", gr).into_bytes()
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
                    if t.starts_with('@') {
                        Some(GrammarRef::parse(t))
                    } else {
                        None
                    }
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
                    if t.starts_with('@') {
                        Some(GrammarRef::parse(t))
                    } else {
                        None
                    }
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
                let name = rest.split(['(', '-', '@', ':']).next().unwrap_or(rest);
                MirrorAST::Zoom(ZoomNode {
                    name: Identifier::new(name),
                    target: None,
                    params: vec![],
                    grammar_ref: None,
                    children: vec![],
                    body: None,
                    is_abstract: rest.ends_with(":abstract"),
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
            "in" => {
                MirrorAST::In(GrammarRef::parse(rest))
            }
            "out" => {
                MirrorAST::Out(GrammarRef::parse(rest))
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
                    buf.extend_from_slice(t.to_string().as_bytes());
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
                    buf.extend_from_slice(t.to_string().as_bytes());
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
                    buf.extend_from_slice(gr.to_string().as_bytes());
                }
                if z.is_abstract {
                    buf.extend_from_slice(b":abstract");
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
            MirrorAST::In(gr) => {
                hash_tagged("in", gr.to_string().as_bytes())
            }
            MirrorAST::Out(gr) => {
                hash_tagged("out", gr.to_string().as_bytes())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// prism::Addressable -- bridge to prism's Oid type for MerkleTree
// ---------------------------------------------------------------------------

impl prism::Addressable for MirrorAST {
    fn oid(&self) -> prism::Oid {
        // Convert kernel Oid -> prism Oid (both are String newtypes)
        prism::Oid::new(self.content_oid().as_ref())
    }
}

// ---------------------------------------------------------------------------
// MerkleTree -- content-addressed tree traversal
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
            MirrorAST::In(_) => EMPTY_CHILDREN,
            MirrorAST::Out(_) => EMPTY_CHILDREN,
        }
    }
}

// ---------------------------------------------------------------------------
// Display for MirrorAST
// ---------------------------------------------------------------------------

impl std::fmt::Display for MirrorAST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MirrorAST::Focus(node) => write!(f, "focus {}", node.name),
            MirrorAST::Project(node) => write!(f, "project {}", node.name),
            MirrorAST::Split(node) => write!(f, "split {}", node.name),
            MirrorAST::Zoom(node) => write!(f, "zoom {}", node.name),
            MirrorAST::Refract(node) => write!(f, "refract {}", node.name),
            MirrorAST::In(gr) => write!(f, "in {}", gr),
            MirrorAST::Out(gr) => write!(f, "out {}", gr),
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
            MirrorAST::In(_) => "in",
            MirrorAST::Out(_) => "out",
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
            MirrorAST::In(gr) => gr.grammar_name(),
            MirrorAST::Out(gr) => gr.grammar_name(),
        }
    }

    /// The declaration-level tag for this AST node.
    ///
    /// Returns the keyword string the parser originally used to create this node.
    /// Focus(@name) -> "grammar", Focus(name) -> "focus",
    /// Split -> "type", Zoom -> "action", Refract -> "property",
    /// In -> "in", Out -> "out", Project -> "project".
    pub fn decl_tag(&self) -> &'static str {
        match self {
            MirrorAST::Focus(f) => {
                if f.name.as_str().starts_with('@') { "grammar" } else { "focus" }
            }
            MirrorAST::Project(_) => "project",
            MirrorAST::Split(_) => "type",
            MirrorAST::Zoom(_) => "action",
            MirrorAST::Refract(_) => "property",
            MirrorAST::In(_) => "in",
            MirrorAST::Out(_) => "out",
        }
    }

    /// Is this node abstract? (has the \ intent hole)
    pub fn is_abstract(&self) -> bool {
        match self {
            MirrorAST::Zoom(z) => z.is_abstract,
            _ => false,
        }
    }

    /// Stringly-typed params projection (for consumers that need Vec<String>).
    pub fn params_as_strings(&self) -> Vec<String> {
        match self {
            MirrorAST::Split(s) => s.params.iter().map(|p| p.as_str().to_string()).collect(),
            MirrorAST::Zoom(z) => z.params.iter().map(|f| {
                if f.type_ref.as_str() == "_" {
                    f.name.as_str().to_string()
                } else {
                    format!("{}:{}", f.name.as_str(), f.type_ref.as_str())
                }
            }).collect(),
            MirrorAST::Refract(r) => r.params.iter().map(|f| {
                if f.type_ref.as_str() == "_" {
                    f.name.as_str().to_string()
                } else {
                    format!("{}:{}", f.name.as_str(), f.type_ref.as_str())
                }
            }).collect(),
            _ => Vec::new(),
        }
    }

    /// Stringly-typed variants projection (for consumers that need Vec<String>).
    pub fn variants_as_strings(&self) -> Vec<String> {
        match self {
            MirrorAST::Split(s) => {
                if let Some(ref body) = s.body {
                    match body {
                        TypeBody::Enum(vs) => vs.iter().map(|v| v.as_str().to_string()).collect(),
                        TypeBody::Struct(fields) => fields.iter().map(|f| {
                            format!("{}:{}", f.name.as_str(), f.type_ref.as_str())
                        }).collect(),
                        TypeBody::Alias(a) => vec![a.as_str().to_string()],
                        TypeBody::Unit => Vec::new(),
                    }
                } else {
                    s.variants.iter().map(|v| v.as_str().to_string()).collect()
                }
            }
            MirrorAST::Refract(r) => {
                r.target.as_ref().map(|t| vec![t.as_str().to_string()]).unwrap_or_default()
            }
            _ => Vec::new(),
        }
    }

    /// Grammar reference as string (for Zoom nodes).
    pub fn grammar_ref_str(&self) -> Option<String> {
        match self {
            MirrorAST::Zoom(z) => z.grammar_ref.as_ref().map(|gr| gr.to_string()),
            _ => None,
        }
    }

    /// Return type as string (for Zoom nodes).
    pub fn return_type_str(&self) -> Option<String> {
        match self {
            MirrorAST::Zoom(z) => z.target.as_ref().map(|t| t.as_str().to_string()),
            _ => None,
        }
    }

    /// Parent reference as string (for Focus nodes with targets).
    pub fn parent_ref_str(&self) -> Option<String> {
        match self {
            MirrorAST::Focus(f) => f.target.as_ref().map(|t| t.to_string()),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers -- param parsing
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

    // -- GrammarOid tests --

    #[test]
    fn grammar_oid_strips_at() {
        let g = GrammarOid::new("@test");
        assert_eq!(g.name(), "test");
        assert_eq!(g.to_string(), "@test");
    }

    #[test]
    fn grammar_oid_no_at() {
        let g = GrammarOid::new("code");
        assert_eq!(g.name(), "code");
        assert_eq!(g.to_string(), "@code");
    }

    #[test]
    fn grammar_oid_equality() {
        assert_eq!(GrammarOid::new("test"), GrammarOid::new("@test"));
        assert_ne!(GrammarOid::new("a"), GrammarOid::new("b"));
    }

    #[test]
    fn grammar_oid_to_oid_deterministic() {
        let a = GrammarOid::new("@test").to_oid();
        let b = GrammarOid::new("test").to_oid();
        assert_eq!(a, b);
    }

    // -- GrammarRef tests --

    #[test]
    fn grammar_ref_parse_simple() {
        let r = GrammarRef::parse("@prism");
        assert_eq!(r.grammar.name(), "prism");
        assert_eq!(r.identifier, None);
        assert_eq!(r.to_string(), "@prism");
    }

    #[test]
    fn grammar_ref_parse_with_identifier() {
        let r = GrammarRef::parse("@code/rust");
        assert_eq!(r.grammar.name(), "code");
        assert_eq!(r.identifier, Some("rust".to_string()));
        assert_eq!(r.to_string(), "@code/rust");
    }

    #[test]
    fn grammar_ref_parse_without_at() {
        let r = GrammarRef::parse("prism");
        assert_eq!(r.grammar.name(), "prism");
        assert_eq!(r.identifier, None);
    }

    #[test]
    fn grammar_ref_new_constructor() {
        let r = GrammarRef::new("@code", Some("rust".to_string()));
        assert_eq!(r.grammar.name(), "code");
        assert_eq!(r.identifier, Some("rust".to_string()));
        assert_eq!(r.to_string(), "@code/rust");
    }

    #[test]
    fn grammar_ref_display() {
        let gr = GrammarRef::parse("@code/rust");
        assert_eq!(format!("{}", gr), "@code/rust");
    }

    #[test]
    fn grammar_ref_to_oid_deterministic() {
        let a = GrammarRef::parse("@test").to_oid();
        let b = GrammarRef::parse("@test").to_oid();
        assert_eq!(a, b);
    }

    #[test]
    fn grammar_ref_to_oid_different_for_different_refs() {
        let a = GrammarRef::parse("@test").to_oid();
        let b = GrammarRef::parse("@code").to_oid();
        assert_ne!(a, b);
    }

    #[test]
    fn grammar_ref_hash_and_ord() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(GrammarRef::parse("@a"));
        set.insert(GrammarRef::parse("@a"));
        set.insert(GrammarRef::parse("@b"));
        assert_eq!(set.len(), 2);

        assert!(GrammarRef::parse("@a") < GrammarRef::parse("@b"));
    }

    // -- In/Out node tests --

    #[test]
    fn in_node_construction() {
        let node = MirrorAST::In(GrammarRef::parse("@prism"));
        assert_eq!(node.name(), "prism");
        assert_eq!(node.kind_name(), "in");
        assert_eq!(node.decl_tag(), "in");
    }

    #[test]
    fn out_node_construction() {
        let node = MirrorAST::Out(GrammarRef::parse("@cli"));
        assert_eq!(format!("{}", node), "out @cli");
        assert_eq!(node.kind_name(), "out");
        assert_eq!(node.decl_tag(), "out");
    }

    #[test]
    fn in_node_with_identifier() {
        let node = MirrorAST::In(GrammarRef::parse("@code/rust"));
        assert_eq!(node.name(), "code");
        assert_eq!(format!("{}", node), "in @code/rust");
    }

    #[test]
    fn in_out_are_leaves() {
        let in_node = MirrorAST::In(GrammarRef::parse("@prism"));
        let out_node = MirrorAST::Out(GrammarRef::parse("@cli"));
        assert!(in_node.is_leaf());
        assert!(out_node.is_leaf());
        assert_eq!(in_node.degree(), 0);
        assert_eq!(out_node.degree(), 0);
    }

    #[test]
    fn in_out_content_oid_deterministic() {
        let a = MirrorAST::In(GrammarRef::parse("@prism"));
        let b = MirrorAST::In(GrammarRef::parse("@prism"));
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn in_out_different_oids() {
        let in_node = MirrorAST::In(GrammarRef::parse("@prism"));
        let out_node = MirrorAST::Out(GrammarRef::parse("@prism"));
        assert_ne!(in_node.content_oid(), out_node.content_oid());
    }

    #[test]
    fn encode_decode_in() {
        let ast = MirrorAST::In(GrammarRef::parse("@prism"));
        let encoded = ast.encode();
        let decoded = MirrorAST::decode(&encoded).unwrap();
        assert_eq!(decoded.kind_name(), "in");
        assert_eq!(decoded.name(), "prism");
    }

    #[test]
    fn encode_decode_out() {
        let ast = MirrorAST::Out(GrammarRef::parse("@cli/format"));
        let encoded = ast.encode();
        let decoded = MirrorAST::decode(&encoded).unwrap();
        assert_eq!(decoded.kind_name(), "out");
        assert_eq!(decoded.name(), "cli");
    }

    // -- No Module variant test --

    #[test]
    fn no_module_variant() {
        // Module is gone. Focus replaces it.
        let node = MirrorAST::Focus(FocusNode {
            name: Identifier::new("test"),
            target: None,
            children: vec![],
        });
        assert_eq!(node.name(), "test");
        assert_eq!(node.kind_name(), "focus");
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
            grammar_ref: Some(GrammarRef::parse("@email")),
            children: vec![],
            body: None,
            is_abstract: false,
        });
        assert_eq!(node.kind_name(), "zoom");
    }

    #[test]
    fn zoom_abstract_flag() {
        // abstract action -> zoom with is_abstract = true
        let node = MirrorAST::Zoom(ZoomNode {
            name: Identifier::new("resolve"),
            params: vec![],
            target: None,
            grammar_ref: None,
            children: vec![],
            body: None,
            is_abstract: true,
        });
        assert!(node.is_abstract());
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
    fn project_construction() {
        // project X -- extract a view
        let proj = MirrorAST::Project(ProjectNode {
            name: Identifier::new("items"),
            target: None,
            children: vec![],
        });
        assert_eq!(proj.kind_name(), "project");
    }

    // -- Focus replacing Module --

    #[test]
    fn focus_as_root_module() {
        let child = MirrorAST::Split(SplitNode {
            name: Identifier::new("id"),
            variants: vec![],
            params: vec![],
            body: Some(TypeBody::Unit),
            children: vec![],
        });
        let root = MirrorAST::Focus(FocusNode {
            name: Identifier::new("root"),
            target: None,
            children: vec![child],
        });
        assert_eq!(root.children().len(), 1);
        assert_eq!(root.name(), "root");
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

    // -- MerkleTree tests --

    #[test]
    fn leaf_has_no_children() {
        let leaf = MirrorAST::In(GrammarRef::parse("@x"));
        assert!(leaf.is_leaf());
        assert_eq!(leaf.degree(), 0);
    }

    #[test]
    fn branch_has_children() {
        let child = MirrorAST::In(GrammarRef::parse("@x"));
        let parent = MirrorAST::Focus(FocusNode {
            name: Identifier::new("mod"),
            target: None,
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
}
