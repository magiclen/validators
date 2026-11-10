#![cfg(all(feature = "test", feature = "derive", feature = "semver_req"))]

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

#[cfg(feature = "serde_json")]
#[test]
fn serde_roundtrip() {
    use validators::{Result, errors::SemverError, serde_json};

    let req: Validator = serde_json::from_str(r#"">=0.4""#).unwrap();
    let json = serde_json::to_string(&req).unwrap();
    let wrapped: Result<Validator, SemverError> = serde_json::from_str(&json).unwrap();

    assert_eq!(r#"">=0.4""#, json);
    assert_eq!(">=0.4", wrapped.into_std_result().unwrap().0.to_string());
    assert!(serde_json::from_str::<Validator>(r#""00.0.0""#).is_err());
}
