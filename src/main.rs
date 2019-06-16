#![feature(proc_macro_hygiene, decl_macro, type_alias_enum_variants)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::collections::HashMap;

use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::shortener::{Shortener, UrlShortener};

mod shortener;
mod schema;
mod db;
mod repository;

#[derive(FromForm, Debug)]
struct UrlShortenRequest {
    url: String,
    hash: Option<String>,
}

#[get("/<id>")]
fn lookup(id: String, conn: db::Connection) -> Result<Redirect, Status> {
    match db::urls::find_one(&conn, &id) {
        None => Err(Status::NotFound),
        Some(res) => Ok(Redirect::to(res.url)),
    }
}

#[get("/console")]
fn console() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("console", &context)
}

#[post("/shorten", data = "<request>")]
fn shorten(request: Form<UrlShortenRequest>, conn: db::Connection) -> Template {

    let random_hash = UrlShortener::new().next_id();

    let hash = match &request.hash {
        Some(user_hash) if !user_hash.is_empty() => user_hash,
        _ => &random_hash,
    };

    match db::urls::insert(&conn, hash, &request.url) {
        None => {
            let mut context: HashMap<&str, &str> = HashMap::new();
            context.insert("error", "the requested hash already exists");
            Template::render("console", &context)
        },
        Some(res) => {
            let mut context: HashMap<&str, &str> = HashMap::new();
            context.insert("hash", &res.hash);
            Template::render("console", &context)
        }
    }
}

#[catch(404)]
fn not_found() -> String {
    let error: serde_json::Value = serde_json::json!({
        "error": "given key could not be found"
    });

    error.to_string()
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .attach(db::Connection::fairing())
        .attach(Template::fairing())
        .mount("/", routes!(lookup, shorten, console))
        .launch();
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_cache_store_and_get() {}
}