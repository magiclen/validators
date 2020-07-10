#![cfg(feature = "boolean")]

#[macro_use]
extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(boolean)]
struct Validator(bool);

#[test]
fn basic() {
    assert!(Validator::validate_str("").is_err());
    assert!(Validator::validate_str("xzy").is_err());
    assert!(Validator::validate_str("onion").is_err());
}

#[test]
fn bool_true() {
    assert!(Validator::parse_str("t").unwrap().0);
    assert!(Validator::parse_str("true").unwrap().0);
    assert!(Validator::parse_str("y").unwrap().0);
    assert!(Validator::parse_str("yes").unwrap().0);
    assert!(Validator::parse_str("on").unwrap().0);
    assert!(Validator::parse_str("1").unwrap().0);

    assert!(Validator::parse_char('t').unwrap().0);
    assert!(Validator::parse_char('y').unwrap().0);
    assert!(Validator::parse_char('1').unwrap().0);

    assert!(Validator::parse_i128(1).unwrap().0);
    assert!(Validator::parse_u128(1).unwrap().0);
}

#[test]
fn bool_false() {
    assert!(!Validator::parse_str("f").unwrap().0);
    assert!(!Validator::parse_str("false").unwrap().0);
    assert!(!Validator::parse_str("n").unwrap().0);
    assert!(!Validator::parse_str("no").unwrap().0);
    assert!(!Validator::parse_str("off").unwrap().0);
    assert!(!Validator::parse_str("0").unwrap().0);

    assert!(!Validator::parse_char('f').unwrap().0);
    assert!(!Validator::parse_char('n').unwrap().0);
    assert!(!Validator::parse_char('0').unwrap().0);

    assert!(!Validator::parse_i128(0).unwrap().0);
    assert!(!Validator::parse_u128(0).unwrap().0);
}
