extern crate harsh;

use harsh::HarshBuilder;

fn main() {
    let harsh = HarshBuilder::new().init().unwrap();
    match std::env::args().nth(1) {
        None => println!("provide something to decode, plzkthx"),
        Some(ref value) => println!("{:?}", harsh.decode(value)),
    }
}
