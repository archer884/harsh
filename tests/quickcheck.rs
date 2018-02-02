#[macro_use]
extern crate quickcheck;
extern crate harsh;

use quickcheck::TestResult;

quickcheck!{
    fn decode_no_panic(encoded: String) -> () {
        let harsh = harsh::Harsh::default();

        let _ = harsh.decode(encoded);
    }
}

quickcheck!{
    fn encode_always_decodable(numbers: Vec<u64>) -> TestResult {
        if numbers.is_empty() {
            return TestResult::discard();
        }
        let harsh = harsh::Harsh::default();

        let encoded = harsh.encode(&numbers).expect("Unable to encode numbers");
        harsh.decode(encoded).expect("Unable to decode value");
        TestResult::passed()
    }
}

quickcheck!{
    fn min_length_always_met(numbers: Vec<u64>, min_length: usize) -> TestResult {
        if numbers.is_empty() {
            return TestResult::discard();
        }
        let harsh = harsh::HarshBuilder::new().length(min_length).init().expect("Unable to create harsh");

        let encoded = harsh.encode(&numbers).expect("Unable to encode numbers");
        assert!(encoded.len() >= min_length);
        TestResult::passed()
    }
}
