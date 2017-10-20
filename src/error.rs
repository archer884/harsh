use std::error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, Error>;

static ALPHABET_LENGTH_MESSAGE: &str = "The provided alphabet does not contain enough unique characters";
static ILLEGAL_CHARACTER_MESSAGE: &str = "The provided alphabet contains an illegal character";

/// Represents potential errors encountered during `Harsh` initialization and usage.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn alphabet_length() -> Self {
        Self { kind: ErrorKind::AlphabetLength }
    }

    pub(crate) fn illegal_character(c: char) -> Self {
        Self { kind: ErrorKind::IllegalCharacter(c) }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    /// Alphabet contains insufficient distinct characters
    AlphabetLength,

    /// Alphabet contains an illegal character
    IllegalCharacter(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::AlphabetLength => write!(f, "{}", ALPHABET_LENGTH_MESSAGE),
            ErrorKind::IllegalCharacter(c) => write!(f, "{} ({})", ILLEGAL_CHARACTER_MESSAGE, c),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::AlphabetLength => ALPHABET_LENGTH_MESSAGE,
            ErrorKind::IllegalCharacter(_) => ILLEGAL_CHARACTER_MESSAGE,
        }
    }
}
