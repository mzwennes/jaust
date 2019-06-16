use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::hashed_urls;

#[derive(Debug, Queryable, Insertable)]
pub struct HashedUrl {
    pub hash: String,
    pub url: String,
}

pub fn insert<'a>(conn: &PgConnection, hash: &'a str, url: &'a str) -> Option<HashedUrl> {
    let new_url = HashedUrl {
        hash: hash.to_string(),
        url: url.to_string(),
    };

    diesel::insert_into(hashed_urls::table)
        .values(&new_url)
        .get_result::<HashedUrl>(conn)
        .ok()
}

pub fn find_one(conn: &PgConnection, hash: &str) -> Option<HashedUrl> {
    hashed_urls::table
        .filter(hashed_urls::hash.eq(hash))
        .first::<HashedUrl>(conn)
        .map_err(|error| eprintln!("error finding hash {}", error))
        .ok()
}
