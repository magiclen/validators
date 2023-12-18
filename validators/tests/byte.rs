#![cfg(all(feature = "test", feature = "derive", feature = "byte"))]

use validators::{
    byte_unit,
    prelude::{validators_prelude::RangeOption, *},
};

fn check_range<T: PartialOrd>(v: T, range: RangeOption<T>) -> bool {
    if let RangeOption::Inside {
        max,
        min,
        inclusive,
    } = range
    {
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
    } else {
        false
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
        ($( { $( $p:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[derive(Validator)]
                    #[validator(byte($($p,)*))]
                    pub struct B(byte_unit::Byte);

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        B,
                    );
                }
            )*
        }
    }

    test! {
        {
            range(min = 0),
        },
        {
            range(max = 0),
        },
        {
            range(min = 0, max = 0),
        },
        {
            range(min = 0, max = 0),
        },
        {
            range(min = 0, max = 1, inclusive = false),
        },
        {
            ignore_case = false,
        },
    }
}
