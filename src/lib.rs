#[macro_use]
pub mod kernel;
pub mod ast;
pub mod compile;
pub mod domain;
pub mod filter;
pub mod parse;
pub mod resolve;
pub mod tree;

// Re-export kernel types at the crate root.
pub use kernel::{
    Addressable, Composed, ComposedError, ContentAddressed, Latent, Oid, Setting, Trace, TraceOid,
    Vector,
};

pub use domain::filesystem::{Filesystem, Folder, FolderOid};
pub use parse::{Parse, ParseError};
pub use resolve::{
    Conversation, ConversationOid, Namespace, OutputNode, Resolve, ResolveError, Template,
    TemplateProvider,
};
pub use tree::Tree;

// Re-export fragmentation's in-memory repo for downstream use.
pub use fragmentation::repo::Repo;
