extern crate regex;

use self::regex::Regex;
use super::{ValidatorOption, Validated};

use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum DomainError {
    IncorrectFormat,
    IncorrectPort,
    PortNotAllow,
    PortNotFound,
    LocalhostNotAllow,
    LocalhostNotFound,
}

pub type DomainResult = Result<Domain, DomainError>;

pub struct DomainValidator {
    pub port: ValidatorOption,
    pub localhost: ValidatorOption,
}

pub type DomainPort = u16;

#[derive(Clone)]
pub struct Domain {
    top_level_domain: usize,
    domain: usize,
    sub_domain: usize,
    port: DomainPort,
    port_index: usize,
    full_domain: String,
    full_domain_len: usize,
    is_localhost: bool,
}

impl Domain {
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

    pub fn get_domain(&self) -> &str {
        if self.top_level_domain != self.full_domain_len {
            &self.full_domain[self.domain..(self.top_level_domain - 1)]
        } else {
            if self.port_index != self.full_domain_len {
                &self.full_domain[self.domain..(self.port_index - 1)]
            } else {
                &self.full_domain[self.domain..]
            }
        }
    }

    pub fn get_sub_domain(&self) -> Option<&str> {
        if self.sub_domain != self.full_domain_len {
            if self.domain != self.full_domain_len {
                Some(&self.full_domain[self.sub_domain..(self.domain - 1)])
            } else {
                Some(&self.full_domain[self.sub_domain..])
            }
        } else {
            None
        }
    }

    pub fn get_full_domain(&self) -> &str {
        &self.full_domain
    }

    pub fn get_full_domain_without_port(&self) -> &str {
        if self.port_index != self.full_domain_len {
            &self.full_domain[..(self.port_index - 1)]
        } else {
            &self.full_domain
        }
    }

    pub fn get_port(&self) -> Option<DomainPort> {
        if self.port_index != self.full_domain_len {
            Some(self.port)
        } else {
            None
        }
    }

    pub fn is_localhost(&self) -> bool {
        self.is_localhost
    }
}

impl Validated for Domain {}

impl Display for Domain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.full_domain)?;
        Ok(())
    }
}

impl PartialEq for Domain {
    fn eq(&self, other: &Self) -> bool {
        if self.port != other.port || self.port_index != other.port_index {
            return false;
        }

        self.get_full_domain_without_port().to_lowercase().eq(&other.get_full_domain_without_port().to_lowercase())
    }

    fn ne(&self, other: &Self) -> bool {
        if self.port != other.port || self.port_index != other.port_index {
            return true;
        }

        self.get_full_domain_without_port().to_lowercase().ne(&other.get_full_domain_without_port().to_lowercase())
    }
}

impl DomainValidator {
    pub fn is_domain(&self, full_domain: &str) -> bool {
        self.parse_inner(full_domain).is_ok()
    }

    pub fn parse_string(&self, full_domain: String) -> DomainResult {
        let mut domain_inner = self.parse_inner(&full_domain)?;

        domain_inner.full_domain = full_domain;

        Ok(domain_inner)
    }

    pub fn parse_str(&self, full_domain: &str) -> DomainResult {
        let mut domain_inner = self.parse_inner(full_domain)?;

        domain_inner.full_domain = full_domain.to_string();

        Ok(domain_inner)
    }

    fn parse_inner(&self, full_domain: &str) -> DomainResult {
        let re = Regex::new(r"^([\S&&[^.:/]]{1,255})(\.([\S&&[^.:/]]{1,255}))?(\.([\S&&[^.:/]]{1,255}))?(:(\d{1,5}))?$").unwrap();

        let c = match re.captures(&full_domain) {
            Some(c) => c,
            None => return Err(DomainError::IncorrectFormat)
        };

        let full_domain_len = full_domain.len();

        let mut is_localhost = false;

        let mut sub_domain = full_domain_len;

        let mut domain = match c.get(1) {
            Some(m) => {
                m.start()
            }
            None => {
                panic!("impossible");
            }
        };

        match c.get(3) {
            Some(m) => {
                sub_domain = domain;
                domain = m.start();

                if domain - sub_domain > 64 {
                    return Err(DomainError::IncorrectFormat);
                }
            }
            None => ()
        };

        let mut port = 0u16;

        let port_index = match c.get(7) {
            Some(m) => {
                if self.port.not_allow() {
                    return Err(DomainError::PortNotAllow);
                }

                let index = m.start();

                port = match full_domain[index..m.end()].parse::<u16>() {
                    Ok(p) => p,
                    Err(_) => return Err(DomainError::IncorrectPort)
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

        let top_level_domain = match c.get(5) {
            Some(m) => {
                if m.end() > 255 {
                    return Err(DomainError::IncorrectFormat);
                }

                if self.localhost.must() {
                    return Err(DomainError::LocalhostNotFound);
                }

                m.start()
            }
            None => {
                if sub_domain == full_domain_len {
                    if self.localhost.allow() {
                        let domain_str = if port_index != full_domain_len {
                            &full_domain[domain..(port_index - 1)]
                        } else {
                            &full_domain[domain..]
                        };

                        let lowered_domain = domain_str.to_lowercase();

                        if "localhost".ne(&lowered_domain) {
                            return Err(DomainError::IncorrectFormat);
                        }

                        is_localhost = true;

                        full_domain_len
                    } else {
                        return Err(DomainError::IncorrectFormat);
                    }
                } else {
                    if self.localhost.must() {
                        return Err(DomainError::LocalhostNotFound);
                    }

                    let top_level_domain = domain;
                    domain = sub_domain;
                    sub_domain = full_domain_len;

                    top_level_domain
                }
            }
        };

        Ok(Domain {
            top_level_domain,
            domain,
            sub_domain,
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
    fn test_domain_methods() {
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
    ( $name:ident, $port:expr, $localhost:expr ) => {
        #[derive(Clone)]
        pub struct $name(Domain);

        impl From<$name> for Domain {
            fn from(d: $name) -> Self {
                d.0
            }
        }

        impl Validated for $name {}

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }

            fn ne(&self, other: &Self) -> bool {
                self.0.ne(&other.0)
            }
        }

        impl PartialEq<Domain> for $name {
            fn eq(&self, other: &Domain) -> bool {
                self.0.eq(&other)
            }

            fn ne(&self, other: &Domain) -> bool {
                self.0.ne(&other)
            }
        }

        impl $name {
            pub fn from_string(full_domain: String) -> Result<$name, DomainError> {
                let dc = DomainValidator {
                    port: $port,
                    localhost: $localhost,
                };

                Ok($name(dc.parse_string(full_domain)?))
            }

            pub fn from_str(full_domain: &str) -> Result<$name, DomainError> {
                let dc = DomainValidator {
                    port: $port,
                    localhost: $localhost,
                };

                Ok($name(dc.parse_str(full_domain)?))
            }

            pub fn from_domain(domain: Domain) -> Result<$name, DomainError> {
                 match $port {
                    ValidatorOption::Must => {
                        if domain.port_index == domain.full_domain_len {
                            return Err(DomainError::PortNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if domain.port_index == domain.full_domain_len {
                            return Err(DomainError::PortNotAllow)
                        }
                    }
                    _=>()
                }
                match $localhost {
                    ValidatorOption::Must => {
                        if !domain.is_localhost {
                            return Err(DomainError::LocalhostNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if domain.is_localhost {
                            return Err(DomainError::LocalhostNotAllow)
                        }
                    }
                    _=>()
                }

                Ok($name(domain))
            }

            pub fn into_domain(self) -> Domain {
                self.0
            }

            pub fn as_domain(&self) -> &Domain {
                &self.0
            }
        }

        impl $name {
            pub fn get_top_level_domain(&self) -> Option<&str> {
                self.0.get_top_level_domain()
            }

            pub fn get_domain(&self) -> &str {
                self.0.get_domain()
            }

            pub fn get_sub_domain(&self) -> Option<&str> {
                self.0.get_sub_domain()
            }

            pub fn get_full_domain(&self) -> &str {
                self.0.get_full_domain()
            }
        }
    };
}

extend!(DomainLocalhostableWithPort, ValidatorOption::Must, ValidatorOption::Allow);

impl DomainLocalhostableWithPort {
    pub fn get_full_domain_without_port(&self) -> &str {
        self.0.get_full_domain_without_port()
    }

    pub fn get_port(&self) -> DomainPort {
        self.0.get_port().unwrap()
    }

    pub fn is_localhost(&self) -> bool {
        self.0.is_localhost
    }
}

extend!(DomainLocalhostableAllowPort, ValidatorOption::Allow, ValidatorOption::Allow);

impl DomainLocalhostableAllowPort {
    pub fn get_full_domain_without_port(&self) -> &str {
        self.0.get_full_domain_without_port()
    }

    pub fn get_port(&self) -> Option<DomainPort> {
        self.0.get_port()
    }

    pub fn is_localhost(&self) -> bool {
        self.0.is_localhost
    }
}

extend!(DomainLocalhostableWithoutPort, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl DomainLocalhostableWithoutPort {
    pub fn is_localhost(&self) -> bool {
        self.0.is_localhost
    }
}

extend!(DomainUnlocalhostableWithPort, ValidatorOption::Must, ValidatorOption::NotAllow);

impl DomainUnlocalhostableWithPort {
    pub fn get_full_domain_without_port(&self) -> &str {
        self.0.get_full_domain_without_port()
    }

    pub fn get_port(&self) -> DomainPort {
        self.0.get_port().unwrap()
    }
}

extend!(DomainUnlocalhostableAllowPort, ValidatorOption::Allow, ValidatorOption::NotAllow);

impl DomainUnlocalhostableAllowPort {
    pub fn get_full_domain_without_port(&self) -> &str {
        self.0.get_full_domain_without_port()
    }

    pub fn get_port(&self) -> Option<DomainPort> {
        self.0.get_port()
    }
}

extend!(DomainUnlocalhostableWithoutPort, ValidatorOption::NotAllow, ValidatorOption::NotAllow);

impl DomainUnlocalhostableWithoutPort {}