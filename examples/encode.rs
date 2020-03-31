use harsh::Harsh;

fn main() {
    let harsh = Harsh::default();
    println!("{:?}", harsh.encode(&read_values()));
}

fn read_values() -> Vec<u64> {
    std::env::args()
        .skip(1)
        .filter_map(|n| n.parse::<u64>().ok())
        .collect()
}
