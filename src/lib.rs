#[macro_use]
pub mod kernel;
pub mod ast;
pub mod compile;
pub mod domain;
pub mod ffi;
pub mod filter;
pub mod packages;
pub mod parse;
pub mod prism;
pub mod resolve;

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
