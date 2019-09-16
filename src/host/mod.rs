use super::{Validated, ValidatedWrapper, ValidatorOption};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::str::Utf8Error;

use super::domain::{Domain, DomainError, DomainValidator};
use super::ipv4::{IPv4, IPv4Error, IPv4Validator};
use super::ipv6::{IPv6, IPv6Error, IPv6Validator};

#[derive(Debug, PartialEq, Clone)]
pub enum HostError {
    Domain(DomainError),
    IPv4(IPv4Error),
    IPv6(IPv6Error),
    NoValidator,
    UTF8Error(Utf8Error),
}

impl Display for HostError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for HostError {}

impl From<DomainError> for HostError {
    #[inline]
    fn from(err: DomainError) -> Self {
        HostError::Domain(err)
    }
}

impl From<IPv4Error> for HostError {
    #[inline]
    fn from(err: IPv4Error) -> Self {
        HostError::IPv4(err)
    }
}

impl From<IPv6Error> for HostError {
    #[inline]
    fn from(err: IPv6Error) -> Self {
        HostError::IPv6(err)
    }
}

impl From<Utf8Error> for HostError {
    #[inline]
    fn from(err: Utf8Error) -> Self {
        HostError::UTF8Error(err)
    }
}

pub type HostResult = Result<Host, HostError>;

#[derive(Debug, PartialEq)]
pub struct HostValidator {
    pub domain: Option<DomainValidator>,
    pub ipv4: Option<IPv4Validator>,
    pub ipv6: Option<IPv6Validator>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Host {
    Domain(Domain),
    IPv4(IPv4),
    IPv6(IPv6),
}

impl Host {
    #[inline]
    pub fn get_full_host(&self) -> &str {
        match self {
            Host::Domain(d) => d.get_full_domain(),
            Host::IPv4(d) => d.get_full_ipv4(),
            Host::IPv6(d) => d.get_full_ipv6(),
        }
    }

    #[inline]
    pub fn get_full_host_without_port(&self) -> &str {
        match self {
            Host::Domain(d) => d.get_full_domain_without_port(),
            Host::IPv4(d) => d.get_full_ipv4_without_port(),
            Host::IPv6(d) => d.get_full_ipv6_without_port(),
        }
    }

    #[inline]
    pub fn get_port(&self) -> Option<u16> {
        match self {
            Host::Domain(d) => d.get_port(),
            Host::IPv4(d) => d.get_port(),
            Host::IPv6(d) => d.get_port(),
        }
    }

    #[inline]
    pub fn is_local(&self) -> bool {
        match self {
            Host::Domain(d) => d.is_localhost(),
            Host::IPv4(d) => d.is_local(),
            Host::IPv6(d) => d.is_local(),
        }
    }

    #[inline]
    pub fn into_string(self) -> String {
        match self {
            Host::Domain(d) => d.into_string(),
            Host::IPv4(d) => d.into_string(),
            Host::IPv6(d) => d.into_string(),
        }
    }
}

impl Deref for Host {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Host::Domain(d) => d.get_full_domain(),
            Host::IPv4(d) => d.get_full_ipv4(),
            Host::IPv6(d) => d.get_full_ipv6(),
        }
    }
}

impl Validated for Host {}

impl Debug for Host {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_enum!(Host::{(Domain(v): (let .v = v)), (IPv4(v): (let .v = v)), (IPv6(v): (let .v = v))}, f, self);
    }
}

impl Display for Host {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Host::Domain(d) => f.write_str(d.get_domain()),
            Host::IPv4(d) => f.write_str(d.get_full_ipv4()),
            Host::IPv6(d) => f.write_str(d.get_full_ipv6()),
        }
    }
}

impl HostValidator {
    #[inline]
    pub fn is_host(&self, full_host: &str) -> bool {
        self.parse_inner(full_host).is_ok()
    }

    #[inline]
    pub fn parse_string(&self, full_host: String) -> HostResult {
        self.parse_inner(&full_host)
    }

    #[inline]
    pub fn parse_str(&self, full_host: &str) -> HostResult {
        self.parse_inner(full_host)
    }

    fn parse_inner(&self, full_host: &str) -> HostResult {
        let mut err = Err(HostError::NoValidator);

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
        if let Some(ref v) = self.domain {
            match v.parse_str(full_host) {
                Ok(r) => return Ok(Host::Domain(r)),
                Err(e) => {
                    err = Err(HostError::Domain(e));
                }
            }
        }

        err
    }
}

#[cfg(test)]
mod tests {
    use super::super::ValidatorOption;
    use super::*;

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
    ($name:ident, $local:expr) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name(Host);

        impl From<$name> for Host {
            #[inline]
            fn from(d: $name) -> Self {
                d.0
            }
        }

        impl Deref for $name {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                match &self.0 {
                    Host::Domain(d) => d.get_full_domain(),
                    Host::IPv4(d) => d.get_full_ipv4(),
                    Host::IPv6(d) => d.get_full_ipv6(),
                }
            }
        }

        impl Validated for $name {}

        impl ValidatedWrapper for $name {
            type Error = HostError;

            #[inline]
            fn from_string(host: String) -> Result<Self, Self::Error> {
                $name::from_string(host)
            }

            #[inline]
            fn from_str(host: &str) -> Result<Self, Self::Error> {
                $name::from_str(host)
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
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl $name {
            #[inline]
            pub fn from_string(host: String) -> Result<$name, HostError> {
                Ok($name($name::create_validator().parse_string(host)?))
            }

            #[inline]
            #[allow(clippy::should_implement_trait)]
            pub fn from_str(host: &str) -> Result<$name, HostError> {
                Ok($name($name::create_validator().parse_str(host)?))
            }

            pub fn from_host(host: Host) -> Result<$name, HostError> {
                {
                    match &host {
                        Host::Domain(h) => {
                            match $local {
                                ValidatorOption::Must => {
                                    if !h.is_localhost() {
                                        return Err(HostError::Domain(
                                            DomainError::LocalhostNotFound,
                                        ));
                                    }
                                }
                                ValidatorOption::NotAllow => {
                                    if h.is_localhost() {
                                        return Err(HostError::Domain(
                                            DomainError::LocalhostNotAllow,
                                        ));
                                    }
                                }
                                _ => (),
                            }
                        }
                        Host::IPv4(h) => {
                            match $local {
                                ValidatorOption::Must => {
                                    if !h.is_local() {
                                        return Err(HostError::IPv4(IPv4Error::LocalNotFound));
                                    }
                                }
                                ValidatorOption::NotAllow => {
                                    if h.is_local() {
                                        return Err(HostError::IPv4(IPv4Error::LocalNotAllow));
                                    }
                                }
                                _ => (),
                            }
                        }
                        Host::IPv6(h) => {
                            match $local {
                                ValidatorOption::Must => {
                                    if !h.is_local() {
                                        return Err(HostError::IPv6(IPv6Error::LocalNotFound));
                                    }
                                }
                                ValidatorOption::NotAllow => {
                                    if h.is_local() {
                                        return Err(HostError::IPv6(IPv6Error::LocalNotAllow));
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }

                Ok($name(host))
            }

            #[inline]
            pub fn into_host(self) -> Host {
                self.0
            }

            #[inline]
            pub fn as_host(&self) -> &Host {
                &self.0
            }

            #[inline]
            fn create_validator() -> HostValidator {
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

                HostValidator {
                    domain: Some(dv),
                    ipv4: Some(iv4),
                    ipv6: Some(iv6),
                }
            }
        }

        impl $name {
            #[inline]
            pub fn get_full_host(&self) -> &str {
                match &self.0 {
                    Host::Domain(d) => d.get_full_domain(),
                    Host::IPv4(d) => d.get_full_ipv4(),
                    Host::IPv6(d) => d.get_full_ipv6(),
                }
            }

            #[inline]
            pub fn get_full_host_without_port(&self) -> &str {
                match &self.0 {
                    Host::Domain(d) => d.get_full_domain_without_port(),
                    Host::IPv4(d) => d.get_full_ipv4_without_port(),
                    Host::IPv6(d) => d.get_full_ipv6_without_port(),
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = HostError;

            #[inline]
            fn from_str(s: &str) -> Result<$name, HostError> {
                $name::from_str(s)
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromFormValue<'a> for $name {
            type Error = HostError;

            #[inline]
            fn from_form_value(
                form_value: &'a ::rocket::http::RawStr,
            ) -> Result<Self, Self::Error> {
                $name::from_string(form_value.url_decode()?)
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromParam<'a> for $name {
            type Error = HostError;

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
                            "a host({:?}) string",
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
                serializer.serialize_str(self.get_full_host())
            }
        }
    };
}

extend!(HostLocalable, ValidatorOption::Allow);

impl HostLocalable {
    #[inline]
    pub fn is_local(&self) -> bool {
        match &self.0 {
            Host::Domain(d) => d.is_localhost(),
            Host::IPv4(d) => d.is_local(),
            Host::IPv6(d) => d.is_local(),
        }
    }

    #[inline]
    pub fn get_port(&self) -> Option<u16> {
        match &self.0 {
            Host::Domain(d) => d.get_port(),
            Host::IPv4(d) => d.get_port(),
            Host::IPv6(d) => d.get_port(),
        }
    }
}

extend!(HostUnlocalable, ValidatorOption::NotAllow);

impl HostUnlocalable {}
