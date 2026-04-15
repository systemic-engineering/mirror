//! mirror — fold | prism | traversal | lens | iso — the thing you look into that looks back.
//!
//! `.mirror` files → content-addressed AST → compiled modules → verified domains.
//!
//! ## Architecture
//!
//! - **ast** — The mirror AST (Atom/Ref/Body/Call/Prism)
//! - **mirror_runtime** — Spectral content-addressed compilation pipeline
//! - **kernel** — Content addressing, Oid, Trace, Vector trait hierarchy
//! - **lambda_phases** — Parse.then(Resolve).then(Properties).then(Emit)
//! - **dispatch** — Value, Args, Response
//!
//! ## Re-exports
//!
//! Kernel types (`Oid`, `Trace`, `Vector`, `ContentAddressed`) are re-exported at crate root.
//! The `prism` crate is aliased as `beam` for BEAM integration.

pub extern crate prism as prism_crate;
pub use prism_crate as beam;

#[macro_use]
pub mod kernel;
pub mod ast;
pub mod ast_prism;
pub mod dispatch;
pub mod domain;
pub mod prism;

// Kernel types at the crate root.
pub use kernel::{
    Addressable, Composed, ComposedError, ContentAddressed, Latent, Oid, Setting, Trace, TraceOid,
    Vector,
};

pub use domain::filesystem::{Filesystem, Folder, FolderOid};
pub use prism::Prism;
pub use store::{ForeignKey, MirrorOid, Shard};

// Fragmentation traits for content-addressed storage.
pub use fragmentation::repo::Repo;
pub use fragmentation::store::Store;

pub mod bundle;
pub mod cli;
pub mod declaration;
pub mod emit_code;
pub mod emit_rust;
pub mod generate_crate;
pub mod generated;
pub mod lambda_phases;
pub mod git_prism;
pub mod git_store;
pub mod loss;
pub mod lsp;
pub mod mirror_ast;
pub mod mirror_runtime;
pub mod parse;
pub mod shard;
pub mod shatter_format;
#[cfg(feature = "git")]
pub mod sign;
pub mod spec;
pub mod store;
