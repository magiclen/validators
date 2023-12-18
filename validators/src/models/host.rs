use alloc::string::String;
use core::fmt::{self, Display, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};

/// Used for the `email` and `host` validators to differentiate the host name type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Host {
    Domain(String),
    IPv4(Ipv4Addr),
    IPv6(Ipv6Addr),
}

impl Display for Host {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Domain(s) => Display::fmt(s, f),
            Self::IPv4(ip) => Display::fmt(ip, f),
            Self::IPv6(ip) => Display::fmt(ip, f),
        }
    }
}

impl From<Ipv4Addr> for Host {
    #[inline]
    fn from(value: Ipv4Addr) -> Self {
        Self::IPv4(value)
    }
}

impl From<Ipv6Addr> for Host {
    #[inline]
    fn from(value: Ipv6Addr) -> Self {
        Self::IPv6(value)
    }
}
