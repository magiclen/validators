extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper, ValidatorOption};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::Utf8Error;

lazy_static! {
    static ref DOMAIN_RE: Regex = {
        Regex::new(r"^(([^\x00-\x20\x2E\x2F\x3A\x40\x7F-\xFF]{1,63}\.)*?)*?([^\x00-\x20\x2E\x2F\x3A\x40\x7F-\xFF]{1,63})(\.[^\x00-\x20\x2E\x2F\x3A\x40\x7F-\xFF]{1,63})?(:\d{1,5})?$").unwrap()
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum DomainError {
    IncorrectFormat,
    IncorrectPort,
    PortNotAllow,
    PortNotFound,
    LocalhostNotAllow,
    LocalhostNotFound,
    UTF8Error(Utf8Error),
}

impl Display for DomainError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for DomainError {}

impl From<Utf8Error> for DomainError {
    #[inline]
    fn from(err: Utf8Error) -> Self {
        DomainError::UTF8Error(err)
    }
}

pub type DomainResult = Result<Domain, DomainError>;

#[derive(Debug, PartialEq)]
pub struct DomainValidator {
    pub port: ValidatorOption,
    pub localhost: ValidatorOption,
}

#[derive(Clone)]
pub struct Domain {
    top_level_domain: usize,
    domain: usize,
    port: u16,
    port_index: usize,
    full_domain: String,
    full_domain_len: usize,
    is_localhost: bool,
}

impl Domain {
    #[inline]
    pub fn get_top_level_domain(&self) -> Option<&str> {
        if self.top_level_domain != self.full_domain_len {
            if self.port_index != self.full_domain_len {
                Some(&self.full_domain[self.top_level_domain..(self.port_index - 1)])
            } else {
                Some(&self.full_domain[self.top_level_domain..])
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn get_domain(&self) -> &str {
        if self.top_level_domain != self.full_domain_len {
            &self.full_domain[self.domain..(self.top_level_domain - 1)]
        } else if self.port_index != self.full_domain_len {
            &self.full_domain[self.domain..(self.port_index - 1)]
        } else {
            &self.full_domain[self.domain..]
        }
    }

    #[inline]
    pub fn get_sub_domain(&self) -> Option<&str> {
        if self.domain > 0 {
            Some(&self.full_domain[..(self.domain - 1)])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_full_domain(&self) -> &str {
        &self.full_domain
    }

    #[inline]
    pub fn get_full_domain_without_port(&self) -> &str {
        if self.port_index != self.full_domain_len {
            &self.full_domain[..(self.port_index - 1)]
        } else {
            &self.full_domain
        }
    }

    #[inline]
    pub fn get_port(&self) -> Option<u16> {
        if self.port_index != self.full_domain_len {
            Some(self.port)
        } else {
            None
        }
    }

    #[inline]
    pub fn is_localhost(&self) -> bool {
        self.is_localhost
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.full_domain
    }
}

impl Deref for Domain {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.full_domain
    }
}

impl Validated for Domain {}

impl Debug for Domain {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(Domain, f, self, let .0 = self.full_domain);
    }
}

impl Display for Domain {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.full_domain)?;
        Ok(())
    }
}

impl PartialEq for Domain {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.full_domain.eq(&other.full_domain)
    }
}

impl Eq for Domain {}

impl Hash for Domain {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_domain.hash(state);
    }
}

impl DomainValidator {
    #[inline]
    pub fn is_domain(&self, full_domain: &str) -> bool {
        self.parse_inner(full_domain).is_ok()
    }

    #[inline]
    pub fn parse_string(&self, full_domain: String) -> DomainResult {
        let mut domain_inner = self.parse_inner(&full_domain)?;

        domain_inner.full_domain = full_domain;

        Ok(domain_inner)
    }

    #[inline]
    pub fn parse_str(&self, full_domain: &str) -> DomainResult {
        let mut domain_inner = self.parse_inner(full_domain)?;

        domain_inner.full_domain.push_str(full_domain);

        Ok(domain_inner)
    }

    fn parse_inner(&self, full_domain: &str) -> DomainResult {
        let c = match DOMAIN_RE.captures(&full_domain) {
            Some(c) => c,
            None => return Err(DomainError::IncorrectFormat),
        };

        let full_domain_len = full_domain.len();

        let mut is_localhost;

        let domain = match c.get(3) {
            Some(m) => {
                let lowered_domain = m.as_str().to_lowercase();

                is_localhost = "localhost".eq(&lowered_domain);

                if self.localhost.must() && !is_localhost {
                    return Err(DomainError::LocalhostNotFound);
                }

                if m.end() > 255 {
                    return Err(DomainError::IncorrectFormat);
                }

                m.start()
            }
            None => {
                unreachable!();
            }
        };

        let top_level_domain = match c.get(4) {
            Some(m) => {
                if is_localhost {
                    if self.localhost.must() {
                        return Err(DomainError::LocalhostNotFound);
                    } else {
                        is_localhost = false;
                    }
                }

                if m.end() > 255 {
                    return Err(DomainError::IncorrectFormat);
                }

                m.start() + 1
            }
            None => {
                if is_localhost {
                    if self.localhost.not_allow() {
                        return Err(DomainError::LocalhostNotAllow);
                    }
                } else {
                    return Err(DomainError::IncorrectFormat);
                }

                full_domain_len
            }
        };

        if c.get(1).is_some() && is_localhost {
            return Err(DomainError::LocalhostNotFound);
        }

        let mut port = 0u16;

        let port_index = match c.get(5) {
            Some(m) => {
                if self.port.not_allow() {
                    return Err(DomainError::PortNotAllow);
                }

                let index = m.start() + 1;

                port = match full_domain[index..m.end()].parse::<u16>() {
                    Ok(p) => p,
                    Err(_) => return Err(DomainError::IncorrectPort),
                };

                index
            }
            None => {
                if self.port.must() {
                    return Err(DomainError::PortNotFound);
                }
                full_domain_len
            }
        };

        Ok(Domain {
            top_level_domain,
            domain,
            port,
            port_index,
            full_domain: String::new(),
            full_domain_len,
            is_localhost,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_methods_lv1() {
        let domain = "tool.magiclen.org:8080".to_string();

        let dv = DomainValidator {
            port: ValidatorOption::Allow,
            localhost: ValidatorOption::NotAllow,
        };

        let domain = dv.parse_string(domain).unwrap();

        assert_eq!("tool.magiclen.org:8080", domain.get_full_domain());
        assert_eq!("tool.magiclen.org", domain.get_full_domain_without_port());
        assert_eq!("org", domain.get_top_level_domain().unwrap());
        assert_eq!("tool", domain.get_sub_domain().unwrap());
        assert_eq!("magiclen", domain.get_domain());
        assert_eq!(8080, domain.get_port().unwrap());
        assert_eq!(false, domain.is_localhost());
    }

    #[test]
    fn test_domain_methods_lv2() {
        let domain = "www.tool.magiclen.org:8080".to_string();

        let dv = DomainValidator {
            port: ValidatorOption::Allow,
            localhost: ValidatorOption::NotAllow,
        };

        let domain = dv.parse_string(domain).unwrap();

        assert_eq!("www.tool.magiclen.org:8080", domain.get_full_domain());
        assert_eq!("www.tool.magiclen.org", domain.get_full_domain_without_port());
        assert_eq!("org", domain.get_top_level_domain().unwrap());
        assert_eq!("www.tool", domain.get_sub_domain().unwrap());
        assert_eq!("magiclen", domain.get_domain());
        assert_eq!(8080, domain.get_port().unwrap());
        assert_eq!(false, domain.is_localhost());
    }

    #[test]
    fn test_domain_methods_lv3() {
        let domain = "c81223-759.www.tool.magiclen.org:8080".to_string();

        let dv = DomainValidator {
            port: ValidatorOption::Allow,
            localhost: ValidatorOption::NotAllow,
        };

        let domain = dv.parse_string(domain).unwrap();

        assert_eq!("c81223-759.www.tool.magiclen.org:8080", domain.get_full_domain());
        assert_eq!("c81223-759.www.tool.magiclen.org", domain.get_full_domain_without_port());
        assert_eq!("org", domain.get_top_level_domain().unwrap());
        assert_eq!("c81223-759.www.tool", domain.get_sub_domain().unwrap());
        assert_eq!("magiclen", domain.get_domain());
        assert_eq!(8080, domain.get_port().unwrap());
        assert_eq!(false, domain.is_localhost());
    }

    #[test]
    fn test_domain_lv1() {
        let domain = "magiclen.org".to_string();

        let dv = DomainValidator {
            port: ValidatorOption::NotAllow,
            localhost: ValidatorOption::NotAllow,
        };

        dv.parse_string(domain).unwrap();
    }

    #[test]
    fn test_domain_lv2() {
        let domain = "magiclen.org:8080".to_string();

        let dv = DomainValidator {
            port: ValidatorOption::Allow,
            localhost: ValidatorOption::NotAllow,
        };

        dv.parse_string(domain).unwrap();
    }

    #[test]
    fn test_domain_lv3() {
        let domain = "tool.magiclen.org".to_string();

        let dv = DomainValidator {
            port: ValidatorOption::NotAllow,
            localhost: ValidatorOption::NotAllow,
        };

        dv.parse_string(domain).unwrap();
    }

    #[test]
    fn test_domain_lv4() {
        let domain = "tool.magiclen.org:8080".to_string();

        let dv = DomainValidator {
            port: ValidatorOption::Allow,
            localhost: ValidatorOption::NotAllow,
        };

        dv.parse_string(domain).unwrap();
    }

    #[test]
    fn test_local_host_lv1() {
        let domain = "localhost".to_string();

        let dv = DomainValidator {
            port: ValidatorOption::NotAllow,
            localhost: ValidatorOption::Allow,
        };

        dv.parse_string(domain).unwrap();
    }

    #[test]
    fn test_local_host_lv2() {
        let domain = "localhost:8080".to_string();

        let dv = DomainValidator {
            port: ValidatorOption::Allow,
            localhost: ValidatorOption::Allow,
        };

        dv.parse_string(domain).unwrap();
    }
}

// TODO ----------

macro_rules! extend {
    ($name:ident, $port:expr, $localhost:expr) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name(Domain);

        impl From<$name> for Domain {
            #[inline]
            fn from(d: $name) -> Self {
                d.0
            }
        }

        impl Deref for $name {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0.full_domain
            }
        }

        impl Validated for $name {}

        impl ValidatedWrapper for $name {
            type Error = DomainError;

            #[inline]
            fn from_string(full_domain: String) -> Result<Self, Self::Error> {
                $name::from_string(full_domain)
            }

            #[inline]
            fn from_str(full_domain: &str) -> Result<Self, Self::Error> {
                $name::from_str(full_domain)
            }
        }

        impl Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.0))?;
                Ok(())
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl $name {
            #[inline]
            pub fn from_string(full_domain: String) -> Result<$name, DomainError> {
                Ok($name($name::create_validator().parse_string(full_domain)?))
            }

            #[inline]
            #[allow(clippy::should_implement_trait)]
            pub fn from_str(full_domain: &str) -> Result<$name, DomainError> {
                Ok($name($name::create_validator().parse_str(full_domain)?))
            }

            #[inline]
            pub fn from_domain(domain: Domain) -> Result<$name, DomainError> {
                match $port {
                    ValidatorOption::Must => {
                        if domain.port_index == domain.full_domain_len {
                            return Err(DomainError::PortNotFound);
                        }
                    }
                    ValidatorOption::NotAllow => {
                        if domain.port_index == domain.full_domain_len {
                            return Err(DomainError::PortNotAllow);
                        }
                    }
                    _ => (),
                }
                match $localhost {
                    ValidatorOption::Must => {
                        if !domain.is_localhost {
                            return Err(DomainError::LocalhostNotFound);
                        }
                    }
                    ValidatorOption::NotAllow => {
                        if domain.is_localhost {
                            return Err(DomainError::LocalhostNotAllow);
                        }
                    }
                    _ => (),
                }

                Ok($name(domain))
            }

            #[inline]
            pub fn into_domain(self) -> Domain {
                self.0
            }

            #[inline]
            pub fn as_domain(&self) -> &Domain {
                &self.0
            }

            #[inline]
            fn create_validator() -> DomainValidator {
                DomainValidator {
                    port: $port,
                    localhost: $localhost,
                }
            }
        }

        impl $name {
            #[inline]
            pub fn get_top_level_domain(&self) -> Option<&str> {
                self.0.get_top_level_domain()
            }

            #[inline]
            pub fn get_domain(&self) -> &str {
                self.0.get_domain()
            }

            #[inline]
            pub fn get_sub_domain(&self) -> Option<&str> {
                self.0.get_sub_domain()
            }

            #[inline]
            pub fn get_full_domain(&self) -> &str {
                self.0.get_full_domain()
            }
        }

        impl std::str::FromStr for $name {
            type Err = DomainError;

            #[inline]
            fn from_str(s: &str) -> Result<$name, DomainError> {
                $name::from_str(s)
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromFormValue<'a> for $name {
            type Error = DomainError;

            #[inline]
            fn from_form_value(
                form_value: &'a ::rocket::http::RawStr,
            ) -> Result<Self, Self::Error> {
                $name::from_string(form_value.url_decode()?)
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromParam<'a> for $name {
            type Error = DomainError;

            #[inline]
            fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
                $name::from_string(param.url_decode()?)
            }
        }

        #[cfg(feature = "serdely")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>, {
                struct StringVisitor;

                impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
                    type Value = $name;

                    #[inline]
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_fmt(format_args!(
                            "a domain({:?}) string",
                            $name::create_validator()
                        ))
                    }

                    #[inline]
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error, {
                        $name::from_str(v).map_err(|err| E::custom(err.to_string()))
                    }

                    #[inline]
                    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error, {
                        $name::from_string(v).map_err(|err| E::custom(err.to_string()))
                    }
                }

                deserializer.deserialize_string(StringVisitor)
            }
        }

        #[cfg(feature = "serdely")]
        impl ::serde::Serialize for $name {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer, {
                serializer.serialize_str(self.get_full_domain())
            }
        }
    };
}

extend!(DomainLocalhostableWithPort, ValidatorOption::Must, ValidatorOption::Allow);

impl DomainLocalhostableWithPort {
    #[inline]
    pub fn get_full_domain_without_port(&self) -> &str {
        self.0.get_full_domain_without_port()
    }

    #[inline]
    pub fn get_port(&self) -> u16 {
        self.0.get_port().unwrap()
    }

    #[inline]
    pub fn is_localhost(&self) -> bool {
        self.0.is_localhost
    }
}

extend!(DomainLocalhostableAllowPort, ValidatorOption::Allow, ValidatorOption::Allow);

impl DomainLocalhostableAllowPort {
    #[inline]
    pub fn get_full_domain_without_port(&self) -> &str {
        self.0.get_full_domain_without_port()
    }

    #[inline]
    pub fn get_port(&self) -> Option<u16> {
        self.0.get_port()
    }

    #[inline]
    pub fn is_localhost(&self) -> bool {
        self.0.is_localhost
    }
}

extend!(DomainLocalhostableWithoutPort, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl DomainLocalhostableWithoutPort {
    #[inline]
    pub fn is_localhost(&self) -> bool {
        self.0.is_localhost
    }
}

extend!(DomainUnlocalhostableWithPort, ValidatorOption::Must, ValidatorOption::NotAllow);

impl DomainUnlocalhostableWithPort {
    #[inline]
    pub fn get_full_domain_without_port(&self) -> &str {
        self.0.get_full_domain_without_port()
    }

    #[inline]
    pub fn get_port(&self) -> u16 {
        self.0.get_port().unwrap()
    }
}

extend!(DomainUnlocalhostableAllowPort, ValidatorOption::Allow, ValidatorOption::NotAllow);

impl DomainUnlocalhostableAllowPort {
    #[inline]
    pub fn get_full_domain_without_port(&self) -> &str {
        self.0.get_full_domain_without_port()
    }

    #[inline]
    pub fn get_port(&self) -> Option<u16> {
        self.0.get_port()
    }
}

extend!(DomainUnlocalhostableWithoutPort, ValidatorOption::NotAllow, ValidatorOption::NotAllow);

impl DomainUnlocalhostableWithoutPort {}
