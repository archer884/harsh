use harsh::{HarshBuilder, Result};

fn main() -> Result<()> {
    let harsh = HarshBuilder::new().init()?;
    println!("{:?}", harsh.encode(&read_values()));
    Ok(())
}

fn read_values() -> Vec<u64> {
    std::env::args()
        .skip(1)
        .filter_map(|n| n.parse::<u64>().ok())
        .collect()
}
