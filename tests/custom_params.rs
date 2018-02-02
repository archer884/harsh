extern crate harsh;

use harsh::HarshBuilder;

const TEST_CASES: [(&str, &[u64]); 14] = [
    ("nej1m3d5a6yn875e7gr9kbwpqol02q", &[0]),
    ("dw1nqdp92yrajvl9v6k3gl5mb0o8ea", &[1]),
    ("onqr0bk58p642wldq14djmw21ygl39", &[928_728]),
    ("18apy3wlqkjvd5h1id7mn5ore2d06b", &[1, 2, 3]),
    ("o60edky1ng3vl9hbfavwr5pa2q8mb9", &[1, 0, 0]),
    ("o60edky1ng3vlqfbfp4wr5pa2q8mb9", &[0, 0, 1]),
    ("qek2a08gpl575efrfd7yomj9dwbr63", &[0, 0, 0]),
    ("m3d5a6yn875rae8y81a94gr9kbwpqo", &[1_000_000_000_000]),
    ("1q3y98ln48w96kpo0wgk314w5mak2d", &[0x1F_FFFF_FFFF_FFFF]),
    (
        "op7qrcdc3cgc2c0cbcrcoc5clce4d6",
        &[5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
    ),
    (
        "5430bd2jo0lxyfkfjfyojej5adqdy4",
        &[10_000_000_000, 0, 0, 0, 999_999_999_999_999],
    ),
    (
        "aa5kow86ano1pt3e1aqm239awkt9pk380w9l3q6",
        &[0x1F_FFFF_FFFF_FFFF, 0x1F_FFFF_FFFF_FFFF, 0x1F_FFFF_FFFF_FFFF],
    ),
    (
        "mmmykr5nuaabgwnohmml6dakt00jmo3ainnpy2mk",
        &[1_000_000_001, 1_000_000_002, 1_000_000_003, 1_000_000_004, 1_000_000_005],
    ),
    (
        "w1hwinuwt1cbs6xwzafmhdinuotpcosrxaz0fahl",
        &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
        ],
    ),
];

#[test]
fn custom_params() {
    let harsh = HarshBuilder::new()
        .salt("this is my salt")
        .length(30)
        .alphabet("xzal86grmb4jhysfoqp3we7291kuct5iv0nd")
        .init()
        .unwrap();

    for &(hash, values) in &TEST_CASES {
        assert_eq!(hash, harsh.encode(values).unwrap());
        assert_eq!(values, &harsh.decode(hash).unwrap()[..]);
    }
}
