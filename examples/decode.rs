use harsh::{HarshBuilder, Result};
use std::env;

fn main() -> Result<()> {
    let harsh = HarshBuilder::new().init()?;
    let input = env::args().nth(1).expect("Wut?");
    println!("{:?}", harsh.decode(input));
    Ok(())
}
