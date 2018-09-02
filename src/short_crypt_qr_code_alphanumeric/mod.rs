extern crate regex;

use self::regex::Regex;
use super::Validated;

use std::fmt::{self, Display, Debug, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum ShortCryptQRCodeAlphanumericComponentError {
    IncorrectFormat,
}

pub type ShortCryptQRCodeAlphanumericComponentResult = Result<ShortCryptQRCodeAlphanumericComponent, ShortCryptQRCodeAlphanumericComponentError>;

pub struct ShortCryptQRCodeAlphanumericComponentValidator {}

#[derive(Clone)]
pub struct ShortCryptQRCodeAlphanumericComponent {
    short_crypt_qr_code_alphanumeric_url: String,
}

impl ShortCryptQRCodeAlphanumericComponent {
    pub fn get_short_crypt_qr_code_alphanumeric_url(&self) -> &str {
        &self.short_crypt_qr_code_alphanumeric_url
    }
}

impl Validated for ShortCryptQRCodeAlphanumericComponent {}

impl Debug for ShortCryptQRCodeAlphanumericComponent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("ShortCryptQRCodeAlphanumericComponent({})", self.short_crypt_qr_code_alphanumeric_url))?;
        Ok(())
    }
}

impl Display for ShortCryptQRCodeAlphanumericComponent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.short_crypt_qr_code_alphanumeric_url)?;
        Ok(())
    }
}

impl PartialEq for ShortCryptQRCodeAlphanumericComponent {
    fn eq(&self, other: &Self) -> bool {
        self.short_crypt_qr_code_alphanumeric_url.eq(&other.short_crypt_qr_code_alphanumeric_url)
    }

    fn ne(&self, other: &Self) -> bool {
        self.short_crypt_qr_code_alphanumeric_url.ne(&other.short_crypt_qr_code_alphanumeric_url)
    }
}

impl ShortCryptQRCodeAlphanumericComponentValidator {
    pub fn is_short_crypt_qr_code_alphanumeric_url(&self, short_crypt_qr_code_alphanumeric_url: &str) -> bool {
        self.parse_inner(short_crypt_qr_code_alphanumeric_url).is_ok()
    }

    pub fn parse_string(&self, short_crypt_qr_code_alphanumeric_url: String) -> ShortCryptQRCodeAlphanumericComponentResult {
        let mut short_crypt_qr_code_alphanumeric_url_inner = self.parse_inner(&short_crypt_qr_code_alphanumeric_url)?;

        short_crypt_qr_code_alphanumeric_url_inner.short_crypt_qr_code_alphanumeric_url = short_crypt_qr_code_alphanumeric_url;

        Ok(short_crypt_qr_code_alphanumeric_url_inner)
    }

    pub fn parse_str(&self, short_crypt_qr_code_alphanumeric_url: &str) -> ShortCryptQRCodeAlphanumericComponentResult {
        let mut short_crypt_qr_code_alphanumeric_url_inner = self.parse_inner(short_crypt_qr_code_alphanumeric_url)?;

        short_crypt_qr_code_alphanumeric_url_inner.short_crypt_qr_code_alphanumeric_url = short_crypt_qr_code_alphanumeric_url.to_string();

        Ok(short_crypt_qr_code_alphanumeric_url_inner)
    }

    fn parse_inner(&self, short_crypt_qr_code_alphanumeric_url: &str) -> ShortCryptQRCodeAlphanumericComponentResult {
        let re = Regex::new(r"^([A-Z0-9]{8})*([A-Z0-9]|[A-Z0-9]{3}|[A-Z0-9]{5,6}|[A-Z0-9]{8})$").unwrap();

        if re.is_match(short_crypt_qr_code_alphanumeric_url) {
            Ok(ShortCryptQRCodeAlphanumericComponent {
                short_crypt_qr_code_alphanumeric_url: String::new(),
            })
        } else {
            Err(ShortCryptQRCodeAlphanumericComponentError::IncorrectFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_crypt_qr_code_alphanumeric_url_methods() {
        let short_crypt_qr_code_alphanumeric_url = "3BHNNR45XZH8PU".to_string();

        let scqacv = ShortCryptQRCodeAlphanumericComponentValidator {};

        let short_crypt_qr_code_alphanumeric_url = scqacv.parse_string(short_crypt_qr_code_alphanumeric_url).unwrap();

        assert_eq!("3BHNNR45XZH8PU", short_crypt_qr_code_alphanumeric_url.get_short_crypt_qr_code_alphanumeric_url());
    }

    #[test]
    fn test_short_crypt_qr_code_alphanumeric_url_lv1() {
        let short_crypt_qr_code_alphanumeric_url = "3BHNNR45XZH8PU".to_string();

        let scqacv = ShortCryptQRCodeAlphanumericComponentValidator {};

        scqacv.parse_string(short_crypt_qr_code_alphanumeric_url).unwrap();
    }
}

// ShortCryptQRCodeAlphanumericComponent's wrapper struct is itself
impl ShortCryptQRCodeAlphanumericComponent {
    pub fn from_string(short_crypt_qr_code_alphanumeric_url: String) -> Result<ShortCryptQRCodeAlphanumericComponent, ShortCryptQRCodeAlphanumericComponentError> {
        let bv = ShortCryptQRCodeAlphanumericComponentValidator {};

        bv.parse_string(short_crypt_qr_code_alphanumeric_url)
    }

    pub fn from_str(short_crypt_qr_code_alphanumeric_url: &str) -> Result<ShortCryptQRCodeAlphanumericComponent, ShortCryptQRCodeAlphanumericComponentError> {
        let bv = ShortCryptQRCodeAlphanumericComponentValidator {};

        bv.parse_str(short_crypt_qr_code_alphanumeric_url)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for ShortCryptQRCodeAlphanumericComponent {
    type Error = ShortCryptQRCodeAlphanumericComponentError;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        ShortCryptQRCodeAlphanumericComponent::from_str(form_value)
    }
}