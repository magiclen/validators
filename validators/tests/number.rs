#![cfg(all(feature = "test", feature = "derive", feature = "number"))]

use validators::prelude::{validators_prelude::RangeOption, *};

#[cfg(feature = "serde_json")]
#[test]
fn deserialize_integers() {
    use validators::{Result as ValidationResult, serde_json};

    #[derive(Validator)]
    #[validator(number)]
    struct Number(f64);

    for (json, expected) in [("1", 1.0), ("-1", -1.0)] {
        let number: Number = serde_json::from_str(json).unwrap();
        let result: ValidationResult<Number, validators_prelude::NumberError> =
            serde_json::from_str(json).unwrap();

        assert_eq!(expected, number.0);
        assert_eq!(expected, result.into_std_result().unwrap().0);
    }
}

#[cfg(feature = "serde_json")]
#[test]
fn serde_roundtrip_non_finite() {
    use validators::serde_json;

    #[derive(Validator)]
    #[validator(number)]
    struct Number(f64);

    let json = serde_json::to_string(&Number::parse_f64(f64::NAN).unwrap()).unwrap();
    let number: Number = serde_json::from_str(&json).unwrap();

    assert_eq!(r#""NaN""#, json);
    assert!(number.0.is_nan());

    for (expected_json, value) in [(r#""inf""#, f64::INFINITY), (r#""-inf""#, f64::NEG_INFINITY)] {
        let json = serde_json::to_string(&Number::parse_f64(value).unwrap()).unwrap();
        let number: Number = serde_json::from_str(&json).unwrap();

        assert_eq!(expected_json, json);
        assert_eq!(value, number.0);
    }
}

#[cfg(feature = "serde")]
#[test]
fn deserialize_binary_f32() {
    use validators::serde::{self, Deserialize, de};

    struct BinaryF32(f32);

    impl<'de> de::Deserializer<'de> for BinaryF32 {
        type Error = de::value::Error;

        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f64 char str string bytes byte_buf option
            unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
        }

        fn deserialize_any<V: de::Visitor<'de>>(
            self,
            _visitor: V,
        ) -> Result<V::Value, Self::Error> {
            Err(de::Error::custom("deserialize_any is not supported"))
        }

        fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
            visitor.visit_f32(self.0)
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }

    #[derive(Validator)]
    #[validator(number(range(Inside(min = 0, max = 1))))]
    struct Number(f32);

    assert_eq!(0.5, Number::deserialize(BinaryF32(0.5)).unwrap().0);
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
                    #[allow(dead_code)]
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
                    #[allow(dead_code)]
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
