#![cfg(all(feature = "test", feature = "derive", feature = "ip"))]

use validators::prelude::*;

#[test]
fn basic() {
    macro_rules! test_case {
        ($test:ident, $validator:ident) => {
            type Validator = $validator;
            let test = $test;

            test("", false);
            test("127.0.0.1", Validator::V_LOCAL.allow() && !Validator::V_PORT.must());
            test("127.0.0.1:8080", Validator::V_LOCAL.allow() && Validator::V_PORT.allow());
            test("127.0.0.1:+8080", false);
            test("[::1]:+8080", false);
            test("168.17.212.1", !Validator::V_LOCAL.must() && !Validator::V_PORT.must());
            test("168.17.212.1:8080", !Validator::V_LOCAL.must() && Validator::V_PORT.allow());
            test(
                "0000:0000:0000:0000:0000:0000:370:7348",
                !Validator::V_LOCAL.must() && !Validator::V_PORT.must(),
            );
            test(
                "[0000:0000:0000:0000:0000:0000:370:7348]",
                !Validator::V_LOCAL.must() && !Validator::V_PORT.must(),
            );
            test(
                "[0000:0000:0000:0000:0000:0000:370:7348]:8080",
                !Validator::V_LOCAL.must() && Validator::V_PORT.allow(),
            );
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
                    #[validator(ip($($p($v),)*))]
                    pub struct IPAllowPort {
                        pub ip: std::net::IpAddr,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(ip($($p($v),)*port(Must)))]
                    pub struct IPWithPort {
                        pub ip: std::net::IpAddr,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(ip($($p($v),)*port(Disallow)))]
                    pub struct IPWithoutPort(pub std::net::IpAddr);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        IPAllowPort,
                        IPWithPort,
                        IPWithoutPort,
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
fn bracket_errors() {
    use std::net::IpAddr;

    use validators::errors::IpError;

    #[derive(Validator)]
    #[validator(ip(port(Disallow)))]
    struct IpWithoutPort(IpAddr);

    assert!(matches!(IpWithoutPort::validate_str("[::1]:80"), Err(IpError::PortDisallow)));

    for input in ["[::1", "[", "[::1]x"] {
        assert!(matches!(IpWithoutPort::validate_str(input), Err(IpError::Invalid)), "{input}");
    }
}

#[test]
fn port_disallow_errors() {
    use std::net::IpAddr;

    use validators::errors::IpError;

    #[derive(Validator)]
    #[validator(ip(port(Disallow)))]
    struct IpWithoutPort(IpAddr);

    assert!(matches!(IpWithoutPort::validate_str("127.0.0.1:80"), Err(IpError::PortDisallow)));

    // `host:port` has only one colon.
    for input in ["1::2::3", "fe80::1%eth0", "127.0.0.1:80:90"] {
        assert!(matches!(IpWithoutPort::validate_str(input), Err(IpError::Invalid)), "{input}");
    }
}

#[test]
fn uri_authority_string() {
    use std::net::IpAddr;

    #[derive(Validator)]
    #[validator(ip(port(Allow)))]
    struct IpAllowPort {
        ip:   IpAddr,
        port: Option<u16>,
    }

    for (input, expected) in [
        ("127.0.0.1", "127.0.0.1"),
        ("127.0.0.1:8080", "127.0.0.1:8080"),
        ("::1", "[::1]"),
        ("[::1]:8080", "[::1]:8080"),
    ] {
        let ip = IpAllowPort::parse_str(input).unwrap();

        assert_eq!(expected, ip.to_uri_authority_string(), "{input}");

        #[cfg(feature = "serde_json")]
        {
            use validators::serde_json;

            let json = serde_json::to_string(&ip).unwrap();
            let decoded: IpAllowPort = serde_json::from_str(&json).unwrap();

            assert_eq!(serde_json::to_string(expected).unwrap(), json);
            assert_eq!(ip.ip, decoded.ip);
            assert_eq!(ip.port, decoded.port);
        }
    }
}
