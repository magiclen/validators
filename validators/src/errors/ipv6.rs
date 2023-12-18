use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `ipv6` validator.
#[derive(Debug, Clone)]
pub enum Ipv6Error {
    /// Incorrect IP data.
    Invalid,
    /// May not be valid, but it is guaranteed that the IP is not local.
    LocalMust,
    /// May not be valid, but it is guaranteed that the IP is local.
    LocalDisallow,
    /// May not be valid, but missing a port is guaranteed.
    PortMust,
    /// May not be valid, and the port part seems to exist.
    PortDisallow,
}

impl Display for Ipv6Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Invalid => f.write_str("invalid IPv6"),
            Self::LocalMust => f.write_str("must be local"),
            Self::LocalDisallow => f.write_str("must not be local"),
            Self::PortMust => f.write_str("port not found"),
            Self::PortDisallow => f.write_str("port not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for Ipv6Error {}
