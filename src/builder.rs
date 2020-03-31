use crate::{harsh::Harsh, shuffle};
use std::{error, fmt, result};

const DEFAULT_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const DEFAULT_SEPARATORS: &[u8] = b"cfhistuCFHISTU";

pub type Result<T, E = BuildHarshError> = result::Result<T, E>;

/// Represents potential errors encountered during `Harsh` initialization.
#[derive(Clone, Debug)]
pub enum BuildHarshError {
    /// Error returned when the provided alphabet has insufficient distinct elements
    AlphabetLength,

    /// Provided alphabet contains an illegal character
    IllegalCharacter(char),

    /// Error returned when a separator character is not found in the alphabet
    Separator,
}

impl fmt::Display for BuildHarshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        static ALPHABET_LENGTH_MESSAGE: &str =
            "The provided alphabet does not contain enough unique characters";
        static ILLEGAL_CHARACTER_MESSAGE: &str =
            "The provided alphabet contains an illegal character";
        static SEPARATOR_MESSAGE: &str =
            "The provided separators contain a character not found in the alphabet";

        match self {
            BuildHarshError::AlphabetLength => write!(f, "{}", ALPHABET_LENGTH_MESSAGE),
            BuildHarshError::IllegalCharacter(c) => {
                write!(f, "{} ({})", ILLEGAL_CHARACTER_MESSAGE, c)
            }
            BuildHarshError::Separator => write!(f, "{}", SEPARATOR_MESSAGE),
        }
    }
}

impl error::Error for BuildHarshError {}

/// A builder used to configure and create a Harsh instance.
#[derive(Debug, Default)]
pub struct HarshBuilder {
    salt: Option<Vec<u8>>,
    alphabet: Option<Vec<u8>>,
    separators: Option<Vec<u8>>,
    hash_length: usize,
}

impl HarshBuilder {
    /// Creates a new `HarshBuilder` instance.
    pub fn new() -> HarshBuilder {
        HarshBuilder {
            salt: None,
            alphabet: None,
            separators: None,
            hash_length: 0,
        }
    }

    /// Provides a salt.
    ///
    /// Note that this salt will be converted into a `[u8]` before use, meaning
    /// that multi-byte utf8 character values should be avoided.
    pub fn salt<T: Into<Vec<u8>>>(mut self, salt: T) -> HarshBuilder {
        self.salt = Some(salt.into());
        self
    }

    /// Provides an alphabet.
    ///
    /// Note that this alphabet will be converted into a `[u8]` before use, meaning
    /// that multi-byte utf8 character values should be avoided.
    pub fn alphabet<T: Into<Vec<u8>>>(mut self, alphabet: T) -> HarshBuilder {
        self.alphabet = Some(alphabet.into());
        self
    }

    /// Provides a set of separators.
    ///
    /// Note that these separators will be converted into a `[u8]` before use,
    /// meaning that multi-byte utf8 character values should be avoided.
    pub fn separators<T: Into<Vec<u8>>>(mut self, separators: T) -> HarshBuilder {
        self.separators = Some(separators.into());
        self
    }

    /// Provides a minimum hash length.
    ///
    /// Keep in mind that hashes produced may be longer than this length.
    pub fn length(mut self, hash_length: usize) -> HarshBuilder {
        self.hash_length = hash_length;
        self
    }

    /// Initializes a new `Harsh` based on the `HarshBuilder`.
    ///
    /// This method will consume the `HarshBuilder`.
    pub fn build(self) -> Result<Harsh> {
        const MINIMUM_ALPHABET_LENGTH: usize = 16;

        let alphabet = unique_alphabet(&self.alphabet)?;
        if alphabet.len() < MINIMUM_ALPHABET_LENGTH {
            return Err(BuildHarshError::AlphabetLength);
        }

        let salt = self.salt.unwrap_or_else(Vec::new);
        let (mut alphabet, mut separators) =
            alphabet_and_separators(&self.separators, &alphabet, &salt);
        let guards = guards(&mut alphabet, &mut separators);

        Ok(Harsh::initialize(
            alphabet.into_boxed_slice(),
            guards.into_boxed_slice(),
            self.hash_length,
            salt.into_boxed_slice(),
            separators.into_boxed_slice(),
        ))
    }
}

fn unique_alphabet(alphabet: &Option<Vec<u8>>) -> Result<Vec<u8>> {
    use std::collections::HashSet;

    match *alphabet {
        None => {
            let mut vec = vec![0; DEFAULT_ALPHABET.len()];
            vec.clone_from_slice(DEFAULT_ALPHABET);
            Ok(vec)
        }

        Some(ref alphabet) => {
            let mut reg = HashSet::new();
            let mut ret = Vec::new();

            for &item in alphabet {
                if item == b' ' {
                    return Err(BuildHarshError::IllegalCharacter(item as char));
                }

                if !reg.contains(&item) {
                    ret.push(item);
                    reg.insert(item);
                }
            }

            if ret.len() < 16 {
                Err(BuildHarshError::AlphabetLength)
            } else {
                Ok(ret)
            }
        }
    }
}

fn alphabet_and_separators(
    separators: &Option<Vec<u8>>,
    alphabet: &[u8],
    salt: &[u8],
) -> (Vec<u8>, Vec<u8>) {
    const SEPARATOR_DIV: f64 = 3.5;

    let separators = match *separators {
        None => DEFAULT_SEPARATORS,
        Some(ref separators) => separators,
    };

    let mut separators: Vec<_> = separators
        .iter()
        .cloned()
        .filter(|item| alphabet.contains(item))
        .collect();
    let mut alphabet: Vec<_> = alphabet
        .iter()
        .cloned()
        .filter(|item| !separators.contains(item))
        .collect();

    shuffle(&mut separators, salt);

    if separators.is_empty() || (alphabet.len() as f64 / separators.len() as f64) > SEPARATOR_DIV {
        let length = match (alphabet.len() as f64 / SEPARATOR_DIV).ceil() as usize {
            1 => 2,
            n => n,
        };

        if length > separators.len() {
            let diff = length - separators.len();
            separators.extend_from_slice(&alphabet[..diff]);
            alphabet = alphabet[diff..].to_vec();
        } else {
            separators = separators[..length].to_vec();
        }
    }

    shuffle(&mut alphabet, salt);
    (alphabet, separators)
}

fn guards(alphabet: &mut Vec<u8>, separators: &mut Vec<u8>) -> Vec<u8> {
    const GUARD_DIV: f64 = 12.0;

    let guard_count = (alphabet.len() as f64 / GUARD_DIV).ceil() as usize;
    if alphabet.len() < 3 {
        let guards = separators[..guard_count].to_vec();
        separators.drain(..guard_count);
        guards
    } else {
        let guards = alphabet[..guard_count].to_vec();
        alphabet.drain(..guard_count);
        guards
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn alphabet_and_separator_generation() {
        use super::{DEFAULT_ALPHABET, DEFAULT_SEPARATORS};

        let (alphabet, separators) = super::alphabet_and_separators(
            &Some(DEFAULT_SEPARATORS.to_vec()),
            DEFAULT_ALPHABET,
            b"this is my salt",
        );

        assert_eq!(
            "AdG05N6y2rljDQak4xgzn8ZR1oKYLmJpEbVq3OBv9WwXPMe7",
            alphabet.iter().map(|&u| u as char).collect::<String>()
        );

        assert_eq!(
            "UHuhtcITCsFifS",
            separators.iter().map(|&u| u as char).collect::<String>()
        );
    }

    #[test]
    fn alphabet_and_separator_generation_with_few_separators() {
        use super::DEFAULT_ALPHABET;

        let separators = b"fu";
        let (alphabet, separators) = super::alphabet_and_separators(
            &Some(separators.to_vec()),
            DEFAULT_ALPHABET,
            b"this is my salt",
        );

        assert_eq!(
            "4RVQrYM87wKPNSyTBGU1E6FIC9ALtH0ZD2Wxz3vs5OXJ",
            alphabet.iter().map(|&u| u as char).collect::<String>()
        );

        assert_eq!(
            "ufabcdeghijklmnopq",
            separators.iter().map(|&u| u as char).collect::<String>()
        );
    }
}
