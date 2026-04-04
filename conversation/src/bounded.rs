//! Bounded memory store — byte-bounded LIFO with settled protection.
//!
//! The 1202 alarm for conversation. When the store is full,
//! the oldest non-settled entries are evicted to make room.
//! Settled entries survive shedding — they're the crystals.
//!
//! No git, no DashMap, no external deps. HashMap + VecDeque.

use std::collections::{HashMap, HashSet, VecDeque};

pub use prism::Pressure;

/// A pressure event: what was shed and why.
#[derive(Clone, Debug)]
pub struct Shed {
    pub evicted: Vec<String>,
    pub trigger: String,
}

/// Byte-bounded in-memory store with settled protection.
///
/// Entries have a key, value, and byte size. When an insert would
/// exceed capacity, the oldest non-settled entries are evicted.
/// Settled entries are protected — they survive shedding.
pub struct BoundedMemoryStore<V> {
    entries: HashMap<String, (V, usize)>,
    order: VecDeque<String>,
    settled: HashSet<String>,
    max_bytes: usize,
    total_bytes: usize,
    sheds: Vec<Shed>,
}

impl<V> BoundedMemoryStore<V> {
    /// Create a store with the given byte capacity.
    pub fn new(max_bytes: usize) -> Self {
        BoundedMemoryStore {
            entries: HashMap::new(),
            order: VecDeque::new(),
            settled: HashSet::new(),
            max_bytes,
            total_bytes: 0,
            sheds: Vec::new(),
        }
    }

    /// Insert a value with its byte size. Evicts oldest non-settled if over capacity.
    /// Returns the keys that were evicted.
    pub fn insert(&mut self, key: String, value: V, size_bytes: usize) -> Vec<String> {
        // Remove old entry if key already exists
        if let Some((_, old_size)) = self.entries.remove(&key) {
            self.total_bytes = self.total_bytes.saturating_sub(old_size);
            self.order.retain(|k| k != &key);
        }

        // Evict until we have room
        let mut evicted = Vec::new();
        while self.total_bytes + size_bytes > self.max_bytes {
            match self.evict_oldest_unsettled() {
                Some(evicted_key) => evicted.push(evicted_key),
                None => break, // only settled entries remain — accept overpressure
            }
        }

        self.entries.insert(key.clone(), (value, size_bytes));
        self.order.push_front(key);
        self.total_bytes += size_bytes;

        if !evicted.is_empty() {
            self.sheds.push(Shed {
                evicted: evicted.clone(),
                trigger: "insert".to_string(),
            });
        }

        evicted
    }

    /// Get a reference to a value.
    pub fn get(&self, key: &str) -> Option<&V> {
        self.entries.get(key).map(|(v, _)| v)
    }

    /// Mark a key as settled. Settled entries survive eviction.
    pub fn settle(&mut self, key: &str) {
        if self.entries.contains_key(key) {
            self.settled.insert(key.to_string());
        }
    }

    /// Unsettle a key. It becomes evictable again.
    pub fn unsettle(&mut self, key: &str) {
        self.settled.remove(key);
    }

    /// Is this key settled?
    pub fn is_settled(&self, key: &str) -> bool {
        self.settled.contains(key)
    }

    /// Current memory pressure. 0.0 = empty, 1.0 = at capacity.
    pub fn pressure(&self) -> Pressure {
        if self.max_bytes == 0 {
            return Pressure::new(1.0);
        }
        Pressure::new(self.total_bytes as f64 / self.max_bytes as f64)
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Is the store empty?
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Current byte usage.
    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    /// Maximum byte capacity.
    pub fn capacity(&self) -> usize {
        self.max_bytes
    }

    /// Number of settled entries.
    pub fn settled_count(&self) -> usize {
        self.settled.len()
    }

    /// Shed history.
    pub fn sheds(&self) -> &[Shed] {
        &self.sheds
    }

    /// Remove a specific key.
    pub fn remove(&mut self, key: &str) -> Option<V> {
        if let Some((value, size)) = self.entries.remove(key) {
            self.total_bytes = self.total_bytes.saturating_sub(size);
            self.order.retain(|k| k != key);
            self.settled.remove(key);
            Some(value)
        } else {
            None
        }
    }

    /// Clear everything.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.order.clear();
        self.settled.clear();
        self.total_bytes = 0;
    }

    /// Evict the oldest non-settled entry. Returns the evicted key.
    fn evict_oldest_unsettled(&mut self) -> Option<String> {
        // Walk from back (oldest) to front, find first non-settled
        let pos = self.order.iter().rev()
            .position(|k| !self.settled.contains(k))?;
        let idx = self.order.len() - 1 - pos;
        let key = self.order.remove(idx)?;

        if let Some((_, size)) = self.entries.remove(&key) {
            self.total_bytes = self.total_bytes.saturating_sub(size);
        }
        self.settled.remove(&key);
        Some(key)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_store() {
        let store: BoundedMemoryStore<String> = BoundedMemoryStore::new(1024);
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
        assert_eq!(store.total_bytes(), 0);
        assert_eq!(store.capacity(), 1024);
        assert_eq!(store.pressure().ratio(), 0.0);
    }

    #[test]
    fn insert_and_get() {
        let mut store = BoundedMemoryStore::new(1024);
        let evicted = store.insert("a".into(), "hello".to_string(), 5);
        assert!(evicted.is_empty());
        assert_eq!(store.get("a"), Some(&"hello".to_string()));
        assert_eq!(store.len(), 1);
        assert_eq!(store.total_bytes(), 5);
    }

    #[test]
    fn get_missing() {
        let store: BoundedMemoryStore<String> = BoundedMemoryStore::new(1024);
        assert_eq!(store.get("ghost"), None);
    }

    #[test]
    fn overwrite_updates_size() {
        let mut store = BoundedMemoryStore::new(1024);
        store.insert("a".into(), "short".to_string(), 5);
        store.insert("a".into(), "much longer value".to_string(), 17);
        assert_eq!(store.len(), 1);
        assert_eq!(store.total_bytes(), 17);
        assert_eq!(store.get("a"), Some(&"much longer value".to_string()));
    }

    #[test]
    fn evicts_oldest_when_full() {
        let mut store = BoundedMemoryStore::new(10);
        store.insert("a".into(), 1, 4);
        store.insert("b".into(), 2, 4);
        // "c" needs 4 bytes, total would be 12 > 10, evict "a" (oldest)
        let evicted = store.insert("c".into(), 3, 4);
        assert_eq!(evicted, vec!["a"]);
        assert_eq!(store.get("a"), None);
        assert_eq!(store.get("b"), Some(&2));
        assert_eq!(store.get("c"), Some(&3));
        assert_eq!(store.total_bytes(), 8);
    }

    #[test]
    fn evicts_multiple_when_needed() {
        let mut store = BoundedMemoryStore::new(10);
        store.insert("a".into(), 1, 3);
        store.insert("b".into(), 2, 3);
        store.insert("c".into(), 3, 3);
        // "d" needs 5 bytes, total=9+5=14 > 10, evict "a"(3)→11, "b"(3)→8
        let evicted = store.insert("d".into(), 4, 5);
        assert_eq!(evicted, vec!["a", "b"]);
        assert_eq!(store.get("c"), Some(&3));
        assert_eq!(store.get("d"), Some(&4));
        assert_eq!(store.total_bytes(), 8);
    }

    #[test]
    fn settled_entries_survive_eviction() {
        let mut store = BoundedMemoryStore::new(10);
        store.insert("a".into(), 1, 4);
        store.insert("b".into(), 2, 4);
        store.settle("a"); // protect "a"

        // "c" needs 4 bytes → evicts "b" (oldest non-settled), not "a"
        let evicted = store.insert("c".into(), 3, 4);
        assert_eq!(evicted, vec!["b"]);
        assert_eq!(store.get("a"), Some(&1)); // survived
        assert_eq!(store.get("b"), None);     // evicted
        assert_eq!(store.get("c"), Some(&3));
    }

    #[test]
    fn settle_and_unsettle() {
        let mut store = BoundedMemoryStore::new(1024);
        store.insert("a".into(), 1, 4);
        assert!(!store.is_settled("a"));

        store.settle("a");
        assert!(store.is_settled("a"));
        assert_eq!(store.settled_count(), 1);

        store.unsettle("a");
        assert!(!store.is_settled("a"));
        assert_eq!(store.settled_count(), 0);
    }

    #[test]
    fn settle_nonexistent_is_noop() {
        let mut store: BoundedMemoryStore<i32> = BoundedMemoryStore::new(1024);
        store.settle("ghost"); // doesn't add to settled set
        assert_eq!(store.settled_count(), 0);
    }

    #[test]
    fn overpressure_when_only_settled_remain() {
        let mut store = BoundedMemoryStore::new(8);
        store.insert("a".into(), 1, 4);
        store.insert("b".into(), 2, 4);
        store.settle("a");
        store.settle("b");

        // Can't evict anything — both settled. Overpressure accepted.
        let evicted = store.insert("c".into(), 4, 4);
        assert!(evicted.is_empty());
        assert_eq!(store.len(), 3);
        assert_eq!(store.total_bytes(), 12); // over max_bytes
        assert_eq!(store.pressure().ratio(), 1.0); // clamped by Pressure
    }

    #[test]
    fn pressure_tracks_usage() {
        let mut store = BoundedMemoryStore::new(100);
        assert_eq!(store.pressure().ratio(), 0.0);
        store.insert("a".into(), 1, 50);
        assert!((store.pressure().ratio() - 0.5).abs() < f64::EPSILON);
        store.insert("b".into(), 2, 50);
        assert!((store.pressure().ratio() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn zero_capacity() {
        let store: BoundedMemoryStore<i32> = BoundedMemoryStore::new(0);
        assert_eq!(store.pressure().ratio(), 1.0);
    }

    #[test]
    fn remove() {
        let mut store = BoundedMemoryStore::new(1024);
        store.insert("a".into(), 42, 10);
        store.settle("a");
        let val = store.remove("a");
        assert_eq!(val, Some(42));
        assert_eq!(store.len(), 0);
        assert_eq!(store.total_bytes(), 0);
        assert!(!store.is_settled("a"));
    }

    #[test]
    fn remove_missing() {
        let mut store: BoundedMemoryStore<i32> = BoundedMemoryStore::new(1024);
        assert_eq!(store.remove("ghost"), None);
    }

    #[test]
    fn clear() {
        let mut store = BoundedMemoryStore::new(1024);
        store.insert("a".into(), 1, 10);
        store.insert("b".into(), 2, 10);
        store.settle("a");
        store.clear();
        assert!(store.is_empty());
        assert_eq!(store.total_bytes(), 0);
        assert_eq!(store.settled_count(), 0);
    }

    #[test]
    fn shed_history() {
        let mut store = BoundedMemoryStore::new(10);
        store.insert("a".into(), 1, 6);
        store.insert("b".into(), 2, 6); // evicts "a"
        assert_eq!(store.sheds().len(), 1);
        assert_eq!(store.sheds()[0].evicted, vec!["a"]);
        assert_eq!(store.sheds()[0].trigger, "insert");
    }
}
