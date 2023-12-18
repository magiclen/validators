use std::net::IpAddr;

use crate::functions::{is_local_ipv4, is_local_ipv6};

/// Determine whether the input `IpAddr` is local.
#[inline]
pub const fn is_local_ip(addr: IpAddr) -> bool {
    match addr {
        IpAddr::V4(addr) => is_local_ipv4(addr),
        IpAddr::V6(addr) => is_local_ipv6(addr),
    }
}
