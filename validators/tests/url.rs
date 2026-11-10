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

#[cfg(feature = "serde_json")]
#[test]
fn serde_roundtrip() {
    use validators::{Result, errors::UrlError, serde_json};

    let url: Validator = serde_json::from_str(r#""https://example.org/""#).unwrap();
    let json = serde_json::to_string(&url).unwrap();
    let wrapped: Result<Validator, UrlError> = serde_json::from_str(&json).unwrap();

    assert_eq!(r#""https://example.org/""#, json);
    assert_eq!("https://example.org/", wrapped.into_std_result().unwrap().0.as_str());
    assert!(serde_json::from_str::<Validator>(r#""""#).is_err());
}
