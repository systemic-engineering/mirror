pub mod ast;
pub mod domain;
pub mod filter;
pub mod parse;
pub mod resolve;
pub mod tree;

// Re-export story modules as crate-level modules.
// This lets internal code use `crate::trace::ContentAddressed` etc.
pub use story::actor;
pub use story::beat;
pub use story::identity;
pub use story::traceable;
pub use story::optics;
pub use story::scene;
pub use story::trace;

// Re-export story types at the crate root (story vocabulary).
pub use story::actor::Actor;
pub use story::beat::Beat;
pub use story::identity::{Email, Identity, Name, Node, Signal, Signature, Signed, System};
pub use story::traceable::{Composed, ComposedError, Fallback, Iso, Traceable, When};
pub use story::optics::{
    rewrite, NotFound, Prism, PrismAsTraversal, TracingPrism, SelectPrism, Traversal,
    TracingTraversal,
};
pub use story::scene::{Addressable, Scene};
pub use story::trace::{ContentAddressed, Event, Oid, Session, Trace, TraceOid};

pub use domain::filesystem::{Filesystem, Folder};
pub use domain::git::{Git, GitNode};
pub use parse::{Parse, ParseError};
pub use resolve::{Conversation, Resolve, ResolveError};
pub use tree::Tree;
