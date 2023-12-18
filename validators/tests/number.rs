#![cfg(all(feature = "test", feature = "derive", feature = "number"))]

use validators::prelude::{validators_prelude::RangeOption, *};

fn check_range<T: PartialOrd>(v: T, range: RangeOption<T>) -> bool {
    match range {
        RangeOption::Inside {
            max,
            min,
            inclusive,
        } => {
            if let Some(min) = min {
                if v < min {
                    return false;
                }
            }

            if let Some(max) = max {
                if inclusive {
                    if v > max {
                        return false;
                    }
                } else if v >= max {
                    return false;
                }
            }

            true
        },
        RangeOption::Outside {
            max,
            min,
            inclusive,
        } => match min {
            Some(min) => match max {
                Some(max) => {
                    if inclusive {
                        !(v >= min && v <= max)
                    } else {
                        !(v >= min && v < max)
                    }
                },
                None => v < min,
            },
            None => match max {
                Some(max) => {
                    if inclusive {
                        v > max
                    } else {
                        v >= max
                    }
                },
                None => true,
            },
        },
        RangeOption::Unlimited => true,
    }
}

#[test]
fn basic() {
    macro_rules! test_case {
        ($test:ident, $validator:ident) => {
            type Validator = $validator;
            let test = $test;

            test("", false);
            test("0", !Validator::V_NAN.must() && check_range(0.0, Validator::V_RANGE));
            test("1", !Validator::V_NAN.must() && check_range(1.0, Validator::V_RANGE));
            test("-1", !Validator::V_NAN.must() && check_range(-1.0, Validator::V_RANGE));
            test("NaN", Validator::V_NAN.allow());
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
                    #[validator(number($($p($v),)*conflict(Allow)))]
                    pub struct F32(f32);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        F32,
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
                    #[validator(number($($p($v),)*conflict(Allow)))]
                    pub struct F64(f64);

                    // test_inner!(
                    //     stringify! {
                    //         $(
                    //             $p = $v,
                    //         )*
                    //     };
                    //     F64,
                    // );
                }
            )*
        }
    }

    test! {
        {
            nan => Allow,
            range => Unlimited,
        },
        {
            nan => Must,
            range => Unlimited,
        },
        {
            nan => Disallow,
            range => Unlimited,
        },
        {
            nan => Allow,
            range => Inside(min = 0),
        },
        {
            nan => Must,
            range => Inside(min = 0),
        },
        {
            nan => Disallow,
            range => Inside(min = 0),
        },
        {
            nan => Allow,
            range => Inside(max = 0),
        },
        {
            nan => Must,
            range => Inside(max = 0),
        },
        {
            nan => Disallow,
            range => Inside(max = 0),
        },
        {
            nan => Allow,
            range => Inside(min = 0, max = 0),
        },
        {
            nan => Must,
            range => Inside(min = 0, max = 0),
        },
        {
            nan => Disallow,
            range => Inside(min = 0, max = 0),
        },
        {
            nan => Allow,
            range => Outside(min = 0, max = 0),
        },
        {
            nan => Must,
            range => Outside(min = 0, max = 0),
        },
        {
            nan => Disallow,
            range => Outside(min = 0, max = 0),
        },
        {
            nan => Disallow,
            range => Inside(min = -1, max = 1),
        },
        {
            nan => Disallow,
            range => Outside(min = -1, max = 1),
        },
        {
            nan => Disallow,
            range => Outside(min = 0, max = 1, inclusive = false),
        },
    }

    test2! {
        {
            nan => Allow,
            range => Unlimited,
        },
        {
            nan => Must,
            range => Unlimited,
        },
        {
            nan => Disallow,
            range => Unlimited,
        },
        {
            nan => Allow,
            range => Inside(min = 0),
        },
        {
            nan => Must,
            range => Inside(min = 0),
        },
        {
            nan => Disallow,
            range => Inside(min = 0),
        },
        {
            nan => Allow,
            range => Inside(max = 0),
        },
        {
            nan => Must,
            range => Inside(max = 0),
        },
        {
            nan => Disallow,
            range => Inside(max = 0),
        },
        {
            nan => Allow,
            range => Inside(min = 0, max = 0),
        },
        {
            nan => Must,
            range => Inside(min = 0, max = 0),
        },
        {
            nan => Disallow,
            range => Inside(min = 0, max = 0),
        },
        {
            nan => Allow,
            range => Outside(min = 0, max = 0),
        },
        {
            nan => Must,
            range => Outside(min = 0, max = 0),
        },
        {
            nan => Disallow,
            range => Outside(min = 0, max = 0),
        },
        {
            nan => Disallow,
            range => Inside(min = -1, max = 1),
        },
        {
            nan => Disallow,
            range => Outside(min = -1, max = 1),
        },
        {
            nan => Disallow,
            range => Outside(min = 0, max = 1, inclusive = false),
        },
    }
}
