#![cfg(feature = "base32_decoded")]

#[macro_use]
extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[test]
fn basic() {
    macro_rules! test {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[derive(Validator)]
                    #[validator(base32_decoded($($p($v),)*))]
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
                    test("GEZDGNBVGY3TQOI=", Validator::V_PADDING.allow());
                    test("GEZDGNBVGY3TQOI", !Validator::V_PADDING.must());
                    test("GEZDGNBV=GY3TQOI", false);
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
            padding => NotAllow,
        },
    }
}

#[test]
fn decode() {
    #[derive(Validator)]
    #[validator(base32_decoded)]
    struct Validator(Vec<u8>);

    let base32_decoded = Validator::parse_str("GEZDGNBVGY3TQOI=").unwrap();

    assert_eq!(b"123456789", base32_decoded.0.as_slice());
}
