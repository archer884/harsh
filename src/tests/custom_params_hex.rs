use harsh::HarshBuilder;

const TEST_CASES: [(&'static str, &'static str); 8] =
    [
        ("0dbq3jwa8p4b3gk6gb8bv21goerm96", "deadbeef"),
        ("190obdnk4j02pajjdande7aqj628mr", "abcdef123456"),
        ("a1nvl5d9m3yo8pj1fqag8p9pqw4dyl", "ABCDDD6666DDEEEEEEEEE"),
        ("1nvlml93k3066oas3l9lr1wn1k67dy", "507f1f77bcf86cd799439011"),
        (
            "mgyband33ye3c6jj16yq1jayh6krqjbo",
            "f00000fddddddeeeee4444444ababab",
        ),
        (
            "9mnwgllqg1q2tdo63yya35a9ukgl6bbn6qn8",
            "abcdef123456abcdef123456abcdef123456",
        ),
        (
            "edjrkn9m6o69s0ewnq5lqanqsmk6loayorlohwd963r53e63xmml29",
            "f000000000000000000000000000000000000000000000000000f",
        ),
        (
            "grekpy53r2pjxwyjkl9aw0k3t5la1b8d5r1ex9bgeqmy93eata0eq0",
            "fffffffffffffffffffffffffffffffffffffffffffffffffffff",
        ),
    ];

#[test]
fn custom_params_hex() {
    let harsh = HarshBuilder::new()
        .salt("this is my salt")
        .length(30)
        .alphabet("xzal86grmb4jhysfoqp3we7291kuct5iv0nd")
        .init()
        .unwrap();

    for &(hash, value) in &TEST_CASES {
        assert_eq!(
            hash,
            harsh.encode_hex(value).unwrap(),
            "failed to encode \"{}\"",
            value
        );
        assert_eq!(
            value.to_lowercase(),
            harsh.decode_hex(hash).unwrap(),
            "failed to decode \"{}\"",
            hash
        );
    }
}
