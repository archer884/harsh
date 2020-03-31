use crate::harsh::Harsh;

#[test]
fn small_alphabet() {
    assert!(
        !Harsh::new().alphabet("1234567890").build().is_ok(),
        "should throw an error with a small alphabet"
    );
}

#[test]
fn spaces_in_alphabet() {
    assert!(
        !Harsh::new()
            .alphabet("a cdefghijklmnopqrstuvwxyz")
            .build()
            .is_ok(),
        "should throw an error when alphabet includes spaces"
    );
}

#[test]
fn should_fail_for_encoding_nothing() {
    assert_eq!(
        "",
        &Harsh::default().encode(&[]),
        "should return None when encoding an empty array"
    );
}

#[test]
#[should_panic]
fn should_fail_for_decoding_nothing() {
    Harsh::default().decode("").unwrap();
}

#[test]
#[should_panic]
fn should_fail_for_decoding_invalid_id() {
    Harsh::default().decode("f").unwrap();
}

#[test]
#[should_panic]
fn should_fail_when_encoding_non_hex_input() {
    Harsh::default().encode_hex("z").unwrap();
}

#[test]
#[should_panic]
fn should_fail_when_hex_decoding_invalid_id() {
    Harsh::default().decode_hex("f").unwrap();
}
