pub mod ast;
pub mod domain;
pub mod filter;
pub mod parse;
pub mod resolve;
pub mod tree;

// Re-export story modules as crate-level modules.
// This lets internal code use `crate::narrative::ContentAddressed` etc.
pub use story::actor;
pub use story::beat;
pub use story::identity;
pub use story::optics;
pub use story::setting;
pub use story::narrative;
pub use story::narrate;

// Re-export story types at the crate root (story vocabulary).
pub use story::actor::Actor;
pub use story::beat::Beat;
pub use story::identity::{Email, Identity, Name, Node, Signal, Signature, Signed, System};
pub use story::optics::{
    rewrite, NotFound, Prism, PrismAsTraversal, SelectPrism, NarratingPrism, NarratingTraversal,
    Traversal,
};
pub use story::setting::{Addressable, Setting};
pub use story::narrative::{ContentAddressed, Event, EventOid, Oid, Session, Narrative, NarrativeOid};
pub use story::narrate::{Composed, ComposedError, Fallback, Iso, Story, When};

pub use domain::beam::BeamOid;
pub use domain::filesystem::{Filesystem, Folder, FolderOid};
pub use domain::git::{Git, GitNode, GitOid};
pub use parse::{Parse, ParseError};
pub use resolve::{Conversation, ConversationOid, Resolve, ResolveError};
pub use tree::Tree;
