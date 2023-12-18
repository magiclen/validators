#![cfg(all(feature = "test", feature = "derive", feature = "phone"))]

use validators::{
    phonenumber::{country::Id, PhoneNumber},
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
