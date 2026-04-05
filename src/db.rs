//! Mirror storage — bounded, content-addressed, fragmentable.
//!
//! `type MirrorStore = FrgmntStore<Fractal<Mirror>>`
//!
//! Stores compiled Mirror domains in a bounded in-memory cache.
//! Content-addressed by Mirror's Encode impl via Fractal wrapping.
//! No git dependency. Eviction drops — no disk persistence needed
//! because grammars are cheap to recompile from source.

use fragmentation::encoding::Encode;
use fragmentation::fragment::{Fractal, Fragmentable};
use fragmentation::frgmnt_store::FrgmntStore;
use fragmentation::ref_::Ref;
use fragmentation::sha;

use crate::ast::AstNode;
use crate::model::Mirror;
use crate::parse::Parse;
use crate::prism::Prism;
use crate::{ContentAddressed, Vector};

/// Bounded mirror domain store.
pub type MirrorStore = FrgmntStore<Fractal<Mirror>>;

/// Default capacity: 64 MB.
pub const DEFAULT_CAPACITY: usize = 64 * 1024 * 1024;

/// Open a MirrorStore at the given path with default capacity.
pub fn open(path: &str) -> Result<MirrorStore, fragmentation::frgmnt_store::Error> {
    FrgmntStore::open(path, DEFAULT_CAPACITY)
}

/// Open a MirrorStore with a specific byte capacity.
pub fn open_with_capacity(
    path: &str,
    max_bytes: usize,
) -> Result<MirrorStore, fragmentation::frgmnt_store::Error> {
    FrgmntStore::open(path, max_bytes)
}

/// Wrap a Mirror domain as a Fractal shard for storage.
fn wrap(domain: &Mirror) -> (String, Fractal<Mirror>) {
    let oid = domain.content_oid().as_ref().to_string();
    let ref_ = Ref::new(sha::Sha(oid.clone()), domain.domain_name());
    let fractal = Fractal::shard_typed(ref_, domain.clone());
    (oid, fractal)
}

/// Insert a Mirror domain into the store. Returns its content OID.
pub fn insert(store: &MirrorStore, domain: &Mirror) -> String {
    let (oid, fractal) = wrap(domain);
    let size = domain.encode().len();
    store.insert(oid.clone(), fractal, size);
    oid
}

/// Insert a Mirror domain by compiling from source. Returns (OID, Mirror).
pub fn insert_source(store: &MirrorStore, source: &str) -> Result<(String, Mirror), String> {
    let ast: Prism<AstNode> = Parse
        .trace(source.to_string())
        .into_result()
        .map_err(|e| format!("parse: {}", e))?;
    // Find the grammar node in the AST children
    let grammar_node = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .ok_or_else(|| "no grammar declaration found".to_string())?;
    let domain = Mirror::from_grammar(grammar_node)?;
    let oid = insert(store, &domain);
    Ok((oid, domain))
}

/// Retrieve a Mirror domain by OID.
pub fn get(store: &MirrorStore, oid: &str) -> Option<Mirror> {
    store.get(oid).map(|fractal| fractal.data().clone())
}

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

/// Handle `mirror db` subcommands.
pub fn cli(args: &[String]) {
    use std::process;

    if args.is_empty() {
        eprintln!("usage: mirror db init <path> <schema.conv>");
        eprintln!("       mirror db insert <path> <source.conv>");
        eprintln!("       mirror db get <path> <oid>");
        eprintln!("       mirror db status <path>");
        process::exit(1);
    }

    match args[0].as_str() {
        "init" => {
            if args.len() < 3 {
                eprintln!("usage: mirror db init <path> <schema.conv>");
                process::exit(1);
            }
            let store_path = format!("{}/.frgmnt", args[1]);
            let schema_source = std::fs::read_to_string(&args[2]).unwrap_or_else(|e| {
                eprintln!("mirror db: {}: {}", args[2], e);
                process::exit(1);
            });
            let store = open(&store_path).unwrap_or_else(|e| {
                eprintln!("mirror db: {}", e);
                process::exit(1);
            });
            match insert_source(&store, &schema_source) {
                Ok((oid, domain)) => {
                    eprintln!("  init @{} → {}", domain.domain_name(), oid);
                }
                Err(e) => {
                    eprintln!("mirror db: {}", e);
                    process::exit(1);
                }
            }
        }
        "insert" => {
            if args.len() < 3 {
                eprintln!("usage: mirror db insert <path> <source.conv>");
                process::exit(1);
            }
            let store_path = format!("{}/.frgmnt", args[1]);
            let source = std::fs::read_to_string(&args[2]).unwrap_or_else(|e| {
                eprintln!("mirror db: {}: {}", args[2], e);
                process::exit(1);
            });
            let store = open(&store_path).unwrap_or_else(|e| {
                eprintln!("mirror db: {}", e);
                process::exit(1);
            });
            match insert_source(&store, &source) {
                Ok((oid, domain)) => {
                    println!("{}\t@{}", oid, domain.domain_name());
                }
                Err(e) => {
                    eprintln!("mirror db: {}", e);
                    process::exit(1);
                }
            }
        }
        "get" => {
            if args.len() < 3 {
                eprintln!("usage: mirror db get <path> <oid>");
                process::exit(1);
            }
            let store_path = format!("{}/.frgmnt", args[1]);
            let store = open(&store_path).unwrap_or_else(|e| {
                eprintln!("mirror db: {}", e);
                process::exit(1);
            });
            match get(&store, &args[2]) {
                Some(domain) => {
                    println!("@{}", domain.domain_name());
                    for t in &domain.types {
                        let variants: Vec<&str> =
                            t.variants.iter().map(|v| v.name.as_str()).collect();
                        println!("  type {} = {}", t.name, variants.join(" | "));
                    }
                }
                None => {
                    eprintln!("mirror db: not found: {}", args[2]);
                    std::process::exit(1);
                }
            }
        }
        "status" => {
            if args.len() < 2 {
                eprintln!("usage: mirror db status <path>");
                process::exit(1);
            }
            let store_path = format!("{}/.frgmnt", args[1]);
            let store = open(&store_path).unwrap_or_else(|e| {
                eprintln!("mirror db: {}", e);
                process::exit(1);
            });
            eprintln!("  cached:   {} entries", store.cached_len());
            eprintln!("  bytes:    {} / {}", store.total_bytes(), store.capacity());
        }
        cmd => {
            eprintln!("mirror db: unknown command: {}", cmd);
            process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a Mirror directly for testing (bypasses parser).
    fn test_mirror(name: &str, types: &[&str]) -> Mirror {
        use crate::model::{DomainName, Properties, TypeDef, TypeName, Variant, VariantName};
        let ast_types: Vec<TypeDef> = types
            .iter()
            .map(|t| TypeDef {
                name: TypeName::new(*t),
                variants: vec![
                    Variant {
                        name: VariantName::new(format!("{}_a", t)),
                        params: vec![],
                    },
                    Variant {
                        name: VariantName::new(format!("{}_b", t)),
                        params: vec![],
                    },
                ],
            })
            .collect();
        Mirror {
            name: DomainName::new(name),
            types: ast_types,
            actions: vec![],
            lenses: vec![],
            extends: vec![],
            calls: vec![],
            properties: Properties::default(),
        }
    }

    #[test]
    fn mirror_store_insert_and_get() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".frgmnt");
        let store = open(path.to_str().unwrap()).unwrap();
        let domain = test_mirror("test", &["color"]);
        let oid = insert(&store, &domain);
        let retrieved = get(&store, &oid).unwrap();
        let name: &str = retrieved.domain_name().as_ref();
        assert_eq!(name, "test");
    }

    #[test]
    fn mirror_store_missing_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".frgmnt");
        let store = open(path.to_str().unwrap()).unwrap();
        assert!(get(&store, "nonexistent").is_none());
    }

    #[test]
    fn mirror_store_bounded() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".frgmnt");
        let store = open_with_capacity(path.to_str().unwrap(), 500).unwrap();
        for i in 0..20 {
            let domain = test_mirror(&format!("t{}", i), &["x"]);
            insert(&store, &domain);
        }
        assert!(store.total_bytes() <= 500);
    }

    #[test]
    fn mirror_store_content_addressed_dedup() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".frgmnt");
        let store = open(path.to_str().unwrap()).unwrap();
        let domain = test_mirror("dedup", &["x"]);
        let oid1 = insert(&store, &domain);
        let oid2 = insert(&store, &domain);
        assert_eq!(oid1, oid2);
        assert_eq!(store.cached_len(), 1);
    }
}
