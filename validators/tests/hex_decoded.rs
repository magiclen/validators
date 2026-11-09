#![cfg(all(feature = "test", feature = "derive", feature = "hex_decoded"))]

use validators::prelude::*;

#[derive(Validator)]
#[validator(hex_decoded)]
struct Hex(Vec<u8>);

#[derive(Validator)]
#[validator(hex_decoded(case(Upper)))]
struct UpperHex(Vec<u8>);

#[derive(Validator)]
#[validator(hex_decoded(case = Lower))]
struct LowerHex(Vec<u8>);

#[test]
fn decode() {
    let decoded = Hex::parse_str("00aBff").unwrap();

    assert_eq!(&[0, 171, 255], decoded.0.as_slice());
    assert_eq!(3, decoded.len());
    assert_eq!(&[0, 171, 255], UpperHex::parse_u8_slice(b"00ABFF").unwrap().0.as_slice());
    assert_eq!(&[0, 171, 255], LowerHex::parse_vec_u8(*b"00abff").unwrap().0.as_slice());

    assert!(UpperHex::validate_str("0123").is_ok());
    assert!(LowerHex::validate_u8_slice(b"0123").is_ok());
    assert!(UpperHex::validate_u8_slice(b"00abff").is_err());
    assert!(LowerHex::parse_str("00ABFF").is_err());
    assert!(Hex::validate_str("").is_err());
    assert!(Hex::validate_str("ABC").is_err());
}

#[cfg(feature = "serde_json")]
#[test]
fn serde_roundtrip() {
    use validators::{Result, errors::HexDecodedError, serde_json};

    let decoded: UpperHex = serde_json::from_str(r#""00ABFF""#).unwrap();
    let json = serde_json::to_string(&decoded).unwrap();
    let roundtrip: UpperHex = serde_json::from_str(&json).unwrap();
    let wrapped: Result<UpperHex, HexDecodedError> = serde_json::from_str(&json).unwrap();

    assert_eq!(r#""00ABFF""#, json);
    assert_eq!(&[0, 171, 255], roundtrip.0.as_slice());
    assert_eq!(&[0, 171, 255], wrapped.into_std_result().unwrap().0.as_slice());
    assert!(serde_json::from_str::<UpperHex>(r#""00abff""#).is_err());

    assert_eq!(
        r#""00abff""#,
        serde_json::to_string(&LowerHex::parse_str("00abff").unwrap()).unwrap()
    );
    assert_eq!(r#""00ABFF""#, serde_json::to_string(&Hex::parse_str("00aBff").unwrap()).unwrap());
    assert_eq!(
        &[0, 171, 255],
        serde_json::from_str::<UpperHex>("[0,171,255]").unwrap().0.as_slice()
    );
}
