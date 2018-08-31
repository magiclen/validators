extern crate regex;

use super::Validated;

use std::fmt::{self, Display, Formatter};

use super::domain::{DomainValidator, DomainError, Domain};
use super::ipv4::{IPv4Validator, IPv4Error, IPv4};
use super::ipv6::{IPv6Validator, IPv6Error, IPv6};

#[derive(Debug)]
pub enum HostError {
    Domain(DomainError),
    IPv4(IPv4Error),
    IPv6(IPv6Error),
    NoValidator,
}

pub type HostResult = Result<Host, HostError>;

pub struct HostValidator {
    pub domain: Option<DomainValidator>,
    pub ipv4: Option<IPv4Validator>,
    pub ipv6: Option<IPv6Validator>,
}

#[derive(Clone)]
pub enum Host {
    Domain(Domain),
    IPv4(IPv4),
    IPv6(IPv6),
}

impl Host {
    pub fn get_full_host(&self) -> &str {
        match self {
            Host::Domain(d) => d.get_full_domain(),
            Host::IPv4(d) => d.get_full_ipv4(),
            Host::IPv6(d) => d.get_full_ipv6()
        }
    }

    pub fn get_full_host_without_port(&self) -> &str {
        match self {
            Host::Domain(d) => d.get_full_domain_without_port(),
            Host::IPv4(d) => d.get_full_ipv4_without_port(),
            Host::IPv6(d) => d.get_full_ipv6_without_port()
        }
    }
}

impl Validated for Host {}

impl Display for Host {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Host::Domain(d) => d.fmt(f),
            Host::IPv4(d) => d.fmt(f),
            Host::IPv6(d) => d.fmt(f),
        }
    }
}

impl PartialEq for Host {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Host::Domain(d) => {
                match other {
                    Host::Domain(dd) => d.eq(&dd),
                    _ => false,
                }
            }
            Host::IPv4(d) => {
                match other {
                    Host::IPv4(dd) => d.eq(&dd),
                    _ => false,
                }
            }
            Host::IPv6(d) => {
                match other {

                    Host::IPv6(dd) => d.eq(&dd),
                    _ => false,
                }
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        match self {
            Host::Domain(d) => {
                match other {
                    Host::Domain(dd) => d.ne(&dd),
                    _ => true,
                }
            }
            Host::IPv4(d) => {
                match other {
                    Host::IPv4(dd) => d.ne(&dd),
                    _ => true,
                }
            }
            Host::IPv6(d) => {
                match other {
                    Host::IPv6(dd) => d.ne(&dd),
                    _ => true,
                }
            }
        }
    }
}

impl HostValidator {
    pub fn is_domain(&self, full_domain: &str) -> bool {
        self.parse_inner(full_domain).is_ok()
    }

    pub fn parse_string(&self, full_host: String) -> HostResult {
        self.parse_inner(&full_host)
    }

    pub fn parse_str(&self, full_host: &str) -> HostResult {
        self.parse_inner(full_host)
    }

    fn parse_inner(&self, full_host: &str) -> HostResult {
        let mut err = Err(HostError::NoValidator);

        if let Some(ref v) = self.domain {
            match v.parse_str(full_host) {
                Ok(r) => return Ok(Host::Domain(r)),
                Err(e) => {
                    err = Err(HostError::Domain(e));
                }
            }
        }
        if let Some(ref v) = self.ipv4 {
            match v.parse_str(full_host) {
                Ok(r) => return Ok(Host::IPv4(r)),
                Err(e) => {
                    err = Err(HostError::IPv4(e));
                }
            }
        }
        if let Some(ref v) = self.ipv6 {
            match v.parse_str(full_host) {
                Ok(r) => return Ok(Host::IPv6(r)),
                Err(e) => {
                    err = Err(HostError::IPv6(e));
                }
            }
        }

        err
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::ValidatorOption;

    #[test]
    fn test_ipv4_lv1() {
        let domain = "168.17.212.1".to_string();

        let iv = IPv4Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::NotAllow,
            ipv6: ValidatorOption::NotAllow,
        };

        let hv = HostValidator{
            domain: None,
            ipv4: Some(iv),
            ipv6: None
        };

        hv.parse_string(domain).unwrap();
    }
}