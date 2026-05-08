//! Mirror declaration types — domain-specific content-addressed fragments.
//!
//! These types were previously in `coincidence::declaration` and are now
//! defined locally. A MirrorFragment is a `Fractal<MirrorData>` carrying
//! the grammar declaration hierarchy.

use fragmentation::encoding::{Decode, Encode};
use fragmentation::fragment::Fractal;
use fragmentation::ref_::Ref;
use fragmentation::sha::{HashAlg, Sha};

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
// MirrorData — the focused eigenvalues of a declaration
// ---------------------------------------------------------------------------

/// The data payload of a mirror fragment: kind, name, params, variants,
/// plus action/modifier metadata absorbed from the former `Form` struct.
/// These are the eigenvalues of a declaration — what survives projection.
///
/// The extra fields (`grammar_ref`, `body_text`, `is_abstract`, `return_type`,
/// `parent_ref`) are NOT included in `Encode`/`Decode` — they are encoded into
/// `params` and `variants` with prefixes (e.g. `in:@code/rust`, `body:...`,
/// `modifier:abstract`, `returns:...`, `parent:@actor`) for content-addressing.
/// The decode side reconstructs them from those prefixed entries.
#[derive(Clone, Debug, Eq)]
pub struct MirrorData {
    pub kind: DeclKind,
    pub name: String,
    pub params: Vec<String>,
    pub variants: Vec<String>,
    // -- absorbed from Form --
    /// For `action` declarations: the grammar reference (e.g. `@code/rust`).
    pub grammar_ref: Option<String>,
    /// For `action` declarations: the raw body text, brace-balanced but unparsed.
    pub body_text: Option<String>,
    /// Whether this declaration has the `abstract` modifier.
    pub is_abstract: bool,
    /// Optional return type annotation (e.g. `-> [completion]`).
    pub return_type: Option<String>,
    /// For `grammar` declarations: the parent grammar reference (e.g. `@actor`).
    pub parent_ref: Option<String>,
    /// Optic operators found in this declaration.
    /// Parser annotation — not included in Encode/Decode or PartialEq.
    pub optic_ops: Vec<OpticOp>,
}

/// The extra metadata fields (grammar_ref, body_text, is_abstract, return_type,
/// parent_ref) are excluded from equality because they are encoded into
/// params/variants for content-addressing. Structural equality compares the
/// content-addressable surface only.
impl PartialEq for MirrorData {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.name == other.name
            && self.params == other.params
            && self.variants == other.variants
    }
}

impl MirrorData {
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
}

impl Encode for MirrorData {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(self.kind.as_str().as_bytes());
        buf.push(b':');
        buf.extend_from_slice(self.name.as_bytes());
        buf.push(b':');
        for (i, p) in self.params.iter().enumerate() {
            if i > 0 {
                buf.push(b',');
            }
            buf.extend_from_slice(p.as_bytes());
        }
        buf.push(b':');
        for (i, v) in self.variants.iter().enumerate() {
            if i > 0 {
                buf.push(b'|');
            }
            buf.extend_from_slice(v.as_bytes());
        }
        buf
    }
}

impl Decode for MirrorData {
    type Error = String;
    fn decode(bytes: &[u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
        let parts: Vec<&str> = s.splitn(4, ':').collect();
        if parts.len() < 4 {
            return Err(format!(
                "expected 4 colon-separated fields, got {}",
                parts.len()
            ));
        }
        let kind =
            DeclKind::parse(parts[0]).ok_or_else(|| format!("unknown DeclKind: {}", parts[0]))?;
        let name = parts[1].to_string();
        let params = if parts[2].is_empty() {
            Vec::new()
        } else {
            parts[2].split(',').map(|s| s.to_string()).collect()
        };
        let variants = if parts[3].is_empty() {
            Vec::new()
        } else {
            parts[3].split('|').map(|s| s.to_string()).collect()
        };
        Ok(MirrorData {
            kind,
            name,
            params,
            variants,
            grammar_ref: None,
            body_text: None,
            is_abstract: false,
            return_type: None,
            parent_ref: None,
            optic_ops: Vec::new(),
        })
    }
}

// ---------------------------------------------------------------------------
// MirrorFragment — content-addressed declaration tree
// ---------------------------------------------------------------------------

/// A content-addressed mirror declaration: `Fractal<MirrorData>`.
pub type MirrorFragment = Fractal<MirrorData>;

/// The hash type used for mirror fragments.
pub type MirrorHash = Sha;

/// Extension trait for accessing mirror-specific data on fragments.
pub trait MirrorFragmentExt {
    /// Get the MirrorData payload.
    fn mirror_data(&self) -> &MirrorData;
    /// Get the child fragments.
    fn mirror_children(&self) -> &[MirrorFragment];
    /// Get the node-level content hash (SHA-256 of the node's encoded data).
    /// Renamed from `oid()` to avoid ambiguity with `prism_core::Addressable::oid()`.
    fn content_hash(&self) -> &MirrorHash;
}

impl MirrorFragmentExt for MirrorFragment {
    fn mirror_data(&self) -> &MirrorData {
        use fragmentation::fragment::Fragmentable;
        self.data()
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

impl MirrorData {
    /// Encode the extra metadata fields into params/variants for content-addressing.
    /// Returns a new MirrorData with the encoded params/variants suitable for
    /// hashing, plus the original extra fields preserved.
    pub fn encode_for_fragment(&self) -> MirrorData {
        let mut params = self.params.clone();
        let mut variants = self.variants.clone();
        if self.kind == DeclKind::Action
            || self.kind == DeclKind::Recover
            || self.kind == DeclKind::Rescue
        {
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
        if let Some(ref pr) = self.parent_ref {
            params.push(format!("parent:{}", pr));
        }
        MirrorData {
            kind: self.kind.clone(),
            name: self.name.clone(),
            params,
            variants,
            grammar_ref: self.grammar_ref.clone(),
            body_text: self.body_text.clone(),
            is_abstract: self.is_abstract,
            return_type: self.return_type.clone(),
            parent_ref: self.parent_ref.clone(),
            optic_ops: self.optic_ops.clone(),
        }
    }

    /// Decode extra metadata fields from the encoded params/variants of a fragment.
    /// Returns a MirrorData with clean params/variants and the extra fields populated.
    pub fn decode_from_fragment(raw: &MirrorData) -> MirrorData {
        let mut params = Vec::new();
        let mut grammar_ref = None;
        let mut return_type = None;
        let mut is_abstract = false;
        let mut parent_ref = None;
        for p in &raw.params {
            if let Some(gr) = p.strip_prefix("in:") {
                grammar_ref = Some(gr.to_string());
            } else if let Some(rt) = p.strip_prefix("returns:") {
                return_type = Some(rt.to_string());
            } else if p == "modifier:abstract" {
                is_abstract = true;
            } else if let Some(pr) = p.strip_prefix("parent:") {
                parent_ref = Some(pr.to_string());
            } else {
                params.push(p.clone());
            }
        }
        let mut variants = Vec::new();
        let mut body_text = None;
        for v in &raw.variants {
            if let Some(bt) = v.strip_prefix("body:") {
                body_text = Some(bt.to_string());
            } else {
                variants.push(v.clone());
            }
        }
        MirrorData {
            kind: raw.kind.clone(),
            name: raw.name.clone(),
            params,
            variants,
            grammar_ref,
            body_text,
            is_abstract,
            return_type,
            parent_ref,
            optic_ops: raw.optic_ops.clone(),
        }
    }
}

/// Build a MirrorFragment from data and children.
pub fn fragment(data: MirrorData, children: Vec<MirrorFragment>) -> MirrorFragment {
    let encoded = data.encode();
    let hash = Sha::hash(&encoded);
    let ref_ = Ref::new(hash, data.kind.as_str());
    if children.is_empty() {
        Fractal::shard_typed(ref_, data)
    } else {
        Fractal::new_typed(ref_, data, children)
    }
}

/// Build a MirrorFragment from data with extra fields encoded into params/variants.
/// This is the primary constructor when building from parsed declarations —
/// it encodes grammar_ref, body_text, is_abstract, return_type, parent_ref
/// into the content-addressable params/variants before hashing.
pub fn fragment_encoded(data: MirrorData, children: Vec<MirrorFragment>) -> MirrorFragment {
    let encoded_data = data.encode_for_fragment();
    fragment(encoded_data, children)
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

    #[test]
    fn mirror_data_encode() {
        let data = MirrorData::new(DeclKind::Form, "@test", vec!["a".into()], vec!["x".into()]);
        let encoded = data.encode();
        assert_eq!(std::str::from_utf8(&encoded).unwrap(), "form:@test:a:x");
    }

    #[test]
    fn fragment_shard() {
        let data = MirrorData::new(DeclKind::Prism, "focus", Vec::new(), Vec::new());
        let frag = fragment(data.clone(), Vec::new());
        assert_eq!(frag.mirror_data(), &data);
        assert!(frag.mirror_children().is_empty());
    }

    #[test]
    fn fragment_with_children() {
        let child = fragment(
            MirrorData::new(DeclKind::Prism, "focus", Vec::new(), Vec::new()),
            Vec::new(),
        );
        let parent = fragment(
            MirrorData::new(DeclKind::Form, "@test", Vec::new(), Vec::new()),
            vec![child],
        );
        assert_eq!(parent.mirror_children().len(), 1);
    }

    #[test]
    fn fragment_oid_deterministic() {
        let data = MirrorData::new(DeclKind::Type, "id", Vec::new(), Vec::new());
        let a = fragment(data.clone(), Vec::new());
        let b = fragment(data, Vec::new());
        assert_eq!(a.content_hash(), b.content_hash());
    }
}
