//! Store — the universal content-addressed store trait.
//!
//! You give it a value. It gives you back a shard. The OID is computed
//! from the content — never provided by the caller.
//!
//! The store is ternary: get can return Success (found, fresh),
//! Partial (found, stale), or Failure (not found).

use prism::{Imperfect, Loss};

use crate::kernel::{ContentAddressed, Oid};

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
    fn get(&self, oid: &Oid) -> Imperfect<Self::Shard, Self::Error, Self::Loss>;
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

    // -- Test shard type --

    #[derive(Clone, Debug, PartialEq)]
    struct TestShard {
        oid: Oid,
        data: String,
    }

    impl ContentAddressed for TestShard {
        type Oid = Oid;
        fn content_oid(&self) -> Oid {
            self.oid.clone()
        }
    }

    // -- In-memory store --

    struct MemoryStore {
        entries: HashMap<String, TestShard>,
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
        type Shard = TestShard;
        type Error = String;
        type Loss = TestLoss;

        fn insert(
            &mut self,
            value: Self::Value,
        ) -> Imperfect<Self::Shard, Self::Error, Self::Loss> {
            let oid = Oid::hash(value.as_bytes());
            let shard = TestShard {
                oid: oid.clone(),
                data: value,
            };
            self.entries.insert(oid.as_ref().to_string(), shard.clone());
            Imperfect::Success(shard)
        }

        fn get(&self, oid: &Oid) -> Imperfect<Self::Shard, Self::Error, Self::Loss> {
            match self.entries.get(oid.as_ref()) {
                Some(shard) => Imperfect::Success(shard.clone()),
                None => Imperfect::Failure(
                    format!("not found: {}", oid),
                    TestLoss::total(),
                ),
            }
        }
    }

    // -- Store trait tests --

    #[test]
    fn insert_returns_shard_with_content_oid() {
        let mut store = MemoryStore::new();
        let result = store.insert("hello".to_string());
        assert!(result.is_ok());
        let shard = result.ok().unwrap();
        assert_eq!(shard.data, "hello");
        // OID is computed from content
        assert_eq!(shard.oid, Oid::hash(b"hello"));
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
        let missing_oid = Oid::new("nonexistent");
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
        assert_eq!(retrieved.data, original);
    }

    #[test]
    fn shard_content_addressed_trait() {
        let shard = TestShard {
            oid: Oid::new("abc"),
            data: "test".into(),
        };
        assert_eq!(shard.content_oid(), Oid::new("abc"));
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
