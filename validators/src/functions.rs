#[cfg(feature = "std")]
use std::net::{AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr};
#[cfg(feature = "std")]
use std::str::{from_utf8_unchecked, FromStr};

#[cfg(feature = "std")]
#[inline]
pub fn is_local_ip(addr: IpAddr) -> bool {
    match addr {
        IpAddr::V4(addr) => is_local_ipv4(addr),
        IpAddr::V6(addr) => is_local_ipv6(addr),
    }
}

#[cfg(feature = "std")]
#[inline]
pub fn is_local_ipv4(addr: Ipv4Addr) -> bool {
    addr.is_private()
        || addr.is_loopback()
        || addr.is_link_local()
        || addr.is_broadcast()
        || addr.is_documentation()
        || addr.is_unspecified()
}

#[cfg(feature = "std")]
#[inline]
pub fn is_local_ipv6(addr: Ipv6Addr) -> bool {
    let segments = addr.segments();

    let first = segments[0];

    if (first & 0xFF00) == 0xFF00 {
        // is_multicast
        first & 0x000F != 14 // 14 is `std::net::Ipv6MulticastScope::Global`
    } else {
        if segments.starts_with(&[0, 0, 0, 0, 0, 0, 0]) {
            match segments[7] {
                0 | 1 => {
                    // is_loopback
                    // is_unspecified
                    return true;
                },
                _ => (),
            }
        }

        match first & 0xFFC0 {
            0xFE80 | 0xFEC0 => {
                // is_unicast_link_local
                // is_unicast_site_local
                return true;
            },
            _ => (),
        }

        if first & 0xFE00 == 0xFC00 || (first == 0x2001) && (segments[1] == 0xDB8) {
            // is_unique_local
            // is_documentation
            return true;
        }

        match addr.to_ipv4() {
            Some(addr) => is_local_ipv4(addr),
            None => false,
        }
    }
}

#[inline]
pub fn is_local_domain<S: AsRef<str>>(s: S) -> bool {
    let bytes = s.as_ref().as_bytes();

    debug_assert!(!bytes.is_empty());

    let length_dec = bytes.len() - 1;

    let bytes = if bytes[length_dec] == b'.' { &bytes[..length_dec] } else { bytes };

    bytes.eq_ignore_ascii_case(b"localhost")
}

#[inline]
pub fn is_at_least_two_labels_domain<S: AsRef<str>>(s: S) -> bool {
    let s = s.as_ref();

    debug_assert!(!s.is_empty());

    s
        .bytes()
        .take(s.len() - 1) // to avoid "."-ended domain
        .any(|e| e == b'.')
}

#[cfg(feature = "std")]
#[inline]
pub fn parse_ipv4_allow_an_ended_dot<S: AsRef<str>>(s: S) -> Result<Ipv4Addr, AddrParseError> {
    let s = s.as_ref();
    let bytes = s.as_bytes();

    debug_assert!(!bytes.is_empty());

    let length = bytes.len();

    let s = if length > 0 && bytes[length - 1] == b'.' {
        unsafe { from_utf8_unchecked(&bytes[..(length - 1)]) }
    } else {
        s
    };

    Ipv4Addr::from_str(s)
}
