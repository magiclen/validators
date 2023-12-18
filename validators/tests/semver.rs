#![cfg(all(feature = "test", feature = "derive", feature = "semver"))]

use validators::{prelude::*, semver::Version};

#[derive(Validator)]
#[validator(semver)]
pub struct Validator(pub Version);

#[test]
fn basic() {
    assert!(Validator::validate_str("").is_err());
    assert!(Validator::validate_str("0.0.0").is_ok());
    assert!(Validator::validate_str("00.0.0").is_err());
    assert!(Validator::validate_str("0.0").is_err());
    assert!(Validator::validate_str("0.0.0-beta.1").is_ok());
}
