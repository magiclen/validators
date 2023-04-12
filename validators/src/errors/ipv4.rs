use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum IPv4Error {
    /// the fallback variant
    Invalid,
    /// may not be valid but it is guaranteed that the IPv4 is not local
    LocalMust,
    /// may not be valid but it is guaranteed that the IPv4 is local
    LocalNotAllow,
    /// may not be valid but missing a port is guaranteed
    PortMust,
    /// may not be valid and the port part seems to exist
    PortNotAllow,
}

impl Display for IPv4Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            IPv4Error::Invalid => f.write_str("invalid domain or IPv4"),
            IPv4Error::LocalMust => f.write_str("must be local"),
            IPv4Error::LocalNotAllow => f.write_str("must not be local"),
            IPv4Error::PortMust => f.write_str("port not found"),
            IPv4Error::PortNotAllow => f.write_str("port not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for IPv4Error {}
