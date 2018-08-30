extern crate regex;

use self::regex::Regex;
use super::{ValidatorOption, Validated};

use std::fmt::{self, Display, Formatter};

use super::domain::{DomainValidator, DomainError};

#[derive(Debug)]
pub enum EmailError {
    IncorrectLocalPart,
    IncorrectDomainPart(DomainError),
}

pub type EmailResult = Result<Email, EmailError>;

pub struct EmailValidator {}

pub struct Email {
    domain: usize,
    full_email: String,
}

impl Email {
    pub fn get_local(&self) -> &str {
        &self.full_email[..(self.domain - 1)]
    }
    pub fn get_domain(&self) -> &str {
        &self.full_email[self.domain..]
    }
    pub fn get_full_email(&self) -> &str {
        &self.full_email
    }
}

impl Validated for Email{}

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

        let domain;

        match c.get(1) {
            Some(m) => {
                domain = m.end() + 1;

                m.start()
            }
            None => {
                panic!("impossible");
            }
        };

        let dv = DomainValidator {
            port: ValidatorOption::NotAllow,
            localhost: ValidatorOption::NotAllow,
        };

        match dv.parse_str(&full_email[domain..]) {
            Ok(_) => (),
            Err(err) => return Err(EmailError::IncorrectDomainPart(err))
        }

        Ok(Email {
            domain,
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
        assert_eq!("magiclen.org", email.get_domain());
    }

    #[test]
    fn test_email_lv1() {
        let email = "len@magiclen.org".to_string();

        let ev = EmailValidator {};

        ev.parse_string(email).unwrap();
    }
}
