#![cfg(all(feature = "test", feature = "derive", feature = "base32"))]

use validators::prelude::*;

#[test]
fn basic() {
    macro_rules! test {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[derive(Validator)]
                    #[validator(base32($($p($v),)*))]
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
            padding => Disallow,
        },
    }
}
