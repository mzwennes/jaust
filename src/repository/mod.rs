use std::fmt;
use std::write;

pub mod redis;
pub mod postgres;
pub mod memory;

pub struct ConnectionError;

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not connect to configured backend.")
    }
}

pub trait Cache {
    fn connect(database_url: &str) -> Result<Self, ConnectionError> where Self: Sized + 'static;
    fn store(&mut self, data: &str) -> String;
    fn lookup(&self, id: &str) -> Option<String>;
}
