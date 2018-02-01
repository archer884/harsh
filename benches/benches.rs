#![feature(test)]

extern crate harsh;
extern crate test;

use test::{Bencher, black_box};

use harsh::{Harsh, HarshBuilder};

const CUSTOM_SALT: &'static str = "i am the salt of the earth";

fn init() -> Harsh {
    black_box(HarshBuilder::new().salt(black_box(CUSTOM_SALT)).length(black_box(20)).init().unwrap())
}

fn data() -> Vec<u64> {
    let mut x = 0u64;
    black_box((0..100).into_iter().map(|i| {
        x = x.wrapping_mul(13).wrapping_add(i);
        x
    }).collect())
}

#[bench]
fn custom_creation(b: &mut Bencher) {
    b.iter(|| init());
}

#[bench]
fn default_creation(b: &mut Bencher) {
    b.iter(|| Harsh::default());
}

#[bench]
fn encode(b: &mut Bencher) {
    let data = data();
    let harsh = init();
    b.iter(|| harsh.encode(&data).unwrap());
}

#[bench]
fn decode(b: &mut Bencher) {
    let data = data();
    let harsh = init();
    let encoded = black_box(harsh.encode(&data).unwrap());
    b.iter(|| harsh.decode(&encoded).unwrap());
}
