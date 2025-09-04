use std::any::Any;
use std::collections::HashMap;

#[cfg(test)]
#[derive(Default, Debug, cucumber::World)]
pub struct TestWorld {
    store: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl TestWorld {
    /// Insert a value under `key`. Overwrites any existing value.
    pub fn insert<T: 'static + Send + Sync>(&mut self, key: &str, value: T) {
        self.store.insert(key.to_string(), Box::new(value));
    }

    /// Borrow a value of type T previously inserted under `key`.
    pub fn get<T: 'static + Send + Sync>(&self, key: &str) -> Option<&T> {
        self.store.get(key).and_then(|b| b.downcast_ref::<T>())
    }

    /// Remove and return a value of type T previously inserted under `key`.
    pub fn _take<T: 'static + Send + Sync>(&mut self, key: &str) -> Option<T> {
        self.store
            .remove(key)
            .and_then(|b| b.downcast::<T>().ok().map(|boxed| *boxed))
    }
}
