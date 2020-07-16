#![cfg(feature = "mac_address")]

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
            separator => Allow(colon),
        },
        {
            case => Upper,
            separator => Allow(colon),
        },
        {
            case => Lower,
            separator => Allow(colon),
        },
        {
            case => Any,
            separator => Must(colon),
        },
        {
            case => Upper,
            separator => Must(colon),
        },
        {
            case => Lower,
            separator => Must(colon),
        },
        {
            case => Any,
            separator => Allow(hyphen),
        },
        {
            case => Upper,
            separator => Allow(hyphen),
        },
        {
            case => Lower,
            separator => Allow(hyphen),
        },
        {
            case => Any,
            separator => Must(hyphen),
        },
        {
            case => Upper,
            separator => Must(hyphen),
        },
        {
            case => Lower,
            separator => Must(hyphen),
        },
        {
            case => Any,
            separator => NotAllow,
        },
        {
            case => Upper,
            separator => NotAllow,
        },
        {
            case => Lower,
            separator => NotAllow,
        },
    }
}
