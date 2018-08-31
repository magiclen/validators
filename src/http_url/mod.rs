extern crate regex;

use self::regex::Regex;
use super::{ValidatorOption, Validated};

use std::fmt::{self, Display, Formatter};

use super::domain::{DomainValidator, DomainError};
use super::ipv4::{IPv4Validator, IPv4Error};
use super::ipv6::{IPv6Validator, IPv6Error};

#[derive(Debug)]
pub enum HttpUrlError {
    IncorrectFormat,
    IncorrectDomainFormat(DomainError),
    IncorrectIPv4Format(IPv4Error),
    IncorrectIPv6Format(IPv6Error),
    LocalNotAllow,
    LocalNotFound,
}

pub type HttpUrlResult = Result<HttpUrl, HttpUrlError>;

pub struct HttpUrlValidator {
    pub local: ValidatorOption,
}

#[derive(Clone)]
pub struct HttpUrl {
    protocol: usize,
    authority: usize,
    path: usize,
    query: DomainPort,
    fragment: usize,
    full_http_url: String,
    full_http_url_len: usize,
    is_https: bool,
}