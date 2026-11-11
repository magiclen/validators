#![cfg(all(feature = "test", feature = "derive", feature = "http_url"))]

use validators::{prelude::*, url};

#[test]
fn basic() {
    macro_rules! test {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(http_url($($p($v),)*))]
                    pub struct Validator {
                        pub url: url::Url,
                        pub is_https: bool,
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
                    test("ftp://example.org/", false);
                    test("https://example.org/", !Validator::V_LOCAL.must());
                    test(" https://example.org/", !Validator::V_LOCAL.must());
                    test("ht\ttps://example.org/", !Validator::V_LOCAL.must());
                    test("http://localhost:3000/", Validator::V_LOCAL.allow());
                    test("ht\ntp://localhost:3000/", Validator::V_LOCAL.allow());
                    test("http://api.LOCALHOST.:3000/", Validator::V_LOCAL.allow());
                    test("http://127.0.0.1:3000/", Validator::V_LOCAL.allow());
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

#[test]
fn normalized_protocol() {
    use validators::errors::HttpURLError;

    #[derive(Validator)]
    #[validator(http_url)]
    struct Address {
        url:      url::Url,
        is_https: bool,
    }

    for (input, expected, is_https) in [
        ("ht\ttps://example.org/", "https://example.org/", true),
        ("h\nttp://example.org/", "http://example.org/", false),
        ("h\rTtPs://example.org/", "https://example.org/", true),
        (" \0HTTPS://example.org/", "https://example.org/", true),
    ] {
        let address = Address::parse_str(input).unwrap();

        assert_eq!(expected, address.url.as_str());
        assert_eq!(is_https, address.is_https);
    }

    assert!(matches!(Address::parse_str("f\ttp://example.org/"), Err(HttpURLError::ProtocolError)));
    assert!(matches!(Address::parse_str("ht\ttps://"), Err(HttpURLError::ProtocolError)));
    assert!(matches!(Address::parse_str("https://"), Err(HttpURLError::ParseError(_))));
}
