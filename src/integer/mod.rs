use super::{Validated, ValidatedWrapper, ValidatorOption};
use super::number::precise;

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub enum IntegerError {
    NegativeNotAllow,
    NegativeNotFound,
    ZeroNotAllow,
    ZeroNotFound,
    ParseError(String),
    UnpreciseError,
}

impl Display for IntegerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for IntegerError {}

pub type IntegerResult = Result<Integer, IntegerError>;

#[derive(Debug, PartialEq)]
pub struct IntegerValidator {
    pub negative: ValidatorOption,
    pub zero: ValidatorOption,
}

#[derive(Clone, PartialEq)]
pub struct Integer {
    value: i128
}

impl Integer {
    pub fn get_integer(&self) -> i128 {
        self.value
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0
    }

    pub fn is_positive(&self) -> bool {
        self.value > 0
    }

    pub fn is_negative(&self) -> bool {
        self.value < 0
    }

    pub unsafe fn from_i128_unchecked(integer: i128) -> Integer {
        Integer {
            value: integer
        }
    }
}

impl Eq for Integer {}

impl Hash for Integer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i128(self.value);
    }
}

impl Deref for Integer {
    type Target = i128;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Validated for Integer {}

impl Debug for Integer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Integer({})", self.value))?;
        Ok(())
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.value))?;
        Ok(())
    }
}

impl IntegerValidator {
    pub fn is_integer(&self, integer: &str) -> bool {
        self.parse_inner(integer).is_ok()
    }

    pub fn parse_string(&self, full_integer: String) -> IntegerResult {
        let value = self.parse_inner(&full_integer)?;

        if !precise(&value.to_string(), &full_integer) {
            return Err(IntegerError::UnpreciseError);
        }

        Ok(value)
    }

    pub fn parse_str(&self, full_integer: &str) -> IntegerResult {
        let value = self.parse_inner(&full_integer)?;

        if !precise(&value.to_string(), &full_integer) {
            return Err(IntegerError::UnpreciseError);
        }

        Ok(value)
    }

    pub fn parse_i128(&self, value: i128) -> IntegerResult {
        if value > 0 {
            if self.negative.must() {
                return Err(IntegerError::NegativeNotFound);
            } else if self.zero.must() {
                return Err(IntegerError::ZeroNotFound);
            }
        } else if value < 0 {
            if self.negative.not_allow() {
                return Err(IntegerError::NegativeNotAllow);
            } else if self.zero.must() {
                return Err(IntegerError::ZeroNotFound);
            }
        } else {
            if self.zero.not_allow() {
                return Err(IntegerError::ZeroNotAllow);
            }
        }

        Ok(Integer {
            value
        })
    }

    pub fn parse_u128(&self, value: u128) -> IntegerResult {
        if value > i128::max_value() as u128 {
            return Err(IntegerError::UnpreciseError);
        }

        self.parse_i128(value as i128)
    }

    pub fn parse_f64(&self, value: f64) -> IntegerResult {
        if value.floor() != value {
            return Err(IntegerError::UnpreciseError);
        }

        self.parse_i128(value as i128)
    }

    pub fn parse_f32(&self, value: f32) -> IntegerResult {
        if value.floor() as f32 != value {
            return Err(IntegerError::UnpreciseError);
        }

        self.parse_i128(value as i128)
    }

    pub fn parse_i8(&self, value: i8) -> IntegerResult {
        self.parse_i128(value as i128)
    }

    pub fn parse_i16(&self, value: i16) -> IntegerResult {
        self.parse_i128(value as i128)
    }

    pub fn parse_i32(&self, value: i32) -> IntegerResult {
        self.parse_i128(value as i128)
    }

    pub fn parse_i64(&self, value: i64) -> IntegerResult {
        self.parse_i128(value as i128)
    }

    pub fn parse_u8(&self, value: u8) -> IntegerResult {
        self.parse_i128(value as i128)
    }

    pub fn parse_u16(&self, value: u16) -> IntegerResult {
        self.parse_i128(value as i128)
    }

    pub fn parse_u32(&self, value: u32) -> IntegerResult {
        self.parse_i128(value as i128)
    }

    pub fn parse_u64(&self, value: u64) -> IntegerResult {
        self.parse_i128(value as i128)
    }

    fn parse_inner(&self, full_integer: &str) -> IntegerResult {
        let value = full_integer.parse::<f64>().map_err(|err| IntegerError::ParseError(err.to_string()))?;
        self.parse_f64(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_methods() {
        let full_integer = "-556".to_string();

        let nv = IntegerValidator {
            negative: ValidatorOption::Allow,
            zero: ValidatorOption::NotAllow,
        };

        let integer = nv.parse_string(full_integer).unwrap();

        assert_eq!(-556, integer.get_integer());
        assert_eq!(false, integer.is_zero());
        assert_eq!(false, integer.is_positive());
        assert_eq!(true, integer.is_negative());
    }

    #[test]
    fn test_integer_lv1() {
        let full_integer = "12345".to_string();

        let nv = IntegerValidator {
            negative: ValidatorOption::NotAllow,
            zero: ValidatorOption::NotAllow,
        };

        nv.parse_string(full_integer).unwrap();
    }

    #[test]
    fn test_integer_lv2() {
        let full_integer = "-12345".to_string();

        let nv = IntegerValidator {
            negative: ValidatorOption::Allow,
            zero: ValidatorOption::NotAllow,
        };

        nv.parse_string(full_integer).unwrap();
    }

    #[test]
    fn test_integer_lv3() {
        let full_integer = "0".to_string();

        let nv = IntegerValidator {
            negative: ValidatorOption::NotAllow,
            zero: ValidatorOption::Allow,
        };

        nv.parse_string(full_integer).unwrap();
    }

    #[test]
    fn test_integer_lv4() {
        let full_integer = "-0".to_string();

        let nv = IntegerValidator {
            negative: ValidatorOption::NotAllow,
            zero: ValidatorOption::Allow,
        };

        nv.parse_string(full_integer).unwrap();
    }

    #[test]
    fn test_integer_lv5() {
        let full_integer = "065".to_string();

        let nv = IntegerValidator {
            negative: ValidatorOption::NotAllow,
            zero: ValidatorOption::Allow,
        };

        nv.parse_string(full_integer).unwrap();
    }

    #[test]
    fn test_integer_lv6() {
        let full_integer = "65.00".to_string();

        let nv = IntegerValidator {
            negative: ValidatorOption::NotAllow,
            zero: ValidatorOption::Allow,
        };

        nv.parse_string(full_integer).unwrap();
    }

    #[test]
    fn test_integer_lv7() {
        let full_integer = "-065.00".to_string();

        let nv = IntegerValidator {
            negative: ValidatorOption::Allow,
            zero: ValidatorOption::Allow,
        };

        nv.parse_string(full_integer).unwrap();
    }
}

// TODO ----------

macro_rules! extend {
    ( $name:ident, $negative:expr, $zero:expr ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name(Integer);

        impl From<$name> for Integer {
            fn from(d: $name) -> Self {
                d.0
            }
        }

        impl Deref for $name {
            type Target = i128;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Validated for $name {}

        impl ValidatedWrapper for $name {
            type Error = IntegerError;

            fn from_string(full_integer: String) -> Result<Self, Self::Error> {
                $name::from_string(full_integer)
            }

            fn from_str(full_integer: &str) -> Result<Self, Self::Error> {
                $name::from_str(full_integer)
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.0))?;
                Ok(())
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl $name {
            pub fn from_string(full_integer: String) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_string(full_integer)?))
            }

            pub fn from_str(full_integer: &str) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_str(full_integer)?))
            }

            pub fn from_integer(integer: Integer) -> Result<$name, IntegerError> {
                 match $negative {
                    ValidatorOption::Must => {
                        if integer.value >= 0 {
                            return Err(IntegerError::NegativeNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if integer.value < 0 {
                            return Err(IntegerError::NegativeNotAllow)
                        }
                    }
                    _=>()
                }
                match $zero {
                    ValidatorOption::Must => {
                        if integer.value >= 0 {
                            return Err(IntegerError::ZeroNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if integer.value >= 0 {
                            return Err(IntegerError::ZeroNotAllow)
                        }
                    }
                    _=>()
                }

                Ok($name(integer))
            }

            pub fn from_f64(value: f64) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_f64(value)?))
            }

            pub fn from_f32(value: f32) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_f32(value)?))
            }

            pub fn from_i8(value: i8) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_i8(value)?))
            }

            pub fn from_i16(value: i16) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_i16(value)?))
            }

            pub fn from_i32(value: i32) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_i32(value)?))
            }

            pub fn from_i64(value: i64) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_i64(value)?))
            }

            pub fn from_i128(value: i128) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_i128(value)?))
            }

            pub fn from_u8(value: u8) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_u8(value)?))
            }

            pub fn from_u16(value: u16) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_u16(value)?))
            }

            pub fn from_u32(value: u32) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_u32(value)?))
            }

            pub fn from_u64(value: u64) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_u64(value)?))
            }

            pub fn from_u128(value: u128) -> Result<$name, IntegerError> {
                Ok($name($name::create_validator().parse_u128(value)?))
            }

            pub fn into_integer(self) -> Integer {
                self.0
            }

            pub fn as_integer(&self) -> &Integer {
                &self.0
            }

            fn create_validator() -> IntegerValidator {
                IntegerValidator {
                    negative: $negative,
                    zero: $zero,
                }
            }
        }

        impl $name {
            pub fn get_integer(&self) -> i128 {
                self.0.value
            }
            pub fn get_number(&self) -> i128 {
                self.0.value
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromFormValue<'a> for $name {
            type Error = IntegerError;

            fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
                $name::from_str(form_value)
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromParam<'a> for $name {
            type Error = IntegerError;

            fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
                $name::from_str(param)
            }
        }

        #[cfg(feature = "serdely")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct IntegerVisitor;

                impl<'de> ::serde::de::Visitor<'de> for IntegerVisitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_fmt(format_args!("a integer({:?})", $name::create_validator()))
                    }

                    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        $name::from_i128(v).map_err(|err| {
                            E::custom(err.to_string())
                        })
                    }
                }

                deserializer.deserialize_i128(IntegerVisitor)
            }
        }

        #[cfg(feature = "serdely")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
                serializer.serialize_i128(self.get_integer())
            }
        }
    };
}

extend!(IntegerAny, ValidatorOption::Allow, ValidatorOption::Allow);

impl IntegerAny {
    pub fn is_zero(&self) -> bool {
        self.0.value == 0
    }

    pub fn is_positive(&self) -> bool {
        self.0.value > 0
    }

    pub fn is_negative(&self) -> bool {
        self.0.value < 0
    }
}

extend!(IntegerGteZero, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl IntegerGteZero {
    pub fn is_zero(&self) -> bool {
        self.0.value == 0
    }

    pub fn is_positive(&self) -> bool {
        self.0.value > 0
    }
}

extend!(IntegerGtZero, ValidatorOption::NotAllow, ValidatorOption::NotAllow);

impl IntegerGtZero {}



extend!(IntegerLteZero, ValidatorOption::Must, ValidatorOption::Allow);

impl IntegerLteZero {
    pub fn is_zero(&self) -> bool {
        self.0.value == 0
    }

    pub fn is_negative(&self) -> bool {
        self.0.value < 0
    }
}

extend!(IntegerLtZero, ValidatorOption::Must, ValidatorOption::NotAllow);

impl IntegerLtZero {}

