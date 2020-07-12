#![cfg(feature = "host")]

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
                Validator::V_LOCAL.allow()
                    && !Validator::V_PORT.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test("127.0.0.1.", false);
            test(
                "127.0.0.1:8080",
                Validator::V_LOCAL.allow()
                    && Validator::V_PORT.allow()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test("127.0.0.1.:8080", false);
            test(
                "168.17.212.1",
                !Validator::V_LOCAL.must()
                    && !Validator::V_PORT.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test("168.17.212.1.", false);
            test(
                "168.17.212.1:8080",
                !Validator::V_LOCAL.must()
                    && Validator::V_PORT.allow()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test("168.17.212.1.:8080", false);
            test("localhost", Validator::V_LOCAL.allow() && !Validator::V_PORT.must());
            test("localhost.", false);
            test("localhost:8080", Validator::V_LOCAL.allow() && Validator::V_PORT.allow());
            test("localhost.:8080", false);
            test(
                "myhost",
                !Validator::V_LOCAL.must()
                    && !Validator::V_PORT.must()
                    && !Validator::V_AT_LEAST_TWO_LABELS.must(),
            );
            test("myhost.", false);
            test(
                "myhost:8080",
                !Validator::V_LOCAL.must()
                    && Validator::V_PORT.allow()
                    && !Validator::V_AT_LEAST_TWO_LABELS.must(),
            );
            test("myhost.:8080", false);
            test(
                "臺灣.tw",
                !Validator::V_LOCAL.must()
                    && !Validator::V_PORT.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test("臺灣.tw.", false);
            test(
                "臺灣.tw:8080",
                !Validator::V_LOCAL.must()
                    && Validator::V_PORT.allow()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test("臺灣.tw.:8080", false);
            test(
                "0000:0000:0000:0000:0000:0000:370:7348",
                !Validator::V_LOCAL.must()
                    && !Validator::V_PORT.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "[0000:0000:0000:0000:0000:0000:370:7348]",
                !Validator::V_LOCAL.must()
                    && !Validator::V_PORT.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "[0000:0000:0000:0000:0000:0000:370:7348]:8080",
                !Validator::V_LOCAL.must()
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
                    #[validator(host($($p($v),)*))]
                    pub struct HostAllowLocalAllowPort {
                        pub host: validators::models::Host,
                        pub is_local: bool,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(host($($p($v),)*port(Must)))]
                    pub struct HostAllowLocalWithPort {
                        pub host: validators::models::Host,
                        pub is_local: bool,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(host($($p($v),)*port(NotAllow)))]
                    pub struct HostAllowLocalWithoutPort {
                        pub host: validators::models::Host,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(host($($p($v),)*local(Must)))]
                    pub struct HostLocalAllowPort {
                        pub host: validators::models::Host,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(host($($p($v),)*local(Must), port(Must)))]
                    pub struct HostLocalWithPort {
                        pub host: validators::models::Host,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(host($($p($v),)*local(Must), port(NotAllow)))]
                    pub struct HostLocalWithoutPort(validators::models::Host);

                    #[derive(Validator)]
                    #[validator(host($($p($v),)*local(NotAllow)))]
                    pub struct HostNonLocalAllowPort {
                        pub host: validators::models::Host,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(host($($p($v),)*local(NotAllow), port(Must)))]
                    pub struct HostNonLocalWithPort {
                        pub host: validators::models::Host,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(host($($p($v),)*local(NotAllow), port(NotAllow)))]
                    pub struct HostNonLocalWithoutPort(validators::models::Host);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        HostAllowLocalAllowPort,
                        HostAllowLocalWithPort,
                        HostAllowLocalWithoutPort,
                        HostLocalAllowPort,
                        HostLocalWithPort,
                        HostLocalWithoutPort,
                        HostNonLocalAllowPort,
                        HostNonLocalWithPort,
                        HostNonLocalWithoutPort,
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
