use std::cell::RefCell;

use sha2::{Digest, Sha256};

use crate::gradient::Gradient;

/// Legacy content-addressing trait. Being replaced by ContentAddressed + Oid value type.
pub trait LegacyOid {
    fn oid(&self) -> String;
}

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
}

/// Direction of a gradient application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Emit,
    Absorb,
}

/// An observation: what a gradient saw and produced.
///
/// Observations always go to a Session.
/// That's what makes it a session.
#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    /// Content address of the input
    pub input: String,
    /// Content address of the output
    pub output: String,
    /// Name of the gradient that ran
    pub gradient: String,
    /// Direction: emit or absorb
    pub direction: Direction,
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

impl<E> LegacyOid for Event<E> {
    fn oid(&self) -> String {
        self.sha.clone()
    }
}

impl<E> ContentAddressed for Event<E> {
    fn content_oid(&self) -> Oid {
        Oid::new(&self.sha)
    }
}

/// A session. The witness.
///
/// Observations go in. Events come out.
/// Observation is what brings forth a session.
/// An actor on their own cannot produce events.
///
/// The session is the only witness. There is no trait —
/// the session IS the concept.
///
/// Uses interior mutability: observations arrive through
/// &self because Gradient::emit operates on shared references.
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

    /// Receive an observation. Convert it to a content-addressed
    /// event in the chain. This is how events are born.
    pub fn observe(&self, observation: &Observation) {
        let dir = match observation.direction {
            Direction::Emit => "emit",
            Direction::Absorb => "absorb",
        };
        let data = format!("{}:{}:{}", dir, observation.input, observation.output);
        self.record(&observation.gradient, data.into_bytes());
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

    // -- Private: the only way to produce events is through observe or seal --

    fn record(&self, message: impl Into<String>, data: Vec<u8>) -> String {
        let event = Event::new(data, message, self.head.borrow().clone());
        let sha = event.sha.clone();
        *self.head.borrow_mut() = Some(sha.clone());
        self.store.borrow_mut().push(event);
        sha
    }
}

/// Witnessed gradient. Wraps any gradient, records to a session.
///
/// When a gradient is Witnessed, every emit and absorb
/// produces an Observation sent to the Session.
/// Zero-cost when the gradient isn't wrapped.
pub struct Witnessed<'a, G> {
    pub gradient: G,
    pub session: &'a Session,
    pub name: String,
}

impl<'a, G> Witnessed<'a, G> {
    pub fn new(gradient: G, session: &'a Session, name: impl Into<String>) -> Self {
        Witnessed {
            gradient,
            session,
            name: name.into(),
        }
    }
}

impl<A, B, G> Gradient<A, B> for Witnessed<'_, G>
where
    A: LegacyOid,
    B: LegacyOid,
    G: Gradient<A, B>,
{
    type Error = G::Error;

    fn emit(&self, source: A) -> Result<B, Self::Error> {
        let input_oid = source.oid();
        let result = self.gradient.emit(source)?;
        let output_oid = result.oid();
        self.session.observe(&Observation {
            input: input_oid,
            output: output_oid,
            gradient: self.name.clone(),
            direction: Direction::Emit,
        });
        Ok(result)
    }

    fn absorb(&self, source: B) -> Result<A, Self::Error> {
        let input_oid = source.oid();
        let result = self.gradient.absorb(source)?;
        let output_oid = result.oid();
        self.session.observe(&Observation {
            input: input_oid,
            output: output_oid,
            gradient: self.name.clone(),
            direction: Direction::Absorb,
        });
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gradient::Gradient;

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
        let t: Trace<i32, ()> = Trace::branch(
            Ok(42),
            Oid::new("root"),
            vec![child],
        );
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
        let outer: Trace<i32, ()> = Trace::branch(
            Ok(6),
            Oid::new("outer"),
            vec![inner],
        );
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
    fn trace_clone_eq() {
        let a: Trace<i32, ()> = Trace::leaf(Ok(42), Oid::new("abc"));
        let b = a.clone();
        assert_eq!(a, b);
    }

    // -- LegacyOid impls for test types --

    impl LegacyOid for i32 {
        fn oid(&self) -> String {
            let mut hasher = Sha256::new();
            hasher.update(self.to_le_bytes());
            hex::encode(hasher.finalize())
        }
    }

    // -- Test gradient --

    struct Double;
    impl Gradient<i32, i32> for Double {
        type Error = ();
        fn emit(&self, source: i32) -> Result<i32, ()> {
            Ok(source * 2)
        }
        fn absorb(&self, source: i32) -> Result<i32, ()> {
            Ok(source / 2)
        }
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
    fn event_oid_is_sha() {
        let e = Event::new(b"hello".to_vec(), "test", None);
        assert_eq!(e.oid(), e.sha);
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
        let g = Witnessed::new(Double, &s, "double");
        g.emit(3).unwrap();

        assert!(s.head().is_some());
    }

    #[test]
    fn session_seal_produces_root() {
        let s = Session::new("test", "reed");
        let g = Witnessed::new(Double, &s, "double");
        g.emit(3).unwrap();

        let root = s.seal("@seal");
        assert_eq!(s.head().as_deref(), Some(root.as_str()));
        assert_eq!(s.len(), 2); // observation + root
    }

    #[test]
    fn session_seal_references_all_events() {
        let s = Session::new("test", "reed");
        let g = Witnessed::new(Double, &s, "double");
        g.emit(3).unwrap();
        g.emit(5).unwrap();
        s.seal("@seal");

        let events = s.events();
        let root = events.last().unwrap();
        let root_data = String::from_utf8(root.data.clone()).unwrap();
        // Root contains the SHAs of all prior events
        assert!(root_data.contains(&events[0].sha));
        assert!(root_data.contains(&events[1].sha));
    }

    // -- The session IS the witness --

    #[test]
    fn observation_brings_forth_events() {
        let session = Session::new("test", "reed");
        let g = Witnessed::new(Double, &session, "double");

        let result = g.emit(3).unwrap();
        assert_eq!(result, 6);

        assert_eq!(session.len(), 1);
    }

    #[test]
    fn observations_chain_as_events() {
        let session = Session::new("test", "reed");
        let g = Witnessed::new(Double, &session, "double");

        g.emit(3).unwrap();
        g.emit(5).unwrap();

        let events = session.events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].parent, None);
        assert_eq!(events[1].parent, Some(events[0].sha.clone()));
    }

    #[test]
    fn event_carries_gradient_name() {
        let session = Session::new("test", "reed");
        let g = Witnessed::new(Double, &session, "double");
        g.emit(3).unwrap();

        let events = session.events();
        assert_eq!(events[0].message, "double");
    }

    #[test]
    fn event_carries_observation_data() {
        let session = Session::new("test", "reed");
        let g = Witnessed::new(Double, &session, "double");
        g.emit(3).unwrap();

        let events = session.events();
        let data = String::from_utf8(events[0].data.clone()).unwrap();
        assert!(data.starts_with("emit:"));
        assert!(data.contains(&3i32.oid()));
        assert!(data.contains(&6i32.oid()));
    }

    #[test]
    fn absorb_observed() {
        let session = Session::new("test", "reed");
        let g = Witnessed::new(Double, &session, "double");
        g.absorb(6).unwrap();

        let events = session.events();
        let data = String::from_utf8(events[0].data.clone()).unwrap();
        assert!(data.starts_with("absorb:"));
    }

    // Shared test stub: always fails. One type = one monomorphization.
    struct Fails;
    impl Gradient<i32, i32> for Fails {
        type Error = ();
        fn emit(&self, _: i32) -> Result<i32, ()> {
            Err(())
        }
        fn absorb(&self, _: i32) -> Result<i32, ()> {
            Err(())
        }
    }

    #[test]
    fn error_produces_no_event() {
        let session = Session::new("test", "reed");
        let g = Witnessed::new(Fails, &session, "fails");

        assert!(g.emit(1).is_err());
        assert!(g.absorb(1).is_err());
        assert!(session.is_empty());
    }

    // -- Composition traces through the session --

    #[test]
    fn composition_traces_through_session() {
        let session = Session::new("test", "reed");
        let g1 = Witnessed::new(Double, &session, "first");
        let g2 = Witnessed::new(Double, &session, "second");

        let mid = g1.emit(3).unwrap();
        let _result = g2.emit(mid).unwrap();

        let events = session.events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].message, "first");
        assert_eq!(events[1].message, "second");
        assert_eq!(events[1].parent, Some(events[0].sha.clone()));
    }

    #[test]
    fn seals_after_observations() {
        let session = Session::new("test", "reed");
        let g = Witnessed::new(Double, &session, "double");

        g.emit(3).unwrap();
        g.emit(5).unwrap();
        let root = session.seal("@done");

        assert_eq!(session.len(), 3); // 2 observations + root
        assert_eq!(session.head().as_deref(), Some(root.as_str()));
    }

    // -- Second-order observation --

    #[test]
    fn same_observation_different_event() {
        let session = Session::new("test", "reed");
        let g = Witnessed::new(Double, &session, "double");

        g.emit(3).unwrap();
        g.emit(3).unwrap();

        let events = session.events();
        // Same observation data (first order)
        assert_eq!(events[0].data, events[1].data);
        // Different event SHA (second order — position in the chain)
        assert_ne!(events[0].sha, events[1].sha);
    }
}
