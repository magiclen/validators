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
                    #[validator(domain($($p($v),)*))]
                    pub struct DomainAllowIPv4AllowLocalAllowPort {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub is_local: bool,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*port(Must)))]
                    pub struct DomainAllowIPv4AllowLocalWithPort {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub is_local: bool,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*port(NotAllow)))]
                    struct DomainAllowIPv4AllowLocalWithoutPort {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*local(Must)))]
                    pub struct DomainAllowIPv4LocalAllowPort {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*local(Must), port(Must)))]
                    pub struct DomainAllowIPv4LocalWithPort {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*local(Must), port(NotAllow)))]
                    struct DomainAllowIPv4LocalWithoutPort {
                        pub domain: String,
                        pub is_ipv4: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*local(NotAllow)))]
                    pub struct DomainAllowIPv4NonLocalAllowPort {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*local(NotAllow), port(Must)))]
                    pub struct DomainAllowIPv4NonLocalWithPort {
                        pub domain: String,
                        pub is_ipv4: bool,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*local(NotAllow), port(NotAllow)))]
                    struct DomainAllowIPv4NonLocalWithoutPort {
                        pub domain: String,
                        pub is_ipv4: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Must), conflict(Allow)))]
                    pub struct DomainIPv4AllowLocalAllowPort {
                        pub domain: String,
                        pub is_local: bool,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Must), port(Must), conflict(Allow)))]
                    pub struct DomainIPv4AllowLocalWithPort {
                        pub domain: String,
                        pub is_local: bool,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Must), port(NotAllow), conflict(Allow)))]
                    struct DomainIPv4AllowLocalWithoutPort {
                        pub domain: String,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Must), local(Must), conflict(Allow)))]
                    pub struct DomainIPv4LocalAllowPort {
                        pub domain: String,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Must), local(Must), port(Must), conflict(Allow)))]
                    pub struct DomainIPv4LocalWithPort {
                        pub domain: String,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(Must), local(Must), port(NotAllow), conflict(Allow)))]
                    struct DomainIPv4LocalWithoutPort(pub String);

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), local(NotAllow)))]
                    pub struct DomainIPv4NonLocalAllowPort {
                        pub domain: String,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), local(NotAllow), port(Must)))]
                    pub struct DomainIPv4NonLocalWithPort {
                        pub domain: String,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), local(NotAllow), port(NotAllow)))]
                    struct DomainIPv4NonLocalWithoutPort(pub String);

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow)))]
                    pub struct DomainNonIPv4AllowLocalAllowPort {
                        pub domain: String,
                        pub is_local: bool,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), port(Must)))]
                    pub struct DomainNonIPv4AllowLocalWithPort {
                        pub domain: String,
                        pub is_local: bool,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), port(NotAllow)))]
                    struct DomainNonIPv4AllowLocalWithoutPort {
                        pub domain: String,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), local(Must)))]
                    pub struct DomainNonIPv4LocalAllowPort {
                        pub domain: String,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), local(Must), port(Must)))]
                    pub struct DomainNonIPv4LocalWithPort {
                        pub domain: String,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), local(Must), port(NotAllow)))]
                    struct DomainNonIPv4LocalWithoutPort(pub String);

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), local(NotAllow)))]
                    pub struct DomainNonIPv4NonLocalAllowPort {
                        pub domain: String,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), local(NotAllow), port(Must)))]
                    pub struct DomainNonIPv4NonLocalWithPort {
                        pub domain: String,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(domain($($p($v),)*ipv4(NotAllow), local(NotAllow), port(NotAllow)))]
                    struct DomainNonIPv4NonLocalWithoutPort(pub String);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        DomainAllowIPv4AllowLocalAllowPort,
                        DomainAllowIPv4AllowLocalWithPort,
                        DomainAllowIPv4AllowLocalWithoutPort,
                        DomainAllowIPv4LocalAllowPort,
                        DomainAllowIPv4LocalWithPort,
                        DomainAllowIPv4LocalWithoutPort,
                        DomainAllowIPv4NonLocalAllowPort,
                        DomainAllowIPv4NonLocalWithPort,
                        DomainAllowIPv4NonLocalWithoutPort,
                        DomainIPv4AllowLocalAllowPort,
                        DomainIPv4AllowLocalWithPort,
                        DomainIPv4AllowLocalWithoutPort,
                        DomainIPv4LocalAllowPort,
                        DomainIPv4LocalWithPort,
                        DomainIPv4LocalWithoutPort,
                        DomainIPv4NonLocalAllowPort,
                        DomainIPv4NonLocalWithPort,
                        DomainIPv4NonLocalWithoutPort,
                        DomainNonIPv4AllowLocalAllowPort,
                        DomainNonIPv4AllowLocalWithPort,
                        DomainNonIPv4AllowLocalWithoutPort,
                        DomainNonIPv4LocalAllowPort,
                        DomainNonIPv4LocalWithPort,
                        DomainNonIPv4LocalWithoutPort,
                        DomainNonIPv4NonLocalAllowPort,
                        DomainNonIPv4NonLocalWithPort,
                        DomainNonIPv4NonLocalWithoutPort
                    );
                }
            )*
        }
    }

    test! {
        {
            at_least_two_labels => Allow,
        },
        {
            at_least_two_labels => Must,
        },
        {
            at_least_two_labels => NotAllow,
        },
    }
}
