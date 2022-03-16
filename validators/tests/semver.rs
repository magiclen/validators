#![cfg(all(feature = "semver", feature = "derive"))]

use validators::prelude::*;
use validators_prelude::semver;

#[derive(Validator)]
#[validator(semver)]
pub struct Validator(pub semver::Version);

#[test]
fn basic() {
    assert!(Validator::validate_str("").is_err());
    assert!(Validator::validate_str("0.0.0").is_ok());
    assert!(Validator::validate_str("00.0.0").is_err());
    assert!(Validator::validate_str("0.0").is_err());
    assert!(Validator::validate_str("0.0.0-beta.1").is_ok());
}
