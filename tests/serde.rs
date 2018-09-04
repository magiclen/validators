#![cfg(feature = "serdely-test")]

#[macro_use]
extern crate validators;
#[macro_use]
extern crate serde_json;

use validators::base32::Base32;
use validators::domain::DomainUnlocalhostableWithoutPort;

#[test]
fn test_de_1() {
    serde_json::from_str::<Base32>("\"EB2GK43UEBWWK43TMFTWKCQK\"").unwrap();
}

#[test]
fn test_se_1() {
    let base32 = Base32::from_str("EB2GK43UEBWWK43TMFTWKCQK").unwrap();

    assert_eq!("\"EB2GK43UEBWWK43TMFTWKCQK\"", json!(base32).to_string());
}

#[test]
fn test_de_2() {
    serde_json::from_str::<DomainUnlocalhostableWithoutPort>("\"magiclen.org\"").unwrap();
}

#[test]
fn test_se_2() {
    let domain = DomainUnlocalhostableWithoutPort::from_str("magiclen.org").unwrap();

    assert_eq!("\"magiclen.org\"", json!(domain).to_string());
}

#[test]
fn test_de_3() {
    validated_customized_regex_string!(S1, "^(Hi|Hello)$");

    serde_json::from_str::<S1>("\"Hi\"").unwrap();
}

#[test]
fn test_se_3() {
    validated_customized_regex_string!(S1, "^(Hi|Hello)$");

    let s = S1::from_str("Hello").unwrap();

    assert_eq!("\"Hello\"", json!(s).to_string());
}

#[test]
fn test_de_4() {
    validated_customized_ranged_number!(Score, u8, 0, 100);

    serde_json::from_value::<Score>(serde_json::Value::from(23)).unwrap();
}

#[test]
fn test_se_4() {
    validated_customized_ranged_number!(Score, u8, 0, 100);

    let s = Score::from_number(23).unwrap();

    assert_eq!("23", json!(s).to_string());
}

#[test]
fn test_de_5() {
    validated_customized_ranged_length_vec!(Greet, 1, 5);
    validated_customized_regex_string!(S1, "^(Hi|Hello)$");

    let v = vec!["Hi", "Hello"];

    serde_json::from_value::<Greet<S1>>(serde_json::Value::from(v)).unwrap();
}

#[test]
fn test_se_5() {
    validated_customized_regex_string!(S1, "^(Hi|Hello)$");

    let s1 = S1::from_str("Hi").unwrap();
    let s2 = S1::from_str("Hello").unwrap();

    let v = vec![s1, s2];

    assert_eq!("[\"Hi\",\"Hello\"]", json!(v).to_string());
}