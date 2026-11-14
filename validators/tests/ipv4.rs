#![cfg(all(feature = "test", feature = "derive", feature = "ipv4"))]

use validators::prelude::*;

#[test]
fn builtin_type_names() {
    macro_rules! check {
        ($($name:ident),* $(,)?) => {
            $(
                {
                    #[derive(Validator)]
                    #[validator(ipv4(port(Disallow)))]
                    struct $name(core::net::Ipv4Addr);

                    let value = $name::parse_str("127.0.0.1").unwrap();

                    assert_eq!(core::net::Ipv4Addr::LOCALHOST, value.0);
                    assert!(<$name as ValidateString>::validate_str("127.0.0.1").is_ok());
                }
            )*
        };
    }

    check!(Option, Some, None, Ok, Err);
}

#[test]
fn basic() {
    macro_rules! test_case {
        ($test:ident, $validator:ident) => {
            type Validator = $validator;
            let test = $test;

            test("", false);
            test("0000:0000:0000:0000:0000:0000:370:7348", false);
            test("[0000:0000:0000:0000:0000:0000:370:7348]", false);
            test("[0000:0000:0000:0000:0000:0000:370:7348]:8080", false);
            test("127.0.0.1", Validator::V_LOCAL.allow() && !Validator::V_PORT.must());
            test("127.0.0.1:8080", Validator::V_LOCAL.allow() && Validator::V_PORT.allow());
            for input in ["224.0.0.1", "239.255.255.250"] {
                test(input, Validator::V_LOCAL.allow() && !Validator::V_PORT.must());
            }
            test("168.17.212.1", !Validator::V_LOCAL.must() && !Validator::V_PORT.must());
            test("168.17.212.1:8080", !Validator::V_LOCAL.must() && Validator::V_PORT.allow());
        };
    }

    macro_rules! test_inner {
        ( @unit $parameters:expr, $validator:ident) => {
            {
                fn test(s: &str, is_ok: bool) {
                    let panic = match $validator::validate_str(s) {
                        Ok(_) => !is_ok,
                        Err(_) if !is_ok => false,
                        Err(err) => {
                            eprintln!("{}", err);

                            true
                        }
                    };

                    if panic {
                        panic!("{:?}: {} {} expect {}", s, stringify!($validator), $parameters, is_ok);
                    }
                }

                test_case!(test, $validator);
            }
        };
        ( $parameters:expr ; $($validator:ident),* $(,)*) => {
            $(
                test_inner!(
                    @unit
                    $parameters,
                    $validator
                );
            )*
        };
    }

    macro_rules! test {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[derive(Validator)]
                    #[validator(ipv4($($p($v),)*))]
                    pub struct IPv4AllowPort {
                        pub ipv4: std::net::Ipv4Addr,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(ipv4($($p($v),)*port(Must)))]
                    pub struct IPv4WithPort {
                        pub ipv4: std::net::Ipv4Addr,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(ipv4($($p($v),)*port(Disallow)))]
                    pub struct IPv4WithoutPort(pub std::net::Ipv4Addr);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        IPv4AllowPort,
                        IPv4WithPort,
                        IPv4WithoutPort,
                    );
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
        }
    }
}

#[test]
fn local_ranges() {
    use std::net::Ipv4Addr;

    use validators::functions::is_local_ipv4;

    // Each range has an address inside it, and most also have an address just outside it.
    for (expected, ip) in [
        (true, Ipv4Addr::new(0, 1, 2, 3)),
        (false, Ipv4Addr::new(1, 0, 0, 0)),
        (true, Ipv4Addr::new(100, 64, 0, 0)),
        (false, Ipv4Addr::new(100, 128, 0, 0)),
        (true, Ipv4Addr::new(192, 0, 0, 8)),
        (false, Ipv4Addr::new(192, 0, 0, 9)),
        (false, Ipv4Addr::new(192, 88, 99, 1)),
        (true, Ipv4Addr::new(192, 88, 99, 2)),
        (true, Ipv4Addr::new(198, 19, 255, 255)),
        (false, Ipv4Addr::new(198, 20, 0, 0)),
        (false, Ipv4Addr::new(223, 255, 255, 255)),
        (true, Ipv4Addr::new(224, 0, 0, 0)),
        (true, Ipv4Addr::new(224, 0, 0, 255)),
        (false, Ipv4Addr::new(224, 0, 1, 0)),
        (false, Ipv4Addr::new(238, 255, 255, 255)),
        (true, Ipv4Addr::new(239, 0, 0, 0)),
        (true, Ipv4Addr::new(239, 255, 255, 255)),
        (true, Ipv4Addr::new(240, 0, 0, 1)),
        (true, Ipv4Addr::new(255, 255, 255, 255)),
        (false, Ipv4Addr::new(8, 8, 8, 8)),
    ] {
        assert_eq!(expected, is_local_ipv4(ip), "{ip}");
    }
}

#[test]
fn port_disallow_errors() {
    use std::net::Ipv4Addr;

    use validators::errors::Ipv4Error;

    #[derive(Validator)]
    #[validator(ipv4(port(Disallow)))]
    struct Ipv4WithoutPort(Ipv4Addr);

    assert!(matches!(Ipv4WithoutPort::validate_str("127.0.0.1:80"), Err(Ipv4Error::PortDisallow)));

    // `host:port` has only one colon.
    for input in ["::1", "127.0.0.1:80:90"] {
        assert!(matches!(Ipv4WithoutPort::validate_str(input), Err(Ipv4Error::Invalid)), "{input}");
    }
}

#[test]
fn uri_authority_string() {
    use std::net::Ipv4Addr;

    #[derive(Validator)]
    #[validator(ipv4(port(Allow)))]
    struct Ipv4AllowPort {
        ipv4: Ipv4Addr,
        port: Option<u16>,
    }

    for (input, expected) in [("127.0.0.1", "127.0.0.1"), ("127.0.0.1:8080", "127.0.0.1:8080")] {
        let ipv4 = Ipv4AllowPort::parse_str(input).unwrap();

        assert_eq!(expected, ipv4.to_uri_authority_string(), "{input}");

        #[cfg(feature = "serde_json")]
        {
            use validators::serde_json;

            let json = serde_json::to_string(&ipv4).unwrap();
            let decoded: Ipv4AllowPort = serde_json::from_str(&json).unwrap();

            assert_eq!(serde_json::to_string(expected).unwrap(), json);
            assert_eq!(ipv4.ipv4, decoded.ipv4);
            assert_eq!(ipv4.port, decoded.port);
        }
    }
}
