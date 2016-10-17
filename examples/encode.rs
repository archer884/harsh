extern crate harsh;

use harsh::HarshBuilder;

fn main() {
    let harsh = HarshBuilder::new().init().unwrap();
    match read_values() {
        None => println!("provide some numeric args, plzkthx"),
        Some(ref values) => println!("{}", harsh.encode(values).unwrap()),
    }
}

fn read_values() -> Option<Vec<u64>> {
    std::env::args().skip(1).map(|n| n.parse().ok()).collect()
}
