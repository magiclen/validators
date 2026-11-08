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
        use validators::serde_json;

        let json = serde_json::to_string(&base64_decoded).unwrap();
        let decoded: Validator = serde_json::from_str(&json).unwrap();

        assert_eq!(b"1234567890", decoded.0.as_slice());
    }
}
