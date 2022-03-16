#![cfg(all(feature = "semver_req", feature = "derive"))]

use validators::prelude::*;
use validators_prelude::semver;

#[derive(Validator)]
#[validator(semver_req)]
pub struct Validator(pub semver::VersionReq);

#[test]
fn basic() {
    assert!(Validator::validate_str("").is_err());
    assert!(Validator::validate_str("0.0.0").is_ok());
    assert!(Validator::validate_str("00.0.0").is_err());
    assert!(Validator::validate_str("0.0").is_ok());
    assert!(Validator::validate_str("0").is_ok());
    assert!(Validator::validate_str("0.0.0-beta.1").is_ok());
    assert!(Validator::validate_str(">= 0.4").is_ok());
}
