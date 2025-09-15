use std::any::Any;
use std::collections::HashMap;

#[cfg(test)]
#[derive(Debug, cucumber::World)]
pub struct TestWorld {
    store: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl TestWorld {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    /// Insert a value under `key`. Overwrites any existing value.
    pub fn insert<T: 'static + Send + Sync>(&mut self, key: &str, value: T) {
        self.store.insert(key.to_string(), Box::new(value));
    }

    /// Borrow a value of type T previously inserted under `key`.
    pub fn get<T: 'static + Send + Sync>(&self, key: &str) -> Option<&T> {
        self.store.get(key)?.downcast_ref::<T>()
    }

    /// Borrow a mutable value of type T previously inserted under `key`.
    pub fn _get_mut<T: 'static + Send + Sync>(&mut self, key: &str) -> Option<&mut T> {
        self.store.get_mut(key)?.downcast_mut::<T>()
    }

    /// Remove and return a value of type T previously inserted under `key`.
    pub fn _take<T: 'static + Send + Sync>(&mut self, key: &str) -> Option<T> {
        self.store.remove(key)?.downcast::<T>().ok().map(|boxed| *boxed)
    }

    /// Check if a key exists.
    pub fn _contains(&self, key: &str) -> bool { self.store.contains_key(key) }

    /// Clear all entries.
    pub fn _clear(&mut self) { self.store.clear(); }
}

impl Default for TestWorld {
    fn default() -> Self { Self::new() }
}
