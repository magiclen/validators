#![cfg(feature = "line")]

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
                    #[validator(line($($p($v),)*))]
                    struct Validator(String);

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

                    test("", Validator::V_EMPTY.allow());
                    test("   ", Validator::V_EMPTY.allow());
                    test("　　　", Validator::V_EMPTY.allow());
                    test("123", !Validator::V_EMPTY.must());
                }
            )*
        }
    }

    test! {
        {
            empty => Allow,
        },
        {
            empty => Must,
        },
        {
            empty => NotAllow,
        },
    }
}
