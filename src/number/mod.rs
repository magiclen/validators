use super::{Validated, ValidatedWrapper, ValidatorOption};

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub enum NumberError {
    NegativeNotAllow,
    NegativeNotFound,
    ZeroNotAllow,
    ZeroNotFound,
    ParseError(String),
    UnpreciseError,
}

impl Display for NumberError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for NumberError {}

pub type NumberResult = Result<Number, NumberError>;

#[derive(Debug, PartialEq)]
pub struct NumberValidator {
    negative: ValidatorOption,
    zero: ValidatorOption,
}

#[derive(Clone, PartialEq)]
pub struct Number {
    value: f64
}

impl Number {
    pub fn get_number(&self) -> f64 {
        self.value
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0f64
    }

    pub fn is_positive(&self) -> bool {
        self.value > 0f64
    }

    pub fn is_negative(&self) -> bool {
        self.value < 0f64
    }

    pub fn is_integer(&self) -> bool {
        self.value.floor() == self.value
    }

    pub unsafe fn from_f64_unchecked(number: f64) -> Number {
        Number {
            value: number
        }
    }
}

impl Eq for Number {}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bytes: [u8; 8] = unsafe { transmute(self.value) };

        state.write(&bytes);
    }
}

impl Deref for Number {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Validated for Number {}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Number({})", self.value))?;
        Ok(())
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.value))?;
        Ok(())
    }
}

impl NumberValidator {
    pub fn is_number(&self, number: &str) -> bool {
        self.parse_inner(number).is_ok()
    }

    pub fn parse_string(&self, full_number: String) -> NumberResult {
        let value = self.parse_inner(&full_number)?;

        if !precise(&value.to_string(), &full_number) {
            return Err(NumberError::UnpreciseError);
        }

        Ok(value)
    }

    pub fn parse_str(&self, full_number: &str) -> NumberResult {
        let value = self.parse_inner(&full_number)?;

        if !precise(&value.to_string(), &full_number) {
            return Err(NumberError::UnpreciseError);
        }

        Ok(value)
    }

    pub fn parse_f64(&self, value: f64) -> NumberResult {
        if value > 0f64 {
            if self.negative.must() {
                return Err(NumberError::NegativeNotFound);
            } else if self.zero.must() {
                return Err(NumberError::ZeroNotFound);
            }
        } else if value < 0f64 {
            if self.negative.not_allow() {
                return Err(NumberError::NegativeNotAllow);
            } else if self.zero.must() {
                return Err(NumberError::ZeroNotFound);
            }
        } else {
            if self.zero.not_allow() {
                return Err(NumberError::ZeroNotAllow);
            }
        }

        Ok(Number {
            value
        })
    }

    pub fn parse_f32(&self, value: f32) -> NumberResult {
        self.parse_f64(value as f64)
    }

    pub fn parse_i8(&self, value: i8) -> NumberResult {
        self.parse_f64(value as f64)
    }

    pub fn parse_i16(&self, value: i16) -> NumberResult {
        self.parse_f64(value as f64)
    }

    pub fn parse_i32(&self, value: i32) -> NumberResult {
        self.parse_f64(value as f64)
    }

    pub fn parse_i64(&self, value: i64) -> NumberResult {
        if value as f64 as i64 != value {
            return Err(NumberError::UnpreciseError);
        }

        self.parse_f64(value as f64)
    }

    pub fn parse_i128(&self, value: i128) -> NumberResult {
        if value as f64 as i128 != value {
            return Err(NumberError::UnpreciseError);
        }

        self.parse_f64(value as f64)
    }

    pub fn parse_u8(&self, value: u8) -> NumberResult {
        self.parse_f64(value as f64)
    }

    pub fn parse_u16(&self, value: u16) -> NumberResult {
        self.parse_f64(value as f64)
    }

    pub fn parse_u32(&self, value: u32) -> NumberResult {
        self.parse_f64(value as f64)
    }

    pub fn parse_u64(&self, value: u64) -> NumberResult {
        if value as f64 as u64 != value {
            return Err(NumberError::UnpreciseError);
        }

        self.parse_f64(value as f64)
    }

    pub fn parse_u128(&self, value: u128) -> NumberResult {
        if value as f64 as u128 != value {
            return Err(NumberError::UnpreciseError);
        }

        self.parse_f64(value as f64)
    }

    fn parse_inner(&self, full_number: &str) -> NumberResult {
        let value = full_number.parse::<f64>().map_err(|err| NumberError::ParseError(err.to_string()))?;
        self.parse_f64(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_methods() {
        let full_number = "-0.556".to_string();

        let nv = NumberValidator {
            negative: ValidatorOption::Allow,
            zero: ValidatorOption::NotAllow,
        };

        let number = nv.parse_string(full_number).unwrap();

        assert_eq!(-0.556, number.get_number());
        assert_eq!(false, number.is_zero());
        assert_eq!(false, number.is_positive());
        assert_eq!(true, number.is_negative());
        assert_eq!(false, number.is_integer());
    }

    #[test]
    fn test_number_lv1() {
        let full_number = "12345".to_string();

        let nv = NumberValidator {
            negative: ValidatorOption::NotAllow,
            zero: ValidatorOption::NotAllow,
        };

        nv.parse_string(full_number).unwrap();
    }

    #[test]
    fn test_number_lv2() {
        let full_number = "12345.6".to_string();

        let nv = NumberValidator {
            negative: ValidatorOption::NotAllow,
            zero: ValidatorOption::NotAllow,
        };

        nv.parse_string(full_number).unwrap();
    }

    #[test]
    fn test_number_lv3() {
        let full_number = "-12345.6".to_string();

        let nv = NumberValidator {
            negative: ValidatorOption::Allow,
            zero: ValidatorOption::NotAllow,
        };

        nv.parse_string(full_number).unwrap();
    }

    #[test]
    fn test_number_lv4() {
        let full_number = "0".to_string();

        let nv = NumberValidator {
            negative: ValidatorOption::NotAllow,
            zero: ValidatorOption::Allow,
        };

        nv.parse_string(full_number).unwrap();
    }

    #[test]
    fn test_number_lv5() {
        let full_number = "-0".to_string();

        let nv = NumberValidator {
            negative: ValidatorOption::NotAllow,
            zero: ValidatorOption::Allow,
        };

        nv.parse_string(full_number).unwrap();
    }

    #[test]
    fn test_number_lv6() {
        let full_number = "065".to_string();

        let nv = NumberValidator {
            negative: ValidatorOption::NotAllow,
            zero: ValidatorOption::Allow,
        };

        nv.parse_string(full_number).unwrap();
    }

    #[test]
    fn test_number_lv7() {
        let full_number = "65.00".to_string();

        let nv = NumberValidator {
            negative: ValidatorOption::NotAllow,
            zero: ValidatorOption::Allow,
        };

        nv.parse_string(full_number).unwrap();
    }

    #[test]
    fn test_number_lv8() {
        let full_number = "-065.00".to_string();

        let nv = NumberValidator {
            negative: ValidatorOption::Allow,
            zero: ValidatorOption::Allow,
        };

        nv.parse_string(full_number).unwrap();
    }
}

#[doc(hidden)]
pub fn precise(a: &str, b: &str) -> bool {
    let na = a.starts_with("-");
    let a = enclose(a);
    let nb = b.starts_with("-");
    let b = enclose(b);

    if a.len() > 0 {
        a.eq(b) && na == nb
    } else {
        a.eq(b)
    }
}

#[doc(hidden)]
pub fn enclose(number: &str) -> &str {
    let len = number.len();

    let mut start = 0;

    let mut point = None;

    while start < len {
        let s = &number[start..(start + 1)];

        match s {
            "0" | "-" | "+" => (),
            "." => {
                point = Some(start);
                break;
            }
            _ => {
                let mut p = start + 1;
                while p < len {
                    let s = &number[p..(p + 1)];
                    if s.eq(".") {
                        point = Some(p);
                        break;
                    }

                    p += 1;
                }
                break;
            }
        }

        start += 1;
    }

    match point {
        Some(point) => {
            let mut end = len - 1;

            while end > point {
                let s = &number[end..(end + 1)];

                if s.ne("0") {
                    break;
                }
                end -= 1;
            }

            if end == point {
                &number[start..end]
            } else {
                &number[start..(end + 1)]
            }
        }
        None => {
            &number[start..]
        }
    }
}

#[cfg(test)]
mod precise_tests {
    use super::*;

    #[test]
    fn test_enclose_lv1() {
        assert_eq!("1", enclose("1"))
    }

    #[test]
    fn test_enclose_lv2() {
        assert_eq!("1", enclose("1.0"))
    }

    #[test]
    fn test_enclose_lv3() {
        assert_eq!("1", enclose("01.0"))
    }

    #[test]
    fn test_enclose_lv4() {
        assert_eq!("1", enclose("0001.000"))
    }

    #[test]
    fn test_enclose_lv5() {
        assert_eq!(".1", enclose(".100"))
    }

    #[test]
    fn test_enclose_lv6() {
        assert_eq!("1", enclose("001."))
    }

    #[test]
    fn test_enclose_lv7() {
        assert_eq!(".1", enclose("0.1"))
    }

    #[test]
    fn test_enclose_lv8() {
        assert_eq!(".1", enclose("00.1"))
    }

    #[test]
    fn test_enclose_lv9() {
        assert_eq!(".1", enclose("0.10"))
    }

    #[test]
    fn test_enclose_lv10() {
        assert_eq!(".1", enclose("000.100"))
    }

    #[test]
    fn test_enclose_lv11() {
        assert_eq!("", enclose("0.00"))
    }

    #[test]
    fn test_enclose_lv12() {
        assert_eq!("", enclose("0"))
    }

    #[test]
    fn test_enclose_lv13() {
        assert_eq!("1.2", enclose("-001.2"))
    }

    #[test]
    fn test_enclose_lv14() {
        assert_eq!("", enclose(".0"))
    }

    #[test]
    fn test_precise_lv1() {
        assert!(precise("1", "1"))
    }

    #[test]
    fn test_precise_lv2() {
        assert!(precise("1", "1.00"))
    }

    #[test]
    fn test_precise_lv3() {
        assert!(precise("00101.00", "101.00"))
    }

    #[test]
    fn test_precise_lv4() {
        assert!(!precise("-5", "5"))
    }

    #[test]
    fn test_precise_lv5() {
        assert!(precise("-0", "0"))
    }
}

// TODO ----------

macro_rules! extend {
    ( $name:ident, $negative:expr, $zero:expr ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name(Number);

        impl From<$name> for Number {
            fn from(d: $name) -> Self {
                d.0
            }
        }

        impl Deref for $name {
            type Target = f64;

            fn deref(&self) -> &Self::Target {
                &self.0.value
            }
        }

        impl Validated for $name {}

        impl ValidatedWrapper for $name {
            type Error = NumberError;

            fn from_string(full_number: String) -> Result<Self, Self::Error>{
                $name::from_string(full_number)
            }

            fn from_str(full_number: &str) -> Result<Self, Self::Error>{
                $name::from_str(full_number)
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
            pub fn from_string(full_number: String) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_string(full_number)?))
            }

            pub fn from_str(full_number: &str) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_str(full_number)?))
            }

            pub fn from_number(number: Number) -> Result<$name, NumberError> {
                 match $negative {
                    ValidatorOption::Must => {
                        if number.value >= 0f64 {
                            return Err(NumberError::NegativeNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if number.value < 0f64 {
                            return Err(NumberError::NegativeNotAllow)
                        }
                    }
                    _=>()
                }
                match $zero {
                    ValidatorOption::Must => {
                        if number.value >= 0f64 {
                            return Err(NumberError::ZeroNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if number.value >= 0f64 {
                            return Err(NumberError::ZeroNotAllow)
                        }
                    }
                    _=>()
                }

                Ok($name(number))
            }

            pub fn from_f64(value: f64) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_f64(value)?))
            }

            pub fn from_f32(value: f32) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_f32(value)?))
            }

            pub fn from_i8(value: i8) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_i8(value)?))
            }

            pub fn from_i16(value: i16) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_i16(value)?))
            }

            pub fn from_i32(value: i32) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_i32(value)?))
            }

            pub fn from_i64(value: i64) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_i64(value)?))
            }

            pub fn from_i128(value: i128) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_i128(value)?))
            }

            pub fn from_u8(value: u8) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_u8(value)?))
            }

            pub fn from_u16(value: u16) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_u16(value)?))
            }

            pub fn from_u32(value: u32) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_u32(value)?))
            }

            pub fn from_u64(value: u64) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_u64(value)?))
            }

            pub fn from_u128(value: u128) -> Result<$name, NumberError> {
                Ok($name($name::create_validator().parse_u128(value)?))
            }

            pub fn into_number(self) -> Number {
                self.0
            }

            pub fn as_number(&self) -> &Number {
                &self.0
            }

            fn create_validator() -> NumberValidator {
                NumberValidator {
                    negative: $negative,
                    zero: $zero,
                }
            }
        }

        impl $name {
            pub fn get_number(&self) -> f64 {
                self.0.value
            }

            pub fn is_integer(&self) -> bool {
                self.0.value.floor() == self.0.value
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromFormValue<'a> for $name {
            type Error = NumberError;

            fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_str(form_value)
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromParam<'a> for $name {
            type Error = NumberError;

            fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
                $name::from_str(param)
            }
        }

        #[cfg(feature = "serdely")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct NumberVisitor;

                impl<'de> ::serde::de::Visitor<'de> for NumberVisitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_fmt(format_args!("a number({:?})", $name::create_validator()))
                    }

                    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        $name::from_f64(v).map_err(|err| {
                            E::custom(err.to_string())
                        })
                    }
                }

                deserializer.deserialize_f64(NumberVisitor)
            }
        }

        #[cfg(feature = "serdely")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
                serializer.serialize_f64(self.get_number())
            }
        }
    };
}

extend!(NumberAny, ValidatorOption::Allow, ValidatorOption::Allow);

impl NumberAny {
    pub fn is_zero(&self) -> bool {
        self.0.value == 0f64
    }

    pub fn is_positive(&self) -> bool {
        self.0.value > 0f64
    }

    pub fn is_negative(&self) -> bool {
        self.0.value < 0f64
    }
}

extend!(NumberGteZero, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl NumberGteZero {
    pub fn is_zero(&self) -> bool {
        self.0.value == 0f64
    }

    pub fn is_positive(&self) -> bool {
        self.0.value > 0f64
    }
}

extend!(NumberGtZero, ValidatorOption::NotAllow, ValidatorOption::NotAllow);

impl NumberGtZero {}



extend!(NumberLteZero, ValidatorOption::Must, ValidatorOption::Allow);

impl NumberLteZero {
    pub fn is_zero(&self) -> bool {
        self.0.value == 0f64
    }

    pub fn is_negative(&self) -> bool {
        self.0.value < 0f64
    }
}

extend!(NumberLtZero, ValidatorOption::Must, ValidatorOption::NotAllow);

impl NumberLtZero {}

