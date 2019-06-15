use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::repository::{Cache, CacheError};
use crate::repository::schema::hashed_urls::dsl::*;

pub struct PostgresCache {
    conn: PgConnection
}

#[derive(Queryable, Insertable)]
pub struct HashedUrl {
    pub hash: String,
    pub url: String,
}

impl PostgresCache {
    pub fn new(database_url: &str) -> PostgresCache {
        let postgres = PgConnection::establish(database_url).unwrap();

        PostgresCache { conn: postgres }
    }
}

impl Cache for PostgresCache {
    fn store(&mut self, key: &str, value: &str) -> Result<(), CacheError> {
        let shortened_url = HashedUrl {
            hash: key.to_string(),
            url: value.to_string(),
        };

        diesel::insert_into(hashed_urls)
            .values(shortened_url)
            .get_result(&self.conn)
            .expect("error saving new value")
    }

    fn lookup(&self, id: &str) -> Option<String> {
        HashedUrl::belonging_to(id)
            .load(&self.conn)
    }
}