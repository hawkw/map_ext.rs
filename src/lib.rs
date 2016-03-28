#![cfg_attr( features = "unstable"
           , feature(augmented_assignments, op_assign_traits) )]

//! Extensions to `std::collections::HashMap`
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;

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

pub trait Update<K, V> {
    fn update<F>(&mut self, key: K, f: F, default: V)
    where F: FnOnce(&mut V);
}

impl<K, V> Update<K, V> for HashMap<K, V>
where K: Hash + Eq {

    fn update<F>(&mut self, key: K, f: F, default: V)
    where F: FnOnce(&mut V) {
        match self.entry(key) {
            Vacant(entry) => { entry.insert(default); }
          , Occupied(mut entry) => { f(entry.get_mut()); }
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
