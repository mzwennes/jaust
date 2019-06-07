#![feature(proc_macro_hygiene, decl_macro, type_alias_enum_variants)]

use std::env;
use std::path::PathBuf;
use std::sync::Mutex;

use rocket::{get, routes, State};
use rocket::response::Redirect;

use crate::repository::Cache;
use crate::repository::redis::RedisCache;

mod shortener;
mod repository;


#[get("/<id>")]
fn lookup(id: String, cache: State<Mutex<RedisCache>>) -> Redirect {
    let cache = cache.lock().unwrap();
    let long_url = cache.lookup(&id).unwrap();
    Redirect::to(format!("https://{}", long_url))
}

#[get("/<path..>")]
fn shorten(path: PathBuf, cache: State<Mutex<RedisCache>>) -> String {
    let url = path.to_str().unwrap();
    let mut cache = cache.lock().unwrap();
    let hash = cache.store(&url);

    let response: serde_json::Value = serde_json::json!({
        "url": url,
        "hash": hash
    });

    response.to_string()
}

fn main() {
    let database_url = env::var("DATABASE_URL")
        .expect("please set the DATABASE_URL env var");

    let database = repository::redis::RedisCache::new(&database_url);

    rocket::ignite()
        .manage(Mutex::new(database))
        .mount("/", routes!(lookup))
        .mount("/shorten", routes!(shorten))
        .launch();
}

#[cfg(test)]
mod tests {
    use crate::repository::{Cache, InMemoryRepository};

    #[test]
    pub fn test_cache_store_and_get() {
        let mut cache = InMemoryRepository::new();

        let long_url = "www.url.com";
        let short_url = cache.store(long_url);
        let lookup = cache.lookup(&short_url);

        assert_eq!(&long_url, &lookup.unwrap())
    }
}