use redis::{Client, Commands, Connection};

use crate::repository::Cache;
use crate::shortener::{Shortener, UrlShortener};

pub struct RedisCache {
    conn: Connection,
    shortener: UrlShortener,
}

impl RedisCache {
    pub fn new(database_url: &str) -> RedisCache {
        let redis = Client::open(database_url)
            .unwrap()
            .get_connection()
            .unwrap();

        RedisCache {
            conn: redis,
            shortener: UrlShortener::new(),
        }
    }
}

impl Cache for RedisCache {
    fn store(&mut self, data: &str) -> String {
        let hash = self.shortener.next_id();
        self.conn.set::<_, _, ()>(&hash, data);
        hash
    }

    fn lookup(&self, id: &str) -> Option<String> {
        self.conn.get::<_, String>(id).ok()
    }
}