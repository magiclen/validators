use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum IPv6Error {
    /// the fallback variant
    Invalid,
    /// may not be valid but it is guaranteed that the IPv6 is not local
    LocalMust,
    /// may not be valid but it is guaranteed that the IPv6 is local
    LocalNotAllow,
    /// may not be valid but missing a port is guaranteed
    PortMust,
    /// may not be valid and the port part seems to exist
    PortNotAllow,
}

impl Display for IPv6Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            IPv6Error::Invalid => f.write_str("invalid domain or IPv6"),
            IPv6Error::LocalMust => f.write_str("must be local"),
            IPv6Error::LocalNotAllow => f.write_str("must not be local"),
            IPv6Error::PortMust => f.write_str("port not found"),
            IPv6Error::PortNotAllow => f.write_str("port not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for IPv6Error {}
