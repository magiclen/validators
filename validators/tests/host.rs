#![cfg(all(feature = "test", feature = "derive", feature = "host"))]

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
                    #[allow(dead_code)] // ignore spurious dead code
                    #[derive(Validator)]
                    #[validator(host($($p($v),)*))]
                    pub struct HostAllowPort {
                        pub host: validators::models::Host,
                        pub port: Option<u16>,
                    }

                    #[allow(dead_code)] // ignore spurious dead code
                    #[derive(Validator)]
                    #[validator(host($($p($v),)*port(Must)))]
                    pub struct HostWithPort {
                        pub host: validators::models::Host,
                        pub port: u16,
                    }

                    #[allow(dead_code)] // ignore spurious dead code
                    #[derive(Validator)]
                    #[validator(host($($p($v),)*port(Disallow)))]
                    pub struct HostWithoutPort(pub validators::models::Host);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        HostAllowPort,
                        HostWithPort,
                        HostWithoutPort,
                    );
                }
            )*
        }
    }

    macro_rules! test2 {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[allow(dead_code)] // ignore spurious dead code
                    #[derive(Validator)]
                    #[validator(host($($p($v),)*))]
                    pub struct HostAllowPortIsLocal {
                        pub host: validators::models::Host,
                        pub port: Option<u16>,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)] // ignore spurious dead code
                    #[derive(Validator)]
                    #[validator(host($($p($v),)*port(Must)))]
                    pub struct HostWithPortIsLocal {
                        pub host: validators::models::Host,
                        pub port: u16,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)] // ignore spurious dead code
                    #[derive(Validator)]
                    #[validator(host($($p($v),)*port(Disallow)))]
                    pub struct HostWithoutPortIsLocal {
                        pub host: validators::models::Host,
                        pub is_local: bool,
                    }

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        HostAllowPortIsLocal,
                        HostWithPortIsLocal,
                        HostWithoutPortIsLocal,
                    );
                }
            )*
        }
    }

    test! {
        {
            local => Allow,
            at_least_two_labels => Allow,
        },
        {
            local => Must,
            at_least_two_labels => Allow,
        },
        {
            local => Must,
            at_least_two_labels => Must,
        },
        {
            local => Must,
            at_least_two_labels => Disallow,
        },
        {
            local => Disallow,
            at_least_two_labels => Allow,
        },
        {
            local => Disallow,
            at_least_two_labels => Must,
        },
        {
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
