use std::collections::BTreeMap;
use std::collections::btree_map::Entry::{ Occupied, Vacant };

use super::{ Update, UpdateOr };

impl<K, V> Update<K, V> for BTreeMap<K, V>
where K: Ord
    , K: Copy {

    fn update<F>(&mut self, key: &K, f: F)
    where F: FnOnce(&mut V) {
        match self.entry(*key) {
            Vacant(_) => { }
          , Occupied(mut entry) => { f(entry.get_mut()); }
        }
    }

}

impl<K, V> UpdateOr<K, V> for BTreeMap<K, V>
where K: Ord
    , K: Copy {

    fn update_or<F>(&mut self, key: &K, f: F, default: V)
    where F: FnOnce(&mut V) {
        match self.entry(*key) {
            Vacant(entry) => { entry.insert(default); }
          , Occupied(mut entry) => { f(entry.get_mut()); }
        }
    }

}
