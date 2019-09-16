extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

lazy_static! {
    static ref BASE64_URL_RE: Regex =
        { Regex::new(r"^([A-Za-z0-9\-_]{4})*[A-Za-z0-9\-_]{2,4}$").unwrap() };
}

#[derive(Debug, PartialEq, Clone)]
pub enum Base64UrlError {
    IncorrectFormat,
}

impl Display for Base64UrlError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for Base64UrlError {}

pub type Base64UrlResult = Result<Base64Url, Base64UrlError>;

#[derive(Debug, PartialEq)]
pub struct Base64UrlValidator {}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Base64Url {
    base64_url: String,
}

impl Base64Url {
    #[inline]
    pub fn get_base64_url(&self) -> &str {
        &self.base64_url
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.base64_url
    }

    #[inline]
    pub unsafe fn from_string_unchecked(base64_url: String) -> Base64Url {
        Base64Url {
            base64_url,
        }
    }
}

impl Deref for Base64Url {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.base64_url
    }
}

impl Validated for Base64Url {}

impl Debug for Base64Url {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(Base64Url, f, self, let .0 = self.base64_url);
    }
}

impl Display for Base64Url {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.base64_url)?;
        Ok(())
    }
}

impl Base64UrlValidator {
    #[inline]
    pub fn is_base64_url(&self, base64_url: &str) -> bool {
        self.parse_inner(base64_url).is_ok()
    }

    #[inline]
    pub fn parse_string(&self, base64_url: String) -> Base64UrlResult {
        let mut base64_url_inner = self.parse_inner(&base64_url)?;

        base64_url_inner.base64_url = base64_url;

        Ok(base64_url_inner)
    }

    #[inline]
    pub fn parse_str(&self, base64_url: &str) -> Base64UrlResult {
        let mut base64_url_inner = self.parse_inner(base64_url)?;

        base64_url_inner.base64_url.push_str(base64_url);

        Ok(base64_url_inner)
    }

    #[inline]
    fn parse_inner(&self, base64_url: &str) -> Base64UrlResult {
        if BASE64_URL_RE.is_match(base64_url) {
            Ok(Base64Url {
                base64_url: String::new(),
            })
        } else {
            Err(Base64UrlError::IncorrectFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_url_methods() {
        let base64_url = "YXJ0aWNsZXM".to_string();

        let bv = Base64UrlValidator {};

        let base64_url = bv.parse_string(base64_url).unwrap();

        assert_eq!("YXJ0aWNsZXM", base64_url.get_base64_url());
    }

    #[test]
    fn test_base64_url_lv1() {
        let base64_url = "YXJ0aWNsZXM".to_string();

        let bv = Base64UrlValidator {};

        bv.parse_string(base64_url).unwrap();
    }
}

// Base64Url's wrapper struct is itself
impl ValidatedWrapper for Base64Url {
    type Error = Base64UrlError;

    #[inline]
    fn from_string(base64_url: String) -> Result<Self, Self::Error> {
        Base64Url::from_string(base64_url)
    }

    #[inline]
    fn from_str(base64_url: &str) -> Result<Self, Self::Error> {
        Base64Url::from_str(base64_url)
    }
}

impl Base64Url {
    #[inline]
    pub fn from_string(base64_url: String) -> Result<Self, Base64UrlError> {
        Base64Url::create_validator().parse_string(base64_url)
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(base64_url: &str) -> Result<Self, Base64UrlError> {
        Base64Url::create_validator().parse_str(base64_url)
    }

    #[inline]
    fn create_validator() -> Base64UrlValidator {
        Base64UrlValidator {}
    }
}

impl FromStr for Base64Url {
    type Err = Base64UrlError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Base64Url::from_str(s)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for Base64Url {
    type Error = Base64UrlError;

    #[inline]
    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Base64Url::from_str(form_value)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for Base64Url {
    type Error = Base64UrlError;

    #[inline]
    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Base64Url::from_str(param)
    }
}

#[cfg(feature = "serdely")]
struct StringVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = Base64Url;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Base64-URL string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        Base64Url::from_str(v).map_err(|err| E::custom(err.to_string()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        Base64Url::from_string(v).map_err(|err| E::custom(err.to_string()))
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for Base64Url {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>, {
        deserializer.deserialize_string(StringVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for Base64Url {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer, {
        serializer.serialize_str(&self.base64_url)
    }
}
