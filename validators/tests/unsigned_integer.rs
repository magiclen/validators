#![cfg(all(feature = "test", feature = "derive", feature = "unsigned_integer"))]

use validators::prelude::{validators_prelude::RangeOption, *};

#[cfg(feature = "serde")]
#[test]
fn deserialize_i64() {
    use validators::{
        Result as ValidationResult,
        serde::{
            Deserialize,
            de::{IntoDeserializer, value::Error},
        },
    };

    #[derive(Validator)]
    #[validator(unsigned_integer)]
    struct Integer(u8);

    let integer =
        Integer::deserialize(IntoDeserializer::<Error>::into_deserializer(42i64)).unwrap();
    let result: ValidationResult<Integer, validators_prelude::UnsignedIntegerError> =
        Deserialize::deserialize(IntoDeserializer::<Error>::into_deserializer(42i64)).unwrap();

    assert_eq!(42, integer.0);
    assert_eq!(42, result.into_std_result().unwrap().0);
}

#[cfg(feature = "serde")]
#[test]
fn deserialize_single_letter_names() {
    use validators::serde::{
        Deserialize,
        de::{IntoDeserializer, value::Error},
    };

    // The generated code also has generic parameters, so these names must still work.
    #[derive(Validator)]
    #[validator(unsigned_integer)]
    struct D(u8);

    #[derive(Validator)]
    #[validator(unsigned_integer)]
    struct E(u8);

    let d = D::deserialize(IntoDeserializer::<Error>::into_deserializer(1u8)).unwrap();
    let e = E::deserialize(IntoDeserializer::<Error>::into_deserializer(2u8)).unwrap();

    assert_eq!(1, d.0);
    assert_eq!(2, e.0);
}

#[cfg(feature = "serde")]
#[test]
fn deserialize_binary_u16() {
    use validators::{
        Result as ValidationResult,
        serde::{self, Deserialize, de},
    };

    struct BinaryU16(u16);

    impl<'de> de::Deserializer<'de> for BinaryU16 {
        type Error = de::value::Error;

        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u32 u64 f32 f64 char str string bytes byte_buf option
            unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
        }

        fn deserialize_any<V: de::Visitor<'de>>(
            self,
            _visitor: V,
        ) -> Result<V::Value, Self::Error> {
            Err(de::Error::custom("deserialize_any is not supported"))
        }

        fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
            visitor.visit_u16(self.0)
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }

    #[derive(Validator)]
    #[validator(unsigned_integer(range(Inside(min = 1, max = 8080))))]
    struct Port(u16);

    assert_eq!(8080, Port::deserialize(BinaryU16(8080)).unwrap().0);

    let result: ValidationResult<Port, validators_prelude::UnsignedIntegerError> =
        Deserialize::deserialize(BinaryU16(8080)).unwrap();

    assert_eq!(8080, result.into_std_result().unwrap().0);
}

#[test]
fn too_large_errors() {
    use validators::errors::UnsignedIntegerError;

    #[derive(Validator)]
    #[validator(unsigned_integer(range(Inside(max = 150))))]
    struct Integer(u8);

    // A string that does not fit in `u8` is reported like a number out of the range.
    for input in ["200", "300"] {
        assert!(
            matches!(Integer::parse_str(input), Err(UnsignedIntegerError::TooLarge)),
            "{input}"
        );
    }

    assert!(matches!(Integer::parse_u64(300), Err(UnsignedIntegerError::TooLarge)));
    assert_eq!(150, Integer::parse_str("150").unwrap().0);
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
            test("2", check_range(2, Validator::V_RANGE));
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
                    #[allow(dead_code)]
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
                    #[allow(dead_code)]
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
            range => Outside(min = 0, max = 1, inclusive = false),
        },
        {
            range => Outside(min = 1, max = 2),
        },
        {
            range => Outside(min = 0, max = 2, inclusive = false),
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
            range => Outside(min = 0, max = 1, inclusive = false),
        },
    }
}
