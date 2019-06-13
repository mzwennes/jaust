use redis::{Client, Commands, Connection};

use crate::repository::{Cache, CacheError};

pub struct RedisCache {
    conn: Connection,
}

impl RedisCache {
    pub fn new(database_url: &str) -> RedisCache {
        let redis = Client::open(database_url)
            .unwrap()
            .get_connection()
            .unwrap();

        RedisCache { conn: redis }
    }
}

impl Cache for RedisCache {
    fn store(&mut self, key: &str, value: &str) -> Result<(), CacheError> {
        let result = self.conn.set::<_, _, ()>(key, value);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CacheError)
        }

    }

    fn lookup(&self, id: &str) -> Option<String> {
        self.conn.get::<_, String>(id).ok()
    }
}