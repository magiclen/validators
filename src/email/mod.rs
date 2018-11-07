extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};
use std::str::Utf8Error;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use super::domain::{Domain, DomainUnlocalhostableWithoutPort, DomainError};

lazy_static! {
    static ref EMAIL_RE: Regex = {
        Regex::new("^(([0-9A-Za-z!#$%&'*+-/=?^_`{|}~&&[^@]]+)|(\"([0-9A-Za-z!#$%&'*+-/=?^_`{|}~ \"(),:;<>@\\[\\\\\\]]+)\"))@").unwrap()
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum EmailError {
    IncorrectLocalPart,
    IncorrectDomainPart(DomainError),
    UTF8Error(Utf8Error),
}

impl Display for EmailError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for EmailError {}

pub type EmailResult = Result<Email, EmailError>;

#[derive(Debug, PartialEq)]
pub struct EmailValidator {}

#[derive(Clone)]
pub struct Email {
    domain: Domain,
    domain_index: usize,
    full_email: String,
}

impl Email {
    pub fn get_local(&self) -> &str {
        &self.full_email[..(self.domain_index - 1)]
    }
    pub fn get_domain(&self) -> &Domain {
        &self.domain
    }
    pub fn get_full_email(&self) -> &str {
        &self.full_email
    }

    pub fn into_string(self) -> String {
        self.full_email
    }
}

impl Deref for Email {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.full_email
    }
}

impl Validated for Email {}

impl Debug for Email {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Email({})", self.full_email))?;
        Ok(())
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.full_email)?;
        Ok(())
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.full_email.eq(&other.full_email)
    }

    fn ne(&self, other: &Self) -> bool {
        self.full_email.ne(&other.full_email)
    }
}

impl Eq for Email {}

impl Hash for Email {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_email.hash(state);
    }
}

impl EmailValidator {
    pub fn is_email(&self, full_email: &str) -> bool {
        self.parse_inner(full_email).is_ok()
    }

    pub fn parse_string(&self, full_email: String) -> EmailResult {
        let mut email_inner = self.parse_inner(&full_email)?;

        email_inner.full_email = full_email;

        Ok(email_inner)
    }

    pub fn parse_str(&self, full_email: &str) -> EmailResult {
        let mut email_inner = self.parse_inner(full_email)?;

        email_inner.full_email.push_str(full_email);

        Ok(email_inner)
    }

    fn parse_inner(&self, full_email: &str) -> EmailResult {
        let c = match EMAIL_RE.captures(&full_email) {
            Some(c) => c,
            None => return Err(EmailError::IncorrectLocalPart)
        };

        let domain_index;

        match c.get(1) {
            Some(m) => {
                domain_index = m.end() + 1;

                m.start()
            }
            None => {
                unreachable!();
            }
        };

        let duwnp = match DomainUnlocalhostableWithoutPort::from_str(&full_email[domain_index..]) {
            Ok(d) => d,
            Err(err) => return Err(EmailError::IncorrectDomainPart(err))
        };

        let domain = duwnp.into_domain();

        Ok(Email {
            domain,
            domain_index,
            full_email: String::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_methods() {
        let email = "len@magiclen.org".to_string();

        let ev = EmailValidator {};

        let email = ev.parse_string(email).unwrap();

        assert_eq!("len@magiclen.org", email.get_full_email());
        assert_eq!("len", email.get_local());
        assert_eq!("magiclen.org", email.get_domain().get_full_domain());
    }

    #[test]
    fn test_email_lv1() {
        let email = "len@magiclen.org".to_string();

        let ev = EmailValidator {};

        ev.parse_string(email).unwrap();
    }
}

// Email's wrapper struct is itself
impl ValidatedWrapper for Email {
    type Error = EmailError;

    fn from_string(email: String) -> Result<Self, Self::Error> {
        Email::from_string(email)
    }

    fn from_str(email: &str) -> Result<Self, Self::Error> {
        Email::from_str(email)
    }
}

impl Email {
    pub fn from_string(full_email: String) -> Result<Self, EmailError> {
        Email::create_validator().parse_string(full_email)
    }

    pub fn from_str(full_email: &str) -> Result<Self, EmailError> {
        Email::create_validator().parse_str(full_email)
    }

    fn create_validator() -> EmailValidator {
        EmailValidator {}
    }
}


#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for Email {
    type Error = EmailError;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Email::from_string(form_value.url_decode().map_err(|err| EmailError::UTF8Error(err))?)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for Email {
    type Error = EmailError;

    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Email::from_string(param.url_decode().map_err(|err| EmailError::UTF8Error(err))?)
    }
}

#[cfg(feature = "serdely")]
struct StringVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = Email;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an email string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
        Email::from_str(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: ::serde::de::Error {
        Email::from_string(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        deserializer.deserialize_string(StringVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        serializer.serialize_str(&self.full_email)
    }
}