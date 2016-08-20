extern crate harsh;

use harsh::HarshFactory;

fn main() {
    match read_values() {
        None => println!("provide some numeric args, plzkthx"),
        Some(ref values) => println!("{}", encode(values)),
    }
}

fn encode(values: &[u64]) -> String {
    let harsh = HarshFactory::new()
        .with_salt("this is my salt")
        .with_hash_length(8)
        .init()
        .expect("failed to initialize harsh");

    harsh.encode(values).expect("failed to encode")
}

fn read_values() -> Option<Vec<u64>> {
    std::env::args().skip(1).map(|n| n.parse().ok()).collect()
}
