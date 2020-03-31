use harsh::Harsh;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let harsh = Harsh::new().build()?;
    println!("{:?}", harsh.encode(&read_values()));
    Ok(())
}

fn read_values() -> Vec<u64> {
    std::env::args()
        .skip(1)
        .filter_map(|n| n.parse::<u64>().ok())
        .collect()
}
