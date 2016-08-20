#![feature(question_mark, type_ascription)]

mod error;
mod harsh;

pub use error::{Error, Result};
pub use harsh::{Harsh, HarshFactory};

const DEFAULT_ALPHABET: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const DEFAULT_SEPARATORS: &'static [u8] = b"cfhistuCFHISTU";
const GUARD_DIV: usize = 12;
const MINIMUM_ALPHABET_LENGTH: usize = 16;
