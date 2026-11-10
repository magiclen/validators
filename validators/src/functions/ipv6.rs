use std::net::{Ipv4Addr, Ipv6Addr};

use crate::functions::is_local_ipv4;

/// Determine whether the input `Ipv6Addr` is local.
///
/// An address is local if it is not globally reachable, such as an unspecified, loopback, link-local, site-local, unique local, documentation, discard-only, dummy (`100:0:0:1::/64`), benchmarking, local-use translation (`64:ff9b:1::/48`), or SRv6 (`5f00::/16`) address, or a multicast address without the global scope.
/// An address that contains an IPv4 address (such as `::ffff:127.0.0.1` or `64:ff9b::7f00:1`) is checked by `is_local_ipv4`.
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

        match segments {
            // 100::/64 (discard-only)
            // 100:0:0:1::/64 (dummy prefix, RFC 9780)
            // 2001:2::/48 (benchmarking)
            // 64:ff9b:1::/48 (IPv4/IPv6 translation for local use)
            [0x100, 0, 0, 0 | 1, ..] | [0x2001, 2, 0, ..] | [0x64, 0xFF9B, 1, ..] => return true,
            // 3fff::/20 (documentation)
            [0x3FFF, second, ..] if second & 0xF000 == 0 => return true,
            // 5f00::/16 (SRv6 segment identifiers)
            [0x5F00, ..] => return true,
            // 64:ff9b::/96 (IPv4/IPv6 translation) keeps an IPv4 address in the last 32 bits.
            // Translators must drop it when that IPv4 address is not global, so check it like `::ffff:a.b.c.d`.
            [0x64, 0xFF9B, 0, 0, 0, 0, high, low] => {
                return is_local_ipv4(Ipv4Addr::from_bits(((high as u32) << 16) | low as u32));
            },
            _ => (),
        }

        match addr.to_ipv4() {
            Some(addr) => is_local_ipv4(addr),
            None => false,
        }
    }
}
