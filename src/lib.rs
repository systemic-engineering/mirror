//! conversation — Typed transformation pipeline language.
//!
//! `.conv` files → content-addressed AST → compiled modules → GenServer → execute.
//!
//! ## Architecture
//!
//! - **parse** — Source → AST tree (frozen bootstrap parser)
//! - **resolve** — Grammar type checking, namespace validation
//! - **compile** — AST → BEAM Erlang Abstract Form (EAF)
//! - **kernel** — Content addressing, Oid, Trace, Vector trait hierarchy
//! - **runtime** — Async compilation + spawn via ractor
//! - **artifact** — Bounded storage for compiled modules (Pressure-based eviction)
//! - **boot** — Parallel multi-layer boot sequence from `boot/` directory
//!
//! ## Re-exports
//!
//! Kernel types (`Oid`, `Trace`, `Vector`, `ContentAddressed`) are re-exported at crate root.
//! The `prism` crate is aliased as `beam` for BEAM integration.

pub extern crate prism as prism_crate;
pub use prism_crate as beam;

#[macro_use]
pub mod kernel;
pub mod abyss;
pub mod actor;
pub mod artifact;
pub mod ast;
pub mod boot;
pub mod check;
pub mod classifier;
pub mod compile;
pub mod domain;
pub mod ffi;
pub mod filter;
pub mod generate;
pub mod logic;
pub mod packages;
pub mod parse;
pub mod prism;
pub mod property;
pub mod resolve;
pub mod spectral;

// Kernel types at the crate root.
pub use kernel::{
    Addressable, Composed, ComposedError, ContentAddressed, Latent, Oid, Setting, Trace, TraceOid,
    Vector,
};

pub use domain::filesystem::{Filesystem, Folder, FolderOid};
pub use parse::{Parse, ParseError};
pub use prism::Prism;
pub use resolve::{
    Conversation, ConversationOid, Namespace, OutputNode, Resolve, ResolveError, Template,
    TemplateProvider,
};

// Fragmentation traits for content-addressed storage.
pub use fragmentation::repo::Repo;
pub use fragmentation::store::Store;

#[cfg(feature = "db")]
pub mod db;

pub mod model;
pub mod runtime;

pub use model::Domain;
pub use runtime::DomainMessage;
pub use runtime::Runtime;
