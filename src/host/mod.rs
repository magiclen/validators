extern crate regex;

use super::{ValidatorOption, Validated};

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

    pub fn is_local(&self) -> bool {
        match self {
            Host::Domain(d) => d.is_localhost(),
            Host::IPv4(d) => d.is_local(),
            Host::IPv6(d) => d.is_local()
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
    pub fn is_host(&self, full_host: &str) -> bool {
        self.parse_inner(full_host).is_ok()
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
    fn test_host_methods() {
        let domain = "168.17.212.1:8080".to_string();

        let iv = IPv4Validator {
            port: ValidatorOption::Allow,
            local: ValidatorOption::NotAllow,
            ipv6: ValidatorOption::NotAllow,
        };

        let hv = HostValidator {
            domain: None,
            ipv4: Some(iv),
            ipv6: None,
        };

        let host = hv.parse_string(domain).unwrap();

        assert_eq!("168.17.212.1:8080", host.get_full_host());
        assert_eq!("168.17.212.1", host.get_full_host_without_port());
        assert_eq!(false, host.is_local());
    }

    #[test]
    fn test_host_lv1() {
        let domain = "168.17.212.1".to_string();

        let iv = IPv4Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::NotAllow,
            ipv6: ValidatorOption::NotAllow,
        };

        let hv = HostValidator {
            domain: None,
            ipv4: Some(iv),
            ipv6: None,
        };

        hv.parse_string(domain).unwrap();
    }
}

// TODO ----------

macro_rules! extend {
    ( $name:ident, $local:expr ) => {
        #[derive(Clone)]
        pub struct $name(Host);

        impl From<$name> for Host {
            fn from(d: $name) -> Self {
                d.0
            }
        }

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

        impl PartialEq<Host> for $name {
            fn eq(&self, other: &Host) -> bool {
                self.0.eq(&other)
            }

            fn ne(&self, other: &Host) -> bool {
                self.0.ne(&other)
            }
        }

        impl $name {
            pub fn from_string(host: String) -> Result<$name, HostError> {
                let dv = DomainValidator {
                    port: ValidatorOption::Allow,
                    localhost: $local,
                };

                let iv4 = IPv4Validator {
                    port: ValidatorOption::Allow,
                    local: $local,
                    ipv6: ValidatorOption::NotAllow,
                };

                let iv6 = IPv6Validator {
                    port: ValidatorOption::Allow,
                    local: $local,
                    ipv4: ValidatorOption::NotAllow,
                };

                let hv = HostValidator{
                    domain: Some(dv),
                    ipv4: Some(iv4),
                    ipv6: Some(iv6)
                };

                Ok($name(hv.parse_string(host)?))
            }

            pub fn from_str(host: &str) -> Result<$name, HostError> {
                let dv = DomainValidator {
                    port: ValidatorOption::Allow,
                    localhost: $local,
                };

                let iv4 = IPv4Validator {
                    port: ValidatorOption::Allow,
                    local: $local,
                    ipv6: ValidatorOption::NotAllow,
                };

                let iv6 = IPv6Validator {
                    port: ValidatorOption::Allow,
                    local: $local,
                    ipv4: ValidatorOption::NotAllow,
                };

                let hv = HostValidator{
                    domain: Some(dv),
                    ipv4: Some(iv4),
                    ipv6: Some(iv6)
                };

                Ok($name(hv.parse_str(host)?))
            }

            pub fn from_host(host: Host) -> Result<$name, HostError> {
                {
                    match &host {
                        Host::Domain(h)=>{
                            match $local {
                                ValidatorOption::Must => {
                                    if !h.is_localhost() {
                                        return Err(HostError::Domain(DomainError::LocalhostNotFound))
                                    }
                                },
                                ValidatorOption::NotAllow => {
                                    if h.is_localhost() {
                                        return Err(HostError::Domain(DomainError::LocalhostNotAllow))
                                    }
                                }
                                _=>()
                            }
                        }
                        Host::IPv4(h)=>{
                            match $local {
                                ValidatorOption::Must => {
                                    if !h.is_local() {
                                        return Err(HostError::IPv4(IPv4Error::LocalNotFound))
                                    }
                                },
                                ValidatorOption::NotAllow => {
                                    if h.is_local() {
                                        return Err(HostError::IPv4(IPv4Error::LocalNotAllow))
                                    }
                                }
                                _=>()
                            }
                        }
                        Host::IPv6(h)=>{
                            match $local {
                                ValidatorOption::Must => {
                                    if !h.is_local() {
                                        return Err(HostError::IPv6(IPv6Error::LocalNotFound))
                                    }
                                },
                                ValidatorOption::NotAllow => {
                                    if h.is_local() {
                                        return Err(HostError::IPv6(IPv6Error::LocalNotAllow))
                                    }
                                }
                                _=>()
                            }
                        }
                    }
                }

                Ok($name(host))
            }

            pub fn into_host(self) -> Host {
                self.0
            }

            pub fn as_host(&self) -> &Host {
                &self.0
            }
        }

        impl $name {
            pub fn get_full_host(&self) -> &str {
                match &self.0 {
                    Host::Domain(d) => d.get_full_domain(),
                    Host::IPv4(d) => d.get_full_ipv4(),
                    Host::IPv6(d) => d.get_full_ipv6()
                }
            }

            pub fn get_full_host_without_port(&self) -> &str {
                match &self.0 {
                    Host::Domain(d) => d.get_full_domain_without_port(),
                    Host::IPv4(d) => d.get_full_ipv4_without_port(),
                    Host::IPv6(d) => d.get_full_ipv6_without_port()
                }
            }
        }
    };
}

extend!(HostLocalable, ValidatorOption::Allow);

impl HostLocalable {
    pub fn is_local(&self) -> bool {
        match &self.0 {
            Host::Domain(d) => d.is_localhost(),
            Host::IPv4(d) => d.is_local(),
            Host::IPv6(d) => d.is_local()
        }
    }
}

extend!(HostUnlocalable, ValidatorOption::NotAllow);

impl HostUnlocalable {
}