extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::fmt::{self, Display, Debug, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum ShortCryptUrlComponentError {
    IncorrectFormat,
}

pub type ShortCryptUrlComponentResult = Result<ShortCryptUrlComponent, ShortCryptUrlComponentError>;

pub struct ShortCryptUrlComponentValidator {}

#[derive(Clone)]
pub struct ShortCryptUrlComponent {
    short_crypt_url_component_url: String,
}

impl ShortCryptUrlComponent {
    pub fn get_short_crypt_url_component_url(&self) -> &str {
        &self.short_crypt_url_component_url
    }
}

impl Validated for ShortCryptUrlComponent {}

impl Debug for ShortCryptUrlComponent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("ShortCryptUrlComponent({})", self.short_crypt_url_component_url))?;
        Ok(())
    }
}

impl Display for ShortCryptUrlComponent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.short_crypt_url_component_url)?;
        Ok(())
    }
}

impl PartialEq for ShortCryptUrlComponent {
    fn eq(&self, other: &Self) -> bool {
        self.short_crypt_url_component_url.eq(&other.short_crypt_url_component_url)
    }

    fn ne(&self, other: &Self) -> bool {
        self.short_crypt_url_component_url.ne(&other.short_crypt_url_component_url)
    }
}

impl ShortCryptUrlComponentValidator {
    pub fn is_short_crypt_url_component_url(&self, short_crypt_url_component_url: &str) -> bool {
        self.parse_inner(short_crypt_url_component_url).is_ok()
    }

    pub fn parse_string(&self, short_crypt_url_component_url: String) -> ShortCryptUrlComponentResult {
        let mut short_crypt_url_component_url_inner = self.parse_inner(&short_crypt_url_component_url)?;

        short_crypt_url_component_url_inner.short_crypt_url_component_url = short_crypt_url_component_url;

        Ok(short_crypt_url_component_url_inner)
    }

    pub fn parse_str(&self, short_crypt_url_component_url: &str) -> ShortCryptUrlComponentResult {
        let mut short_crypt_url_component_url_inner = self.parse_inner(short_crypt_url_component_url)?;

        short_crypt_url_component_url_inner.short_crypt_url_component_url = short_crypt_url_component_url.to_string();

        Ok(short_crypt_url_component_url_inner)
    }

    fn parse_inner(&self, short_crypt_url_component_url: &str) -> ShortCryptUrlComponentResult {
        let re = Regex::new(r"^([A-Za-z0-9\-_]{4})*([A-Za-z0-9\-_]|[A-Za-z0-9\-_]{3,4})$").unwrap();

        if re.is_match(short_crypt_url_component_url) {
            Ok(ShortCryptUrlComponent {
                short_crypt_url_component_url: String::new(),
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
    fn test_short_crypt_url_component_url_methods() {
        let short_crypt_url_component_url = "2E87Wx52-Tvo".to_string();

        let scuv = ShortCryptUrlComponentValidator {};

        let short_crypt_url_component_url = scuv.parse_string(short_crypt_url_component_url).unwrap();

        assert_eq!("2E87Wx52-Tvo", short_crypt_url_component_url.get_short_crypt_url_component_url());
    }

    #[test]
    fn test_short_crypt_url_component_url_lv1() {
        let short_crypt_url_component_url = "2E87Wx52-Tvo".to_string();

        let scuv = ShortCryptUrlComponentValidator {};

        scuv.parse_string(short_crypt_url_component_url).unwrap();
    }
}

// ShortCryptUrlComponent's wrapper struct is itself
impl ValidatedWrapper for ShortCryptUrlComponent {
    type Error = ShortCryptUrlComponentError;

    fn from_string(short_crypt_url_component_url: String) -> Result<Self, Self::Error> {
        ShortCryptUrlComponent::from_string(short_crypt_url_component_url)
    }

    fn from_str(short_crypt_url_component_url: &str) -> Result<Self, Self::Error> {
        ShortCryptUrlComponent::from_str(short_crypt_url_component_url)
    }
}

impl ShortCryptUrlComponent {
    pub fn from_string(short_crypt_url_component_url: String) -> Result<Self, ShortCryptUrlComponentError> {
        let bv = ShortCryptUrlComponentValidator {};

        bv.parse_string(short_crypt_url_component_url)
    }

    pub fn from_str(short_crypt_url_component_url: &str) -> Result<Self, ShortCryptUrlComponentError> {
        let bv = ShortCryptUrlComponentValidator {};

        bv.parse_str(short_crypt_url_component_url)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for ShortCryptUrlComponent {
    type Error = ShortCryptUrlComponentError;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        ShortCryptUrlComponent::from_str(form_value)
    }
}