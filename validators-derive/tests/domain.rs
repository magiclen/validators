#![cfg(feature = "domain")]

#[macro_use]
extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[test]
fn basic() {
    macro_rules! test_case {
        ($test:ident, $validator:ident) => {
            type Validator = $validator;
            let test = $test;

            test("", false);
            test(
                "127.0.0.1",
                Validator::V_IPV4.allow()
                    && Validator::V_LOCAL.allow()
                    && !Validator::V_PORT.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
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
                    #[validator(domain($($p($v),)*port(NotAllow), conflict(Allow)))]
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
                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*))]
                    pub struct DomainAllowIPv4AllowPortIsLocal {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub port: Option<u16>,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*port(Must)))]
                    pub struct DomainAllowIPv4WithPortIsLocal {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub port: u16,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*port(NotAllow)))]
                    #[allow(dead_code)]
                    struct DomainAllowIPv4WithoutPortIsLocal {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Must), conflict(Allow)))]
                    pub struct DomainIPv4AllowPortIsLocal {
                        pub domain: String,
                        pub port: Option<u16>,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Must), port(Must), conflict(Allow)))]
                    pub struct DomainIPv4WithPortIsLocal {
                        pub domain: String,
                        pub port: u16,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Must), port(NotAllow), conflict(Allow)))]
                    #[allow(dead_code)]
                    struct DomainIPv4WithoutPortIsLocal {
                        pub domain: String,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow)))]
                    pub struct DomainNonIPv4AllowPortIsLocal {
                        pub domain: String,
                        pub port: Option<u16>,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), port(Must)))]
                    pub struct DomainNonIPv4WithPortIsLocal {
                        pub domain: String,
                        pub port: u16,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), port(NotAllow)))]
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
            at_least_two_labels => NotAllow,
        },
        {
            ipv4 => Allow,
            local => NotAllow,
            at_least_two_labels => Allow,
        },
        {
            ipv4 => Allow,
            local => NotAllow,
            at_least_two_labels => Must,
        },
        {
            ipv4 => Allow,
            local => NotAllow,
            at_least_two_labels => NotAllow,
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
            at_least_two_labels => NotAllow,
        },
        {
            ipv4 => Must,
            local => NotAllow,
            at_least_two_labels => Allow,
        },
        {
            ipv4 => Must,
            local => NotAllow,
            at_least_two_labels => Must,
        },
        {
            ipv4 => Must,
            local => NotAllow,
            at_least_two_labels => NotAllow,
        },
        {
            ipv4 => NotAllow,
            local => Must,
            at_least_two_labels => Allow,
        },
        {
            ipv4 => NotAllow,
            local => Must,
            at_least_two_labels => Must,
        },
        {
            ipv4 => NotAllow,
            local => Must,
            at_least_two_labels => NotAllow,
        },
        {
            ipv4 => NotAllow,
            local => NotAllow,
            at_least_two_labels => Allow,
        },
        {
            ipv4 => NotAllow,
            local => NotAllow,
            at_least_two_labels => Must,
        },
        {
            ipv4 => NotAllow,
            local => NotAllow,
            at_least_two_labels => NotAllow,
        },
    }

    test2! {
        {
            local => Allow,
            at_least_two_labels => Must,
        },
        {
            local => Allow,
            at_least_two_labels => NotAllow,
        },
    }
}
