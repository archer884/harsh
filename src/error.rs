use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

/// Represents potential errors encountered during `Harsh` initialization and usage.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn alphabet_length() -> Self {
        Self {
            kind: ErrorKind::AlphabetLength,
        }
    }

    pub(crate) fn character_not_in_alphabet(c: char) -> Self {
        Self {
            kind: ErrorKind::CharacterNotInAlphabet(c)
        }
    }

    pub(crate) fn illegal_character(c: char) -> Self {
        Self {
            kind: ErrorKind::IllegalCharacter(c),
        }
    }

    pub(crate) fn other(s: &'static str) -> Self {
        Self {
            kind: ErrorKind::Other(s)
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    /// Alphabet contains insufficient distinct characters
    AlphabetLength,

    /// Character not in alphabet
    CharacterNotInAlphabet(char),

    /// Alphabet contains an illegal character
    IllegalCharacter(char),

    Other(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::AlphabetLength => {
                write!(f, "The provided alphabet does not contain enough unique characters")
            }

            ErrorKind::CharacterNotInAlphabet(c) => {
                write!(f, "Attempted to decode character not in alphabet: {}", c)
            }

            ErrorKind::IllegalCharacter(c) => {
                write!(f, "The provided alphabet contains an illegal character: {}", c)
            }

            ErrorKind::Other(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::AlphabetLength => "The provided alphabet does not contain enough unique characters",
            ErrorKind::CharacterNotInAlphabet(_) => "Attempted to decode character not in alphabet",
            ErrorKind::IllegalCharacter(_) => "The provided alphabet contains an illegal character",
            ErrorKind::Other(s) => s,
        }
    }
}
