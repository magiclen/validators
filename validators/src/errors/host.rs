use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum HostError {
    /// the fallback variant
    Invalid,
    /// may not be valid but it is guaranteed that the domain part is not local
    LocalMust,
    /// may not be valid but it is guaranteed that the domain part is local
    LocalNotAllow,
    /// may not be valid but it is guaranteed that the domain part has only one label
    AtLeastTwoLabelsMust,
    /// may not be valid but it is guaranteed that the domain part has at least two labels
    AtLeastTwoLabelsNotAllow,
    /// may not be valid but missing a port is guaranteed
    PortMust,
    /// may not be valid and the port part seems to exist
    PortNotAllow,
}

impl Display for HostError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            HostError::Invalid => f.write_str("invalid domain or IP"),
            HostError::LocalMust => f.write_str("must be local"),
            HostError::LocalNotAllow => f.write_str("must not be local"),
            HostError::AtLeastTwoLabelsMust => f.write_str("must have at least two labels"),
            HostError::AtLeastTwoLabelsNotAllow => f.write_str("must have only one label"),
            HostError::PortMust => f.write_str("port not found"),
            HostError::PortNotAllow => f.write_str("port not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for HostError {}
