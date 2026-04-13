//! Store — the universal content-addressed store trait.
//!
//! You give it a value. It gives you back a shard. The OID is computed
//! from the content — never provided by the caller.
//!
//! The store is ternary: get can return Success (found, fresh),
//! Partial (found, stale), or Failure (not found).
//!
//! ## MirrorOid
//!
//! The canonical content address. SHA-512 by default (mirror's Oid).
//! When git integration needs SHA-1 bridging, the generic `MirrorOid`
//! will parameterize over `HashAlg`. For now, it's a newtype over Oid.
//!
//! ## Shard
//!
//! A value paired with its content address. The store returns shards,
//! never raw values — you always know how something was addressed.
//!
//! ## ForeignKey
//!
//! Bridge between hash domains. A shard addressed by coincidence hash
//! can carry a foreign key to git's SHA-1 world. Home produces visitors.
//! Visitors don't produce home.

use prism::{Imperfect, Loss};

use crate::kernel::{ContentAddressed, Oid};

// ---------------------------------------------------------------------------
// MirrorOid — the canonical content address
// ---------------------------------------------------------------------------

/// The canonical mirror content address.
///
/// Currently a newtype over SHA-512 [`Oid`]. When git integration
/// requires bridging to SHA-1, this becomes `MirrorOid<H: HashAlg>`.
/// The default is always coincidence's spectral hash.
///
/// ```text
/// MirrorOid              // SHA-512. Home.
/// MirrorOid              // (future: MirrorOid<SHA1> for git. Visiting.)
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MirrorOid(Oid);

impl MirrorOid {
    /// Wrap an existing Oid.
    pub fn new(oid: Oid) -> Self {
        MirrorOid(oid)
    }

    /// Hash raw bytes to produce a MirrorOid.
    pub fn hash(data: &[u8]) -> Self {
        MirrorOid(Oid::hash(data))
    }

    /// Access the inner Oid.
    pub fn as_oid(&self) -> &Oid {
        &self.0
    }
}

impl From<Oid> for MirrorOid {
    fn from(oid: Oid) -> Self {
        MirrorOid(oid)
    }
}

impl AsRef<str> for MirrorOid {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::fmt::Display for MirrorOid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// Shard — value + content address
// ---------------------------------------------------------------------------

/// A value paired with its content address.
///
/// The store produces shards — you never get a value without knowing
/// how it was addressed. The OID is computed from content on insert.
#[derive(Clone, Debug, PartialEq)]
pub struct Shard<V> {
    /// The stored value.
    pub value: V,
    /// The content address of the value.
    pub oid: MirrorOid,
}

impl<V> Shard<V> {
    /// Create a new shard from a value and its computed OID.
    pub fn new(value: V, oid: MirrorOid) -> Self {
        Shard { value, oid }
    }
}

impl<V: Clone> ContentAddressed for Shard<V> {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        self.oid.as_oid().clone()
    }
}

// ---------------------------------------------------------------------------
// ForeignKey — bridge between hash domains
// ---------------------------------------------------------------------------

/// Bridge between hash domains.
///
/// A shard addressed by mirror's hash can carry a foreign key to another
/// hash domain (e.g., git's SHA-1). Home produces visitors. Visitors
/// don't produce home.
pub trait ForeignKey {
    /// The foreign hash as a hex string, if this shard has one.
    fn foreign_hex(&self) -> Option<&str>;
}

// ---------------------------------------------------------------------------
// Store trait
// ---------------------------------------------------------------------------

/// Content-addressed storage. Insert values, retrieve by OID.
///
/// The store computes the content address on insert — the caller never
/// provides an OID. This prevents hash mismatches by construction.
///
/// Returns are ternary via [`Imperfect`]:
/// - `Success(shard)` — fresh, verified.
/// - `Partial(shard, loss)` — found but stale, or retrieved with degradation.
/// - `Failure(error, loss)` — not found, or storage failure.
pub trait Store {
    /// The value type being stored.
    type Value;
    /// The content-addressed wrapper returned by the store.
    type Shard: ContentAddressed;
    /// The error type for storage operations.
    type Error;
    /// The loss type measuring storage degradation.
    type Loss: Loss;

    /// Insert a value. The store computes the OID from content.
    /// Returns the shard — the value wrapped with its content address.
    fn insert(&mut self, value: Self::Value) -> Imperfect<Self::Shard, Self::Error, Self::Loss>;

    /// Retrieve by content address.
    fn get(&self, oid: &MirrorOid) -> Imperfect<Self::Shard, Self::Error, Self::Loss>;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use prism::Loss;
    use std::collections::HashMap;

    // -- Test loss type --

    #[derive(Clone, Debug, Default, PartialEq)]
    struct TestLoss {
        degraded: bool,
    }

    impl Loss for TestLoss {
        fn zero() -> Self {
            TestLoss { degraded: false }
        }
        fn total() -> Self {
            TestLoss { degraded: true }
        }
        fn is_zero(&self) -> bool {
            !self.degraded
        }
        fn combine(self, other: Self) -> Self {
            TestLoss {
                degraded: self.degraded || other.degraded,
            }
        }
    }

    // -- In-memory store using Shard<String> --

    struct MemoryStore {
        entries: HashMap<String, Shard<String>>,
    }

    impl MemoryStore {
        fn new() -> Self {
            MemoryStore {
                entries: HashMap::new(),
            }
        }
    }

    impl Store for MemoryStore {
        type Value = String;
        type Shard = Shard<String>;
        type Error = String;
        type Loss = TestLoss;

        fn insert(
            &mut self,
            value: Self::Value,
        ) -> Imperfect<Self::Shard, Self::Error, Self::Loss> {
            let oid = MirrorOid::hash(value.as_bytes());
            let shard = Shard::new(value, oid.clone());
            self.entries.insert(oid.as_ref().to_string(), shard.clone());
            Imperfect::Success(shard)
        }

        fn get(&self, oid: &MirrorOid) -> Imperfect<Self::Shard, Self::Error, Self::Loss> {
            match self.entries.get(oid.as_ref()) {
                Some(shard) => Imperfect::Success(shard.clone()),
                None => Imperfect::Failure(format!("not found: {}", oid), TestLoss::total()),
            }
        }
    }

    // -- MirrorOid tests --

    #[test]
    fn mirror_oid_hash_deterministic() {
        let a = MirrorOid::hash(b"hello");
        let b = MirrorOid::hash(b"hello");
        assert_eq!(a, b);
    }

    #[test]
    fn mirror_oid_hash_different_input() {
        let a = MirrorOid::hash(b"hello");
        let b = MirrorOid::hash(b"world");
        assert_ne!(a, b);
    }

    #[test]
    fn mirror_oid_from_oid() {
        let oid = Oid::new("abc");
        let mirror_oid = MirrorOid::from(oid.clone());
        assert_eq!(mirror_oid.as_oid(), &oid);
    }

    #[test]
    fn mirror_oid_display() {
        let oid = MirrorOid::new(Oid::new("abc123"));
        assert_eq!(format!("{}", oid), "abc123");
        assert_eq!(oid.as_ref(), "abc123");
    }

    #[test]
    fn mirror_oid_ordering() {
        let a = MirrorOid::new(Oid::new("aaa"));
        let b = MirrorOid::new(Oid::new("bbb"));
        assert!(a < b);
    }

    // -- Shard tests --

    #[test]
    fn shard_content_addressed_trait() {
        let oid = MirrorOid::new(Oid::new("abc"));
        let shard = Shard::new("test".to_string(), oid);
        assert_eq!(shard.content_oid(), Oid::new("abc"));
    }

    #[test]
    fn shard_value_access() {
        let oid = MirrorOid::hash(b"data");
        let shard = Shard::new("data".to_string(), oid);
        assert_eq!(shard.value, "data");
    }

    // -- Store trait tests --

    #[test]
    fn insert_returns_shard_with_content_oid() {
        let mut store = MemoryStore::new();
        let result = store.insert("hello".to_string());
        assert!(result.is_ok());
        let shard = result.ok().unwrap();
        assert_eq!(shard.value, "hello");
        // OID is computed from content
        assert_eq!(shard.oid, MirrorOid::hash(b"hello"));
    }

    #[test]
    fn get_after_insert_returns_same_shard() {
        let mut store = MemoryStore::new();
        let inserted = store.insert("hello".to_string()).ok().unwrap();
        let retrieved = store.get(&inserted.oid);
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.ok().unwrap(), inserted);
    }

    #[test]
    fn get_missing_returns_failure() {
        let store = MemoryStore::new();
        let missing_oid = MirrorOid::new(Oid::new("nonexistent"));
        let result = store.get(&missing_oid);
        assert!(result.is_err());
    }

    #[test]
    fn insert_same_content_same_oid() {
        let mut store = MemoryStore::new();
        let a = store.insert("same".to_string()).ok().unwrap();
        let b = store.insert("same".to_string()).ok().unwrap();
        assert_eq!(a.oid, b.oid);
    }

    #[test]
    fn insert_different_content_different_oid() {
        let mut store = MemoryStore::new();
        let a = store.insert("alpha".to_string()).ok().unwrap();
        let b = store.insert("beta".to_string()).ok().unwrap();
        assert_ne!(a.oid, b.oid);
    }

    #[test]
    fn round_trip_preserves_content() {
        let mut store = MemoryStore::new();
        let original = "content-addressed round trip".to_string();
        let shard = store.insert(original.clone()).ok().unwrap();
        let retrieved = store.get(&shard.oid).ok().unwrap();
        assert_eq!(retrieved.value, original);
    }

    #[test]
    fn test_loss_zero_is_not_degraded() {
        let z = TestLoss::zero();
        assert!(z.is_zero());
        assert!(!z.degraded);
    }

    #[test]
    fn test_loss_total_is_degraded() {
        let t = TestLoss::total();
        assert!(!t.is_zero());
        assert!(t.degraded);
    }

    #[test]
    fn test_loss_combine() {
        let a = TestLoss::zero();
        let b = TestLoss::total();
        let combined = a.combine(b);
        assert!(combined.degraded);
    }
}
