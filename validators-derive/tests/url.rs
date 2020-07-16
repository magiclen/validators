#![cfg(feature = "url")]

#[macro_use]
extern crate validators_derive;

extern crate validators;

use validators::prelude::*;
use validators_prelude::url;

#[derive(Validator)]
#[validator(url)]
pub struct Validator(pub url::Url);

#[test]
fn basic() {
    assert!(Validator::validate_str("").is_err());
    assert!(Validator::validate_str("https://example.org/").is_ok());
    assert!(Validator::validate_str("https:example.org").is_ok());
    assert!(Validator::validate_str("example:").is_ok());
}
