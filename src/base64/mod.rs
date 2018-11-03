extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};
use std::str::Utf8Error;
use std::hash::{Hash, Hasher};

lazy_static! {
    static ref BASE64_RE: Regex = {
        Regex::new("^([A-Za-z0-9+/]{4})*(([A-Za-z0-9+/]{4})|([A-Za-z0-9+/]{3}=)|([A-Za-z0-9+/]{2}==))$").unwrap()
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum Base64Error {
    IncorrectFormat,
    UTF8Error(Utf8Error),
}

impl Display for Base64Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for Base64Error {}

pub type Base64Result = Result<Base64, Base64Error>;

#[derive(Debug, PartialEq)]
pub struct Base64Validator {}

#[derive(Clone)]
pub struct Base64 {
    base64: String,
}

impl Base64 {
    pub fn get_base64(&self) -> &str {
        &self.base64
    }

    pub fn into_string(self) -> String {
        self.base64
    }

    pub unsafe fn from_string_unchecked(base64: String) -> Base64 {
        Base64 {
            base64
        }
    }
}

impl Validated for Base64 {}

impl Debug for Base64 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Base64({})", self.base64))?;
        Ok(())
    }
}

impl Display for Base64 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.base64)?;
        Ok(())
    }
}

impl PartialEq for Base64 {
    fn eq(&self, other: &Self) -> bool {
        self.base64.eq(&other.base64)
    }

    fn ne(&self, other: &Self) -> bool {
        self.base64.ne(&other.base64)
    }
}

impl Eq for Base64 {}

impl Hash for Base64{
    fn hash<H: Hasher>(&self, state: &mut H){
        self.base64.hash(state)
    }
}

impl Base64Validator {
    pub fn is_base64(&self, base64: &str) -> bool {
        self.parse_inner(base64).is_ok()
    }

    pub fn parse_string(&self, base64: String) -> Base64Result {
        let mut base64_inner = self.parse_inner(&base64)?;

        base64_inner.base64 = base64;

        Ok(base64_inner)
    }

    pub fn parse_str(&self, base64: &str) -> Base64Result {
        let mut base64_inner = self.parse_inner(base64)?;

        base64_inner.base64 = base64.to_string();

        Ok(base64_inner)
    }

    fn parse_inner(&self, base64: &str) -> Base64Result {
        if BASE64_RE.is_match(base64) {
            Ok(Base64 {
                base64: String::new(),
            })
        } else {
            Err(Base64Error::IncorrectFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_methods() {
        let base64 = "IHRlc3QgbWVzc2FnZQoK".to_string();

        let bv = Base64Validator {};

        let base64 = bv.parse_string(base64).unwrap();

        assert_eq!("IHRlc3QgbWVzc2FnZQoK", base64.get_base64());
    }

    #[test]
    fn test_base64_lv1() {
        let base64 = "IHRlc3QgbWVzc2FnZQoK".to_string();

        let bv = Base64Validator {};

        bv.parse_string(base64).unwrap();
    }
}

// Base64's wrapper struct is itself
impl ValidatedWrapper for Base64 {
    type Error = Base64Error;

    fn from_string(base64: String) -> Result<Self, Self::Error> {
        Base64::from_string(base64)
    }

    fn from_str(base64: &str) -> Result<Self, Self::Error> {
        Base64::from_str(base64)
    }
}

impl Base64 {
    pub fn from_string(base64: String) -> Result<Self, Base64Error> {
        Base64::create_validator().parse_string(base64)
    }

    pub fn from_str(base64: &str) -> Result<Self, Base64Error> {
        Base64::create_validator().parse_str(base64)
    }

    fn create_validator() -> Base64Validator {
        Base64Validator {}
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for Base64 {
    type Error = Base64Error;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Base64::from_string(form_value.url_decode().map_err(|err| Base64Error::UTF8Error(err))?)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for Base64 {
    type Error = Base64Error;

    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Base64::from_string(param.url_decode().map_err(|err| Base64Error::UTF8Error(err))?)
    }
}

#[cfg(feature = "serdely")]
struct StringVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = Base64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Base64 string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
        Base64::from_str(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: ::serde::de::Error {
        Base64::from_string(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for Base64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        deserializer.deserialize_string(StringVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for Base64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        serializer.serialize_str(&self.base64)
    }
}