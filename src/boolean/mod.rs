extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

lazy_static! {
    pub(crate) static ref TRUE_RE: Regex = { Regex::new(r"^(?i)true|yes|on|y|t|1$").unwrap() };
    pub(crate) static ref FALSE_RE: Regex = { Regex::new(r"^(?i)false|no|off|n|f|0$").unwrap() };
}

#[derive(Debug, PartialEq, Clone)]
pub enum BooleanError {
    IncorrectFormat,
}

impl Display for BooleanError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for BooleanError {}

pub type BooleanResult = Result<Boolean, BooleanError>;

#[derive(Debug, PartialEq)]
pub struct BooleanValidator {}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Boolean {
    boolean: bool,
}

impl Boolean {
    pub fn get_bool(&self) -> bool {
        self.boolean
    }

    #[inline]
    pub fn from_bool(boolean: bool) -> Boolean {
        Boolean {
            boolean,
        }
    }
}

impl Deref for Boolean {
    type Target = bool;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.boolean
    }
}

impl Validated for Boolean {}

impl Debug for Boolean {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(Boolean, f, self, let .0 = self.boolean);
    }
}

impl Display for Boolean {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.boolean {
            f.write_str("true")?;
        } else {
            f.write_str("false")?;
        }
        Ok(())
    }
}

impl BooleanValidator {
    #[inline]
    pub fn is_boolean(&self, boolean: &str) -> bool {
        self.parse_inner(boolean).is_ok()
    }

    #[inline]
    pub fn parse_string(&self, boolean: String) -> BooleanResult {
        self.parse_inner(&boolean)
    }

    #[inline]
    pub fn parse_str(&self, boolean: &str) -> BooleanResult {
        self.parse_inner(boolean)
    }

    #[inline]
    fn parse_inner(&self, boolean: &str) -> BooleanResult {
        if TRUE_RE.is_match(boolean) {
            Ok(Boolean {
                boolean: true,
            })
        } else if FALSE_RE.is_match(boolean) {
            Ok(Boolean {
                boolean: false,
            })
        } else {
            Err(BooleanError::IncorrectFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_methods() {
        let boolean = Boolean::from_str("TRUE").unwrap();

        assert_eq!(true, boolean.get_bool());

        let boolean = Boolean::from_str("FALSE").unwrap();

        assert_eq!(false, boolean.get_bool());
    }

    #[test]
    fn test_boolean_lv1() {
        Boolean::from_str("TRUE").unwrap();
        Boolean::from_str("FALSE").unwrap();
        Boolean::from_str("ON").unwrap();
        Boolean::from_str("OFF").unwrap();
        Boolean::from_str("YES").unwrap();
        Boolean::from_str("NO").unwrap();
        Boolean::from_str("Y").unwrap();
        Boolean::from_str("N").unwrap();
        Boolean::from_str("T").unwrap();
        Boolean::from_str("F").unwrap();
        Boolean::from_str("0").unwrap();
        Boolean::from_str("1").unwrap();
    }
}

// Boolean's wrapper struct is itself
impl ValidatedWrapper for Boolean {
    type Error = BooleanError;

    #[inline]
    fn from_string(boolean: String) -> Result<Self, Self::Error> {
        Boolean::from_string(boolean)
    }

    #[inline]
    fn from_str(boolean: &str) -> Result<Self, Self::Error> {
        Boolean::from_str(boolean)
    }
}

impl Boolean {
    #[inline]
    pub fn from_string(boolean: String) -> Result<Self, BooleanError> {
        Boolean::create_validator().parse_string(boolean)
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(boolean: &str) -> Result<Self, BooleanError> {
        Boolean::create_validator().parse_str(boolean)
    }

    #[inline]
    fn create_validator() -> BooleanValidator {
        BooleanValidator {}
    }
}

impl FromStr for Boolean {
    type Err = BooleanError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Boolean::from_str(s)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for Boolean {
    type Error = BooleanError;

    #[inline]
    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Boolean::from_string(param.to_string())
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for Boolean {
    type Error = BooleanError;

    #[inline]
    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Boolean::from_str(form_value.as_str())
    }
}

#[cfg(feature = "serdely")]
struct StringBooleanVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringBooleanVisitor {
    type Value = Boolean;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Boolean string or a bool value")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        Boolean::from_str(v).map_err(|err| E::custom(err.to_string()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        Boolean::from_string(v).map_err(|err| E::custom(err.to_string()))
    }

    #[inline]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        Ok(Boolean::from_bool(v))
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for Boolean {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>, {
        deserializer.deserialize_any(StringBooleanVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for Boolean {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer, {
        serializer.serialize_bool(self.boolean)
    }
}
