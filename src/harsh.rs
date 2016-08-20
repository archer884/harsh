use {Error, Result};
use {
    DEFAULT_ALPHABET,
    DEFAULT_SEPARATORS,
    GUARD_DIV,
    MINIMUM_ALPHABET_LENGTH,
    SEPARATOR_DIV,
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
        let (alphabet, separators) = alphabet_and_separators(&self.separators, &alphabet, &salt);
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

fn alphabet_and_separators(separators: &Option<Vec<u8>>, alphabet: &[u8], salt: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let separators = match *separators {
        None => DEFAULT_SEPARATORS,
        Some(ref separators) => separators,
    };

    let mut separators: Vec<_> = separators.iter().cloned().filter(|item| alphabet.contains(&item)).collect();
    let mut alphabet: Vec<_> = alphabet.iter().cloned().filter(|item| !separators.contains(&item)).collect();

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
        use {DEFAULT_ALPHABET, DEFAULT_SEPARATORS};

        let (alphabet, separators) = harsh::alphabet_and_separators(&Some(DEFAULT_SEPARATORS.to_vec()), DEFAULT_ALPHABET, b"this is my salt");

        assert_eq!("AdG05N6y2rljDQak4xgzn8ZR1oKYLmJpEbVq3OBv9WwXPMe7", alphabet.iter().map(|&u| u as char).collect(): String);
        assert_eq!("UHuhtcITCsFifS", separators.iter().map(|&u| u as char).collect(): String);
    }

    #[test]
    fn alphabet_and_separator_generation_with_few_separators() {
        use {DEFAULT_ALPHABET};

        let separators = b"fu";
        let (alphabet, separators) = harsh::alphabet_and_separators(&Some(separators.to_vec()), DEFAULT_ALPHABET, b"this is my salt");

        assert_eq!("4RVQrYM87wKPNSyTBGU1E6FIC9ALtH0ZD2Wxz3vs5OXJ", alphabet.iter().map(|&u| u as char).collect(): String);
        assert_eq!("ufabcdeghijklmnopq", separators.iter().map(|&u| u as char).collect(): String);        
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
