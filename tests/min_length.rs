extern crate harsh;

use harsh::HarshBuilder;

const NUMBERS: &'static [u64] = &[1, 2, 3];

#[test]
fn min_length_0() {
    test_minimum_length(0);
}

#[test]
fn min_length_1() {
    test_minimum_length(1);
}

#[test]
fn min_length_10() {
    test_minimum_length(10);
}

#[test]
fn min_length_999() {
    test_minimum_length(999);
}

#[test]
fn min_length_1000() {
    test_minimum_length(1000);
}

fn test_minimum_length(n: usize) {
    let harsh = HarshBuilder::new().length(n).init().unwrap();

    let hash = harsh.encode(NUMBERS).expect("failed to encode values");
    let values = harsh.decode(&hash).expect("failed to decode hash");

    assert_eq!(
        NUMBERS,
        &values[..],
        "encoding/decoding failed at length {}",
        n
    );
    assert!(hash.len() >= n, "length too short for {}", n);
}
