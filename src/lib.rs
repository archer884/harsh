#![feature(question_mark, type_ascription)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

mod error;
mod harsh;

pub use error::{Error, Result};
pub use harsh::{Harsh, HarshFactory};

const DEFAULT_ALPHABET: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const DEFAULT_SEPARATORS: &'static [u8] = b"cfhistuCFHISTU";
const SEPARATOR_DIV: f64 = 3.5;
const GUARD_DIV: f64 = 12.0;
const MINIMUM_ALPHABET_LENGTH: usize = 16;
