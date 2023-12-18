#![cfg(all(feature = "test", feature = "derive", feature = "ipv6"))]

use validators::prelude::*;

#[test]
fn basic() {
    macro_rules! test_case {
        ($test:ident, $validator:ident) => {
            type Validator = $validator;
            let test = $test;

            test("", false);
            test("127.0.0.1", false);
            test("127.0.0.1:8080", false);
            test("168.17.212.1", false);
            test("168.17.212.1:8080", false);
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
                    #[validator(ipv6($($p($v),)*))]
                    pub struct IPv6AllowPort {
                        pub ipv6: std::net::Ipv6Addr,
                        pub port: Option<u16>,
                    }

                    #[derive(Validator)]
                    #[validator(ipv6($($p($v),)*port(Must)))]
                    pub struct IPv6WithPort {
                        pub ipv6: std::net::Ipv6Addr,
                        pub port: u16,
                    }

                    #[derive(Validator)]
                    #[validator(ipv6($($p($v),)*port(Disallow)))]
                    pub struct IPv6WithoutPort(pub std::net::Ipv6Addr);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        IPv6AllowPort,
                        IPv6WithPort,
                        IPv6WithoutPort,
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
