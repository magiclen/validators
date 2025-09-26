#![cfg(all(feature = "test", feature = "derive", feature = "http_ftp_url"))]

use validators::{prelude::*, url};

#[test]
fn basic() {
    macro_rules! test {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[allow(dead_code)] // ignore spurious dead code
                    #[derive(Validator)]
                    #[validator(http_ftp_url($($p($v),)*))]
                    pub struct Validator {
                        pub url: url::Url,
                        pub protocol: validators::models::Protocol,
                    }

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
                    test("example:", false);
                    test("https://example.org/", !Validator::V_LOCAL.must());
                    test("http://localhost:3000/", Validator::V_LOCAL.allow());
                    test("http://127.0.0.1:3000/", Validator::V_LOCAL.allow());
                    test("ftp://example.org/", !Validator::V_LOCAL.must());
                }
            )*
        }
    }

    test! {
        {
            local => Allow,
        },
        {
            local => Must,
        },
        {
            local => Disallow,
        },
    }
}
