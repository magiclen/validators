extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

lazy_static! {
    static ref SHORT_CRYPT_URL_COMPONENT_RE: Regex =
        Regex::new(r"^([A-Za-z0-9\-_]{4})*([A-Za-z0-9\-_]|[A-Za-z0-9\-_]{3,4})$").unwrap();
}

#[derive(Debug, PartialEq, Clone)]
pub enum ShortCryptUrlComponentError {
    IncorrectFormat,
}

impl Display for ShortCryptUrlComponentError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ShortCryptUrlComponentError {}

pub type ShortCryptUrlComponentResult = Result<ShortCryptUrlComponent, ShortCryptUrlComponentError>;

#[derive(Debug, PartialEq)]
pub struct ShortCryptUrlComponentValidator {}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ShortCryptUrlComponent {
    short_crypt_url_component: String,
}

impl ShortCryptUrlComponent {
    #[inline]
    pub fn get_short_crypt_url_component(&self) -> &str {
        &self.short_crypt_url_component
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.short_crypt_url_component
    }

    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn from_string_unchecked(
        short_crypt_url_component: String,
    ) -> ShortCryptUrlComponent {
        ShortCryptUrlComponent {
            short_crypt_url_component,
        }
    }
}

impl Deref for ShortCryptUrlComponent {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.short_crypt_url_component
    }
}

impl Validated for ShortCryptUrlComponent {}

impl Debug for ShortCryptUrlComponent {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(ShortCryptUrlComponent, f, self, let .0 = self.short_crypt_url_component);
    }
}

impl Display for ShortCryptUrlComponent {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.short_crypt_url_component)?;
        Ok(())
    }
}

impl ShortCryptUrlComponentValidator {
    #[inline]
    pub fn is_short_crypt_url_component(&self, short_crypt_url_component: &str) -> bool {
        self.parse_inner(short_crypt_url_component).is_ok()
    }

    #[inline]
    pub fn parse_string(&self, short_crypt_url_component: String) -> ShortCryptUrlComponentResult {
        let mut short_crypt_url_component_inner = self.parse_inner(&short_crypt_url_component)?;

        short_crypt_url_component_inner.short_crypt_url_component = short_crypt_url_component;

        Ok(short_crypt_url_component_inner)
    }

    #[inline]
    pub fn parse_str(&self, short_crypt_url_component: &str) -> ShortCryptUrlComponentResult {
        let mut short_crypt_url_component_inner = self.parse_inner(short_crypt_url_component)?;

        short_crypt_url_component_inner
            .short_crypt_url_component
            .push_str(short_crypt_url_component);

        Ok(short_crypt_url_component_inner)
    }

    #[inline]
    fn parse_inner(&self, short_crypt_url_component: &str) -> ShortCryptUrlComponentResult {
        if SHORT_CRYPT_URL_COMPONENT_RE.is_match(short_crypt_url_component) {
            Ok(ShortCryptUrlComponent {
                short_crypt_url_component: String::new(),
            })
        } else {
            Err(ShortCryptUrlComponentError::IncorrectFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_crypt_url_component_methods() {
        let short_crypt_url_component = "2E87Wx52-Tvo".to_string();

        let scuv = ShortCryptUrlComponentValidator {};

        let short_crypt_url_component = scuv.parse_string(short_crypt_url_component).unwrap();

        assert_eq!("2E87Wx52-Tvo", short_crypt_url_component.get_short_crypt_url_component());
    }

    #[test]
    fn test_short_crypt_url_component_lv1() {
        let short_crypt_url_component = "2E87Wx52-Tvo".to_string();

        let scuv = ShortCryptUrlComponentValidator {};

        scuv.parse_string(short_crypt_url_component).unwrap();
    }
}

// ShortCryptUrlComponent's wrapper struct is itself
impl ValidatedWrapper for ShortCryptUrlComponent {
    type Error = ShortCryptUrlComponentError;

    #[inline]
    fn from_string(short_crypt_url_component: String) -> Result<Self, Self::Error> {
        ShortCryptUrlComponent::from_string(short_crypt_url_component)
    }

    #[inline]
    fn from_str(short_crypt_url_component: &str) -> Result<Self, Self::Error> {
        ShortCryptUrlComponent::from_str(short_crypt_url_component)
    }
}

impl ShortCryptUrlComponent {
    #[inline]
    pub fn from_string(
        short_crypt_url_component: String,
    ) -> Result<Self, ShortCryptUrlComponentError> {
        ShortCryptUrlComponent::create_validator().parse_string(short_crypt_url_component)
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(short_crypt_url_component: &str) -> Result<Self, ShortCryptUrlComponentError> {
        ShortCryptUrlComponent::create_validator().parse_str(short_crypt_url_component)
    }

    #[inline]
    fn create_validator() -> ShortCryptUrlComponentValidator {
        ShortCryptUrlComponentValidator {}
    }
}

impl FromStr for ShortCryptUrlComponent {
    type Err = ShortCryptUrlComponentError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ShortCryptUrlComponent::from_str(s)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for ShortCryptUrlComponent {
    type Error = ShortCryptUrlComponentError;

    #[inline]
    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        ShortCryptUrlComponent::from_str(form_value)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for ShortCryptUrlComponent {
    type Error = ShortCryptUrlComponentError;

    #[inline]
    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        ShortCryptUrlComponent::from_str(param)
    }
}

#[cfg(feature = "serdely")]
struct StringVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = ShortCryptUrlComponent;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a ShortCrypt URL component string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        ShortCryptUrlComponent::from_str(v).map_err(|err| E::custom(err.to_string()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        ShortCryptUrlComponent::from_string(v).map_err(|err| E::custom(err.to_string()))
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for ShortCryptUrlComponent {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>, {
        deserializer.deserialize_string(StringVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for ShortCryptUrlComponent {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer, {
        serializer.serialize_str(&self.short_crypt_url_component)
    }
}
