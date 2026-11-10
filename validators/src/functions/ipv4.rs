use core::str::FromStr;
use std::net::{AddrParseError, Ipv4Addr};

/// Determine whether the input `Ipv4Addr` is local.
///
/// An address is local if it is not globally reachable, such as a private, shared (CGNAT), loopback, link-local, documentation, benchmarking, reserved, or broadcast address.
#[inline]
pub const fn is_local_ipv4(addr: Ipv4Addr) -> bool {
    let [a, b, c, d] = addr.octets();

    addr.is_private()
        || addr.is_loopback()
        || addr.is_link_local()
        || addr.is_documentation()
        // 0.0.0.0/8 ("this network"), including the unspecified address
        || a == 0
        // 100.64.0.0/10 (shared address space, used by CGNAT)
        || (a == 100 && b & 0b1100_0000 == 0b0100_0000)
        // 192.0.0.0/24 (IETF protocol assignments), except 192.0.0.9 and 192.0.0.10 which are globally reachable
        || (a == 192 && b == 0 && c == 0 && d != 9 && d != 10)
        // 192.88.99.2/32 (6a44 relay anycast)
        || matches!([a, b, c, d], [192, 88, 99, 2])
        // 198.18.0.0/15 (benchmarking)
        || (a == 198 && b & 0b1111_1110 == 18)
        // 240.0.0.0/4 (reserved), including the broadcast address
        || a & 0b1111_0000 == 0b1111_0000
}

/// Parse a string to `Ipv4Addr`, allowing an ended dot.
#[inline]
pub fn parse_ipv4_allow_an_ended_dot<S: AsRef<str>>(s: S) -> Result<Ipv4Addr, AddrParseError> {
    let s = s.as_ref();

    Ipv4Addr::from_str(s.strip_suffix('.').unwrap_or(s))
}
