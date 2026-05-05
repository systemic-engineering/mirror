//! ShatterBlob — binary serialization format for optic ASTs.
//!
//! A `.shatter` blob is the serialized form of an `Ast`. The format:
//!
//! ```text
//! [4 bytes] magic: b"SHTR"
//! [1 byte]  version: 1
//! [N bytes] bincode-serialized Ast
//! ```
//!
//! The blob IS the AST. The content_oid of the deserialized AST
//! matches the content_oid of the original — same geometry, same address.

use bincode::Options;
use crate::ast::Ast;

/// Magic bytes for the .shatter binary format.
const SHATTER_MAGIC: &[u8; 4] = b"SHTR";

/// Current format version.
const SHATTER_VERSION: u8 = 1;

/// Maximum payload size for deserialization (64 MiB).
///
/// Prevents unbounded allocation from malicious or corrupted blobs.
/// Both the raw byte length and bincode's internal length-prefixed
/// allocations are bounded by this limit.
const MAX_SHATTER_PAYLOAD: usize = 64 * 1024 * 1024;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors from .shatter blob deserialization.
#[derive(Debug)]
pub enum ShatterError {
    /// The magic bytes don't match `b"SHTR"`.
    InvalidMagic,
    /// The version byte is not supported.
    UnsupportedVersion(u8),
    /// Payload exceeds MAX_SHATTER_PAYLOAD bytes.
    PayloadTooLarge(usize),
    /// bincode deserialization failed.
    Deserialize(bincode::Error),
}

impl std::fmt::Display for ShatterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShatterError::InvalidMagic => write!(f, "invalid shatter magic (expected b\"SHTR\")"),
            ShatterError::UnsupportedVersion(v) => {
                write!(f, "unsupported shatter version: {} (expected {})", v, SHATTER_VERSION)
            }
            ShatterError::PayloadTooLarge(size) => {
                write!(
                    f,
                    "shatter payload too large: {} bytes (max {} bytes / {} MiB)",
                    size,
                    MAX_SHATTER_PAYLOAD,
                    MAX_SHATTER_PAYLOAD / (1024 * 1024)
                )
            }
            ShatterError::Deserialize(e) => write!(f, "shatter deserialize error: {}", e),
        }
    }
}

impl std::error::Error for ShatterError {}

// ---------------------------------------------------------------------------
// Serialize / Deserialize
// ---------------------------------------------------------------------------

/// Bincode options for SHATTER_VERSION 1.
///
/// Must match on both serialize and deserialize sides. Uses the legacy
/// bincode 1.3 config: fixint encoding, little-endian, no trailing byte
/// rejection on the write side.
fn shatter_bincode_options() -> impl Options {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(MAX_SHATTER_PAYLOAD as u64)
        .allow_trailing_bytes()
}

/// Serialize an optic AST to a .shatter blob.
///
/// Format: `SHTR` magic (4 bytes) + version (1 byte) + bincode payload.
pub fn serialize_shatter(ast: &Ast) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(SHATTER_MAGIC);
    buf.push(SHATTER_VERSION);
    buf.extend(
        shatter_bincode_options()
            .serialize(ast)
            .expect("AST serialization cannot fail"),
    );
    buf
}

/// Deserialize a .shatter blob back to an optic AST.
///
/// Validates magic bytes, version, and payload size before deserializing.
/// Uses `bincode::options().with_limit()` to cap internal allocations,
/// preventing denial-of-service from crafted length prefixes.
pub fn deserialize_shatter(bytes: &[u8]) -> Result<Ast, ShatterError> {
    if bytes.len() < 5 || &bytes[0..4] != SHATTER_MAGIC {
        return Err(ShatterError::InvalidMagic);
    }
    if bytes[4] != SHATTER_VERSION {
        return Err(ShatterError::UnsupportedVersion(bytes[4]));
    }
    let payload = &bytes[5..];
    if payload.len() > MAX_SHATTER_PAYLOAD {
        return Err(ShatterError::PayloadTooLarge(payload.len()));
    }
    shatter_bincode_options()
        .deserialize(payload)
        .map_err(ShatterError::Deserialize)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Atom, Body, Ref};
    use crate::kernel::ContentAddressed;

    fn sample_optic() -> Ast {
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
                Ast::Split {
                    root: Box::new(Ast::Atom(Atom::new("origin"))),
                    body: Body::new(vec![
                        Ast::Atom(Atom::new("component_a")),
                        Ast::Atom(Atom::new("component_b")),
                    ]),
                },
                Ast::Refract {
                    mutation: Box::new(Ast::Atom(Atom::new("settle"))),
                    body: Body::new(vec![Ast::Atom(Atom::new("proof"))]),
                },
            ]),
        }
    }

    #[test]
    fn round_trip_simple() {
        let ast = sample_optic();
        let blob = serialize_shatter(&ast);
        let back = deserialize_shatter(&blob).expect("deserialize");
        assert_eq!(ast, back);
    }

    #[test]
    fn round_trip_nested() {
        let ast = sample_nested();
        let blob = serialize_shatter(&ast);
        let back = deserialize_shatter(&blob).expect("deserialize");
        assert_eq!(ast, back);
    }

    #[test]
    fn blob_starts_with_magic() {
        let blob = serialize_shatter(&sample_optic());
        assert_eq!(&blob[0..4], b"SHTR");
        assert_eq!(blob[4], 1);
    }

    #[test]
    fn invalid_magic_rejected() {
        let mut blob = serialize_shatter(&sample_optic());
        blob[0] = b'X';
        let err = deserialize_shatter(&blob).unwrap_err();
        assert!(matches!(err, ShatterError::InvalidMagic));
    }

    #[test]
    fn unsupported_version_rejected() {
        let mut blob = serialize_shatter(&sample_optic());
        blob[4] = 99;
        let err = deserialize_shatter(&blob).unwrap_err();
        assert!(matches!(err, ShatterError::UnsupportedVersion(99)));
    }

    #[test]
    fn empty_bytes_rejected() {
        let err = deserialize_shatter(&[]).unwrap_err();
        assert!(matches!(err, ShatterError::InvalidMagic));
    }

    #[test]
    fn too_short_rejected() {
        let err = deserialize_shatter(b"SHT").unwrap_err();
        assert!(matches!(err, ShatterError::InvalidMagic));
    }

    #[test]
    fn truncated_payload_rejected() {
        let blob = serialize_shatter(&sample_optic());
        // Keep magic + version but truncate payload
        let err = deserialize_shatter(&blob[..6]).unwrap_err();
        assert!(matches!(err, ShatterError::Deserialize(_)));
    }

    #[test]
    fn content_oid_matches_after_round_trip() {
        let ast = sample_optic();
        let blob = serialize_shatter(&ast);
        let back = deserialize_shatter(&blob).expect("deserialize");
        assert_eq!(
            ast.content_oid(),
            back.content_oid(),
            "content_oid of blob must match content_oid of original AST"
        );
    }

    #[test]
    fn content_oid_matches_nested() {
        let ast = sample_nested();
        let blob = serialize_shatter(&ast);
        let back = deserialize_shatter(&blob).expect("deserialize");
        assert_eq!(ast.content_oid(), back.content_oid());
    }

    #[test]
    fn all_optic_variants_round_trip() {
        let asts = vec![
            Ast::Focus {
                target: Box::new(Ast::Atom(Atom::new("t"))),
                body: Body::new(vec![Ast::Atom(Atom::new("b"))]),
            },
            Ast::Project {
                query: Box::new(Ast::Atom(Atom::new("q"))),
                body: Body::new(vec![]),
            },
            Ast::Split {
                root: Box::new(Ast::Ref(Ref::new("r"))),
                body: Body::new(vec![Ast::Atom(Atom::new("c"))]),
            },
            Ast::Zoom {
                perspective: Box::new(Ast::Atom(Atom::new("p"))),
                body: Body::new(vec![]),
            },
            Ast::Refract {
                mutation: Box::new(Ast::Atom(Atom::new("m"))),
                body: Body::new(vec![Ast::Atom(Atom::new("proof"))]),
            },
        ];
        for ast in &asts {
            let blob = serialize_shatter(ast);
            let back = deserialize_shatter(&blob).expect("deserialize");
            assert_eq!(ast, &back, "round-trip failed for {:?}", ast);
            assert_eq!(ast.content_oid(), back.content_oid());
        }
    }

    #[test]
    fn error_display() {
        assert!(format!("{}", ShatterError::InvalidMagic).contains("SHTR"));
        assert!(format!("{}", ShatterError::UnsupportedVersion(42)).contains("42"));
        let too_large = format!("{}", ShatterError::PayloadTooLarge(999));
        assert!(too_large.contains("999"), "PayloadTooLarge should display the size");
        assert!(too_large.contains("64"), "PayloadTooLarge should mention the limit");
    }

    #[test]
    fn oversized_payload_rejected() {
        // A blob with valid magic+version but payload exceeding MAX_SHATTER_PAYLOAD
        // must be rejected before bincode even tries to deserialize.
        let mut blob = Vec::new();
        blob.extend_from_slice(SHATTER_MAGIC);
        blob.push(SHATTER_VERSION);
        // Append a payload larger than MAX_SHATTER_PAYLOAD
        blob.extend(vec![0u8; MAX_SHATTER_PAYLOAD + 1]);
        let err = deserialize_shatter(&blob).unwrap_err();
        assert!(
            matches!(err, ShatterError::PayloadTooLarge(size) if size == MAX_SHATTER_PAYLOAD + 1),
            "expected PayloadTooLarge, got {:?}", err
        );
    }
}
