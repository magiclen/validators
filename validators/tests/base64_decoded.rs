#![cfg(all(feature = "test", feature = "derive", feature = "base64_decoded"))]

use validators::prelude::*;

#[test]
fn basic() {
    macro_rules! test {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(base64_decoded($($p($v),)*))]
                    pub struct Validator(pub Vec<u8>);

                    fn test(s: &str, is_ok: bool) {
                        let panic = match Validator::validate_str(s) {
                            Ok(_) => !is_ok,
                            Err(_) if !is_ok => false,
                            Err(err) => {
                                eprintln!("{}", err);

                                true
                            }
                        };

                        if panic {
                            panic!("{:?}: {} expect {}", s, stringify! {
                                $(
                                    $p = $v,
                                )*
                            }, is_ok);
                        }

                        let panic = match Validator::parse_str(s) {
                            Ok(_) => !is_ok,
                            Err(_) if !is_ok => false,
                            Err(err) => {
                                eprintln!("{}", err);

                                true
                            }
                        };

                        if panic {
                            panic!("{:?}: {} expect {}", s, stringify! {
                                $(
                                    $p = $v,
                                )*
                            }, is_ok);
                        }
                    }

                    test("", false);
                    test("Zm9v", true);
                    test("Qg==aXQ=", false);
                    test("A", false);
                    test("Zh==", false);
                    test("MTIzNDU2Nzg5MA==", Validator::V_PADDING.allow());
                    test("MTIzNDU2Nzg5MA", !Validator::V_PADDING.must());
                    test("MTIzND=U2Nzg5MA", false);
                }
            )*
        }
    }

    test! {
        {
            padding => Allow,
        },
        {
            padding => Must,
        },
        {
            padding => Disallow,
        },
    }
}

#[test]
fn decode() {
    #[derive(Validator)]
    #[validator(base64_decoded)]
    struct Validator(Vec<u8>);

    let base64_decoded = Validator::parse_str("MTIzNDU2Nzg5MA==").unwrap();

    assert_eq!(b"1234567890", base64_decoded.0.as_slice());

    #[cfg(feature = "serde_json")]
    {
        use validators::{Result, errors::Base64DecodedError, serde_json};

        let json = serde_json::to_string(&base64_decoded).unwrap();
        let decoded: Validator = serde_json::from_str(&json).unwrap();
        let wrapped: Result<Validator, Base64DecodedError> = serde_json::from_str(&json).unwrap();

        assert_eq!(b"1234567890", decoded.0.as_slice());
        assert_eq!(b"1234567890", wrapped.into_std_result().unwrap().0.as_slice());

        let empty: Result<Validator, Base64DecodedError> = serde_json::from_str("[]").unwrap();

        assert!(matches!(empty.into_std_result(), Err(Base64DecodedError::Invalid)));
    }
}

#[cfg(feature = "serde")]
#[test]
fn deserialize_binary_bytes() {
    use validators::{
        Result,
        errors::Base64DecodedError,
        serde::{self, Deserialize, de},
    };

    struct BinaryBytes<'a>(&'a [u8]);

    impl<'de> de::Deserializer<'de> for BinaryBytes<'de> {
        type Error = de::value::Error;

        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes byte_buf option unit
            unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
        }

        fn deserialize_any<V: de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> core::result::Result<V::Value, Self::Error> {
            visitor.visit_borrowed_bytes(self.0)
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }

    #[derive(Validator)]
    #[validator(base64_decoded)]
    struct Validator(Vec<u8>);

    let bytes = b"\x00\xFF";
    let decoded = Validator::deserialize(BinaryBytes(bytes)).unwrap();
    let wrapped = Result::<Validator, Base64DecodedError>::deserialize(BinaryBytes(bytes)).unwrap();

    assert_eq!(bytes, decoded.0.as_slice());
    assert_eq!(bytes, wrapped.into_std_result().unwrap().0.as_slice());
}
