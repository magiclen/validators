#![cfg(feature = "unsigned_integer")]

#[macro_use]
extern crate validators_derive;

extern crate validators;

use validators::prelude::*;
use validators::validators_options::ValidatorRangeOption;

fn check_range<T: PartialOrd>(v: T, range: ValidatorRangeOption<T>) -> bool {
    match range {
        ValidatorRangeOption::Inside {
            max,
            min,
        } => {
            if let Some(min) = min {
                if v < min {
                    return false;
                }
            }

            if let Some(max) = max {
                if v > max {
                    return false;
                }
            }

            true
        }
        ValidatorRangeOption::Outside {
            max,
            min,
        } => {
            match min {
                Some(min) => {
                    match max {
                        Some(max) => !(v >= min && v <= max),
                        None => v < min,
                    }
                }
                None => {
                    match max {
                        Some(max) => v > max,
                        None => true,
                    }
                }
            }
        }
        ValidatorRangeOption::NotLimited => true,
    }
}

#[test]
fn basic() {
    macro_rules! test_case {
        ($test:ident, $validator:ident) => {
            type Validator = $validator;
            let test = $test;

            test("", false);
            test("0", check_range(0, Validator::V_RANGE));
            test("1", check_range(1, Validator::V_RANGE));
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
                    #[validator(unsigned_integer($($p($v),)*))]
                    pub struct U8(u8);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        U8,
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
                    #[validator(unsigned_integer($($p($v),)*))]
                    pub struct Usize(usize);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        Usize,
                    );
                }
            )*
        }
    }

    macro_rules! test3 {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[derive(Validator)]
                    #[validator(unsigned_integer($($p($v),)*))]
                    pub struct U128(u128);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        U128,
                    );
                }
            )*
        }
    }

    test! {
        {
            range => NotLimited,
        },
        {
            range => Inside(min = 0),
        },
        {
            range => Inside(max = 0),
        },
        {
            range => Inside(min = 0, max = 0),
        },
        {
            range => Outside(min = 0, max = 0),
        },
    }

    test2! {
        {
            range => NotLimited,
        },
        {
            range => Inside(min = 0),
        },
        {
            range => Inside(max = 0),
        },
        {
            range => Inside(min = 0, max = 0),
        },
        {
            range => Outside(min = 0, max = 0),
        },
    }

    test3! {
        {
            range => NotLimited,
        },
        {
            range => Inside(min = 0),
        },
        {
            range => Inside(max = 0),
        },
        {
            range => Inside(min = 0, max = 0),
        },
        {
            range => Outside(min = 0, max = 0),
        },
    }
}
