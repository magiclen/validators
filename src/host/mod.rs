extern crate regex;

use self::regex::Regex;
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
    pub ipv6: Option<IPv4Validator>,
}

#[derive(Clone)]
pub enum Host {
    Domain(Domain),
    IPv4(IPv4),
    IPv6(IPv6),
}

impl Host {
    pub fn get_full_host(&self) -> String {
        match self {
            Host::Domain(d) => d.get_full_domain().to_string(),
            Host::IPv4(d) => d.get_ipv4_address().to_string(),
            Host::IPv6(d) => d.get_ipv6_address().to_string()
        }
    }
}