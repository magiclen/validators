use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum DomainError {
    /// the fallback variant
    Invalid,
    /// may not be valid but it is guaranteed that the domain part is not an IPv4
    IPv4Must,
    /// may not be valid but it is guaranteed that the domain part is an IPv4
    IPv4NotAllow,
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

impl Display for DomainError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            DomainError::Invalid => f.write_str("invalid domain"),
            DomainError::IPv4Must => f.write_str("must use an IPv4"),
            DomainError::IPv4NotAllow => f.write_str("must not use an IPv4"),
            DomainError::LocalMust => f.write_str("must be local"),
            DomainError::LocalNotAllow => f.write_str("must not be local"),
            DomainError::AtLeastTwoLabelsMust => f.write_str("must have at least two labels"),
            DomainError::AtLeastTwoLabelsNotAllow => f.write_str("must have only one label"),
            DomainError::PortMust => f.write_str("port not found"),
            DomainError::PortNotAllow => f.write_str("port not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for DomainError {}
