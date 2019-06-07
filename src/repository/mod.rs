use std::collections::HashMap;

use crate::shortener::{Shortener, UrlShortener};

pub mod redis;

pub trait Cache {
    fn store(&mut self, data: &str) -> String;
    fn lookup(&self, id: &str) -> Option<String>;
}

pub struct InMemoryRepository {
    urls: HashMap<String, String>,
    shortener: UrlShortener,
}

impl InMemoryRepository {
    pub fn new() -> InMemoryRepository {
        InMemoryRepository {
            urls: HashMap::new(),
            shortener: UrlShortener::new(),
        }
    }
}

impl Cache for InMemoryRepository {
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