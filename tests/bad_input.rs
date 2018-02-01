extern crate harsh;

use harsh::{Harsh, HarshBuilder};

#[test]
fn small_alphabet() {
    assert!(
        !HarshBuilder::new().alphabet("1234567890").init().is_ok(),
        "should throw an error with a small alphabet"
    );
}

#[test]
fn spaces_in_alphabet() {
    assert!(
        !HarshBuilder::new()
            .alphabet("a cdefghijklmnopqrstuvwxyz")
            .init()
            .is_ok(),
        "should throw an error when alphabet includes spaces"
    );
}

#[test]
fn should_return_none_for_encoding_nothing() {
    assert_eq!(
        None,
        Harsh::default().encode(&[]),
        "should return None when encoding an empty array"
    );
}

#[test]
fn should_return_none_for_decoding_nothing() {
    assert_eq!(
        None,
        Harsh::default().decode(""),
        "should return None when decoding nothing"
    );
}

#[test]
fn should_return_none_for_decoding_invalid_id() {
    assert_eq!(
        None,
        Harsh::default().decode("f"),
        "should return None when decoding an invalid id"
    );
}

#[test]
fn should_return_none_when_encoding_non_hex_input() {
    assert_eq!(
        None,
        Harsh::default().encode_hex("z"),
        "should return None when hex-encoding non-hex input"
    );
}

#[test]
fn should_return_none_when_hex_decoding_invalid_id() {
    assert_eq!(
        None,
        Harsh::default().decode_hex("f"),
        "should return None when hex-decoding an invalid id"
    );
}
