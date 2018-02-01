extern crate harsh;

use harsh::HarshBuilder;

const NUMBERS: [u64; 3] = [1, 2, 3];

#[test]
fn bad_alphabet() {
    test_alphabet("cCsSfFhHuUiItT01", "should work with the worst alphabet");
}

#[test]
fn separators_alphabet() {
    test_alphabet(
        "abdegjklCFHISTUc",
        "should work with half the alphabet being separators",
    );
}

#[test]
fn two_separators() {
    test_alphabet(
        "abdegjklmnopqrSF",
        "should work with exactly two separators",
    );
}

#[test]
fn no_separators() {
    test_alphabet(
        "abdegjklmnopqrvwxyzABDEGJKLMNOPQRVWXYZ1234567890",
        "should work with no separators",
    );
}

#[test]
fn long_alphabet() {
    test_alphabet(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890`~!@#$%^&*()-_=+\\|'\";:/?.>,<{[}]",
        "should work with super-long alphabet",
    );
}

#[test]
fn weird_alphabet() {
    test_alphabet(
        "`~!@#$%^&*()-_=+\\|'\";:/?.>,<{[}]",
        "should work with a weird alphabet",
    );
}

fn test_alphabet(alphabet: &str, message: &str) {
    let harsh = HarshBuilder::new().alphabet(alphabet).init().unwrap();
    let encoded = harsh.encode(&NUMBERS).unwrap();
    let decoded = harsh.decode(encoded).unwrap();

    assert_eq!(NUMBERS, &decoded[..], "{}", message);
}
