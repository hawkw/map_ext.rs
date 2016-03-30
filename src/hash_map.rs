use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;

use super::{Update, UpdateOr};

#[cfg(features = "unstable")]
pub struct HashMapAddable<K: Hash + Eq, V>(HashMap<K,V>);
#[cfg(features = "unstable")]
impl<K,V> std::ops::AddAssign<(K, V)> for HashMapAddable<K, V>
where K: Hash + Eq {
    // type Rhs = (K, V);
    fn add_assign(&mut self, (key, value): (K, V)) {
        self.0.insert(key, value);
    }
}


impl<K, V> Update<K, V> for HashMap<K, V>
where K: Hash + Eq,
      K: Copy {

    fn update<F>(&mut self, key: &K, f: F)
    where F: FnOnce(&mut V) {
        match self.entry(*key) {
            Vacant(_) => { }
          , Occupied(mut entry) => { f(entry.get_mut()); }
        }
    }

}

impl<K, V> UpdateOr<K, V> for HashMap<K, V>
where K: Hash + Eq,
      K: Copy {

    fn update_or<F>(&mut self, key: &K, f: F, default: V)
    where F: FnOnce(&mut V) {
        match self.entry(*key) {
            Vacant(entry) => { entry.insert(default); }
          , Occupied(mut entry) => { f(entry.get_mut()); }
        }
    }

}
