//! Artifact storage — bounded storage for compiled modules.
//!
//! The runtime produces artifacts via `Runtime::compile`. The store holds them
//! and reports `StoreState` with memory `Pressure` (from prism crate).
//! The caller decides eviction policy based on state().

use std::collections::HashMap;

use crate::model::DomainName;
pub use prism::Pressure;

use fragmentation::encoding::{Decode, Encode};
use fragmentation::fragment::{self, Fractal};
use fragmentation::ref_::Ref;
use fragmentation::sha::{HashAlg, Sha};
use fragmentation_git::bounded_store::GitBoundedStore;

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

    /// Retrieve an artifact by domain name. Returns an owned clone.
    fn get(&self, domain: &DomainName) -> Option<Self::Artifact>;

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

impl<A: Clone> ArtifactStore for MemoryStore<A> {
    type Artifact = A;

    fn put(&mut self, domain: DomainName, artifact: A) {
        self.artifacts.insert(domain, artifact);
    }

    fn get(&self, domain: &DomainName) -> Option<A> {
        self.artifacts.get(domain).cloned()
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
// GitStore
// ---------------------------------------------------------------------------

/// Git-backed byte-bounded artifact store.
///
/// Hot artifacts in memory, cold in git. Content-addressed — same bytes = same OID.
/// Eviction spills to git. Byte-bounded via fragmentation's `GitBoundedStore`.
pub struct GitStore<E: Encode + Decode + Clone> {
    store: GitBoundedStore<E>,
    domains: HashMap<DomainName, String>, // domain → OID
    capacity_bytes: usize,
    spilled: usize,
}

impl<E: Encode + Decode + Clone> GitStore<E> {
    /// Open a git-backed artifact store with a byte capacity.
    pub fn open(repo_path: &str, capacity_bytes: usize) -> Result<Self, String> {
        let store = GitBoundedStore::open(repo_path, capacity_bytes).map_err(|e| e.to_string())?;
        Ok(GitStore {
            store,
            domains: HashMap::new(),
            capacity_bytes,
            spilled: 0,
        })
    }

    /// Number of cached artifacts.
    pub fn cached_len(&self) -> usize {
        self.store.cached_len()
    }

    /// Flush all cached artifacts to git.
    pub fn flush(&self) {
        self.store.flush();
    }
}

/// Build a Fractal shard from data + label for content-addressed storage.
fn artifact_fractal<E: Encode + Clone>(data: E, label: &str) -> (String, Fractal<E>) {
    let sha = Sha::hash(label.as_bytes());
    let ref_ = Ref::new(sha, label);
    let shard: Fractal<E> = Fractal::shard_typed(ref_, data);
    let oid = fragment::content_oid(&shard);
    (oid, shard)
}

impl<E: Encode + Decode + Clone> ArtifactStore for GitStore<E> {
    type Artifact = E;

    fn put(&mut self, domain: DomainName, artifact: E) {
        let label = domain.as_str();
        let (oid, fractal) = artifact_fractal(artifact, label);
        self.store.insert(oid.clone(), fractal);
        self.domains.insert(domain, oid);
    }

    fn get(&self, domain: &DomainName) -> Option<E> {
        let oid = self.domains.get(domain)?;
        let fractal = self.store.get(oid)?;
        match fractal {
            Fractal::Shard { data, .. } => Some(data),
            Fractal::Fractal { data, .. } => Some(data),
            Fractal::Lens { data, .. } => Some(data),
        }
    }

    fn evict(&mut self, domain: &DomainName) {
        if self.domains.remove(domain).is_some() {
            self.spilled += 1;
        }
    }

    fn state(&self) -> StoreState {
        let cached = self.domains.len();
        let pressure = if self.capacity_bytes == 0 {
            1.0
        } else {
            self.store.total_bytes() as f64 / self.capacity_bytes as f64
        };
        StoreState {
            pressure: Pressure::new(pressure),
            cached,
            spilled: self.spilled,
            capacity: self.capacity_bytes,
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
    // (git2 and tempfile used by GitStore tests below)

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
        assert_eq!(store.get(&domain), Some("artifact".to_string()));
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
        assert_eq!(store.get(&domain), Some("new".to_string()));
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

    // -- GitStore tests --

    #[test]
    fn git_store_put_and_get() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();
        let mut store: GitStore<Vec<u8>> =
            GitStore::open(dir.path().to_str().unwrap(), 64_000).unwrap();
        let domain = DomainName::new("test");
        store.put(domain.clone(), b"artifact-data".to_vec());
        let retrieved = store.get(&domain);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), b"artifact-data".to_vec());
    }

    #[test]
    fn git_store_get_missing() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();
        let store: GitStore<Vec<u8>> =
            GitStore::open(dir.path().to_str().unwrap(), 64_000).unwrap();
        assert!(store.get(&DomainName::new("ghost")).is_none());
    }

    #[test]
    fn git_store_evict() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();
        let mut store: GitStore<Vec<u8>> =
            GitStore::open(dir.path().to_str().unwrap(), 64_000).unwrap();
        let domain = DomainName::new("test");
        store.put(domain.clone(), b"data".to_vec());
        assert!(store.get(&domain).is_some());
        store.evict(&domain);
        assert!(store.get(&domain).is_none());
        assert_eq!(store.state().spilled, 1);
    }

    #[test]
    fn git_store_evict_missing_is_noop() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();
        let mut store: GitStore<Vec<u8>> =
            GitStore::open(dir.path().to_str().unwrap(), 64_000).unwrap();
        store.evict(&DomainName::new("ghost"));
        assert_eq!(store.state().spilled, 0);
    }

    #[test]
    fn git_store_state_empty() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();
        let store: GitStore<Vec<u8>> =
            GitStore::open(dir.path().to_str().unwrap(), 10_000).unwrap();
        let state = store.state();
        assert_eq!(state.cached, 0);
        assert_eq!(state.spilled, 0);
        assert_eq!(state.capacity, 10_000);
        assert_eq!(state.pressure.ratio(), 0.0);
    }

    #[test]
    fn git_store_state_pressure() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();
        let mut store: GitStore<Vec<u8>> =
            GitStore::open(dir.path().to_str().unwrap(), 64_000).unwrap();
        store.put(DomainName::new("a"), b"1".to_vec());
        store.put(DomainName::new("b"), b"2".to_vec());
        let state = store.state();
        assert_eq!(state.cached, 2);
        assert!(state.pressure.ratio() > 0.0);
    }

    #[test]
    fn git_store_cached_len() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();
        let mut store: GitStore<Vec<u8>> =
            GitStore::open(dir.path().to_str().unwrap(), 64_000).unwrap();
        assert_eq!(store.cached_len(), 0);
        store.put(DomainName::new("a"), b"data".to_vec());
        assert!(store.cached_len() >= 1);
    }

    #[test]
    fn git_store_flush() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();
        let mut store: GitStore<Vec<u8>> =
            GitStore::open(dir.path().to_str().unwrap(), 64_000).unwrap();
        store.put(DomainName::new("a"), b"data".to_vec());
        store.flush(); // should not panic
    }

    #[test]
    fn git_store_zero_capacity() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();
        let store: GitStore<Vec<u8>> = GitStore::open(dir.path().to_str().unwrap(), 0).unwrap();
        assert_eq!(store.state().pressure.ratio(), 1.0);
    }
}
