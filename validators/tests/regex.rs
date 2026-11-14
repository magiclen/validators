#![cfg(all(feature = "test", feature = "derive", feature = "regex"))]

use std::sync::LazyLock;

use validators::{prelude::*, regex::Regex};

#[test]
fn regex_macro_expression() {
    macro_rules! validators {
        ($value:ident, $list:ident, $pattern:expr) => {
            #[derive(Validator)]
            #[validator(regex(regex = $pattern))]
            struct $value(String);

            #[derive(Validator)]
            #[validator(regex(regex($pattern)))]
            struct $list(String);
        };
    }

    validators!(Literal, LiteralList, "^a$");
    validators!(Existing, ExistingList, RE_POKER);

    assert_eq!("a", Literal::parse_str("a").unwrap().0);
    assert_eq!("a", LiteralList::parse_str("a").unwrap().0);
    assert_eq!("J", Existing::parse_str("J").unwrap().0);
    assert_eq!("J", ExistingList::parse_str("J").unwrap().0);
}

#[test]
fn parse_shadowed_name() {
    #[derive(Validator)]
    #[validator(regex(regex("^a$")))]
    struct S(String);

    assert_eq!("a", S::parse_str("a").unwrap().0);
    assert_eq!("a", S::parse_string(String::from("a")).unwrap().0);
}

static RE_NON_ZERO_NUMBERS: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[1-9]+$").unwrap());

static RE_POKER: LazyLock<Regex> = LazyLock::new(|| Regex::new("^([AJQK1-9]|10)$").unwrap());

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
