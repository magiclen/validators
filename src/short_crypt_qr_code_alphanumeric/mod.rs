extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

lazy_static! {
    static ref SHORT_CRYPT_QR_CODE_ALPHANUMERIC_RE: Regex = {
        Regex::new(r"^([A-Z0-9]{8})*([A-Z0-9]|[A-Z0-9]{3}|[A-Z0-9]{5,6}|[A-Z0-9]{8})$").unwrap()
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum ShortCryptQRCodeAlphanumericError {
    IncorrectFormat,
}

impl Display for ShortCryptQRCodeAlphanumericError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ShortCryptQRCodeAlphanumericError {}

pub type ShortCryptQRCodeAlphanumericResult =
    Result<ShortCryptQRCodeAlphanumeric, ShortCryptQRCodeAlphanumericError>;

#[derive(Debug, PartialEq)]
pub struct ShortCryptQRCodeAlphanumericValidator {}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ShortCryptQRCodeAlphanumeric {
    short_crypt_qr_code_alphanumeric: String,
}

impl ShortCryptQRCodeAlphanumeric {
    #[inline]
    pub fn get_short_crypt_qr_code_alphanumeric_url(&self) -> &str {
        &self.short_crypt_qr_code_alphanumeric
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.short_crypt_qr_code_alphanumeric
    }

    #[inline]
    pub unsafe fn from_string_unchecked(
        short_crypt_qr_code_alphanumeric: String,
    ) -> ShortCryptQRCodeAlphanumeric {
        ShortCryptQRCodeAlphanumeric {
            short_crypt_qr_code_alphanumeric,
        }
    }
}

impl Deref for ShortCryptQRCodeAlphanumeric {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.short_crypt_qr_code_alphanumeric
    }
}

impl Validated for ShortCryptQRCodeAlphanumeric {}

impl Debug for ShortCryptQRCodeAlphanumeric {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(ShortCryptQRCodeAlphanumeric, f, self, let .0 = self.short_crypt_qr_code_alphanumeric);
    }
}

impl Display for ShortCryptQRCodeAlphanumeric {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.short_crypt_qr_code_alphanumeric)?;
        Ok(())
    }
}

impl ShortCryptQRCodeAlphanumericValidator {
    #[inline]
    pub fn is_short_crypt_qr_code_alphanumeric_url(
        &self,
        short_crypt_qr_code_alphanumeric_url: &str,
    ) -> bool {
        self.parse_inner(short_crypt_qr_code_alphanumeric_url).is_ok()
    }

    #[inline]
    pub fn parse_string(
        &self,
        short_crypt_qr_code_alphanumeric_url: String,
    ) -> ShortCryptQRCodeAlphanumericResult {
        let mut short_crypt_qr_code_alphanumeric_url_inner =
            self.parse_inner(&short_crypt_qr_code_alphanumeric_url)?;

        short_crypt_qr_code_alphanumeric_url_inner.short_crypt_qr_code_alphanumeric =
            short_crypt_qr_code_alphanumeric_url;

        Ok(short_crypt_qr_code_alphanumeric_url_inner)
    }

    #[inline]
    pub fn parse_str(
        &self,
        short_crypt_qr_code_alphanumeric_url: &str,
    ) -> ShortCryptQRCodeAlphanumericResult {
        let mut short_crypt_qr_code_alphanumeric_url_inner =
            self.parse_inner(short_crypt_qr_code_alphanumeric_url)?;

        short_crypt_qr_code_alphanumeric_url_inner
            .short_crypt_qr_code_alphanumeric
            .push_str(short_crypt_qr_code_alphanumeric_url);

        Ok(short_crypt_qr_code_alphanumeric_url_inner)
    }

    #[inline]
    fn parse_inner(
        &self,
        short_crypt_qr_code_alphanumeric_url: &str,
    ) -> ShortCryptQRCodeAlphanumericResult {
        if SHORT_CRYPT_QR_CODE_ALPHANUMERIC_RE.is_match(short_crypt_qr_code_alphanumeric_url) {
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

        let short_crypt_qr_code_alphanumeric_url =
            scqacv.parse_string(short_crypt_qr_code_alphanumeric_url).unwrap();

        assert_eq!(
            "3BHNNR45XZH8PU",
            short_crypt_qr_code_alphanumeric_url.get_short_crypt_qr_code_alphanumeric_url()
        );
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

    #[inline]
    fn from_string(short_crypt_qr_code_alphanumeric_url: String) -> Result<Self, Self::Error> {
        ShortCryptQRCodeAlphanumeric::from_string(short_crypt_qr_code_alphanumeric_url)
    }

    #[inline]
    fn from_str(short_crypt_qr_code_alphanumeric_url: &str) -> Result<Self, Self::Error> {
        ShortCryptQRCodeAlphanumeric::from_str(short_crypt_qr_code_alphanumeric_url)
    }
}

impl ShortCryptQRCodeAlphanumeric {
    #[inline]
    pub fn from_string(
        short_crypt_qr_code_alphanumeric_url: String,
    ) -> Result<Self, ShortCryptQRCodeAlphanumericError> {
        ShortCryptQRCodeAlphanumeric::create_validator()
            .parse_string(short_crypt_qr_code_alphanumeric_url)
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(
        short_crypt_qr_code_alphanumeric_url: &str,
    ) -> Result<Self, ShortCryptQRCodeAlphanumericError> {
        ShortCryptQRCodeAlphanumeric::create_validator()
            .parse_str(short_crypt_qr_code_alphanumeric_url)
    }

    #[inline]
    fn create_validator() -> ShortCryptQRCodeAlphanumericValidator {
        ShortCryptQRCodeAlphanumericValidator {}
    }
}

impl FromStr for ShortCryptQRCodeAlphanumeric {
    type Err = ShortCryptQRCodeAlphanumericError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ShortCryptQRCodeAlphanumeric::from_str(s)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for ShortCryptQRCodeAlphanumeric {
    type Error = ShortCryptQRCodeAlphanumericError;

    #[inline]
    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        ShortCryptQRCodeAlphanumeric::from_str(form_value)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for ShortCryptQRCodeAlphanumeric {
    type Error = ShortCryptQRCodeAlphanumericError;

    #[inline]
    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        ShortCryptQRCodeAlphanumeric::from_str(param)
    }
}

#[cfg(feature = "serdely")]
struct StringVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = ShortCryptQRCodeAlphanumeric;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a ShortCrypt QR code alphanumeric string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        ShortCryptQRCodeAlphanumeric::from_str(v).map_err(|err| E::custom(err.to_string()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        ShortCryptQRCodeAlphanumeric::from_string(v).map_err(|err| E::custom(err.to_string()))
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for ShortCryptQRCodeAlphanumeric {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>, {
        deserializer.deserialize_string(StringVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for ShortCryptQRCodeAlphanumeric {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer, {
        serializer.serialize_str(&self.short_crypt_qr_code_alphanumeric)
    }
}
