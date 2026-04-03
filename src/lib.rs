pub extern crate prism as prism_crate;
pub use prism_crate as beam;

#[macro_use]
pub mod kernel;
pub mod actor;
pub mod ast;
pub mod check;
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

// Re-export kernel types at the crate root.
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

// Re-export fragmentation's Repo trait and Store implementation.
pub use fragmentation::repo::Repo;
pub use fragmentation::store::Store;

#[cfg(feature = "db")]
pub mod db;

#[cfg(feature = "lsp")]
pub mod lsp;

pub mod model;
pub mod runtime;

pub use model::Domain;
pub use runtime::DomainMessage;
