#![feature(proc_macro_hygiene, decl_macro, type_alias_enum_variants)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;

use crate::repository::Cache;
use crate::repository::redis::RedisCache;
use crate::shortener::{Shortener, UrlShortener};

mod shortener;
mod repository;

#[derive(FromForm, Debug)]
struct UrlShortenRequest {
    url: String,
    hash: Option<String>,
}

#[get("/<id>")]
fn lookup(id: String, cache: State<Mutex<RedisCache>>) -> Result<Redirect, Status> {
    let cache = cache.lock().unwrap();

    match cache.lookup(&id) {
        None => Err(Status::NotFound),
        Some(url) => Ok(Redirect::to(url)),
    }
}

#[get("/console")]
fn console() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("console", &context)
}

#[post("/shorten", data = "<request>")]
fn shorten(request: Form<UrlShortenRequest>, cache: State<Mutex<RedisCache>>) -> Template {
    let mut cache = cache.lock().unwrap();

    let random_hash = UrlShortener::new().next_id();

    let hash = match &request.hash {
        Some(user_hash) if !user_hash.is_empty() => user_hash,
        _ => &random_hash,
    };

    let result = cache.store(hash, &request.url);

    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("hash", hash);
    Template::render("console", &context)
}

#[catch(404)]
fn not_found() -> String {
    let error: serde_json::Value = serde_json::json!({
        "error": "given key could not be found"
    });

    error.to_string()
}

fn main() {
    let database_url = env::var("DATABASE_URL")
        .expect("please set the DATABASE_URL env var");

    let database = repository::redis::RedisCache::new(&database_url);

    rocket::ignite()
        .register(catchers![not_found])
        .manage(Mutex::new(database))
        .attach(Template::fairing())
        .mount("/", routes!(lookup, shorten, console))
        .launch();
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_cache_store_and_get() {
    }
}