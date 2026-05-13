//! mirror — fold | prism | traversal | lens | iso — the thing you look into that looks back.
//!
//! `.mirror` files → content-addressed AST → compiled modules → verified domains.
//!
//! ## Architecture
//!
//! - **mirror_ast** — The typed AST (7 variants, 5 operations)
//! - **kernel** — Content addressing, Oid, Trace, Vector trait hierarchy
//! - **dirac** — Eigenvalues, spectral triples, Connes distance
//! - **prism** — Content-addressed tree primitive
//!
//! Everything else is grammar. The grammars exist. The tokenizer is next.

pub extern crate prism as prism_crate;
pub use prism_crate as beam;

#[macro_use]
pub mod kernel;
pub mod cli;
pub mod dirac;
pub mod mirror_ast;
pub mod prism;
pub mod tokenize;

// Kernel types at the crate root.
pub use kernel::{
    Addressable, Composed, ComposedError, ContentAddressed, Latent, Oid, Setting, Trace, TraceOid,
    Vector,
};

pub use prism::Prism;
