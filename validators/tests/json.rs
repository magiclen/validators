#![cfg(all(feature = "test", feature = "derive", feature = "json"))]

use validators::prelude::*;

#[derive(Validator)]
#[validator(json)]
struct JSONString(String);

#[derive(Validator)]
#[validator(json)]
struct JSONBoolean(bool);

#[test]
fn basic() {
    assert!(JSONString::validate_str("123").is_err());
    assert!(JSONString::validate_str("\"123\"").is_ok());
    assert!(JSONBoolean::validate_str("true").is_ok());
}
