use harsh::Harsh;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let harsh = Harsh::new().build()?;
    let input = env::args().nth(1).expect("Wut?");
    println!("{:?}", harsh.decode(input));
    Ok(())
}
