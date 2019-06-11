#![feature(proc_macro_hygiene, decl_macro, type_alias_enum_variants, bind_by_move_pattern_guards)]

#[macro_use]
extern crate rocket;

use std::env;
use std::env::VarError;
use std::path::PathBuf;
use std::sync::Mutex;

use rocket::http::Status;
use rocket::response::Redirect;
use rocket::State;

use crate::repository::Cache;
use crate::repository::postgres::PostgresCache;

mod shortener;
mod repository;


#[get("/<id>")]
fn lookup(id: String, cache: State<Mutex<Cache>>) -> Result<Redirect, Status> {
    let cache = cache.lock().unwrap();

    match cache.lookup(&id) {
        None => Err(Status::NotFound),
        Some(url) => Ok(Redirect::to(format!("https://{}", url))),
    }
}

#[get("/<path..>")]
fn shorten(path: PathBuf, cache: State<Mutex<Cache>>) -> String {
    let url = path.to_str().unwrap();
    let mut cache = cache.lock().unwrap();
    let hash = cache.store(&url);

    let response: serde_json::Value = serde_json::json!({
        "url": url,
        "hash": hash
    });

    response.to_string()
}

#[catch(404)]
fn not_found() -> String {
    let error: serde_json::Value = serde_json::json!({
        "error": "given key could not be found"
    });

    error.to_string()
}

fn main() {
    dotenv::dotenv().ok();
    let database: Box<dyn Cache> = choose_database_backend(env::var("JAUST_DB_TYPE"));

    rocket::ignite()
        .register(catchers![not_found])
        .manage(Mutex::new(database))
        .mount("/", routes!(lookup))
        .mount("/shorten", routes!(shorten))
        .launch();
}

fn choose_database_backend(backend: Result<String, VarError>) -> Box<dyn Cache> {
    let database_url = env::var("DATABASE_URL")
        .expect("please set the DATABASE_URL env var");

    let database = repository::postgres::PostgresCache::connect(&database_url);
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