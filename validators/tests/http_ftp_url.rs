#![cfg(all(feature = "test", feature = "derive", feature = "http_ftp_url"))]

use validators::{prelude::*, url};

#[test]
fn basic() {
    macro_rules! test {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[allow(dead_code)]
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
                    test("ht\ttps://example.org/", !Validator::V_LOCAL.must());
                    test("http://localhost:3000/", Validator::V_LOCAL.allow());
                    test("ht\ntp://localhost:3000/", Validator::V_LOCAL.allow());
                    test("http://127.0.0.1:3000/", Validator::V_LOCAL.allow());
                    test("ftp://example.org/", !Validator::V_LOCAL.must());
                    test(" ftp://example.org/", !Validator::V_LOCAL.must());
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
    use validators::{errors::HttpFtpURLError, models::Protocol};

    #[derive(Validator)]
    #[validator(http_ftp_url)]
    struct Address {
        url:      url::Url,
        protocol: Protocol,
    }

    for (input, expected, protocol) in [
        ("ht\ttps://example.org/", "https://example.org/", Protocol::HTTPS),
        ("h\nttp://example.org/", "http://example.org/", Protocol::HTTP),
        ("f\rtP://example.org/", "ftp://example.org/", Protocol::FTP),
        (" \0FTP://example.org/", "ftp://example.org/", Protocol::FTP),
    ] {
        let address = Address::parse_str(input).unwrap();

        assert_eq!(expected, address.url.as_str());
        assert_eq!(protocol, address.protocol);
    }

    assert!(matches!(
        Address::parse_str("w\ts://example.org/"),
        Err(HttpFtpURLError::ProtocolError)
    ));
    assert!(matches!(Address::parse_str("ft\tp://"), Err(HttpFtpURLError::ParseError(_))));
    assert!(matches!(Address::parse_str("f\ttp://a b"), Err(HttpFtpURLError::ParseError(_))));
    assert!(matches!(Address::parse_str("ftp://"), Err(HttpFtpURLError::ParseError(_))));
}
