#![cfg_attr( features = "unstable"
           , feature(augmented_assignments, op_assign_traits) )]

//! Extensions to `std::collections::HashMap` and `std::collections::BTreeMap`.
pub trait UpdateOr<K, V> {
    fn update_or<F>(&mut self, key: &K, f: F, default: V)
    where F: FnOnce(&mut V);
}

pub trait Update<K, V> {
    fn update<F>(&mut self, key: &K, f: F)
    where F: FnOnce(&mut V);
}

pub mod btree_map;
pub use self::btree_map::*;

pub mod hash_map;
pub use self::hash_map::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
