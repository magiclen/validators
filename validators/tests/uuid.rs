#![cfg(all(feature = "uuid", feature = "derive"))]

use validators::prelude::*;

#[test]
fn basic() {
    macro_rules! test {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[derive(Validator)]
                    #[validator(uuid($($p($v),)*))]
                    pub struct Validator(pub u128);

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
                    test("a866664a-f9d3-4dde-89cb-182015fa4f41", Validator::V_CASE.lower() && Validator::V_SEPARATOR.allow() == Some(b'-'));
                    test("A866664A-F9D3-4DDE-89CB-182015FA4F41", Validator::V_CASE.upper() && Validator::V_SEPARATOR.allow() == Some(b'-'));
                    test("A866664A-f9D3-4dde-89CB-182015FA4F41", Validator::V_CASE.any() && Validator::V_SEPARATOR.allow() == Some(b'-'));
                    test("a866664a f9d3 4dde 89cb 182015fa4f41", false);
                    test("a866664af9d34dde89cb182015fa4f41", Validator::V_CASE.lower() && !Validator::V_SEPARATOR.must().is_some());
                    test("A866664AF9D34DDE89CB182015FA4F41", Validator::V_CASE.upper() && !Validator::V_SEPARATOR.must().is_some());
                    test("A866664AF9D34dde89CB182015FA4F41", Validator::V_CASE.any() && !Validator::V_SEPARATOR.must().is_some());
                }
            )*
        }
    }

    test! {
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
