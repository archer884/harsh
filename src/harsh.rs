use {Error, Result};
use {
    DEFAULT_ALPHABET,
    DEFAULT_SEPARATORS,
    GUARD_DIV,
    MINIMUM_ALPHABET_LENGTH,
    // SEPARATOR_DIV,
};

pub struct Harsh {
    salt: Vec<u8>,
    alphabet: Vec<u8>,
    separators: Vec<u8>,
    hash_length: usize,
    guards: Vec<u8>,
}

impl Harsh {
    pub fn encode(&self, values: &[i64]) -> Option<String> {
        // TODO: decide if this makes sense vs returning a blank string
        if values.len() == 0 {
            return None;
        }

        let nhash = create_nhash(values);

        let mut alphabet = self.alphabet.clone();
        let mut buffer = String::new();

        let idx = (nhash % alphabet.len() as i64) as usize;
        let lottery = alphabet[idx];
        buffer.push(lottery as char);

        for (idx, &value) in values.iter().enumerate() {
            let mut value = value;

            let temp = {
                let mut temp = Vec::with_capacity(self.salt.len() + alphabet.len() + 1);
                temp.push(lottery);
                temp.extend_from_slice(&self.salt);
                temp.extend_from_slice(&alphabet);
                temp
            };

            let alphabet_len = alphabet.len();
            shuffle(&mut alphabet, &temp[..alphabet_len]);

            let last = hash(value, &alphabet);
            buffer.push_str(&last);

            if idx + 1 < values.len() {
                value %= (last.bytes().nth(0).unwrap_or(0) as usize + idx) as i64;
                buffer.push(self.separators[(value % self.separators.len() as i64) as usize] as char);
            }
        }

        if buffer.len() < self.hash_length {
            let guard_index = (nhash as usize + buffer.bytes().nth(0).expect("hellfire and damnation") as usize) % self.guards.len();
            let guard = self.guards[guard_index];
            buffer.insert(0, guard as char);

            if buffer.len() < self.hash_length {
                let guard_index = (nhash as usize + buffer.bytes().nth(2).expect("hellfire and damnation") as usize) % self.guards.len();
                let guard = self.guards[guard_index];
                buffer.push(guard as char);
            }
        }

        let half_length = alphabet.len() / 2;
        while buffer.len() < self.hash_length {
            {
                let alphabet_copy = alphabet.clone(); // stupid borrowck -.-
                shuffle(&mut alphabet, &alphabet_copy);
            }
            println!("{}", String::from_utf8_lossy(&alphabet));

            let (left, right) = alphabet.split_at(half_length);
            buffer = format!("{}{}{}", String::from_utf8_lossy(right), buffer, String::from_utf8_lossy(left));

            let excess = buffer.len() as i32 - self.hash_length as i32;
            if excess > 0 {
                let marker = excess as usize / 2;
                buffer = buffer[marker..marker + self.hash_length].to_owned();
            }
        }

        Some(buffer)
    }

    pub fn decode(&self, value: &str) -> Option<Vec<i64>> {
        if value.len() == 0 {
            return None;
        }

        let value = self.unpad_value(value).as_bytes();
        let mut alphabet = self.alphabet.clone();
        
        let lottery = value[0];
        let value = &value[1..];
        let segments: Vec<_> = value.split(|u| self.separators.contains(u)).collect();

        Some(segments.into_iter().map(|segment| {
            let buffer = {
                let mut buffer = Vec::with_capacity(self.salt.len() + alphabet.len() + 1);
                buffer.push(lottery);
                buffer.extend_from_slice(&self.salt);
                buffer.extend_from_slice(&alphabet);
                buffer
            };

            let alphabet_len = alphabet.len();
            shuffle(&mut alphabet, &buffer[..alphabet_len]);
            unhash(&segment, &alphabet)
        }).collect())
    }

    #[inline]
    fn unpad_value<'a>(&'a self, value: &'a str) -> &'a str {
        let segments: Vec<_> = value.split(|c| self.guards.contains(&(c as u8))).collect();

        match segments.len() {
            1 => value,
            2 | 3 => segments[1],
            _ => panic!("why the hell would you use three guards?")
        }
    }
}

pub struct HarshFactory {
    salt: Option<Vec<u8>>,
    alphabet: Option<Vec<u8>>,
    separators: Option<Vec<u8>>,
    hash_length: usize,
}

impl HarshFactory {
    pub fn new() -> HarshFactory {
        HarshFactory {
            salt: None,
            alphabet: None,
            separators: None,
            hash_length: 0,
        }
    }

    pub fn with_salt<T: Into<Vec<u8>>>(mut self, salt: T) -> HarshFactory {
        self.salt = Some(salt.into());
        self
    }

    pub fn with_alphabet<T: Into<Vec<u8>>>(mut self, alphabet: T) -> HarshFactory {
        self.alphabet = Some(alphabet.into());
        self
    }

    pub fn with_separators<T: Into<Vec<u8>>>(mut self, separators: T) -> HarshFactory {
        self.separators = Some(separators.into());
        self
    }

    pub fn with_hash_length(mut self, hash_length: usize) -> HarshFactory {
        self.hash_length = hash_length;
        self
    }

    pub fn init(self) -> Result<Harsh> {
        let alphabet = unique_alphabet(&self.alphabet)?;
        if alphabet.len() < MINIMUM_ALPHABET_LENGTH {
            return Err(Error::AlphabetLength);
        }

        let salt = self.salt.unwrap_or_else(|| Vec::new());

        // TODO: burn less ram here, plz
        let mut separators = separators(&self.separators, &alphabet)?;
        let mut alphabet = remove_separators(&alphabet, &separators);

        shuffle(&mut separators, &salt);
        shuffle(&mut alphabet, &salt);

        let (guards, alphabet) = guards(&alphabet, &separators);

        Ok(Harsh {
            salt: salt,
            alphabet: alphabet,
            separators: separators,
            hash_length: self.hash_length,
            guards: guards,
        })
    }
}

#[inline]
fn create_nhash(values: &[i64]) -> i64 {
    values.iter().enumerate().fold(0, |a, (idx, value)| a + (value % (idx + 100) as i64))
}

fn unique_alphabet(alphabet: &Option<Vec<u8>>) -> Result<Vec<u8>> {
    match *alphabet {
        None => Ok(DEFAULT_ALPHABET.iter().cloned().collect()),
        Some(ref alphabet) => {
            let mut alphabet: Vec<_> = alphabet.iter().cloned().collect();
            alphabet.sort();
            alphabet.dedup();

            if alphabet.len() < 16 {
                Err(Error::AlphabetLength)
            } else {
                Ok(alphabet)
            }
        }
    }
}

fn separators(separators: &Option<Vec<u8>>, alphabet: &[u8]) -> Result<Vec<u8>> {
    match *separators {
        None => Ok(DEFAULT_SEPARATORS.iter().cloned().collect()),
        Some(ref separators) => {
            separators.iter().map(|&separator| {
                if alphabet.binary_search(&separator).is_err() {
                    Err(Error::Separator)
                } else {
                    Ok(separator)
                }
            }).collect()
        }
    }
}

fn remove_separators(alphabet: &[u8], separators: &[u8]) -> Vec<u8> {
    alphabet.iter().filter(|c| !separators.contains(c)).cloned().collect()
}

fn guards(alphabet: &[u8], separators: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let guard_count = alphabet.len() / GUARD_DIV;
    match alphabet.len() {
        0...2 => (
            separators[..guard_count].iter().cloned().collect(),
            alphabet.iter().cloned().collect(),
        ),
        _ => (
            alphabet[..guard_count].iter().cloned().collect(),
            alphabet[guard_count..].iter().cloned().collect(),
        )
    }
}

fn shuffle(values: &mut [u8], salt: &[u8]) {
    if salt.len() == 0 {
        return;
    }

    let values_length = values.len();
    let salt_length = salt.len();
    let (mut v, mut p) = (0, 0);

    for i in (1..values_length).map(|i| values_length - i) {
        v = v % salt_length;
        
        let n = salt[v] as usize;
        p += n;
        let j = (n + v + p) % i;

        values.swap(i, j);
        v += 1;
    }
}

fn hash(mut value: i64, alphabet: &[u8]) -> String {
    let mut hash = Vec::new();
    loop {
        hash.push(alphabet[value as usize % alphabet.len()]);
        value /= alphabet.len() as i64;

        if value <= 0 {
            return hash.iter().rev().cloned().map(|item| item as char).collect();
        }
    }
}

fn unhash(input: &[u8], alphabet: &[u8]) -> i64 {
    input.iter().enumerate().fold(0, |a, (idx, &value)| {
        let pos = alphabet.iter().position(|&item| item == value).expect("what a world, what a world!");
        a + (pos * alphabet.len().pow((input.len() - idx - 1) as u32)) as i64
    })
}

#[cfg(test)]
mod tests {
    use harsh::{self, HarshFactory};

    #[test]
    fn can_encode() {
        let harsh = HarshFactory::new()
            .with_salt("this is my salt")
            .init()
            .expect("failed to initialize harsh");

        assert_eq!("laHquq", harsh.encode(&[1, 2, 3]).expect("failed to encode"));
    }

    #[test]
    fn can_encode_with_guards() {
        let harsh = HarshFactory::new()
            .with_salt("this is my salt")
            .with_hash_length(8)
            .init()
            .expect("failed to initialize harsh");

        assert_eq!("GlaHquq0", harsh.encode(&[1, 2, 3]).expect("failed to encode"));
    }

    #[test]
    fn can_encode_with_padding() {
        let harsh = HarshFactory::new()
            .with_salt("this is my salt")
            .with_hash_length(12)
            .init()
            .expect("failed to initialize harsh");

        assert_eq!("9LGlaHquq06D", harsh.encode(&[1, 2, 3]).expect("failed to encode"));
    }

    #[test]
    fn can_decode() {
        let harsh = HarshFactory::new()
            .with_salt("this is my salt")
            .init()
            .expect("failed to initialize harsh");

        assert_eq!(&[1i64, 2, 3], &harsh.decode("laHquq").expect("failed to decode")[..]);
    }

    #[test]
    fn can_decode_with_guards() {
        let harsh = HarshFactory::new()
            .with_salt("this is my salt")
            .with_hash_length(8)
            .init()
            .expect("failed to initialize harsh");

        assert_eq!(&[1i64, 2, 3], &harsh.decode("GlaHquq0").expect("failed to decode")[..]);
    }

    #[test]
    fn can_decode_with_padding() {
        let harsh = HarshFactory::new()
            .with_salt("this is my salt")
            .with_hash_length(12)
            .init()
            .expect("failed to initialize harsh");

        assert_eq!(&[1i64, 2, 3], &harsh.decode("9LGlaHquq06D").expect("failed to decode")[..]);
    }

    #[test]
    fn create_nhash() {
        let values = &[1, 2, 3];
        let nhash = harsh::create_nhash(values);
        assert_eq!(6, nhash);
    }

    #[test]
    fn hash() {
        let result = harsh::hash(22, b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!("w", result);
    }

    #[test]
    fn alphabet_and_separator_generation() {
        let default_alphabet = "12345asdfzxcvqwer";
        let default_separators = "12345";

        /// Reference implementation from https://github.com/charsyam/hashids_rust/blob/master/src/lib.rs#L138-L180
        /// All credit to user charsyam on github
        /// 
        /// The MIT License (MIT)

        /// Copyright (c) 2015 charsyam

        /// Permission is hereby granted, free of charge, to any person obtaining a copy
        /// of this software and associated documentation files (the "Software"), to deal
        /// in the Software without restriction, including without limitation the rights
        /// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
        /// copies of the Software, and to permit persons to whom the Software is
        /// furnished to do so, subject to the following conditions:

        /// The above copyright notice and this permission notice shall be included in all
        /// copies or substantial portions of the Software.

        /// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
        /// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
        /// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
        /// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
        /// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
        /// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
        /// SOFTWARE.
        fn reference(separators: &str, alphabet: &str) -> (String, String) {
            use std::collections::HashMap;
            
            let mut check_separator_map = HashMap::new();
            let mut check_alphabet_map = HashMap::new();

            let mut modified_separators = String::new();
            let mut modified_alphabet = String::new();

            for c in separators.chars() {
                check_separator_map.insert(c, 1);
            }

            for c in alphabet.chars() {
                check_alphabet_map.insert(c, 1);
            }

            for c in separators.chars() {
                if check_alphabet_map.contains_key(&c) {
                    modified_separators.push(c);
                }
            }

            for c in alphabet.chars() {
                if !check_separator_map.contains_key(&c) {
                    modified_alphabet.push(c);
                }
            }

            (modified_separators, modified_alphabet)
        }

        let separators = harsh::separators(&Some(default_separators.bytes().collect()), default_alphabet.as_ref()).expect("invalid separators in test");
        let alphabet = harsh::remove_separators(default_alphabet.as_ref(), &separators);
        let (ref_separators, ref_alphabet) = reference(default_separators, default_alphabet);

        assert_eq!(ref_separators.bytes().collect(): Vec<_>, separators);
        assert_eq!(ref_alphabet.bytes().collect(): Vec<_>, alphabet);
    }

    #[test]
    fn shuffle() {
        let salt = b"1234";
        let mut values = "asdfzxcvqwer".bytes().collect(): Vec<_>;
        harsh::shuffle(&mut values, salt);

        assert_eq!("vdwqfrzcsxae", String::from_utf8_lossy(&values));
    }

    #[test]
    fn unpad_value() {
        let padded_value = "9LGlaHquq06D";
        let unpadded_value = "laHquq";

        let harsh = HarshFactory::new()
            .with_salt("this is my salt")
            .with_hash_length(12)
            .init()
            .expect("failed to initialize harsh");

        assert_eq!(unpadded_value, harsh.unpad_value(padded_value));
    }
}
