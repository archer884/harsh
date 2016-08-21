extern crate harsh;

use harsh::HarshFactory;

fn main() {
    let harsh = HarshFactory::new().init().unwrap();
    match std::env::args().nth(1) {
        None => println!("provide something to decode, plzkthx"),
        Some(ref value) => println!("{:?}", harsh.decode(value)),
    }
}
