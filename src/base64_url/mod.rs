extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::fmt::{self, Display, Debug, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Base64UrlError {
    IncorrectFormat,
}

pub type Base64UrlResult = Result<Base64Url, Base64UrlError>;

pub struct Base64UrlValidator {}

#[derive(Clone)]
pub struct Base64Url {
    base64_url: String,
}

impl Base64Url {
    pub fn get_base64_url(&self) -> &str {
        &self.base64_url
    }
}

impl Validated for Base64Url {}

impl Debug for Base64Url {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Base64Url({})", self.base64_url))?;
        Ok(())
    }
}

impl Display for Base64Url {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.base64_url)?;
        Ok(())
    }
}

impl PartialEq for Base64Url {
    fn eq(&self, other: &Self) -> bool {
        self.base64_url.eq(&other.base64_url)
    }

    fn ne(&self, other: &Self) -> bool {
        self.base64_url.ne(&other.base64_url)
    }
}

impl Base64UrlValidator {
    pub fn is_base64_url(&self, base64_url: &str) -> bool {
        self.parse_inner(base64_url).is_ok()
    }

    pub fn parse_string(&self, base64_url: String) -> Base64UrlResult {
        let mut base64_url_inner = self.parse_inner(&base64_url)?;

        base64_url_inner.base64_url = base64_url;

        Ok(base64_url_inner)
    }

    pub fn parse_str(&self, base64_url: &str) -> Base64UrlResult {
        let mut base64_url_inner = self.parse_inner(base64_url)?;

        base64_url_inner.base64_url = base64_url.to_string();

        Ok(base64_url_inner)
    }

    fn parse_inner(&self, base64_url: &str) -> Base64UrlResult {
        let re = Regex::new(r"^([A-Za-z0-9\-_]{4})*[A-Za-z0-9\-_]{2,4}$").unwrap();

        if re.is_match(base64_url) {
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

    fn from_string(base64_url: String) -> Result<Self, Self::Error> {
        Base64Url::from_string(base64_url)
    }

    fn from_str(base64_url: &str) -> Result<Self, Self::Error> {
        Base64Url::from_str(base64_url)
    }
}

impl Base64Url {
    pub fn from_string(base64_url: String) -> Result<Self, Base64UrlError> {
        let bv = Base64UrlValidator {};

        bv.parse_string(base64_url)
    }

    pub fn from_str(base64_url: &str) -> Result<Self, Base64UrlError> {
        let bv = Base64UrlValidator {};

        bv.parse_str(base64_url)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for Base64Url {
    type Error = Base64UrlError;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Base64Url::from_str(form_value)
    }
}