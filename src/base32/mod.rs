extern crate regex;

use self::regex::Regex;
use super::Validated;

use std::fmt::{self, Display, Debug, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Base32Error {
    IncorrectFormat,
}

pub type Base32Result = Result<Base32, Base32Error>;

pub struct Base32Validator {}

#[derive(Clone)]
pub struct Base32 {
    base32: String,
}

impl Base32 {
    pub fn get_base32(&self) -> &str {
        &self.base32
    }
}

impl Validated for Base32 {}

impl Debug for Base32 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Base32({})", self.base32))?;
        Ok(())
    }
}

impl Display for Base32 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.base32)?;
        Ok(())
    }
}

impl PartialEq for Base32 {
    fn eq(&self, other: &Self) -> bool {
        self.base32.eq(&other.base32)
    }

    fn ne(&self, other: &Self) -> bool {
        self.base32.ne(&other.base32)
    }
}

impl Base32Validator {
    pub fn is_base32(&self, base32: &str) -> bool {
        self.parse_inner(base32).is_ok()
    }

    pub fn parse_string(&self, base32: String) -> Base32Result {
        let mut base32_inner = self.parse_inner(&base32)?;

        base32_inner.base32 = base32;

        Ok(base32_inner)
    }

    pub fn parse_str(&self, base32: &str) -> Base32Result {
        let mut base32_inner = self.parse_inner(base32)?;

        base32_inner.base32 = base32.to_string();

        Ok(base32_inner)
    }

    fn parse_inner(&self, base32: &str) -> Base32Result {
        let re = Regex::new("^([A-Z2-7]{8})*(([A-Z2-7]{8})|([A-Z2-7]{7}=)|([A-Z2-7]{5}===)|([A-Z2-7]{4}====)|([A-Z2-7]{2}======))$").unwrap();

        if re.is_match(base32) {
            Ok(Base32 {
                base32: String::new(),
            })
        } else {
            Err(Base32Error::IncorrectFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base32_methods() {
        let base32 = "EB2GK43UEBWWK43TMFTWKCQK".to_string();

        let bv = Base32Validator {};

        let base32 = bv.parse_string(base32).unwrap();

        assert_eq!("EB2GK43UEBWWK43TMFTWKCQK", base32.get_base32());
    }

    #[test]
    fn test_base32_lv1() {
        let base32 = "EB2GK43UEBWWK43TMFTWKCQK".to_string();

        let bv = Base32Validator {};

        bv.parse_string(base32).unwrap();
    }
}

// Base32's wrapper struct is itself
impl Base32 {
    pub fn from_string(base32: String) -> Result<Base32, Base32Error> {
        let bv = Base32Validator {};

        bv.parse_string(base32)
    }

    pub fn from_str(base32: &str) -> Result<Base32, Base32Error> {
        let bv = Base32Validator {};

        bv.parse_str(base32)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for Base32 {
    type Error = Base32Error;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Base32::from_str(form_value)
    }
}