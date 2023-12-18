#![cfg(all(feature = "test", feature = "derive", feature = "base64_url_decoded"))]

use validators::prelude::*;

#[test]
fn basic() {
    macro_rules! test {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[derive(Validator)]
                    #[validator(base64_url_decoded($($p($v),)*))]
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
                    test("MTIzNDU2Nz-5MA==", Validator::V_PADDING.allow());
                    test("MTIzNDU2Nz-5MA", !Validator::V_PADDING.must());
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
    #[validator(base64_url_decoded)]
    struct Validator(Vec<u8>);

    let base64_url_decoded = Validator::parse_str("MTIzNDU2Nzg5MA==").unwrap();

    assert_eq!(b"1234567890", base64_url_decoded.0.as_slice());
}
