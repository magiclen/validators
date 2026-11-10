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

#[cfg(feature = "serde_json")]
#[test]
fn serde_roundtrip() {
    use validators::{Result, errors::SemverError, serde_json};

    let version: Validator = serde_json::from_str(r#""1.2.3-beta.1""#).unwrap();
    let json = serde_json::to_string(&version).unwrap();
    let wrapped: Result<Validator, SemverError> = serde_json::from_str(&json).unwrap();

    assert_eq!(r#""1.2.3-beta.1""#, json);
    assert_eq!("1.2.3-beta.1", wrapped.into_std_result().unwrap().0.to_string());
    assert!(serde_json::from_str::<Validator>(r#""00.0.0""#).is_err());
}
