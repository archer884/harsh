use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harsh::Harsh;

static CUSTOM_SALT: &str = "In the beginning, God created the heaven and the earth.";

fn default_initialization(c: &mut Criterion) {
    c.bench_function("Default initialization", |b| b.iter(Harsh::default));
}

fn custom_initialization(c: &mut Criterion) {
    c.bench_function("Custom initialization", |b| {
        b.iter(|| {
            black_box(
                Harsh::builder()
                    .salt(black_box(CUSTOM_SALT))
                    .length(black_box(20))
                    .build()
                    .unwrap(),
            )
        })
    });
}

fn encode(c: &mut Criterion) {
    let harsh = Harsh::builder().salt(CUSTOM_SALT).build().unwrap();
    c.bench_function("Encode", |b| {
        b.iter(|| black_box(harsh.encode(black_box(&[1, 2, 3, 4, 5]))))
    });
}

fn decode(c: &mut Criterion) {
    let harsh = Harsh::builder().salt(CUSTOM_SALT).build().unwrap();
    let encoded = harsh.encode(&[1, 2, 3, 4, 5]);
    c.bench_function("Decode", |b| {
        b.iter(|| black_box(harsh.decode(black_box(&encoded))))
    });
}

criterion_group!(
    benches,
    default_initialization,
    custom_initialization,
    encode,
    decode,
);

criterion_main!(benches);
