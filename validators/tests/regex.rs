#![cfg(all(feature = "test", feature = "derive", feature = "regex"))]

use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use validators::{prelude::*, regex::Regex};

lazy_static! {
    static ref RE_NON_ZERO_NUMBERS: Regex = Regex::new("^[1-9]+$").unwrap();
}

static RE_POKER: Lazy<Regex> = Lazy::new(|| Regex::new("^([AJQK1-9]|10)$").unwrap());

#[derive(Validator)]
#[validator(regex(regex = "^[0-9a-fA-F]+$"))]
pub struct Hex(pub String);

#[derive(Validator)]
#[validator(regex(regex = RE_NON_ZERO_NUMBERS))]
pub struct NonZeroNumbers(pub String);

#[derive(Validator)]
#[validator(regex(regex = RE_POKER))]
pub struct Poker(pub String);

#[test]
fn basic() {
    assert!(Hex::validate_str("1Ab").is_ok());
    assert!(Hex::validate_str("1AG").is_err());

    assert!(NonZeroNumbers::validate_str("12345").is_ok());
    assert!(NonZeroNumbers::validate_str("012345").is_err());

    assert!(Poker::validate_str("1").is_ok());
    assert!(Poker::validate_str("10").is_ok());
    assert!(Poker::validate_str("J").is_ok());
    assert!(Poker::validate_str("0").is_err());
}
