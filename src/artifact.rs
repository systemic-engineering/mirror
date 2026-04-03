//! Artifact storage — bounded storage for compilation artifacts.
//!
//! The runtime produces artifacts via `Runtime::compile`. The store holds them.
//! The caller decides eviction policy based on `state()`.

use std::collections::HashMap;

use crate::model::DomainName;
pub use prism::Pressure;

// ---------------------------------------------------------------------------
// StoreState
// ---------------------------------------------------------------------------

/// What the store knows about itself.
#[derive(Clone, Debug)]
pub struct StoreState {
    /// Memory pressure. 0.0 = empty, 1.0 = capacity.
    pub pressure: Pressure,
    /// Number of artifacts in memory.
    pub cached: usize,
    /// Number of artifacts evicted to backing store.
    pub spilled: usize,
    /// Total capacity.
    pub capacity: usize,
}

// ---------------------------------------------------------------------------
// ArtifactStore trait
// ---------------------------------------------------------------------------

/// Bounded storage for compilation artifacts.
///
/// The runtime produces artifacts. The store holds them. The caller
/// decides eviction policy based on `state()`.
pub trait ArtifactStore {
    type Artifact;

    /// Store an artifact for a domain.
    fn put(&mut self, domain: DomainName, artifact: Self::Artifact);

    /// Retrieve an artifact by domain name.
    fn get(&self, domain: &DomainName) -> Option<&Self::Artifact>;

    /// Evict an artifact from the store.
    fn evict(&mut self, domain: &DomainName);

    /// The store's current state.
    fn state(&self) -> StoreState;
}

// ---------------------------------------------------------------------------
// MemoryStore
// ---------------------------------------------------------------------------

/// In-memory artifact store. No eviction, no backing store.
/// For tests and one-shot CLI usage.
pub struct MemoryStore<A> {
    artifacts: HashMap<DomainName, A>,
    capacity: usize,
}

impl<A> MemoryStore<A> {
    /// Create a new memory store with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            artifacts: HashMap::new(),
            capacity,
        }
    }

    /// Number of stored artifacts.
    pub fn len(&self) -> usize {
        self.artifacts.len()
    }

    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.artifacts.is_empty()
    }
}

impl<A> ArtifactStore for MemoryStore<A> {
    type Artifact = A;

    fn put(&mut self, domain: DomainName, artifact: A) {
        self.artifacts.insert(domain, artifact);
    }

    fn get(&self, domain: &DomainName) -> Option<&A> {
        self.artifacts.get(domain)
    }

    fn evict(&mut self, domain: &DomainName) {
        self.artifacts.remove(domain);
    }

    fn state(&self) -> StoreState {
        let cached = self.artifacts.len();
        let pressure = if self.capacity == 0 {
            1.0
        } else {
            cached as f64 / self.capacity as f64
        };
        StoreState {
            pressure: Pressure::new(pressure),
            cached,
            spilled: 0,
            capacity: self.capacity,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- StoreState tests --

    #[test]
    fn store_state_construction() {
        let state = StoreState {
            pressure: Pressure::new(0.5),
            cached: 10,
            spilled: 5,
            capacity: 20,
        };
        assert_eq!(state.cached, 10);
        assert_eq!(state.spilled, 5);
        assert_eq!(state.capacity, 20);
        assert!((state.pressure.ratio() - 0.5).abs() < f64::EPSILON);
    }

    // -- MemoryStore tests --

    #[test]
    fn memory_store_put_get() {
        let mut store: MemoryStore<String> = MemoryStore::new(10);
        let domain = DomainName::new("test");
        store.put(domain.clone(), "artifact".to_string());
        assert_eq!(store.get(&domain), Some(&"artifact".to_string()));
    }

    #[test]
    fn memory_store_get_missing() {
        let store: MemoryStore<String> = MemoryStore::new(10);
        assert_eq!(store.get(&DomainName::new("ghost")), None);
    }

    #[test]
    fn memory_store_evict() {
        let mut store: MemoryStore<String> = MemoryStore::new(10);
        let domain = DomainName::new("test");
        store.put(domain.clone(), "artifact".to_string());
        store.evict(&domain);
        assert_eq!(store.get(&domain), None);
    }

    #[test]
    fn memory_store_evict_missing_is_noop() {
        let mut store: MemoryStore<String> = MemoryStore::new(10);
        store.evict(&DomainName::new("ghost")); // no panic
    }

    #[test]
    fn memory_store_state_empty() {
        let store: MemoryStore<String> = MemoryStore::new(10);
        let state = store.state();
        assert_eq!(state.cached, 0);
        assert_eq!(state.spilled, 0);
        assert_eq!(state.capacity, 10);
        assert_eq!(state.pressure.ratio(), 0.0);
    }

    #[test]
    fn memory_store_state_pressure_rises() {
        let mut store: MemoryStore<String> = MemoryStore::new(4);
        store.put(DomainName::new("a"), "1".into());
        store.put(DomainName::new("b"), "2".into());
        let state = store.state();
        assert_eq!(state.cached, 2);
        assert!((state.pressure.ratio() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn memory_store_state_at_capacity() {
        let mut store: MemoryStore<String> = MemoryStore::new(2);
        store.put(DomainName::new("a"), "1".into());
        store.put(DomainName::new("b"), "2".into());
        let state = store.state();
        assert_eq!(state.pressure.ratio(), 1.0);
    }

    #[test]
    fn memory_store_state_zero_capacity() {
        let store: MemoryStore<String> = MemoryStore::new(0);
        let state = store.state();
        assert_eq!(state.pressure.ratio(), 1.0);
    }

    #[test]
    fn memory_store_overwrite() {
        let mut store: MemoryStore<String> = MemoryStore::new(10);
        let domain = DomainName::new("test");
        store.put(domain.clone(), "old".to_string());
        store.put(domain.clone(), "new".to_string());
        assert_eq!(store.get(&domain), Some(&"new".to_string()));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn memory_store_len_and_is_empty() {
        let mut store: MemoryStore<String> = MemoryStore::new(10);
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
        store.put(DomainName::new("a"), "1".into());
        assert!(!store.is_empty());
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn memory_store_spilled_always_zero() {
        let mut store: MemoryStore<String> = MemoryStore::new(2);
        store.put(DomainName::new("a"), "1".into());
        store.put(DomainName::new("b"), "2".into());
        store.put(DomainName::new("c"), "3".into()); // over capacity, but no eviction
        assert_eq!(store.state().spilled, 0);
        assert_eq!(store.state().cached, 3);
    }
}
