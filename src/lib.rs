#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
//! Minimalistic version of PISA written in Rust.

use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    pub docno: String,
    pub content: String,
}

impl Document {
    fn new<S: Into<String>>(docno: S, content: S) -> Self {
        Self {
            docno: docno.into(),
            content: content.into(),
        }
    }
}

fn skip_whitespaces<B>(bytes: &mut Peekable<B>)
where
    B: Iterator<Item = std::io::Result<u8>>,
{
    while let Some(byte) = bytes.peek() {
        if !byte.as_ref().map(u8::is_ascii_whitespace).unwrap_or(false) {
            break;
        }
        bytes.next();
    }
}

pub mod parser;
