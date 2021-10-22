use crate::{builder::HarshBuilder, shuffle};
use std::{error, fmt, result, str};

type Result<T, E = Error> = result::Result<T, E>;

#[derive(Clone, Debug)]
pub enum Error {
    Hex,
    Decode(DecodeError),
}

#[derive(Clone, Debug)]
pub enum DecodeError {
    Value,
    Hash,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeError::Value => f.write_str("Found bad value"),
            DecodeError::Hash => f.write_str("Malformed hashid"),
        }
    }
}

impl error::Error for DecodeError {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Hex => f.write_str("Failed to decode hex value"),
            Error::Decode(e) => match e {
                DecodeError::Value => f.write_str("Found bad value"),
                DecodeError::Hash => f.write_str("Malformed hashid"),
            },
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Hex => None,
            Error::Decode(ref e) => Some(e),
        }
    }
}

/// A hashids-compatible hasher.
///
/// It's probably not a great idea to use the default, because in that case
/// your values will be entirely trivial to decode. On the other hand, this is
/// not intended to be cryptographically-secure, so go nuts!
#[derive(Clone, Debug)]
pub struct Harsh {
    alphabet: Box<[u8]>,
    guards: Box<[u8]>,
    hash_length: usize,
    salt: Box<[u8]>,
    separators: Box<[u8]>,
}

impl Harsh {
    /// Create a default instance of Harsh.
    pub fn new() -> Self {
        HarshBuilder::new()
            .build()
            .expect("Default options should not fail")
    }

    /// Build a new instance of Harsh.
    pub fn builder() -> HarshBuilder {
        HarshBuilder::new()
    }

    pub(crate) fn initialize(
        alphabet: Box<[u8]>,
        guards: Box<[u8]>,
        hash_length: usize,
        salt: Box<[u8]>,
        separators: Box<[u8]>,
    ) -> Self {
        Harsh {
            alphabet,
            guards,
            hash_length,
            salt,
            separators,
        }
    }

    /// Encodes a slice of `u64` values into a single hashid.
    pub fn encode(&self, values: &[u64]) -> String {
        if values.is_empty() {
            return String::new();
        }

        let nhash = create_nhash(values);

        let mut alphabet = self.alphabet.clone();
        let mut buffer = String::new();

        let idx = (nhash % alphabet.len() as u64) as usize;
        let lottery = alphabet[idx];
        buffer.push(lottery as char);

        for (idx, &value) in values.iter().enumerate() {
            let mut value = value;
            let mut temp = Vec::with_capacity(self.salt.len() + alphabet.len() + 1);
            temp.push(lottery);
            temp.extend_from_slice(&self.salt);
            temp.extend_from_slice(&alphabet);

            let alphabet_len = alphabet.len();
            shuffle(&mut alphabet, &temp[..alphabet_len]);

            let last = hash(value, &alphabet);
            buffer.push_str(&last);

            if idx + 1 < values.len() {
                value %= (last.bytes().next().unwrap_or(0) as usize + idx) as u64;
                buffer
                    .push(self.separators[(value % self.separators.len() as u64) as usize] as char);
            }
        }

        if buffer.len() < self.hash_length {
            let guard_index = (nhash as usize
                + buffer.bytes().next().expect("hellfire and damnation") as usize)
                % self.guards.len();
            let guard = self.guards[guard_index];
            buffer.insert(0, guard as char);

            if buffer.len() < self.hash_length {
                let guard_index = (nhash as usize
                    + buffer.as_bytes()[2] as usize)
                    % self.guards.len();
                let guard = self.guards[guard_index];
                buffer.push(guard as char);
            }
        }

        let half_length = alphabet.len() / 2;
        while buffer.len() < self.hash_length {
            {
                let alphabet_copy = alphabet.clone();
                shuffle(&mut alphabet, &alphabet_copy);
            }

            let (left, right) = alphabet.split_at(half_length);
            buffer = format!(
                "{}{}{}",
                String::from_utf8_lossy(right),
                buffer,
                String::from_utf8_lossy(left)
            );

            let excess = buffer.len() as i32 - self.hash_length as i32;
            if excess > 0 {
                let marker = excess as usize / 2;
                buffer = buffer[marker..marker + self.hash_length].to_owned();
            }
        }

        buffer
    }

    /// Decodes a single hashid into a slice of `u64` values.
    pub fn decode<T: AsRef<str>>(&self, input: T) -> Result<Vec<u64>> {
        let mut value = input.as_ref().as_bytes();

        if let Some(guard_idx) = value.iter().position(|u| self.guards.contains(u)) {
            value = &value[(guard_idx + 1)..];
        }

        if let Some(guard_idx) = value.iter().rposition(|u| self.guards.contains(u)) {
            value = &value[..guard_idx];
        }

        if value.len() < 2 {
            return Err(Error::Decode(DecodeError::Hash));
        }

        let mut alphabet = self.alphabet.clone();

        let lottery = value[0];
        let value = &value[1..];
        let segments = value.split(|u| self.separators.contains(u));

        let result: Option<Vec<_>> = segments
            .into_iter()
            .map(|segment| {
                let mut buffer = Vec::with_capacity(self.salt.len() + alphabet.len() + 1);
                buffer.push(lottery);
                buffer.extend_from_slice(&self.salt);
                buffer.extend_from_slice(&alphabet);

                let alphabet_len = alphabet.len();
                shuffle(&mut alphabet, &buffer[..alphabet_len]);
                unhash(segment, &alphabet)
            })
            .collect();

        match result {
            None => Err(Error::Decode(DecodeError::Value)),
            Some(result) => {
                if self.encode(&result) == input.as_ref() {
                    Ok(result)
                } else {
                    Err(Error::Decode(DecodeError::Hash))
                }
            }
        }
    }

    /// Encodes a hex string into a hashid.
    pub fn encode_hex(&self, hex: &str) -> Result<String> {
        let values: Option<Vec<_>> = hex
            .as_bytes()
            .chunks(12)
            .map(|chunk| {
                str::from_utf8(chunk)
                    .ok()
                    .and_then(|s| u64::from_str_radix(&("1".to_owned() + s), 16).ok())
            })
            .collect();

        match values {
            Some(values) => Ok(self.encode(&values)),
            None => Err(Error::Hex),
        }
    }

    /// Decodes a hashid into a hex string.
    pub fn decode_hex(&self, value: &str) -> Result<String> {
        use std::fmt::Write;

        let values = self.decode(value)?;

        let mut result = String::new();
        let mut buffer = String::new();

        for n in values {
            write!(buffer, "{:x}", n).unwrap();
            result.push_str(&buffer[1..]);
            buffer.clear();
        }

        Ok(result)
    }
}

impl Default for Harsh {
    fn default() -> Self {
        Harsh::new()
    }
}

#[inline]
fn create_nhash(values: &[u64]) -> u64 {
    values
        .iter()
        .enumerate()
        .fold(0, |a, (idx, value)| a + (value % (idx + 100) as u64))
}

fn hash(mut value: u64, alphabet: &[u8]) -> String {
    let length = alphabet.len() as u64;
    let mut hash = Vec::new();

    loop {
        hash.push(alphabet[(value % length) as usize]);
        value /= length;

        if value == 0 {
            hash.reverse();
            return String::from_utf8(hash).expect("omg fml");
        }
    }
}

fn unhash(input: &[u8], alphabet: &[u8]) -> Option<u64> {
    input.iter().enumerate().fold(Some(0), |a, (idx, &value)| {
        let pos = alphabet.iter().position(|&item| item == value)? as u64;
        let b = (alphabet.len() as u64).checked_pow((input.len() - idx - 1) as u32)?;
        let c = pos.checked_mul(b)?;
        a.map(|a| a + c)
    })
}

#[cfg(test)]
mod tests {
    use super::{Harsh, HarshBuilder};

    #[test]
    fn harsh_default_does_not_panic() {
        Harsh::default();
    }

    #[test]
    fn can_encode() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .build()
            .expect("failed to initialize harsh");

        assert_eq!(
            "4o6Z7KqxE",
            harsh.encode(&[1226198605112]),
            "error encoding [1226198605112]"
        );
        assert_eq!("laHquq", harsh.encode(&[1, 2, 3]));
    }

    #[test]
    fn can_encode_with_guards() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .length(8)
            .build()
            .expect("failed to initialize harsh");

        assert_eq!("GlaHquq0", harsh.encode(&[1, 2, 3]));
    }

    #[test]
    fn can_encode_with_padding() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .length(12)
            .build()
            .expect("failed to initialize harsh");

        assert_eq!("9LGlaHquq06D", harsh.encode(&[1, 2, 3]));
    }

    #[test]
    fn can_decode() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .build()
            .expect("failed to initialize harsh");

        assert_eq!(
            &[1226198605112],
            &harsh.decode("4o6Z7KqxE").expect("failed to decode")[..],
            "error decoding \"4o6Z7KqxE\""
        );
        assert_eq!(
            &[1u64, 2, 3],
            &harsh.decode("laHquq").expect("failed to decode")[..]
        );
    }

    #[test]
    fn can_decode_with_guards() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .length(8)
            .build()
            .expect("failed to initialize harsh");

        assert_eq!(
            &[1u64, 2, 3],
            &harsh.decode("GlaHquq0").expect("failed to decode")[..]
        );
    }

    #[test]
    fn can_decode_with_padding() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .length(12)
            .build()
            .expect("failed to initialize harsh");

        assert_eq!(
            &[1u64, 2, 3],
            &harsh.decode("9LGlaHquq06D").expect("failed to decode")[..]
        );
    }

    #[test]
    fn can_encode_hex() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .build()
            .expect("failed to initialize harsh");

        assert_eq!(
            "lzY",
            &harsh.encode_hex("FA").expect("Failed to encode"),
            "error encoding `FA`"
        );
        assert_eq!(
            "MemE",
            &harsh.encode_hex("26dd").expect("Failed to encode"),
            "error encoding `26dd`"
        );
        assert_eq!(
            "eBMrb",
            &harsh.encode_hex("FF1A").expect("Failed to encode"),
            "error encoding `FF1A`"
        );
        assert_eq!(
            "D9NPE",
            &harsh.encode_hex("12abC").expect("Failed to encode"),
            "error encoding `12abC`"
        );
        assert_eq!(
            "9OyNW",
            &harsh.encode_hex("185b0").expect("Failed to encode"),
            "error encoding `185b0`"
        );
        assert_eq!(
            "MRWNE",
            &harsh.encode_hex("17b8d").expect("Failed to encode"),
            "error encoding `17b8d`"
        );
        assert_eq!(
            "4o6Z7KqxE",
            &harsh.encode_hex("1d7f21dd38").expect("Failed to encode"),
            "error encoding `1d7f21dd38`"
        );
        assert_eq!(
            "ooweQVNB",
            &harsh.encode_hex("20015111d").expect("Failed to encode"),
            "error encoding `20015111d`"
        );
        assert_eq!(
            "kRNrpKlJ",
            &harsh.encode_hex("deadbeef").expect("Failed to encode"),
            "error encoding `deadbeef`"
        );

        let harsh = HarshBuilder::new().build().unwrap();
        assert_eq!(
            "y42LW46J9luq3Xq9XMly",
            &harsh
                .encode_hex("507f1f77bcf86cd799439011",)
                .expect("failed to encode",),
            "error encoding `507f1f77bcf86cd799439011`"
        );
    }

    #[test]
    fn can_encode_hex_with_guards() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .length(10)
            .build()
            .expect("failed to initialize harsh");

        assert_eq!(
            "GkRNrpKlJd",
            &harsh.encode_hex("deadbeef").expect("Failed to encode"),
        );
    }

    #[test]
    fn can_encode_hex_with_padding() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .length(12)
            .build()
            .expect("failed to initialize harsh");

        assert_eq!(
            "RGkRNrpKlJde",
            &harsh.encode_hex("deadbeef").expect("Failed to encode"),
        );
    }

    #[test]
    fn can_decode_hex() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .build()
            .expect("failed to initialize harsh");

        assert_eq!(
            "fa",
            harsh.decode_hex("lzY").expect("failed to decode"),
            "error decoding `FA`"
        );
        assert_eq!(
            "26dd",
            harsh.decode_hex("MemE").expect("failed to decode"),
            "error decoding `26dd`"
        );
        assert_eq!(
            "ff1a",
            harsh.decode_hex("eBMrb").expect("failed to decode"),
            "error decoding `FF1A`"
        );
        assert_eq!(
            "12abc",
            harsh.decode_hex("D9NPE").expect("failed to decode"),
            "error decoding `12abC`"
        );
        assert_eq!(
            "185b0",
            harsh.decode_hex("9OyNW").expect("failed to decode"),
            "error decoding `185b0`"
        );
        assert_eq!(
            "17b8d",
            harsh.decode_hex("MRWNE").expect("failed to decode"),
            "error decoding `17b8d`"
        );
        assert_eq!(
            "1d7f21dd38",
            harsh.decode_hex("4o6Z7KqxE").expect("failed to decode"),
            "error decoding `1d7f21dd38`"
        );
        assert_eq!(
            "20015111d",
            harsh.decode_hex("ooweQVNB").expect("failed to decode"),
            "error decoding `20015111d`"
        );
        assert_eq!(
            "deadbeef",
            harsh.decode_hex("kRNrpKlJ").expect("failed to decode"),
            "error decoding `deadbeef`"
        );

        let harsh = HarshBuilder::new().build().unwrap();
        assert_eq!(
            "507f1f77bcf86cd799439011",
            harsh
                .decode_hex("y42LW46J9luq3Xq9XMly",)
                .expect("failed to decode",),
            "error decoding `y42LW46J9luq3Xq9XMly`"
        );
    }

    #[test]
    fn can_decode_hex_with_guards() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .length(10)
            .build()
            .expect("failed to initialize harsh");

        assert_eq!(
            "deadbeef",
            harsh.decode_hex("GkRNrpKlJd").expect("failed to decode"),
            "failed to decode GkRNrpKlJd"
        );
    }

    #[test]
    fn can_decode_hex_with_padding() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .length(12)
            .build()
            .expect("failed to initialize harsh");

        assert_eq!(
            "deadbeef",
            harsh.decode_hex("RGkRNrpKlJde").expect("failed to decode"),
            "failed to decode RGkRNrpKlJde"
        );
    }

    #[test]
    fn can_encode_with_custom_alphabet() {
        let harsh = HarshBuilder::new()
            .alphabet("abcdefghijklmnopqrstuvwxyz")
            .build()
            .expect("failed to initialize harsh");

        assert_eq!(
            "mdfphx",
            harsh.encode(&[1, 2, 3]),
            "failed to encode [1, 2, 3]"
        );
    }

    #[test]
    #[should_panic]
    fn can_decode_with_invalid_alphabet() {
        let harsh = Harsh::default();
        harsh.decode("this$ain't|a\number").unwrap();
    }

    #[test]
    fn can_decode_with_custom_alphabet() {
        let harsh = HarshBuilder::new()
            .alphabet("abcdefghijklmnopqrstuvwxyz")
            .build()
            .expect("failed to initialize harsh");

        assert_eq!(
            &[1, 2, 3],
            &harsh.decode("mdfphx").expect("failed to decode")[..],
            "failed to decode mdfphx"
        );
    }

    #[test]
    fn create_nhash() {
        let values = &[1, 2, 3];
        let nhash = super::create_nhash(values);
        assert_eq!(6, nhash);
    }

    #[test]
    fn hash() {
        let result = super::hash(22, b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!("w", result);
    }

    #[test]
    fn shuffle() {
        let salt = b"1234";
        let mut values = "asdfzxcvqwer".bytes().collect::<Vec<_>>();
        super::shuffle(&mut values, salt);

        assert_eq!("vdwqfrzcsxae", String::from_utf8_lossy(&values));
    }

    #[test]
    fn guard_characters_should_be_added_to_left_first() {
        let harsh = HarshBuilder::new().length(3).build().unwrap();
        let hashed_value = harsh.encode(&[1]);

        assert_eq!(&hashed_value, "ejR");
        assert_eq!(vec![1], harsh.decode("ejR").unwrap());
    }

    #[test]
    #[should_panic]
    fn appended_garbage_data_invalidates_hashid() {
        let harsh = HarshBuilder::new().length(4).build().unwrap();
        let id = harsh.encode(&[1, 2]) + "12";
        harsh.decode(id).unwrap();
    }
}
