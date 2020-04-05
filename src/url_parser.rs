use std::str::Utf8Error;

use percent_encoding::{AsciiSet, CONTROLS, percent_decode, utf8_percent_encode};

pub struct UrlParser;

const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ').add(b'"')
    .add(b'<').add(b'>')
    .add(b'`').add(b'{')
    .add(b'}');

impl UrlParser {
    /// Out of safety this function first decodes and then encodes a URL.
    pub fn parse(url: String) -> Result<String, Utf8Error> {
        let iter = percent_decode(url.as_bytes());
        let decoded = iter.decode_utf8()?;
        let iter = utf8_percent_encode(&decoded, FRAGMENT);
        Ok(iter.collect())
    }
}