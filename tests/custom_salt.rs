extern crate harsh;

use harsh::HarshBuilder;

#[test]
fn empty_salt() {
    test_salt("", "should work with ''");
}

#[test]
fn spaces_salt() {
    test_salt("   ", "should work with '   '");
}

#[test]
fn ordinary_salt() {
    test_salt("this is my salt", "should work with 'this is my salt'");
}

#[test]
fn long_salt() {
    test_salt(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890`~!@#$%^&*()-_=+\\|'\";:/?.>,<{[}]",
        "should work with a really long salt",
    );
}

#[test]
fn weird_salt() {
    test_salt(
        "`~!@#$%^&*()-_=+\\|'\";:/?.>,<{[}]",
        "should work with a weird salt",
    )
}

fn test_salt(salt: &str, message: &str) {
    assert!(HarshBuilder::new().salt(salt).init().is_ok(), "{}", message);
}
