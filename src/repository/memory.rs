use std::collections::HashMap;

use crate::repository::{Cache, ConnectionError};
use crate::shortener::{Shortener, UrlShortener};

pub struct InMemoryRepository {
    urls: HashMap<String, String>,
    shortener: UrlShortener,
}

impl Cache for InMemoryRepository {
    fn connect(database_url: &str) -> Result<InMemoryRepository, ConnectionError> {
        Ok(InMemoryRepository {
            urls: HashMap::new(),
            shortener: UrlShortener::new(),
        })
    }

    fn store(&mut self, data: &str) -> String {
        let hash = self.shortener.next_id();
        self.urls.insert(hash.to_string(), data.to_string());
        hash
    }

    fn lookup(&self, id: &str) -> Option<String> {
        let owned = self.urls.get(id).unwrap().to_string();
        Some(owned)
    }
}