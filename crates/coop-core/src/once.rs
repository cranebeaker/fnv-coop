//! First-resolver-wins table: a value computed once and then reused.
//!
//! Used for leveled picks and leveled loot, which the host resolves exactly
//! once; later askers get the first answer, never a re-roll.

use std::collections::BTreeMap;

/// Keyed write-once map.
#[derive(Clone, Debug, Default)]
pub struct OnceMap<K: Ord, V: Clone> {
    map: BTreeMap<K, V>,
}

impl<K: Ord, V: Clone> OnceMap<K, V> {
    /// Empty table.
    pub fn new() -> Self {
        OnceMap {
            map: BTreeMap::new(),
        }
    }

    /// Returns the stored value, or stores `v` and returns it. The boolean is
    /// true when `v` won.
    pub fn resolve(&mut self, key: K, v: V) -> (V, bool) {
        match self.map.entry(key) {
            std::collections::btree_map::Entry::Occupied(e) => (e.get().clone(), false),
            std::collections::btree_map::Entry::Vacant(e) => (e.insert(v).clone(), true),
        }
    }

    /// The stored value, if any.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    /// Forgets a key (for example when a container resets).
    pub fn forget(&mut self, key: &K) {
        self.map.remove(key);
    }

    /// Entries held.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// True when empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_wins() {
        let mut m = OnceMap::new();
        assert_eq!(m.resolve(1u32, 10u32), (10, true));
        assert_eq!(m.resolve(1, 99), (10, false));
        assert_eq!(m.resolve(2, 99), (99, true));
        m.forget(&1);
        assert_eq!(m.resolve(1, 5), (5, true));
        assert_eq!(m.len(), 2);
    }
}
