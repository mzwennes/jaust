use rocket_contrib::databases::diesel;

pub mod urls;

#[database("diesel_postgres_pool")]
pub struct Connection(diesel::PgConnection);
