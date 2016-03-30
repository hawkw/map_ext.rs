#![cfg_attr( features = "unstable"
           , feature(augmented_assignments, op_assign_traits) )]

//! Extensions to `std::collections::HashMap` and `std::collections::BTreeMap`.

/// Trait adding an `update` method to maps, allowing their entries to be
/// updated.
pub trait Update<K, V> {
    /// Update the entry for the given key by applying a function to the value.
    ///
    /// If there is an entry in the map corresponding to `key`, updates the
    /// value by applying the function `f` to it. Otherwise, if there is
    /// no entry, does nothing.
    ///
    /// # Arguments
    ///   - `key`: the key to the entry to update
    ///   - `f`: a function taking `&mut V` to update the entry's value
    ///
    fn update<F>(&mut self, key: &K, f: F)
    where F: FnOnce(&mut V);
}

/// Trait adding an `update_or` method to maps, allowing their entries to be
/// updated with a default value.
pub trait UpdateOr<K, V> {
    /// Update the entry for given key, or insert a default.
    ///
    /// If there is an entry in the map corresponding to `key`, updates the
    /// value by applying the function `f` to it. Otherwise, if there is
    /// no entry, inserts the default value into the map for that key.
    ///
    /// # Arguments
    ///   - `key`: the key to the entry to update
    ///   - `f`: a function taking `&mut V` to update the entry's value
    ///   - `default`: a default value to insert if there is no existing value
    ///
    fn update_or<F>(&mut self, key: &K, f: F, default: V)
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
