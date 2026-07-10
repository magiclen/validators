use core::str::FromStr;
use std::net::{AddrParseError, Ipv4Addr};

/// Determine whether the input `Ipv4Addr` is local.
#[inline]
pub const fn is_local_ipv4(addr: Ipv4Addr) -> bool {
    addr.is_private()
        || addr.is_loopback()
        || addr.is_link_local()
        || addr.is_broadcast()
        || addr.is_documentation()
        || addr.is_unspecified()
}

/// Parse a string to `Ipv4Addr`, allowing an ended dot.
#[inline]
pub fn parse_ipv4_allow_an_ended_dot<S: AsRef<str>>(s: S) -> Result<Ipv4Addr, AddrParseError> {
    let s = s.as_ref();

    Ipv4Addr::from_str(s.strip_suffix('.').unwrap_or(s))
}
