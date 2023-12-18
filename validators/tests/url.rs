#![cfg(all(feature = "test", feature = "derive", feature = "url"))]

use validators::{prelude::*, url::Url};

#[derive(Validator)]
#[validator(url)]
pub struct Validator(pub Url);

#[test]
fn basic() {
    assert!(Validator::validate_str("").is_err());
    assert!(Validator::validate_str("https://example.org/").is_ok());
    assert!(Validator::validate_str("https:example.org").is_ok());
    assert!(Validator::validate_str("example:").is_ok());
}
