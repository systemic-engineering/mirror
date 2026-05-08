//! Mirror declaration types — domain-specific content-addressed fragments.
//!
//! These types were previously in `coincidence::declaration` and are now
//! defined locally. A MirrorFragment is a `Fractal<MirrorAST>` carrying
//! the grammar declaration hierarchy.

use fragmentation::encoding::{Decode, Encode};
use fragmentation::fragment::Fractal;
use fragmentation::ref_::Ref;
use fragmentation::sha::{HashAlg, Sha};

use crate::mirror_ast::MirrorAST;

// ---------------------------------------------------------------------------
// DeclKind — the kind of a mirror declaration
// ---------------------------------------------------------------------------

/// The structural kind of a declaration in the mirror grammar.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeclKind {
    // Structural
    Form,
    Type,
    Prism,
    In,
    Out,
    // Property system
    Property,
    Fold,
    Requires,
    Invariant,
    Ensures,
    // Prism operations (used as declaration keywords in .mirror)
    Focus,
    Project,
    Split,
    Zoom,
    Refract,
    // Optics
    Traversal,
    Lens,
    // Actions
    Action,
    // Error handling
    Recover,
    Rescue,
    // Grammar
    Grammar,
    // Templates
    Template,
    // Assertion / configuration
    Default,
    Binding,
}

impl DeclKind {
    /// Parse a keyword string into a DeclKind.
    pub fn parse(s: &str) -> Option<DeclKind> {
        match s {
            "form" => Some(DeclKind::Form),
            "type" => Some(DeclKind::Type),
            "prism" => Some(DeclKind::Prism),
            "in" => Some(DeclKind::In),
            "out" => Some(DeclKind::Out),
            "property" => Some(DeclKind::Property),
            "fold" => Some(DeclKind::Fold),
            "requires" => Some(DeclKind::Requires),
            "invariant" => Some(DeclKind::Invariant),
            "ensures" => Some(DeclKind::Ensures),
            "focus" => Some(DeclKind::Focus),
            "project" => Some(DeclKind::Project),
            "split" => Some(DeclKind::Split),
            "zoom" => Some(DeclKind::Zoom),
            "refract" => Some(DeclKind::Refract),
            "traversal" => Some(DeclKind::Traversal),
            "lens" => Some(DeclKind::Lens),
            "action" => Some(DeclKind::Action),
            "recover" => Some(DeclKind::Recover),
            "rescue" => Some(DeclKind::Rescue),
            "grammar" => Some(DeclKind::Grammar),
            "template" => Some(DeclKind::Template),
            "default" => Some(DeclKind::Default),
            "binding" => Some(DeclKind::Binding),
            _ => None,
        }
    }

    /// The keyword string for this kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            DeclKind::Form => "form",
            DeclKind::Type => "type",
            DeclKind::Prism => "prism",
            DeclKind::In => "in",
            DeclKind::Out => "out",
            DeclKind::Property => "property",
            DeclKind::Fold => "fold",
            DeclKind::Requires => "requires",
            DeclKind::Invariant => "invariant",
            DeclKind::Ensures => "ensures",
            DeclKind::Focus => "focus",
            DeclKind::Project => "project",
            DeclKind::Split => "split",
            DeclKind::Zoom => "zoom",
            DeclKind::Refract => "refract",
            DeclKind::Traversal => "traversal",
            DeclKind::Lens => "lens",
            DeclKind::Action => "action",
            DeclKind::Recover => "recover",
            DeclKind::Rescue => "rescue",
            DeclKind::Grammar => "grammar",
            DeclKind::Template => "template",
            DeclKind::Default => "default",
            DeclKind::Binding => "binding",
        }
    }
}

// ---------------------------------------------------------------------------
// OpticOp — the five prism operations as operator tokens
// ---------------------------------------------------------------------------

/// The six optics, classified by their operator token.
///
/// These are the shared kernel between Rust and .mirror: the same six
/// operators mean the same thing on both sides of the glass wall.
///
/// The three core operators declare superpositions:
///
/// ```text
/// =    Iso      superposition preserved  (bidirectional, lossless)
/// <=   Fold     superposition collapsed  (one-directional, loss accumulates)
/// |    Split    superposition branched   (variants)
/// ```
///
/// The three structural operators navigate them:
///
/// ```text
/// ()   Focus    grouping / function call (structural, not a single token)
/// ->   Zoom     flow / return type / transformation
/// ..   Refract  spread / range / settlement
/// ```
///
/// Every `<=` in a `.mirror` file is an observation that returns `Imperfect`:
/// the fold IS the measurement, and the measurement carries loss.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpticOp {
    /// `=` — superposition preserved. The iso: what goes in comes out unchanged.
    Iso,
    /// `<=` — superposition collapsed. The fold: one-directional, loss accumulates.
    /// Every fold returns `Imperfect` — the observation IS the measurement.
    Fold,
    /// `|` — superposition branched. The split: one of many.
    Split,
    /// `()` — grouping / function call. The focus: look closer.
    /// Note: parentheses are delimiters in the tokenizer, not a single token.
    /// Focus is implicit in the grammar structure (params, grouping).
    Focus,
    /// `->`, `|>`, `<|`, `/` — flow / transformation. The zoom: move between levels.
    Zoom,
    /// `..` — spread / range / settlement. The refract: scatter and reconverge.
    Refract,
    /// `<` — subset relation. The type is contained in the referenced type.
    Subset,
    /// `>` — superset relation. The type contains the referenced type.
    Superset,
    /// `!=` — not-iso. The types are related but not equivalent.
    NotIso,
    /// `>=` — unfold. The dual of fold: one-directional expansion.
    Unfold,
}

impl OpticOp {
    /// Classify an operator token as an optic operation.
    pub fn from_token(token: &str) -> Option<OpticOp> {
        match token {
            "=" => Some(OpticOp::Iso),
            "<=" => Some(OpticOp::Fold),
            "|" => Some(OpticOp::Split),
            "->" | "|>" | "<|" | "/" => Some(OpticOp::Zoom),
            "+" => Some(OpticOp::Zoom), // combine / accumulate
            ".." => Some(OpticOp::Refract),
            "<" => Some(OpticOp::Subset),
            ">" => Some(OpticOp::Superset),
            "!=" => Some(OpticOp::NotIso),
            "=>" => Some(OpticOp::Unfold),
            "<-" => Some(OpticOp::Zoom),
            _ => None,
        }
    }

    /// The canonical single-token representation of this optic.
    pub fn as_str(&self) -> &'static str {
        match self {
            OpticOp::Iso => "=",
            OpticOp::Fold => "<=",
            OpticOp::Split => "|",
            OpticOp::Focus => "()",
            OpticOp::Zoom => "->",
            OpticOp::Refract => "..",
            OpticOp::Subset => "<",
            OpticOp::Superset => ">",
            OpticOp::NotIso => "!=",
            OpticOp::Unfold => "=>",
        }
    }

    /// The DeclKind that corresponds to this optic operation, if any.
    /// Focus and Project are DeclKind variants; Iso has no direct DeclKind.
    pub fn to_decl_kind(&self) -> Option<DeclKind> {
        match self {
            OpticOp::Fold => Some(DeclKind::Fold),
            OpticOp::Split => Some(DeclKind::Split),
            OpticOp::Zoom => Some(DeclKind::Zoom),
            OpticOp::Refract => Some(DeclKind::Refract),
            OpticOp::Focus => Some(DeclKind::Focus),
            OpticOp::Iso => None, // Iso is structural (=), not a declaration keyword
            OpticOp::Subset => None,
            OpticOp::Superset => None,
            OpticOp::NotIso => None,
            OpticOp::Unfold => None,
        }
    }
}

impl OpticOp {
    /// Classify a declaration keyword as its corresponding optic operation.
    pub fn from_decl_kind(kind: &DeclKind) -> Option<OpticOp> {
        match kind {
            DeclKind::Fold => Some(OpticOp::Fold),
            DeclKind::Focus => Some(OpticOp::Focus),
            DeclKind::Split => Some(OpticOp::Split),
            DeclKind::Zoom => Some(OpticOp::Zoom),
            DeclKind::Refract => Some(OpticOp::Refract),
            _ => None,
        }
    }
}

impl std::fmt::Display for OpticOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// ---------------------------------------------------------------------------
// MirrorData — projection of MirrorAST into stringly-typed view
// ---------------------------------------------------------------------------

use crate::mirror_ast::{Field, FocusNode, Identifier, GrammarRef, ProjectNode, RefractNode, SplitNode, TypeBody, ZoomNode};

/// Stringly-typed projection of a MirrorAST node.
///
/// This is NOT a storage type. It's a read-only view that lets consumers
/// access the old (kind, name, params, variants) interface while the AST
/// is the canonical representation.
#[derive(Clone, Debug, PartialEq)]
pub struct MirrorData {
    pub kind: DeclKind,
    pub name: String,
    pub params: Vec<String>,
    pub variants: Vec<String>,
    pub grammar_ref: Option<String>,
    pub body_text: Option<String>,
    pub is_abstract: bool,
    pub return_type: Option<String>,
    pub parent_ref: Option<String>,
    pub optic_ops: Vec<OpticOp>,
}

impl MirrorData {
    /// Create a new MirrorData (for building AST nodes from parse output).
    pub fn new(
        kind: DeclKind,
        name: impl Into<String>,
        params: Vec<String>,
        variants: Vec<String>,
    ) -> Self {
        MirrorData {
            kind,
            name: name.into(),
            params,
            variants,
            grammar_ref: None,
            body_text: None,
            is_abstract: false,
            return_type: None,
            parent_ref: None,
            optic_ops: Vec::new(),
        }
    }

    /// Project a MirrorAST node into a MirrorData view.
    pub fn from_ast(ast: &MirrorAST) -> Self {
        match ast {
            MirrorAST::Focus(f) => {
                let is_grammar = f.name.as_str().starts_with('@');
                let kind = if is_grammar { DeclKind::Grammar } else { DeclKind::Focus };
                let mut data = MirrorData::new(kind, f.name.as_str(), Vec::new(), Vec::new());
                data.parent_ref = f.target.as_ref().map(|t| t.as_str().to_string());
                data
            }
            MirrorAST::Project(p) => {
                if p.target.is_some() {
                    // import
                    MirrorData::new(DeclKind::In, p.name.as_str(), Vec::new(), Vec::new())
                } else {
                    // export
                    MirrorData::new(DeclKind::Out, p.name.as_str(), Vec::new(), Vec::new())
                }
            }
            MirrorAST::Split(s) => {
                let params: Vec<String> = s.params.iter().map(|p| p.as_str().to_string()).collect();
                let variants: Vec<String> = if let Some(ref body) = s.body {
                    match body {
                        TypeBody::Enum(vs) => vs.iter().map(|v| v.as_str().to_string()).collect(),
                        TypeBody::Struct(fields) => fields.iter().map(|f| format!("{}:{}", f.name.as_str(), f.type_ref.as_str())).collect(),
                        TypeBody::Alias(a) => vec![a.as_str().to_string()],
                        TypeBody::Unit => Vec::new(),
                    }
                } else {
                    s.variants.iter().map(|v| v.as_str().to_string()).collect()
                };
                MirrorData::new(DeclKind::Type, s.name.as_str(), params, variants)
            }
            MirrorAST::Zoom(z) => {
                let params: Vec<String> = z.params.iter().map(|f| {
                    if f.type_ref.as_str() == "_" {
                        f.name.as_str().to_string()
                    } else {
                        format!("{}:{}", f.name.as_str(), f.type_ref.as_str())
                    }
                }).collect();
                let mut data = MirrorData::new(DeclKind::Action, z.name.as_str(), params, Vec::new());
                data.return_type = z.target.as_ref().map(|t| t.as_str().to_string());
                data.grammar_ref = z.grammar_ref.as_ref().map(|gr| gr.as_str().to_string());
                data
            }
            MirrorAST::Refract(r) => {
                let params: Vec<String> = r.params.iter().map(|f| {
                    if f.type_ref.as_str() == "_" {
                        f.name.as_str().to_string()
                    } else {
                        format!("{}:{}", f.name.as_str(), f.type_ref.as_str())
                    }
                }).collect();
                let mut data = MirrorData::new(DeclKind::Property, r.name.as_str(), params, Vec::new());
                if let Some(ref t) = r.target {
                    data.variants.push(t.as_str().to_string());
                }
                data
            }
            MirrorAST::Abstract(inner) => {
                let mut data = MirrorData::from_ast(inner);
                data.is_abstract = true;
                data
            }
            MirrorAST::Module(m) => {
                MirrorData::new(DeclKind::Form, m.name.as_str(), Vec::new(), Vec::new())
            }
        }
    }

    /// Decode a MirrorData from a fragment's MirrorAST payload.
    /// Compatibility shim: reads the AST and projects it.
    pub fn decode_from_fragment(ast: &MirrorAST) -> Self {
        Self::from_ast(ast)
    }
}

/// Build a MirrorFragment from a MirrorData (backward compat).
/// Converts MirrorData -> MirrorAST -> fragment.
pub fn fragment_encoded(data: MirrorData, children: Vec<MirrorFragment>) -> MirrorFragment {
    let ast = mirror_ast_from_data(&data);
    let child_frags = children;
    fragment(ast, child_frags)
}

/// Convert a MirrorData back into a MirrorAST node.
fn mirror_ast_from_data(data: &MirrorData) -> MirrorAST {
    match data.kind {
        DeclKind::Grammar => {
            let name = if data.name.starts_with('@') {
                Identifier::new(&data.name)
            } else {
                Identifier::new(format!("@{}", data.name))
            };
            let parent = data.parent_ref.as_ref().map(|p| {
                if p.starts_with('@') { GrammarRef::new(p) }
                else { GrammarRef::new(format!("@{}", p)) }
            });
            MirrorAST::Focus(FocusNode {
                name,
                target: parent,
                children: vec![],
            })
        }
        DeclKind::Form => {
            if data.name.starts_with('@') {
                MirrorAST::Focus(FocusNode {
                    name: Identifier::new(&data.name),
                    target: data.parent_ref.as_ref().map(|p| {
                        if p.starts_with('@') { GrammarRef::new(p) }
                        else { GrammarRef::new(format!("@{}", p)) }
                    }),
                    children: vec![],
                })
            } else {
                MirrorAST::Module(crate::mirror_ast::ModuleNode {
                    name: Identifier::new(&data.name),
                    children: vec![],
                })
            }
        }
        DeclKind::Type => {
            let body = if !data.variants.is_empty() {
                if data.variants.iter().any(|v| v.contains(':')) {
                    Some(TypeBody::Struct(data.variants.iter().map(|v| {
                        if let Some((n, t)) = v.split_once(':') {
                            Field { name: Identifier::new(n.trim()), type_ref: Identifier::new(t.trim()) }
                        } else {
                            Field { name: Identifier::new(v), type_ref: Identifier::new("_") }
                        }
                    }).collect()))
                } else {
                    Some(TypeBody::Enum(data.variants.iter().map(|v| Identifier::new(v)).collect()))
                }
            } else {
                Some(TypeBody::Unit)
            };
            let params: Vec<Identifier> = data.params.iter().map(|p| Identifier::new(p)).collect();
            MirrorAST::Split(SplitNode {
                name: Identifier::new(&data.name),
                variants: vec![],
                params,
                body,
                children: vec![],
            })
        }
        DeclKind::Action | DeclKind::Template | DeclKind::Recover | DeclKind::Rescue => {
            let params = MirrorAST::params_to_fields(&data.params);
            MirrorAST::Zoom(ZoomNode {
                name: Identifier::new(&data.name),
                params,
                target: data.return_type.as_deref().map(Identifier::new),
                grammar_ref: data.grammar_ref.as_deref().map(|gr| {
                    if gr.starts_with('@') { GrammarRef::new(gr) }
                    else { GrammarRef::new(format!("@{}", gr)) }
                }),
                children: vec![],
                body: None,
            })
        }
        DeclKind::Property | DeclKind::Requires | DeclKind::Invariant | DeclKind::Ensures => {
            let params = MirrorAST::params_to_fields(&data.params);
            let fold_target = data.variants.first().map(|v| Identifier::new(v));
            MirrorAST::Refract(RefractNode {
                name: Identifier::new(&data.name),
                params,
                target: fold_target,
                children: vec![],
            })
        }
        DeclKind::Fold => {
            MirrorAST::Refract(RefractNode {
                name: Identifier::new(&data.name),
                params: vec![],
                target: data.params.first().map(|p| Identifier::new(p)),
                children: vec![],
            })
        }
        DeclKind::In => {
            let target_str = &data.name;
            let target = if target_str.starts_with('@') {
                GrammarRef::new(target_str)
            } else {
                GrammarRef::new(format!("@{}", target_str))
            };
            MirrorAST::Project(ProjectNode {
                name: Identifier::new(target_str),
                target: Some(target),
                children: vec![],
            })
        }
        DeclKind::Out | DeclKind::Default | DeclKind::Binding => {
            MirrorAST::Project(ProjectNode {
                name: Identifier::new(&data.name),
                target: None,
                children: vec![],
            })
        }
        DeclKind::Focus | DeclKind::Traversal | DeclKind::Lens => {
            MirrorAST::Focus(FocusNode {
                name: Identifier::new(&data.name),
                target: data.params.first().and_then(|p| {
                    if p.starts_with('@') { Some(GrammarRef::new(p)) } else { None }
                }),
                children: vec![],
            })
        }
        DeclKind::Project => {
            MirrorAST::Project(ProjectNode {
                name: Identifier::new(&data.name),
                target: data.params.first().and_then(|p| {
                    if p.starts_with('@') { Some(GrammarRef::new(p)) } else { None }
                }),
                children: vec![],
            })
        }
        DeclKind::Split => {
            MirrorAST::Split(SplitNode {
                name: Identifier::new(&data.name),
                variants: data.variants.iter().map(|v| Identifier::new(v)).collect(),
                params: vec![],
                body: None,
                children: vec![],
            })
        }
        DeclKind::Zoom => {
            MirrorAST::Zoom(ZoomNode {
                name: Identifier::new(&data.name),
                target: data.params.first().map(|p| Identifier::new(p)),
                params: vec![],
                grammar_ref: None,
                children: vec![],
                body: None,
            })
        }
        DeclKind::Refract => {
            MirrorAST::Refract(RefractNode {
                name: Identifier::new(&data.name),
                target: data.params.first().map(|p| Identifier::new(p)),
                params: vec![],
                children: vec![],
            })
        }
        DeclKind::Prism => {
            MirrorAST::Module(crate::mirror_ast::ModuleNode {
                name: Identifier::new(&data.name),
                children: vec![],
            })
        }
    }
}

// ---------------------------------------------------------------------------
// MirrorFragment — content-addressed declaration tree
// ---------------------------------------------------------------------------

/// A content-addressed mirror declaration: `Fractal<MirrorAST>`.
pub type MirrorFragment = Fractal<MirrorAST>;

/// The hash type used for mirror fragments.
pub type MirrorHash = Sha;

/// Extension trait for accessing mirror-specific data on fragments.
pub trait MirrorFragmentExt {
    /// Get the MirrorAST payload.
    fn mirror_ast(&self) -> &MirrorAST;
    /// Get a MirrorData projection (backward compat for stringly-typed access).
    fn mirror_data(&self) -> MirrorData;
    /// Get the child fragments.
    fn mirror_children(&self) -> &[MirrorFragment];
    /// Get the node-level content hash (SHA-256 of the node's encoded data).
    fn content_hash(&self) -> &MirrorHash;
}

impl MirrorFragmentExt for MirrorFragment {
    fn mirror_ast(&self) -> &MirrorAST {
        use fragmentation::fragment::Fragmentable;
        self.data()
    }

    fn mirror_data(&self) -> MirrorData {
        use fragmentation::fragment::Fragmentable;
        MirrorData::from_ast(self.data())
    }

    fn mirror_children(&self) -> &[MirrorFragment] {
        use fragmentation::fragment::Fragmentable;
        self.children()
    }

    fn content_hash(&self) -> &MirrorHash {
        use fragmentation::fragment::Fragmentable;
        &self.self_ref().sha
    }
}

/// Build a MirrorFragment from a MirrorAST node and children.
pub fn fragment(ast: MirrorAST, children: Vec<MirrorFragment>) -> MirrorFragment {
    let encoded = ast.encode();
    let hash = Sha::hash(&encoded);
    let ref_ = Ref::new(hash, ast.kind_name());
    if children.is_empty() {
        Fractal::shard_typed(ref_, ast)
    } else {
        Fractal::new_typed(ref_, ast, children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // OpticOp tests
    // -----------------------------------------------------------------------

    #[test]
    fn operator_iso_maps_to_equals() {
        assert_eq!(OpticOp::from_token("="), Some(OpticOp::Iso));
    }

    #[test]
    fn operator_split_maps_to_pipe() {
        assert_eq!(OpticOp::from_token("|"), Some(OpticOp::Split));
    }

    #[test]
    fn operator_fold_maps_to_arrow_left() {
        assert_eq!(OpticOp::from_token("<="), Some(OpticOp::Fold));
    }

    #[test]
    fn operator_zoom_maps_to_arrow() {
        assert_eq!(OpticOp::from_token("->"), Some(OpticOp::Zoom));
    }

    #[test]
    fn operator_zoom_maps_to_pipe_arrow() {
        assert_eq!(OpticOp::from_token("|>"), Some(OpticOp::Zoom));
    }

    #[test]
    fn operator_zoom_maps_to_reverse_pipe() {
        assert_eq!(OpticOp::from_token("<|"), Some(OpticOp::Zoom));
    }

    #[test]
    fn operator_zoom_maps_to_slash() {
        assert_eq!(OpticOp::from_token("/"), Some(OpticOp::Zoom));
    }

    #[test]
    fn operator_refract_maps_to_dotdot() {
        assert_eq!(OpticOp::from_token(".."), Some(OpticOp::Refract));
    }

    #[test]
    fn unknown_operator_returns_none() {
        assert_eq!(OpticOp::from_token("?"), None);
    }

    #[test]
    fn operator_plus_maps_to_zoom() {
        assert_eq!(OpticOp::from_token("+"), Some(OpticOp::Zoom));
    }

    #[test]
    fn operator_subset_maps_to_less_than() {
        assert_eq!(OpticOp::from_token("<"), Some(OpticOp::Subset));
    }

    #[test]
    fn operator_superset_maps_to_greater_than() {
        assert_eq!(OpticOp::from_token(">"), Some(OpticOp::Superset));
    }

    #[test]
    fn operator_not_iso_maps_to_bang_equals() {
        assert_eq!(OpticOp::from_token("!="), Some(OpticOp::NotIso));
    }

    #[test]
    fn operator_unfold_maps_to_fat_arrow() {
        // Design spec: => is Unfold (not >=)
        assert_eq!(OpticOp::from_token("=>"), Some(OpticOp::Unfold));
    }

    #[test]
    fn operator_reverse_zoom_maps_to_left_arrow() {
        assert_eq!(OpticOp::from_token("<-"), Some(OpticOp::Zoom));
    }

    #[test]
    fn old_unfold_token_no_longer_matches() {
        // >= was the old Unfold token, now retired
        assert_eq!(OpticOp::from_token(">="), None);
    }

    #[test]
    fn optic_op_as_str_roundtrips_through_from_token() {
        // Iso, Split, Fold, Zoom, Refract, Subset, Superset, NotIso, Unfold
        // roundtrip through from_token.
        // Focus is structural (parentheses), so it has no single-token parse.
        for op in [
            OpticOp::Iso,
            OpticOp::Split,
            OpticOp::Fold,
            OpticOp::Zoom,
            OpticOp::Refract,
            OpticOp::Subset,
            OpticOp::Superset,
            OpticOp::NotIso,
            OpticOp::Unfold,
        ] {
            let s = op.as_str();
            assert_eq!(
                OpticOp::from_token(s),
                Some(op.clone()),
                "as_str -> from_token must roundtrip for {:?}",
                op
            );
        }
        // Focus can't roundtrip: "()" is not a single token. Verify as_str is correct.
        assert_eq!(OpticOp::Focus.as_str(), "()");
        assert_eq!(OpticOp::from_token("()"), None);
    }

    #[test]
    fn optic_op_display() {
        assert_eq!(format!("{}", OpticOp::Iso), "=");
        assert_eq!(format!("{}", OpticOp::Split), "|");
        assert_eq!(format!("{}", OpticOp::Fold), "<=");
        assert_eq!(format!("{}", OpticOp::Focus), "()");
        assert_eq!(format!("{}", OpticOp::Zoom), "->");
        assert_eq!(format!("{}", OpticOp::Refract), "..");
        assert_eq!(format!("{}", OpticOp::Subset), "<");
        assert_eq!(format!("{}", OpticOp::Superset), ">");
        assert_eq!(format!("{}", OpticOp::NotIso), "!=");
        assert_eq!(format!("{}", OpticOp::Unfold), "=>");
    }

    #[test]
    fn optic_op_to_decl_kind() {
        assert_eq!(OpticOp::Split.to_decl_kind(), Some(DeclKind::Split));
        assert_eq!(OpticOp::Zoom.to_decl_kind(), Some(DeclKind::Zoom));
        assert_eq!(OpticOp::Refract.to_decl_kind(), Some(DeclKind::Refract));
        assert_eq!(OpticOp::Focus.to_decl_kind(), Some(DeclKind::Focus));
        assert_eq!(OpticOp::Fold.to_decl_kind(), Some(DeclKind::Fold));
        assert_eq!(OpticOp::Iso.to_decl_kind(), None);
        assert_eq!(OpticOp::Subset.to_decl_kind(), None);
        assert_eq!(OpticOp::Superset.to_decl_kind(), None);
        assert_eq!(OpticOp::NotIso.to_decl_kind(), None);
        assert_eq!(OpticOp::Unfold.to_decl_kind(), None);
    }

    #[test]
    fn optic_op_from_decl_kind() {
        assert_eq!(
            OpticOp::from_decl_kind(&DeclKind::Split),
            Some(OpticOp::Split)
        );
        assert_eq!(
            OpticOp::from_decl_kind(&DeclKind::Zoom),
            Some(OpticOp::Zoom)
        );
        assert_eq!(
            OpticOp::from_decl_kind(&DeclKind::Refract),
            Some(OpticOp::Refract)
        );
        assert_eq!(
            OpticOp::from_decl_kind(&DeclKind::Focus),
            Some(OpticOp::Focus)
        );
        assert_eq!(
            OpticOp::from_decl_kind(&DeclKind::Fold),
            Some(OpticOp::Fold)
        );
        assert_eq!(OpticOp::from_decl_kind(&DeclKind::Type), None);
        assert_eq!(OpticOp::from_decl_kind(&DeclKind::Grammar), None);
    }

    // -----------------------------------------------------------------------
    // DeclKind tests
    // -----------------------------------------------------------------------

    #[test]
    fn decl_kind_parse_roundtrip_all_variants() {
        // Every DeclKind variant must roundtrip through parse/as_str.
        let all_kinds = [
            DeclKind::Form,
            DeclKind::Type,
            DeclKind::Prism,
            DeclKind::In,
            DeclKind::Out,
            DeclKind::Property,
            DeclKind::Fold,
            DeclKind::Requires,
            DeclKind::Invariant,
            DeclKind::Ensures,
            DeclKind::Focus,
            DeclKind::Project,
            DeclKind::Split,
            DeclKind::Zoom,
            DeclKind::Refract,
            DeclKind::Traversal,
            DeclKind::Lens,
            DeclKind::Action,
            DeclKind::Recover,
            DeclKind::Rescue,
            DeclKind::Grammar,
            DeclKind::Template,
            DeclKind::Default,
            DeclKind::Binding,
        ];
        for kind in &all_kinds {
            assert_eq!(
                DeclKind::parse(kind.as_str()),
                Some(kind.clone()),
                "roundtrip failed for {:?}",
                kind
            );
        }
        // Ensure we tested every variant — count must match.
        assert_eq!(all_kinds.len(), 24, "must test all 24 DeclKind variants");
    }

    #[test]
    fn decl_kind_parse_unknown() {
        assert_eq!(DeclKind::parse("unknown"), None);
    }
}
