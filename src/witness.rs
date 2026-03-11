use std::cell::RefCell;

use sha2::{Digest, Sha256};

// ---------------------------------------------------------------------------
// Oid — value type. The content address itself.
// ---------------------------------------------------------------------------

/// A content address. Value type — wraps the hash.
///
/// Same content = same Oid. That's the whole idea.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Oid(String);

impl Oid {
    pub fn new(hash: impl Into<String>) -> Self {
        Oid(hash.into())
    }
}

impl AsRef<str> for Oid {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Oid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// ContentAddressed — anything that can produce a content address.
// ---------------------------------------------------------------------------

/// Anything that can produce a content address.
///
/// The fundamental property of data in Conversation:
/// same content = same Oid. Trees get this from fragmentation.
/// Events get this from their SHA. Everything is addressable.
pub trait ContentAddressed {
    fn content_oid(&self) -> Oid;
}

// ---------------------------------------------------------------------------
// Trace — the self-similar witness tree.
// ---------------------------------------------------------------------------

/// The trace of a gradient application. Self-similar — it's trees.
///
/// Leaf: a single transformation step. One gradient, one result.
/// Branch: a composed transformation. Children are the sub-traces.
///
/// Every node carries a result AND an Oid. Both success and failure
/// are witnessed. Both are content-addressed.
#[derive(Clone, Debug, PartialEq)]
pub enum Trace<T, E> {
    Leaf {
        result: Result<T, E>,
        oid: Oid,
    },
    Branch {
        result: Result<T, E>,
        oid: Oid,
        children: Vec<Trace<T, E>>,
    },
}

impl<T, E> Trace<T, E> {
    /// Create a leaf trace.
    pub fn leaf(result: Result<T, E>, oid: Oid) -> Self {
        Trace::Leaf { result, oid }
    }

    /// Create a branch trace with children.
    pub fn branch(result: Result<T, E>, oid: Oid, children: Vec<Trace<T, E>>) -> Self {
        Trace::Branch {
            result,
            oid,
            children,
        }
    }

    /// The result of the transformation.
    pub fn result(&self) -> &Result<T, E> {
        match self {
            Trace::Leaf { result, .. } => result,
            Trace::Branch { result, .. } => result,
        }
    }

    /// The content address of this trace node.
    pub fn oid(&self) -> &Oid {
        match self {
            Trace::Leaf { oid, .. } => oid,
            Trace::Branch { oid, .. } => oid,
        }
    }

    /// The children of this trace (empty for Leaf).
    pub fn children(&self) -> &[Trace<T, E>] {
        match self {
            Trace::Leaf { .. } => &[],
            Trace::Branch { children, .. } => children,
        }
    }

    /// Is this a leaf node?
    pub fn is_leaf(&self) -> bool {
        matches!(self, Trace::Leaf { .. })
    }

    /// Is this a branch node?
    pub fn is_branch(&self) -> bool {
        matches!(self, Trace::Branch { .. })
    }

    /// Is the result Ok?
    pub fn is_ok(&self) -> bool {
        self.result().is_ok()
    }

    /// Is the result Err?
    pub fn is_err(&self) -> bool {
        self.result().is_err()
    }

    /// Consume the trace, returning just the Result.
    pub fn into_result(self) -> Result<T, E> {
        match self {
            Trace::Leaf { result, .. } => result,
            Trace::Branch { result, .. } => result,
        }
    }

    /// Consume the trace, returning the value or panicking on error.
    pub fn unwrap(self) -> T
    where
        E: std::fmt::Debug,
    {
        match self.into_result() {
            Ok(v) => v,
            Err(e) => panic!("called `Trace::unwrap()` on an Err value: {:?}", e),
        }
    }
}

// ---------------------------------------------------------------------------
// ContentAddressed impls for standard types.
// ---------------------------------------------------------------------------

impl ContentAddressed for String {
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        hasher.update(self.as_bytes());
        Oid::new(hex::encode(hasher.finalize()))
    }
}

impl<A: ContentAddressed> ContentAddressed for Vec<A> {
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        hasher.update(b"vec:");
        for item in self {
            hasher.update(item.content_oid().as_ref().as_bytes());
        }
        Oid::new(hex::encode(hasher.finalize()))
    }
}

impl<A: ContentAddressed> ContentAddressed for Option<A> {
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        match self {
            Some(a) => {
                hasher.update(b"some:");
                hasher.update(a.content_oid().as_ref().as_bytes());
            }
            None => {
                hasher.update(b"none");
            }
        }
        Oid::new(hex::encode(hasher.finalize()))
    }
}

impl<A: ContentAddressed, B: ContentAddressed> ContentAddressed for (A, B) {
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        hasher.update(b"tuple:");
        hasher.update(self.0.content_oid().as_ref().as_bytes());
        hasher.update(b",");
        hasher.update(self.1.content_oid().as_ref().as_bytes());
        Oid::new(hex::encode(hasher.finalize()))
    }
}

impl ContentAddressed for i32 {
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        hasher.update(self.to_le_bytes());
        Oid::new(hex::encode(hasher.finalize()))
    }
}

impl ContentAddressed for f64 {
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        hasher.update(self.to_le_bytes());
        Oid::new(hex::encode(hasher.finalize()))
    }
}

/// Content-addressed event. The atom of observation.
///
/// Every observation becomes an event.
/// Events chain through parent links — a session is a commit log.
/// The session emits these. They are the record of the conversation.
///
/// Parametrized over data type, like Tree<E>. Default is raw bytes.
#[derive(Debug, Clone, PartialEq)]
pub struct Event<E = Vec<u8>> {
    /// Content address (SHA-256 of data + message + parent)
    pub sha: String,
    /// The event payload
    pub data: E,
    /// Annotation / signal kind
    pub message: String,
    /// Parent event SHA (chain link)
    pub parent: Option<String>,
}

impl<E: AsRef<[u8]>> Event<E> {
    pub fn new(data: E, message: impl Into<String>, parent: Option<String>) -> Self {
        let message = message.into();
        let sha = Self::compute_sha(data.as_ref(), &message, parent.as_deref());
        Event {
            sha,
            data,
            message,
            parent,
        }
    }

    fn compute_sha(data: &[u8], message: &str, parent: Option<&str>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(message.as_bytes());
        if let Some(p) = parent {
            hasher.update(p.as_bytes());
        }
        hex::encode(hasher.finalize())
    }
}

impl<E> ContentAddressed for Event<E> {
    fn content_oid(&self) -> Oid {
        Oid::new(&self.sha)
    }
}

/// A session. The witness.
///
/// Events go in through `record`. They chain through parent links.
/// The session IS the concept — Trace carries the witness structure,
/// Session accumulates the record.
///
/// Uses interior mutability: events arrive through &self.
#[derive(Debug)]
pub struct Session {
    pub name: String,
    pub author: String,
    store: RefCell<Vec<Event>>,
    head: RefCell<Option<String>>,
}

impl Session {
    pub fn new(name: impl Into<String>, author: impl Into<String>) -> Self {
        Session {
            name: name.into(),
            author: author.into(),
            store: RefCell::new(Vec::new()),
            head: RefCell::new(None),
        }
    }

    /// Record a content-addressed event in the chain.
    pub fn record(&self, message: impl Into<String>, data: Vec<u8>) -> String {
        let event = Event::new(data, message, self.head.borrow().clone());
        let sha = event.sha.clone();
        *self.head.borrow_mut() = Some(sha.clone());
        self.store.borrow_mut().push(event);
        sha
    }

    /// Seal the session with a root commit.
    /// References all prior event SHAs. Returns the root SHA.
    pub fn seal(&self, message: impl Into<String>) -> String {
        let event_shas: Vec<String> = self.store.borrow().iter().map(|e| e.sha.clone()).collect();
        let data = event_shas.join(",").into_bytes();
        self.record(message, data)
    }

    pub fn head(&self) -> Option<String> {
        self.head.borrow().clone()
    }

    /// The events this session has emitted.
    pub fn events(&self) -> Vec<Event> {
        self.store.borrow().clone()
    }

    pub fn len(&self) -> usize {
        self.store.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.borrow().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- Oid value type --

    #[test]
    fn oid_wraps_hash() {
        let oid = Oid::new("abc123");
        assert_eq!(oid.as_ref(), "abc123");
    }

    #[test]
    fn oid_display() {
        let oid = Oid::new("abc123");
        assert_eq!(format!("{}", oid), "abc123");
    }

    #[test]
    fn oid_equality() {
        let a = Oid::new("abc123");
        let b = Oid::new("abc123");
        let c = Oid::new("def456");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn oid_clone_hash() {
        use std::collections::HashSet;
        let a = Oid::new("abc123");
        let b = a.clone();
        assert_eq!(a, b);
        let mut set = HashSet::new();
        set.insert(a);
        assert!(set.contains(&b));
    }

    // -- ContentAddressed --

    #[test]
    fn event_content_oid_matches_sha() {
        let e = Event::new(b"hello".to_vec(), "test", None);
        let oid = e.content_oid();
        assert_eq!(oid.as_ref(), &e.sha);
    }

    #[test]
    fn same_event_same_content_oid() {
        let a = Event::new(b"hello".to_vec(), "test", None);
        let b = Event::new(b"hello".to_vec(), "test", None);
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn different_event_different_content_oid() {
        let a = Event::new(b"hello".to_vec(), "test", None);
        let b = Event::new(b"world".to_vec(), "test", None);
        assert_ne!(a.content_oid(), b.content_oid());
    }

    // -- Trace --

    #[test]
    fn trace_leaf_construction() {
        let t: Trace<i32, ()> = Trace::leaf(Ok(42), Oid::new("abc"));
        assert!(t.is_leaf());
        assert!(!t.is_branch());
        assert_eq!(t.result(), &Ok(42));
        assert_eq!(t.oid(), &Oid::new("abc"));
        assert!(t.children().is_empty());
    }

    #[test]
    fn trace_branch_construction() {
        let child = Trace::leaf(Ok(21), Oid::new("child1"));
        let t: Trace<i32, ()> = Trace::branch(Ok(42), Oid::new("root"), vec![child]);
        assert!(t.is_branch());
        assert!(!t.is_leaf());
        assert_eq!(t.result(), &Ok(42));
        assert_eq!(t.oid(), &Oid::new("root"));
        assert_eq!(t.children().len(), 1);
    }

    #[test]
    fn trace_is_self_similar() {
        let leaf1 = Trace::leaf(Ok(1), Oid::new("a"));
        let leaf2 = Trace::leaf(Ok(2), Oid::new("b"));
        let inner = Trace::branch(Ok(3), Oid::new("inner"), vec![leaf1, leaf2]);
        let outer: Trace<i32, ()> = Trace::branch(Ok(6), Oid::new("outer"), vec![inner]);
        assert_eq!(outer.children().len(), 1);
        assert!(outer.children()[0].is_branch());
        assert_eq!(outer.children()[0].children().len(), 2);
    }

    #[test]
    fn trace_err_is_witnessed() {
        let t: Trace<i32, &str> = Trace::leaf(Err("failed"), Oid::new("err-oid"));
        assert!(t.is_err());
        assert!(!t.is_ok());
        assert_eq!(t.result(), &Err("failed"));
        assert_eq!(t.oid(), &Oid::new("err-oid"));
    }

    #[test]
    fn trace_into_result() {
        let t: Trace<i32, ()> = Trace::leaf(Ok(42), Oid::new("abc"));
        assert_eq!(t.into_result(), Ok(42));

        let t: Trace<i32, ()> = Trace::branch(Ok(99), Oid::new("root"), vec![]);
        assert_eq!(t.into_result(), Ok(99));
    }

    #[test]
    fn trace_unwrap_returns_value() {
        let t: Trace<i32, &str> = Trace::leaf(Ok(42), Oid::new("abc"));
        assert_eq!(t.unwrap(), 42);
    }

    #[test]
    #[should_panic(expected = "called `Trace::unwrap()` on an Err")]
    fn trace_unwrap_panics_on_err() {
        let t: Trace<i32, &str> = Trace::leaf(Err("boom"), Oid::new("err"));
        t.unwrap();
    }

    #[test]
    fn trace_clone_eq() {
        let a: Trace<i32, ()> = Trace::leaf(Ok(42), Oid::new("abc"));
        let b = a.clone();
        assert_eq!(a, b);
    }

    // -- String ContentAddressed --

    #[test]
    fn string_content_addressed() {
        let a = "hello".to_string();
        let b = "hello".to_string();
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn string_different_content_different_oid() {
        let a = "hello".to_string();
        let b = "world".to_string();
        assert_ne!(a.content_oid(), b.content_oid());
    }

    // -- Vec ContentAddressed --

    #[test]
    fn vec_content_addressed_same() {
        let a = vec![1i32, 2, 3];
        let b = vec![1i32, 2, 3];
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn vec_content_addressed_different() {
        let a = vec![1i32, 2, 3];
        let b = vec![1i32, 2, 4];
        assert_ne!(a.content_oid(), b.content_oid());
    }

    // -- Option ContentAddressed --

    #[test]
    fn option_content_addressed() {
        let a: Option<i32> = Some(42);
        let b: Option<i32> = Some(42);
        let c: Option<i32> = None;
        assert_eq!(a.content_oid(), b.content_oid());
        assert_ne!(a.content_oid(), c.content_oid());
    }

    // -- Tuple ContentAddressed --

    #[test]
    fn tuple_content_addressed() {
        let a = (1i32, 2i32);
        let b = (1i32, 2i32);
        let c = (1i32, 3i32);
        assert_eq!(a.content_oid(), b.content_oid());
        assert_ne!(a.content_oid(), c.content_oid());
    }

    // -- Event tests --

    #[test]
    fn event_is_content_addressed() {
        let a = Event::new(b"hello".to_vec(), "test", None);
        let b = Event::new(b"hello".to_vec(), "test", None);
        assert_eq!(a.sha, b.sha);
    }

    #[test]
    fn different_content_different_sha() {
        let a = Event::new(b"hello".to_vec(), "test", None);
        let b = Event::new(b"world".to_vec(), "test", None);
        assert_ne!(a.sha, b.sha);
    }

    #[test]
    fn event_parent_changes_sha() {
        let a = Event::new(b"hello".to_vec(), "test", None);
        let b = Event::new(b"hello".to_vec(), "test", Some("parent".into()));
        assert_ne!(a.sha, b.sha);
    }

    // -- Session tests --

    #[test]
    fn session_starts_empty() {
        let s = Session::new("test", "reed");
        assert!(s.is_empty());
        assert_eq!(s.head(), None);
    }

    #[test]
    fn session_head_tracks_latest() {
        let s = Session::new("test", "reed");
        s.record("step", b"data".to_vec());
        assert!(s.head().is_some());
    }

    #[test]
    fn session_events_chain() {
        let s = Session::new("test", "reed");
        s.record("first", b"a".to_vec());
        s.record("second", b"b".to_vec());

        let events = s.events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].parent, None);
        assert_eq!(events[1].parent, Some(events[0].sha.clone()));
    }

    #[test]
    fn event_carries_message() {
        let s = Session::new("test", "reed");
        s.record("hello", b"data".to_vec());
        assert_eq!(s.events()[0].message, "hello");
    }

    #[test]
    fn session_seal_produces_root() {
        let s = Session::new("test", "reed");
        s.record("step", b"data".to_vec());

        let root = s.seal("@seal");
        assert_eq!(s.head().as_deref(), Some(root.as_str()));
        assert_eq!(s.len(), 2); // event + root
    }

    #[test]
    fn session_seal_references_all_events() {
        let s = Session::new("test", "reed");
        s.record("first", b"a".to_vec());
        s.record("second", b"b".to_vec());
        s.seal("@seal");

        let events = s.events();
        let root = events.last().unwrap();
        let root_data = String::from_utf8(root.data.clone()).unwrap();
        assert!(root_data.contains(&events[0].sha));
        assert!(root_data.contains(&events[1].sha));
    }

    #[test]
    fn same_data_different_event_by_position() {
        let s = Session::new("test", "reed");
        s.record("step", b"same".to_vec());
        s.record("step", b"same".to_vec());

        let events = s.events();
        // Same data (first order)
        assert_eq!(events[0].data, events[1].data);
        // Different SHA (second order — position in chain)
        assert_ne!(events[0].sha, events[1].sha);
    }
}
