use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::repository::{Cache, ConnectionError};
use crate::shortener::{Shortener, UrlShortener};

pub struct PostgresCache {
    conn: PgConnection,
    shortener: UrlShortener,
}

impl Cache for PostgresCache {
    fn connect(database_url: &str) -> Result<PostgresCache, ConnectionError> {
        let postgres = PgConnection::establish(&database_url);

        match postgres {
            Ok(pg) => Ok(PostgresCache {
                conn: pg,
                shortener: UrlShortener::new(),
            }),
            Err(error) => Err(ConnectionError)
        }
    }

    fn store(&mut self, data: &str) -> String {
        unimplemented!()
    }

    fn lookup(&self, id: &str) -> Option<String> {
        unimplemented!()
    }
}