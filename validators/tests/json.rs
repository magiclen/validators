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
    use validators::serde_json;

    #[derive(Validator)]
    #[validator(json)]
    struct JSONArray(Vec<bool>);

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
}
