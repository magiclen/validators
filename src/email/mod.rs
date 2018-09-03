extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::fmt::{self, Display, Debug, Formatter};
use std::str::Utf8Error;

use super::domain::{Domain, DomainUnlocalhostableWithoutPort, DomainError};

#[derive(Debug, PartialEq, Clone)]
pub enum EmailError {
    IncorrectLocalPart,
    IncorrectDomainPart(DomainError),
    UTF8Error(Utf8Error),
}

pub type EmailResult = Result<Email, EmailError>;

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
        if self.get_local().ne(other.get_local()) {
            return false;
        }

        self.get_domain().eq(other.get_domain())
    }

    fn ne(&self, other: &Self) -> bool {
        if self.get_local().ne(other.get_local()) {
            return true;
        }

        self.get_domain().ne(other.get_domain())
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

        email_inner.full_email = full_email.to_string();

        Ok(email_inner)
    }

    fn parse_inner(&self, full_email: &str) -> EmailResult {
        let re = Regex::new("^(([0-9A-Za-z!#$%&'*+-/=?^_`{|}~&&[^@]]+)|(\"([0-9A-Za-z!#$%&'*+-/=?^_`{|}~ \"(),:;<>@\\[\\\\\\]]+)\"))@").unwrap();

        let c = match re.captures(&full_email) {
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
                panic!("impossible");
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
        let ev = EmailValidator {};

        ev.parse_string(full_email)
    }

    pub fn from_str(full_email: &str) -> Result<Self, EmailError> {
        let ev = EmailValidator {};

        ev.parse_str(full_email)
    }
}


#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for Email {
    type Error = EmailError;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Email::from_string(form_value.url_decode().map_err(|err| EmailError::UTF8Error(err))?)
    }
}