use std::error::Error as ErrorTrait;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, Error>;

const ALPHABET_LENGTH_MESSAGE: &'static str = "The provided alphabet does not contain enough unique characters";
const ILLEGAL_CHARACTER_MESSAGE: &'static str = "The provided alphabet contains an illegal character";
const SEPARATOR_MESSAGE: &'static str = "The provided separators contain a character not found in the alphabet";

/// Represents potential errors encountered during `Harsh` initialization.
#[derive(Debug)]
pub enum Error {
    /// Error returned when the provided alphabet has insufficient distinct elements
    AlphabetLength,

    /// Provided alphabet contains an illegal character
    IllegalCharacter(char),

    /// Error returned when a separator character is not found in the alphabet
    Separator,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AlphabetLength => write!(f, "{}", ALPHABET_LENGTH_MESSAGE),
            Error::IllegalCharacter(c) => write!(f, "{} ({})", ILLEGAL_CHARACTER_MESSAGE, c),
            Error::Separator => write!(f, "{}", SEPARATOR_MESSAGE),
        }
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AlphabetLength => ALPHABET_LENGTH_MESSAGE,
            Error::IllegalCharacter(_) => ILLEGAL_CHARACTER_MESSAGE,
            Error::Separator => SEPARATOR_MESSAGE,
        }
    }
}
