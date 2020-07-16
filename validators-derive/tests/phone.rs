#![cfg(feature = "phone")]

#[macro_use]
extern crate validators_derive;

extern crate validators;

use validators::prelude::*;
use validators_prelude::phonenumber;

#[derive(Validator)]
#[validator(phone)]
pub struct InternationalPhone(pub phonenumber::PhoneNumber);

#[derive(Validator)]
#[validator(phone(TW))]
pub struct TWPhone(pub phonenumber::PhoneNumber);

#[derive(Validator)]
#[validator(phone(TW, US))]
pub struct TWorUSPhone(
    pub std::collections::HashMap<phonenumber::country::Id, phonenumber::PhoneNumber>,
);

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
