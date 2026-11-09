#![cfg(all(feature = "test", feature = "derive", feature = "phone"))]

use validators::{
    phonenumber::{PhoneNumber, country::Id},
    prelude::*,
};

#[derive(Validator)]
#[validator(phone)]
pub struct InternationalPhone(pub PhoneNumber);

#[derive(Validator)]
#[validator(phone(countries(TW)))]
pub struct TWPhone(pub PhoneNumber);

#[derive(Validator)]
#[validator(phone(countries(TW, US)))]
pub struct TWorUSPhone(pub std::collections::HashMap<Id, PhoneNumber>);

#[test]
fn basic() {
    assert!(InternationalPhone::validate_str("+886912345678").is_ok());
    assert!(InternationalPhone::validate_str("0912345678").is_err());
    assert!(InternationalPhone::validate_str("+14155552671").is_ok());

    assert!(TWPhone::validate_str("+886912345678").is_ok());
    assert!(TWPhone::validate_str("0912345678").is_ok());
    assert!(TWPhone::validate_str("+14155552671").is_err());

    assert!(TWorUSPhone::validate_str("+886912345678").is_ok());
    assert!(TWorUSPhone::validate_str("0912345678").is_ok());
    assert!(TWorUSPhone::validate_str("+14155552671").is_ok());
}

#[test]
fn valid_in_second_country() {
    #[derive(Validator)]
    #[validator(phone(countries(US, BR)))]
    struct USorBRPhone(std::collections::HashMap<Id, PhoneNumber>);

    let phone = USorBRPhone::parse_str("011997123456").unwrap();

    assert!(USorBRPhone::validate_str("011997123456").is_ok());
    assert_eq!(1, phone.0.len());
    assert!(phone.0.contains_key(&Id::BR));
}

#[cfg(feature = "serde_json")]
#[test]
fn serde_roundtrip() {
    use validators::{Result, errors::PhoneError, serde_json};

    // `PhoneNumber` also compares how the country code was written, so compare E.164 strings.
    let phone = TWPhone::parse_str("0912345678").unwrap();
    let json = serde_json::to_string(&phone).unwrap();

    assert_eq!(r#""+886912345678""#, json);
    assert_eq!("+886912345678", serde_json::from_str::<TWPhone>(&json).unwrap().0.to_string());

    let result: Result<TWPhone, PhoneError> = serde_json::from_str(&json).unwrap();

    assert_eq!("+886912345678", result.into_std_result().unwrap().0.to_string());

    let phone = InternationalPhone::parse_str("+1 650-253-0000 ext. 4567").unwrap();
    let json = serde_json::to_string(&phone).unwrap();

    assert_eq!(r#""tel:+1-650-253-0000;ext=4567""#, json);
    assert_eq!(
        Some("4567"),
        serde_json::from_str::<InternationalPhone>(&json)
            .unwrap()
            .0
            .extension()
            .map(|extension| extension.as_ref())
    );

    let phone = TWorUSPhone::parse_str("+14155552671").unwrap();
    let json = serde_json::to_string(&phone).unwrap();

    assert_eq!(r#"{"US":"+14155552671"}"#, json);
    assert_eq!(
        "+14155552671",
        serde_json::from_str::<TWorUSPhone>(&json).unwrap().0[&Id::US].to_string()
    );
    assert!(
        serde_json::from_str::<TWorUSPhone>(r#"{"US":"+14155552671","US":"+14155552671"}"#)
            .is_err()
    );
}
