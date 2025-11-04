#![allow(unused)]

use std::any::Any;
use std::collections::HashMap;

#[cfg(test)]
#[derive(Debug, cucumber::World)]
pub struct TestWorld(HashMap<String, Box<dyn Any + Send + Sync>>);
impl TestWorld {
    pub fn new() -> Self { Self(HashMap::new()) }

    /// Insert a value under `key`. Overwrites any existing value.
    pub fn insert<T: 'static + Send + Sync>(&mut self, key: &str, value: T) {
        self.0.insert(key.to_string(), Box::new(value));
    }

    /// Borrow a value of type T previously inserted under `key`.
    pub fn get<T: 'static + Send + Sync>(&self, key: &str) -> Option<&T> {
        self.0.get(key)?.downcast_ref::<T>()
    }

    /// Borrow a mutable value of type T previously inserted under `key`.
    pub fn get_mut<T: 'static + Send + Sync>(&mut self, key: &str) -> Option<&mut T> {
        self.0.get_mut(key)?.downcast_mut::<T>()
    }

    /// Remove and return a value of type T previously inserted under `key`.
    pub fn take<T: 'static + Send + Sync>(&mut self, key: &str) -> Option<T> {
        self.0.remove(key)?.downcast::<T>().ok().map(|boxed| *boxed)
    }

    /// Check if a key exists.
    pub fn contains(&self, key: &str) -> bool { self.0.contains_key(key) }

    /// Clear all entries.
    pub fn clear(&mut self) { self.0.clear(); }
}

impl Default for TestWorld {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
pub fn parse_matrix_table(table_data: &cucumber::gherkin::Table) -> Vec<f64> {
    table_data
        .rows
        .iter()
        .flatten()
        .filter_map(|item| item.parse().ok())
        .collect()
}
