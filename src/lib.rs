pub mod ast;
pub mod compile;
pub mod domain;
pub mod filter;
pub mod parse;
pub mod resolve;
pub mod tree;

// Re-export story modules as crate-level modules.
pub use story::actor;
pub use story::beat;
pub use story::identity;
pub use story::optics;
pub use story::setting;

// Re-export story types at the crate root (story vocabulary).
pub use story::actor::Actor;
pub use story::beat::Beat;
pub use story::identity::{Email, Identity, Name, Node, Signal, Signature, Signed, System};
pub use story::optics::{
    rewrite, NotFound, Prism, PrismAsTraversal, RecordingPrism, RecordingTraversal, SelectPrism,
    Traversal,
};
pub use story::setting::{Addressable, Setting};
pub use story::{
    Composed, ComposedError, ContentAddressed, Cut, CutOid, Fallback, Iso, Oid, Story, When,
};

pub use domain::beam::BeamOid;
pub use domain::filesystem::{Filesystem, Folder, FolderOid};
pub use domain::git::{Git, GitNode, GitOid};
pub use parse::{Parse, ParseError};
pub use resolve::{
    Conversation, ConversationOid, Namespace, OutputNode, Resolve, ResolveError, Template,
    TemplateProvider,
};
pub use tree::Tree;

// Re-export fragmentation's in-memory repo for downstream use.
pub use fragmentation::repo::Repo;
