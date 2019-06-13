use std::collections::HashMap;

use crate::repository::{Cache, CacheError};

pub struct InMemoryRepository {
    urls: HashMap<String, String>,
}

impl InMemoryRepository {
    pub fn new() -> InMemoryRepository {
        InMemoryRepository {
            urls: HashMap::new()
        }
    }
}

impl Cache for InMemoryRepository {
    fn store(&mut self, key: &str, value: &str) -> Result<(), CacheError> {
        self.urls.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn lookup(&self, id: &str) -> Option<String> {
        let result = self.urls.get(id).unwrap().to_string();
        Some(result)
    }
}