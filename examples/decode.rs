extern crate harsh;

use harsh::HarshFactory;

fn main() {
    match std::env::args().nth(1) {
        None => println!("provide some numeric args, plzkthx"),
        Some(ref value) => println!("{:?}", decode(value)),
    }
}

fn decode(value: &str) -> Vec<i64> {
    let harsh = HarshFactory::new()
        .with_salt("this is my salt")
        .with_hash_length(8)
        .init()
        .expect("failed to initialize harsh");

    harsh.decode(value).expect("failed to decode")
}
