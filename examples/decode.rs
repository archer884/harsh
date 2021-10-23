use std::{env, error::Error};

use harsh::Harsh;

fn main() -> Result<(), Box<dyn Error>> {
    let harsh = Harsh::default();
    let input = env::args().nth(1).expect("Wut?");
    println!("{:?}", harsh.decode(input));
    Ok(())
}
