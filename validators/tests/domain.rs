#![cfg(all(feature = "test", feature = "derive", feature = "domain"))]

use validators::prelude::*;

#[test]
fn ipv4_with_port() {
    #[derive(Validator)]
    #[validator(domain(ipv4(Must), port(Allow)))]
    struct Domain {
        domain: String,
        port:   Option<u16>,
    }

    for input in ["127.0.0.1.:8080", "127。0。0。1。:8080"] {
        let domain = Domain::parse_string(String::from(input)).unwrap();

        assert_eq!("127.0.0.1.", domain.domain);
        assert_eq!(Some(8080), domain.port);
        assert_eq!("127.0.0.1:8080", domain.to_uri_authority_string());

        #[cfg(feature = "serde_json")]
        {
            use validators::serde_json;

            let json = serde_json::to_string(&domain).unwrap();
            let decoded: Domain = serde_json::from_str(&json).unwrap();

            assert_eq!(r#""127.0.0.1.:8080""#, json);
            assert!(decoded.is_fully_qualified());
            assert_eq!(Some(8080), decoded.port);
        }
    }
}

#[cfg(feature = "serde")]
#[test]
fn deserialize_binary_string() {
    use validators::{
        Result,
        errors::DomainError,
        serde::{self, Deserialize, de},
    };

    struct BinaryString<'a>(&'a str);

    impl<'de> de::Deserializer<'de> for BinaryString<'de> {
        type Error = de::value::Error;

        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char string bytes byte_buf option
            unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
        }

        fn deserialize_any<V: de::Visitor<'de>>(
            self,
            _visitor: V,
        ) -> core::result::Result<V::Value, Self::Error> {
            Err(de::Error::custom("deserialize_any is not supported"))
        }

        fn deserialize_str<V: de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> core::result::Result<V::Value, Self::Error> {
            visitor.visit_borrowed_str(self.0)
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }

    #[derive(Validator)]
    #[validator(domain)]
    struct Domain {
        domain: String,
        port:   Option<u16>,
    }

    let input = "example.com.:8080";
    let domain = Domain::deserialize(BinaryString(input)).unwrap();
    let wrapped = Result::<Domain, DomainError>::deserialize(BinaryString(input))
        .unwrap()
        .into_std_result()
        .unwrap();

    assert_eq!("example.com.", domain.domain);
    assert_eq!(Some(8080), domain.port);
    assert_eq!(domain.domain, wrapped.domain);
    assert_eq!(domain.port, wrapped.port);
}

#[test]
fn basic() {
    macro_rules! test_case {
        ($test:ident, $validator:ident) => {
            type Validator = $validator;
            let test = $test;

            test("", false);
            for input in ["127.0.0.1", "127。0。0。1"] {
                test(
                    input,
                    Validator::V_IPV4.allow()
                        && Validator::V_LOCAL.allow()
                        && !Validator::V_PORT.must()
                        && Validator::V_AT_LEAST_TWO_LABELS.allow(),
                );
            }
            test(
                "127.0.0.1.",
                Validator::V_IPV4.allow()
                    && Validator::V_LOCAL.allow()
                    && !Validator::V_PORT.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "127.0.0.1:8080",
                Validator::V_IPV4.allow()
                    && Validator::V_LOCAL.allow()
                    && Validator::V_PORT.allow()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "127.0.0.1.:8080",
                Validator::V_IPV4.allow()
                    && Validator::V_LOCAL.allow()
                    && Validator::V_PORT.allow()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "168.17.212.1",
                Validator::V_IPV4.allow()
                    && !Validator::V_LOCAL.must()
                    && !Validator::V_PORT.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "168.17.212.1.",
                Validator::V_IPV4.allow()
                    && !Validator::V_LOCAL.must()
                    && !Validator::V_PORT.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "168.17.212.1:8080",
                Validator::V_IPV4.allow()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_PORT.allow()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "168.17.212.1.:8080",
                Validator::V_IPV4.allow()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_PORT.allow()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "localhost",
                !Validator::V_IPV4.must()
                    && Validator::V_LOCAL.allow()
                    && !Validator::V_PORT.must(),
            );
            test(
                "localhost.",
                !Validator::V_IPV4.must()
                    && Validator::V_LOCAL.allow()
                    && !Validator::V_PORT.must(),
            );
            test(
                "localhost:8080",
                !Validator::V_IPV4.must()
                    && Validator::V_LOCAL.allow()
                    && Validator::V_PORT.allow(),
            );
            test(
                "localhost.:8080",
                !Validator::V_IPV4.must()
                    && Validator::V_LOCAL.allow()
                    && Validator::V_PORT.allow(),
            );
            test(
                "foo.localhost",
                !Validator::V_IPV4.must()
                    && Validator::V_LOCAL.allow()
                    && !Validator::V_PORT.must()
                    && !Validator::V_AT_LEAST_TWO_LABELS.disallow(),
            );
            test(
                "myhost",
                !Validator::V_IPV4.must()
                    && !Validator::V_LOCAL.must()
                    && !Validator::V_PORT.must()
                    && !Validator::V_AT_LEAST_TWO_LABELS.must(),
            );
            test(
                "myhost.",
                !Validator::V_IPV4.must()
                    && !Validator::V_LOCAL.must()
                    && !Validator::V_PORT.must()
                    && !Validator::V_AT_LEAST_TWO_LABELS.must(),
            );
            test(
                "myhost:8080",
                !Validator::V_IPV4.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_PORT.allow()
                    && !Validator::V_AT_LEAST_TWO_LABELS.must(),
            );
            test(
                "myhost.:8080",
                !Validator::V_IPV4.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_PORT.allow()
                    && !Validator::V_AT_LEAST_TWO_LABELS.must(),
            );
            test(
                "臺灣.tw",
                !Validator::V_IPV4.must()
                    && !Validator::V_LOCAL.must()
                    && !Validator::V_PORT.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "臺灣.tw.",
                !Validator::V_IPV4.must()
                    && !Validator::V_LOCAL.must()
                    && !Validator::V_PORT.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "臺灣.tw:8080",
                !Validator::V_IPV4.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_PORT.allow()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "臺灣.tw.:8080",
                !Validator::V_IPV4.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_PORT.allow()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
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
                    #[validator(domain($($p($v),)*conflict(Allow)))]
                    pub struct DomainAllowPort {
                        pub domain: String,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*port(Must), conflict(Allow)))]
                    pub struct DomainWithPort {
                        pub domain: String,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*port(Disallow), conflict(Allow)))]
                    struct DomainWithoutPort(pub String);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        DomainAllowPort,
                        DomainWithPort,
                        DomainWithoutPort,
                    );
                }
            )*
        }
    }

    macro_rules! test2 {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*))]
                    pub struct DomainAllowIPv4AllowPortIsLocal {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub port: Option<u16>,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*port(Must)))]
                    pub struct DomainAllowIPv4WithPortIsLocal {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub port: u16,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*port(Disallow)))]
                    #[allow(dead_code)]
                    struct DomainAllowIPv4WithoutPortIsLocal {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Must), conflict(Allow)))]
                    pub struct DomainIPv4AllowPortIsLocal {
                        pub domain: String,
                        pub port: Option<u16>,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Must), port(Must), conflict(Allow)))]
                    pub struct DomainIPv4WithPortIsLocal {
                        pub domain: String,
                        pub port: u16,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Must), port(Disallow), conflict(Allow)))]
                    #[allow(dead_code)]
                    struct DomainIPv4WithoutPortIsLocal {
                        pub domain: String,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Disallow)))]
                    pub struct DomainNonIPv4AllowPortIsLocal {
                        pub domain: String,
                        pub port: Option<u16>,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Disallow), port(Must)))]
                    pub struct DomainNonIPv4WithPortIsLocal {
                        pub domain: String,
                        pub port: u16,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Disallow), port(Disallow)))]
                    #[allow(dead_code)]
                    struct DomainNonIPv4WithoutPortIsLocal {
                        pub domain: String,
                        pub is_local: bool,
                    }

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        DomainAllowIPv4AllowPortIsLocal,
                        DomainAllowIPv4WithPortIsLocal,
                        DomainAllowIPv4WithoutPortIsLocal,
                        DomainIPv4AllowPortIsLocal,
                        DomainIPv4WithPortIsLocal,
                        DomainIPv4WithoutPortIsLocal,
                        DomainNonIPv4AllowPortIsLocal,
                        DomainNonIPv4WithPortIsLocal,
                        DomainNonIPv4WithoutPortIsLocal,
                    );
                }
            )*
        }
    }

    test! {
        {
            ipv4 => Allow,
            local => Allow,
            at_least_two_labels => Allow,
        },
        {
            ipv4 => Allow,
            local => Must,
            at_least_two_labels => Allow,
        },
        {
            ipv4 => Allow,
            local => Must,
            at_least_two_labels => Must,
        },
        {
            ipv4 => Allow,
            local => Must,
            at_least_two_labels => Disallow,
        },
        {
            ipv4 => Allow,
            local => Disallow,
            at_least_two_labels => Allow,
        },
        {
            ipv4 => Allow,
            local => Disallow,
            at_least_two_labels => Must,
        },
        {
            ipv4 => Allow,
            local => Disallow,
            at_least_two_labels => Disallow,
        },
        {
            ipv4 => Must,
            local => Must,
            at_least_two_labels => Allow,
        },
        {
            ipv4 => Must,
            local => Must,
            at_least_two_labels => Must,
        },
        {
            ipv4 => Must,
            local => Must,
            at_least_two_labels => Disallow,
        },
        {
            ipv4 => Must,
            local => Disallow,
            at_least_two_labels => Allow,
        },
        {
            ipv4 => Must,
            local => Disallow,
            at_least_two_labels => Must,
        },
        {
            ipv4 => Must,
            local => Disallow,
            at_least_two_labels => Disallow,
        },
        {
            ipv4 => Disallow,
            local => Must,
            at_least_two_labels => Allow,
        },
        {
            ipv4 => Disallow,
            local => Must,
            at_least_two_labels => Must,
        },
        {
            ipv4 => Disallow,
            local => Must,
            at_least_two_labels => Disallow,
        },
        {
            ipv4 => Disallow,
            local => Disallow,
            at_least_two_labels => Allow,
        },
        {
            ipv4 => Disallow,
            local => Disallow,
            at_least_two_labels => Must,
        },
        {
            ipv4 => Disallow,
            local => Disallow,
            at_least_two_labels => Disallow,
        },
    }

    test2! {
        {
            local => Allow,
            at_least_two_labels => Must,
        },
        {
            local => Allow,
            at_least_two_labels => Disallow,
        },
    }
}

#[test]
fn port_disallow_errors() {
    use validators::errors::DomainError;

    #[derive(Validator)]
    #[validator(domain(port(Disallow)))]
    struct DomainWithoutPort(String);

    assert!(matches!(
        DomainWithoutPort::validate_str("example.com:80"),
        Err(DomainError::PortDisallow)
    ));

    // `host:port` has only one colon.
    for input in ["::1", "example.com:80:90"] {
        assert!(
            matches!(DomainWithoutPort::validate_str(input), Err(DomainError::Invalid)),
            "{input}"
        );
    }
}
