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

#[test]
fn serde_roundtrip() {
    use core::time::Duration;

    use validators::serde_json;

    #[derive(Validator)]
    #[validator(json)]
    struct JSONArray(Vec<bool>);

    #[derive(Validator)]
    #[validator(json)]
    struct JSONSigned(i128);

    #[derive(Validator)]
    #[validator(json)]
    struct JSONUnsigned(u128);

    #[derive(Validator)]
    #[validator(json)]
    struct JSONDuration(Duration);

    let boolean = JSONBoolean::parse_bool(true).unwrap();
    let json = serde_json::to_string(&boolean).unwrap();
    assert_eq!("true", json);
    assert!(serde_json::from_str::<JSONBoolean>(&json).unwrap().0);

    let string = JSONString::parse_str("\"abc\"").unwrap();
    let json = serde_json::to_string(&string).unwrap();
    assert_eq!("\"abc\"", json);
    assert_eq!("abc", serde_json::from_str::<JSONString>(&json).unwrap().0);

    let array = JSONArray::parse_str("[true,false]").unwrap();
    let json = serde_json::to_string(&array).unwrap();
    assert_eq!("[true,false]", json);
    assert_eq!(vec![true, false], serde_json::from_str::<JSONArray>(&json).unwrap().0);

    let signed = JSONSigned::parse_i128(i128::MIN).unwrap();
    let json = serde_json::to_string(&signed).unwrap();
    assert_eq!(i128::MIN, serde_json::from_str::<JSONSigned>(&json).unwrap().0);

    let unsigned = JSONUnsigned::parse_u128(u128::MAX).unwrap();
    let json = serde_json::to_string(&unsigned).unwrap();
    assert_eq!(u128::MAX, serde_json::from_str::<JSONUnsigned>(&json).unwrap().0);

    let json = r#"{"secs":1,"nanos":2}"#;
    assert_eq!(Duration::new(1, 2), serde_json::from_str::<JSONDuration>(json).unwrap().0);
    assert!(serde_json::from_str::<JSONDuration>(r#"{"secs":1,"secs":2,"nanos":2}"#).is_err());
}

#[test]
fn non_finite_numbers() {
    use validators::serde_json::Value;

    #[derive(Validator)]
    #[validator(json)]
    struct JSONValue(Value);

    #[derive(Validator)]
    #[validator(json)]
    struct JSONOptionalNumber(Option<f64>);

    #[derive(Validator)]
    #[validator(json)]
    struct JSONNumber(f64);

    assert!(JSONValue::parse_f64(f64::NAN).is_err());
    assert!(JSONOptionalNumber::parse_f64(f64::INFINITY).is_err());
    assert!(JSONNumber::parse_f32(f32::NEG_INFINITY).is_err());
    assert_eq!(1.5, JSONNumber::parse_f64(1.5).unwrap().0);
}

#[test]
fn serde_result_errors() {
    use validators::{Result, errors::JsonError, serde_json};

    type Address = Result<JSONString, JsonError>;

    let invalid_value: Address = serde_json::from_str("123").unwrap();

    assert!(invalid_value.into_std_result().is_err());
    assert!(serde_json::from_str::<Address>("").is_err());
}
