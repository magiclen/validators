use std::net::Ipv6Addr;

use crate::functions::is_local_ipv4;

/// Determine whether the input `Ipv6Addr` is local.
#[inline]
pub const fn is_local_ipv6(addr: Ipv6Addr) -> bool {
    let segments = addr.segments();

    let first = segments[0];

    if (first & 0xFF00) == 0xFF00 {
        // is_multicast
        first & 0x000F != 14 // 14 is `std::net::Ipv6MulticastScope::Global`
    } else {
        match segments {
            [0, 0, 0, 0, 0, 0, 0, 0] | [0, 0, 0, 0, 0, 0, 0, 1] => {
                // is_loopback
                // is_unspecified
                return true;
            },
            _ => (),
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
