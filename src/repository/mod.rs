use std::error::Error;
use std::fmt;

pub mod redis;
pub mod memory;

#[derive(Debug)]
pub struct CacheError;

impl Error for CacheError {}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not write data to storage endpoint.")
    }
}

pub trait Cache {
    fn store(&mut self, key: &str, value: &str) -> Result<(), CacheError>;
    fn lookup(&self, id: &str) -> Option<String>;
}
