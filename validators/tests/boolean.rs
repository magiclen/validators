#![cfg(all(feature = "test", feature = "derive", feature = "boolean"))]

use validators::{Result, errors::BooleanError, prelude::*};

#[derive(Validator)]
#[validator(boolean)]
pub struct Validator(pub bool);

#[cfg(feature = "serde")]
#[test]
fn deserialize_shadowed_names() {
    use validators::serde::{
        Deserialize,
        de::{IntoDeserializer, value::Error},
    };

    #[derive(Validator)]
    #[validator(boolean)]
    struct MyVisitor(bool);

    #[derive(Validator)]
    #[validator(boolean)]
    struct E(bool);

    assert!(MyVisitor::deserialize(IntoDeserializer::<Error>::into_deserializer(true)).unwrap().0);
    assert!(E::deserialize(IntoDeserializer::<Error>::into_deserializer(true)).unwrap().0);
}

#[test]
fn basic() {
    assert!(Validator::validate_str("").is_err());
    assert!(Validator::validate_str("xzy").is_err());
    assert!(Validator::validate_str("onion").is_err());
}

#[test]
fn bool_true() {
    let result: Result<Validator, BooleanError> = true.into();

    assert!(result.into_std_result().unwrap().0);
    assert!(Validator::parse_str("t").unwrap().0);
    assert!(Validator::parse_str("true").unwrap().0);
    assert!(Validator::parse_str("y").unwrap().0);
    assert!(Validator::parse_str("yes").unwrap().0);
    assert!(Validator::parse_str("on").unwrap().0);
    assert!(Validator::parse_str("1").unwrap().0);

    assert!(Validator::parse_char('t').unwrap().0);
    assert!(Validator::parse_char('y').unwrap().0);
    assert!(Validator::parse_char('1').unwrap().0);

    assert!(Validator::parse_i128(1).unwrap().0);
    assert!(Validator::parse_u128(1).unwrap().0);
}

#[test]
fn bool_false() {
    assert!(!Validator::parse_str("f").unwrap().0);
    assert!(!Validator::parse_str("false").unwrap().0);
    assert!(!Validator::parse_str("n").unwrap().0);
    assert!(!Validator::parse_str("no").unwrap().0);
    assert!(!Validator::parse_str("off").unwrap().0);
    assert!(!Validator::parse_str("0").unwrap().0);

    assert!(!Validator::parse_char('f').unwrap().0);
    assert!(!Validator::parse_char('n').unwrap().0);
    assert!(!Validator::parse_char('0').unwrap().0);

    assert!(!Validator::parse_i128(0).unwrap().0);
    assert!(!Validator::parse_u128(0).unwrap().0);
}

#[cfg(feature = "serde")]
#[test]
fn deserialize_binary_bool() {
    use validators::serde::{self, Deserialize, de};

    // Like bincode, this deserializer does not support `deserialize_any`.
    struct BinaryBool(bool);

    impl<'de> de::Deserializer<'de> for BinaryBool {
        type Error = de::value::Error;

        serde::forward_to_deserialize_any! {
            i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes byte_buf option
            unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
        }

        fn deserialize_any<V: de::Visitor<'de>>(
            self,
            _visitor: V,
        ) -> core::result::Result<V::Value, Self::Error> {
            Err(de::Error::custom("deserialize_any is not supported"))
        }

        fn deserialize_bool<V: de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> core::result::Result<V::Value, Self::Error> {
            visitor.visit_bool(self.0)
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }

    let result = Result::<Validator, BooleanError>::deserialize(BinaryBool(true)).unwrap();

    assert!(Validator::deserialize(BinaryBool(true)).unwrap().0);
    assert!(result.into_std_result().unwrap().0);
}
