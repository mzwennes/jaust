use redis::{Client, Commands, Connection};

use crate::repository::{Cache, ConnectionError};
use crate::shortener::{Shortener, UrlShortener};

pub struct RedisCache {
    conn: Connection,
    shortener: UrlShortener,
}

impl Cache for RedisCache {
    fn connect(database_url: &str) -> Result<RedisCache, ConnectionError> {
        let redis = Client::open(database_url)
            .unwrap()
            .get_connection();

        match redis {
            Ok(conn) => Ok(RedisCache {
                conn,
                shortener: UrlShortener::new(),
            }),
            Err(error) => Err(ConnectionError)
        }
    }

    fn store(&mut self, data: &str) -> String {
        let hash = self.shortener.next_id();
        self.conn.set::<_, _, ()>(&hash, data);
        hash
    }

    fn lookup(&self, id: &str) -> Option<String> {
        self.conn.get::<_, String>(id).ok()
    }
}