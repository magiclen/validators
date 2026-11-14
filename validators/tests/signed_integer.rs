#![cfg(all(feature = "test", feature = "derive", feature = "signed_integer"))]

use validators::prelude::{validators_prelude::RangeOption, *};

#[test]
fn range_macro_expression() {
    macro_rules! integer {
        ($min:expr) => {
            #[derive(Validator)]
            #[validator(signed_integer(range(Inside(min = $min))))]
            struct Integer(i8);
        };
    }

    macro_rules! forward {
        ($min:expr) => {
            integer!($min);
        };
    }

    forward!(-5);

    assert_eq!(-5, Integer::parse_i8(-5).unwrap().0);
    assert_eq!(0, Integer::parse_str("0").unwrap().0);
}

#[cfg(feature = "full")]
#[test]
fn range_expression() {
    #[derive(Validator)]
    #[validator(signed_integer(range(Inside(-5..=5))))]
    struct Integer(i8);

    assert_eq!(-5, Integer::parse_i8(-5).unwrap().0);
    assert_eq!(5, Integer::parse_i8(5).unwrap().0);
}

#[cfg(feature = "full")]
#[test]
fn range_macro_range_expression() {
    macro_rules! integer {
        ($range:expr) => {
            #[derive(Validator)]
            #[validator(signed_integer(range(Inside = $range)))]
            struct Integer(i8);
        };
    }

    integer!(-5..=5);

    assert_eq!(-5, Integer::parse_i8(-5).unwrap().0);
    assert!(Integer::parse_i8(6).is_err());
}

#[cfg(feature = "serde_json")]
#[test]
fn deserialize_positive_integer() {
    use validators::{Result as ValidationResult, serde_json};

    #[derive(Validator)]
    #[validator(signed_integer)]
    struct Integer(i8);

    let json = serde_json::to_string(&Integer::parse_i8(42).unwrap()).unwrap();
    let integer: Integer = serde_json::from_str(&json).unwrap();
    let result: ValidationResult<Integer, validators_prelude::SignedIntegerError> =
        serde_json::from_str(&json).unwrap();

    assert_eq!(42, integer.0);
    assert_eq!(42, result.into_std_result().unwrap().0);
}

fn check_range<T: PartialOrd>(v: T, range: RangeOption<T>) -> bool {
    match range {
        RangeOption::Inside {
            max,
            min,
            inclusive,
        } => {
            if let Some(min) = min
                && v < min
            {
                return false;
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
            test("0", check_range(0, Validator::V_RANGE));
            test("1", check_range(1, Validator::V_RANGE));
            test("-1", check_range(-1, Validator::V_RANGE));
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
                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(signed_integer($($p($v),)*))]
                    pub struct I8(i8);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        I8,
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
                    #[validator(signed_integer($($p($v),)*))]
                    pub struct Isize(isize);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        Isize,
                    );
                }
            )*
        }
    }

    macro_rules! test3 {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(signed_integer($($p($v),)*))]
                    pub struct I128(i128);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        I128,
                    );
                }
            )*
        }
    }

    test! {
        {
            range => Unlimited,
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
        {
            range => Inside(min = -1, max = 1),
        },
        {
            range => Outside(min = -1, max = 1),
        },
        {
            range => Outside(min = 0, max = 1, inclusive = false),
        },
    }

    test2! {
        {
            range => Unlimited,
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
        {
            range => Inside(min = -1, max = 1),
        },
        {
            range => Outside(min = -1, max = 1),
        },
        {
            range => Outside(min = 0, max = 1, inclusive = false),
        },
    }

    test3! {
        {
            range => Unlimited,
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
        {
            range => Inside(min = -1, max = 1),
        },
        {
            range => Outside(min = -1, max = 1),
        },
        {
            range => Outside(min = 0, max = 1, inclusive = false),
        },
    }
}
