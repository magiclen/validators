#![cfg(all(feature = "test", feature = "derive", feature = "base64"))]

use validators::prelude::*;

#[test]
fn rejects_invalid_utf8_with_shadowed_err() {
    use validators::errors::Base64Error;

    #[allow(dead_code, non_snake_case)]
    fn Err(_: Base64Error) -> Result<(), Base64Error> {
        Ok(())
    }

    #[derive(Validator)]
    #[validator(base64)]
    struct Base64(String);

    let bytes = [0xFF, b'A', b'A', b'A'];

    assert!(Base64::parse_vec_u8(bytes).is_err());
    assert!(Base64::parse_u8_slice(bytes).is_err());
    assert_eq!("AAAA", Base64::parse_vec_u8(b"AAAA".to_vec()).unwrap().0);
}

#[cfg(feature = "serde")]
#[test]
fn deserialize_binary_string() {
    use validators::{
        Result,
        errors::Base64Error,
        serde::{self, Deserialize, de},
    };

    // Like bincode, this deserializer does not support `deserialize_any`.
    struct BinaryString<'a>(&'a str);

    impl<'de> de::Deserializer<'de> for BinaryString<'de> {
        type Error = de::value::Error;

        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char string bytes byte_buf option
            unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
        }

        fn deserialize_any<V: de::Visitor<'de>>(
            self,
            _visitor: V,
        ) -> core::result::Result<V::Value, Self::Error> {
            Err(de::Error::custom("deserialize_any is not supported"))
        }

        fn deserialize_str<V: de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> core::result::Result<V::Value, Self::Error> {
            visitor.visit_borrowed_str(self.0)
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }

    #[derive(Validator)]
    #[validator(base64)]
    struct Base64(String);

    let input = "QQ==";
    let base64 = Base64::deserialize(BinaryString(input)).unwrap();
    let wrapped = Result::<Base64, Base64Error>::deserialize(BinaryString(input)).unwrap();

    assert_eq!(input, base64.0);
    assert_eq!(input, wrapped.into_std_result().unwrap().0);
}

#[test]
fn basic() {
    macro_rules! test {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(base64($($p($v),)*))]
                    pub struct Validator(pub String);

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
                    }

                    test("", false);
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
