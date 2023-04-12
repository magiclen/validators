use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum IPError {
    /// the fallback variant
    Invalid,
    /// may not be valid but it is guaranteed that the IP is not local
    LocalMust,
    /// may not be valid but it is guaranteed that the IP is local
    LocalNotAllow,
    /// may not be valid but missing a port is guaranteed
    PortMust,
    /// may not be valid and the port part seems to exist
    PortNotAllow,
}

impl Display for IPError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            IPError::Invalid => f.write_str("invalid domain or IP"),
            IPError::LocalMust => f.write_str("must be local"),
            IPError::LocalNotAllow => f.write_str("must not be local"),
            IPError::PortMust => f.write_str("port not found"),
            IPError::PortNotAllow => f.write_str("port not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for IPError {}
