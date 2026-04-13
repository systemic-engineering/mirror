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
        }
    }
}

// ---------------------------------------------------------------------------
// MirrorData — the focused eigenvalues of a declaration
// ---------------------------------------------------------------------------

/// The data payload of a mirror fragment: kind, name, params, variants.
/// These are the eigenvalues of a declaration — what survives projection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MirrorData {
    pub kind: DeclKind,
    pub name: String,
    pub params: Vec<String>,
    pub variants: Vec<String>,
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
    /// Get the content OID.
    fn oid(&self) -> &MirrorHash;
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

    fn oid(&self) -> &MirrorHash {
        use fragmentation::fragment::Fragmentable;
        &self.self_ref().sha
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decl_kind_parse_roundtrip() {
        for kind in [
            DeclKind::Form,
            DeclKind::Type,
            DeclKind::Prism,
            DeclKind::In,
            DeclKind::Out,
        ] {
            assert_eq!(DeclKind::parse(kind.as_str()), Some(kind));
        }
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
        assert_eq!(a.oid(), b.oid());
    }
}
