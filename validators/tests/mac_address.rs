#![cfg(all(feature = "test", feature = "derive", feature = "mac_address"))]

use validators::prelude::*;

#[test]
fn compact_with_hex_separator() {
    #[derive(Validator)]
    #[validator(mac_address(separator(Allow(b'0'))))]
    struct MacAddress(u64);

    let input = "000000000000";

    assert!(MacAddress::validate_str(input).is_ok());
    assert_eq!(0, MacAddress::parse_str(input).unwrap().0);
}

#[test]
fn basic() {
    macro_rules! test {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[derive(Validator)]
                    #[validator(mac_address($($p($v),)*))]
                    pub struct Validator(pub u64);

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
                            Ok(value) if is_ok => {
                                let formatted = value.to_mac_address_string();

                                assert_eq!(value.0, Validator::parse_str(formatted).unwrap().0);

                                #[cfg(feature = "serde_json")]
                                {
                                    let json = validators::serde_json::to_string(&value).unwrap();

                                    assert_eq!(value.0, validators::serde_json::from_str::<Validator>(&json).unwrap().0);
                                }

                                false
                            },
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
                    test("00:00:00:00:", false);
                    test("08:00:27:b2:46:c3", Validator::V_CASE.lower() && Validator::V_SEPARATOR.allow() == Some(b':'));
                    test("08:00:27:B2:46:C3", Validator::V_CASE.upper() && Validator::V_SEPARATOR.allow() == Some(b':'));
                    test("08:00:27:b2:46:C3", Validator::V_CASE.any() && Validator::V_SEPARATOR.allow() == Some(b':'));
                    test("08-00-27-b2-46-c3", Validator::V_CASE.lower() && Validator::V_SEPARATOR.allow() == Some(b'-'));
                    test("08-00-27-B2-46-C3", Validator::V_CASE.upper() && Validator::V_SEPARATOR.allow() == Some(b'-'));
                    test("08-00-27-b2-46-C3", Validator::V_CASE.any() && Validator::V_SEPARATOR.allow() == Some(b'-'));
                    test("080027b246c3", Validator::V_CASE.lower() && !Validator::V_SEPARATOR.must().is_some());
                    test("080027B246C3", Validator::V_CASE.upper() && !Validator::V_SEPARATOR.must().is_some());
                    test("080027b246C3", Validator::V_CASE.any() && !Validator::V_SEPARATOR.must().is_some());
                }
            )*
        }
    }

    test! {
        {
            case => Any,
            separator => Allow(b':'),
        },
        {
            case => Upper,
            separator => Allow(b':'),
        },
        {
            case => Lower,
            separator => Allow(b':'),
        },
        {
            case => Any,
            separator => Must(b':'),
        },
        {
            case => Upper,
            separator => Must(b':'),
        },
        {
            case => Lower,
            separator => Must(b':'),
        },
        {
            case => Any,
            separator => Allow(b'-'),
        },
        {
            case => Upper,
            separator => Allow(b'-'),
        },
        {
            case => Lower,
            separator => Allow(b'-'),
        },
        {
            case => Any,
            separator => Must(b'-'),
        },
        {
            case => Upper,
            separator => Must(b'-'),
        },
        {
            case => Lower,
            separator => Must(b'-'),
        },
        {
            case => Any,
            separator => Disallow,
        },
        {
            case => Upper,
            separator => Disallow,
        },
        {
            case => Lower,
            separator => Disallow,
        },
    }
}
