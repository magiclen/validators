use alloc::string::String;

use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Host {
    /// non-fully qualified and ascii-encoded domain name
    Domain(String),
    IPv4(Ipv4Addr),
    IPv6(Ipv6Addr),
}
