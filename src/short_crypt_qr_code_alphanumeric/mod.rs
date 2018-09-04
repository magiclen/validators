extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum ShortCryptQRCodeAlphanumericError {
    IncorrectFormat,
}

impl Display for ShortCryptQRCodeAlphanumericError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ShortCryptQRCodeAlphanumericError {}

pub type ShortCryptQRCodeAlphanumericResult = Result<ShortCryptQRCodeAlphanumeric, ShortCryptQRCodeAlphanumericError>;

#[derive(Debug, PartialEq)]
pub struct ShortCryptQRCodeAlphanumericValidator {}

#[derive(Clone)]
pub struct ShortCryptQRCodeAlphanumeric {
    short_crypt_qr_code_alphanumeric: String,
}

impl ShortCryptQRCodeAlphanumeric {
    pub fn get_short_crypt_qr_code_alphanumeric_url(&self) -> &str {
        &self.short_crypt_qr_code_alphanumeric
    }
}

impl Validated for ShortCryptQRCodeAlphanumeric {}

impl Debug for ShortCryptQRCodeAlphanumeric {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("ShortCryptQRCodeAlphanumeric({})", self.short_crypt_qr_code_alphanumeric))?;
        Ok(())
    }
}

impl Display for ShortCryptQRCodeAlphanumeric {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.short_crypt_qr_code_alphanumeric)?;
        Ok(())
    }
}

impl PartialEq for ShortCryptQRCodeAlphanumeric {
    fn eq(&self, other: &Self) -> bool {
        self.short_crypt_qr_code_alphanumeric.eq(&other.short_crypt_qr_code_alphanumeric)
    }

    fn ne(&self, other: &Self) -> bool {
        self.short_crypt_qr_code_alphanumeric.ne(&other.short_crypt_qr_code_alphanumeric)
    }
}

impl ShortCryptQRCodeAlphanumericValidator {
    pub fn is_short_crypt_qr_code_alphanumeric_url(&self, short_crypt_qr_code_alphanumeric_url: &str) -> bool {
        self.parse_inner(short_crypt_qr_code_alphanumeric_url).is_ok()
    }

    pub fn parse_string(&self, short_crypt_qr_code_alphanumeric_url: String) -> ShortCryptQRCodeAlphanumericResult {
        let mut short_crypt_qr_code_alphanumeric_url_inner = self.parse_inner(&short_crypt_qr_code_alphanumeric_url)?;

        short_crypt_qr_code_alphanumeric_url_inner.short_crypt_qr_code_alphanumeric = short_crypt_qr_code_alphanumeric_url;

        Ok(short_crypt_qr_code_alphanumeric_url_inner)
    }

    pub fn parse_str(&self, short_crypt_qr_code_alphanumeric_url: &str) -> ShortCryptQRCodeAlphanumericResult {
        let mut short_crypt_qr_code_alphanumeric_url_inner = self.parse_inner(short_crypt_qr_code_alphanumeric_url)?;

        short_crypt_qr_code_alphanumeric_url_inner.short_crypt_qr_code_alphanumeric = short_crypt_qr_code_alphanumeric_url.to_string();

        Ok(short_crypt_qr_code_alphanumeric_url_inner)
    }

    fn parse_inner(&self, short_crypt_qr_code_alphanumeric_url: &str) -> ShortCryptQRCodeAlphanumericResult {
        let re = Regex::new(r"^([A-Z0-9]{8})*([A-Z0-9]|[A-Z0-9]{3}|[A-Z0-9]{5,6}|[A-Z0-9]{8})$").unwrap();

        if re.is_match(short_crypt_qr_code_alphanumeric_url) {
            Ok(ShortCryptQRCodeAlphanumeric {
                short_crypt_qr_code_alphanumeric: String::new(),
            })
        } else {
            Err(ShortCryptQRCodeAlphanumericError::IncorrectFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_crypt_qr_code_alphanumeric_url_methods() {
        let short_crypt_qr_code_alphanumeric_url = "3BHNNR45XZH8PU".to_string();

        let scqacv = ShortCryptQRCodeAlphanumericValidator {};

        let short_crypt_qr_code_alphanumeric_url = scqacv.parse_string(short_crypt_qr_code_alphanumeric_url).unwrap();

        assert_eq!("3BHNNR45XZH8PU", short_crypt_qr_code_alphanumeric_url.get_short_crypt_qr_code_alphanumeric_url());
    }

    #[test]
    fn test_short_crypt_qr_code_alphanumeric_url_lv1() {
        let short_crypt_qr_code_alphanumeric_url = "3BHNNR45XZH8PU".to_string();

        let scqacv = ShortCryptQRCodeAlphanumericValidator {};

        scqacv.parse_string(short_crypt_qr_code_alphanumeric_url).unwrap();
    }
}

// ShortCryptQRCodeAlphanumeric's wrapper struct is itself
impl ValidatedWrapper for ShortCryptQRCodeAlphanumeric {
    type Error = ShortCryptQRCodeAlphanumericError;

    fn from_string(short_crypt_qr_code_alphanumeric_url: String) -> Result<Self, Self::Error> {
        ShortCryptQRCodeAlphanumeric::from_string(short_crypt_qr_code_alphanumeric_url)
    }

    fn from_str(short_crypt_qr_code_alphanumeric_url: &str) -> Result<Self, Self::Error> {
        ShortCryptQRCodeAlphanumeric::from_str(short_crypt_qr_code_alphanumeric_url)
    }
}

impl ShortCryptQRCodeAlphanumeric {
    pub fn from_string(short_crypt_qr_code_alphanumeric_url: String) -> Result<Self, ShortCryptQRCodeAlphanumericError> {
        ShortCryptQRCodeAlphanumeric::create_validator().parse_string(short_crypt_qr_code_alphanumeric_url)
    }

    pub fn from_str(short_crypt_qr_code_alphanumeric_url: &str) -> Result<Self, ShortCryptQRCodeAlphanumericError> {
        ShortCryptQRCodeAlphanumeric::create_validator().parse_str(short_crypt_qr_code_alphanumeric_url)
    }

    fn create_validator() -> ShortCryptQRCodeAlphanumericValidator {
        ShortCryptQRCodeAlphanumericValidator {}
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for ShortCryptQRCodeAlphanumeric {
    type Error = ShortCryptQRCodeAlphanumericError;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        ShortCryptQRCodeAlphanumeric::from_str(form_value)
    }
}

#[cfg(feature = "serdely")]
struct StringVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = ShortCryptQRCodeAlphanumeric;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an ShortCrypt QR code alphanumeric string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
        ShortCryptQRCodeAlphanumeric::from_str(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: ::serde::de::Error {
        ShortCryptQRCodeAlphanumeric::from_string(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for ShortCryptQRCodeAlphanumeric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {deserializer.deserialize_string(StringVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for ShortCryptQRCodeAlphanumeric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        serializer.serialize_str(&self.short_crypt_qr_code_alphanumeric)
    }
}