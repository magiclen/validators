extern crate regex;

use self::regex::Regex;
use super::Validated;

use std::fmt::{self, Display, Debug, Formatter};
use std::str::Utf8Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Base64Error {
    IncorrectFormat,
    UTF8Error(Utf8Error),
}

pub type Base64Result = Result<Base64, Base64Error>;

pub struct Base64Validator {}

#[derive(Clone)]
pub struct Base64 {
    base64: String,
}

impl Base64 {
    pub fn get_base64(&self) -> &str {
        &self.base64
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
        let re = Regex::new("^([A-Za-z0-9+/]{4})*(([A-Za-z0-9+/]{4})|([A-Za-z0-9+/]{3}=)|([A-Za-z0-9+/]{2}==))$").unwrap();

        if re.is_match(base64) {
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
impl Base64 {
    pub fn from_string(base64: String) -> Result<Base64, Base64Error> {
        let bv = Base64Validator {};

        bv.parse_string(base64)
    }

    pub fn from_str(base64: &str) -> Result<Base64, Base64Error> {
        let bv = Base64Validator {};

        bv.parse_str(base64)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for Base64 {
    type Error = Base64Error;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Base64::from_string(form_value.url_decode().map_err(|err| Base64Error::UTF8Error(err))?)
    }
}