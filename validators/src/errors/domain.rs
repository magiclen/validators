use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `domain` validator.
#[derive(Debug, Clone)]
pub enum DomainError {
    /// Incorrect domain data.
    Invalid,
    /// May not be valid, but it is guaranteed that the domain part is not an IPv4.
    IPv4Must,
    /// May not be valid, but it is guaranteed that the domain part is an IPv4.
    IPv4Disallow,
    /// May not be valid, but it is guaranteed that the domain part is not local.
    LocalMust,
    /// May not be valid, but it is guaranteed that the domain part is local.
    LocalDisallow,
    /// May not be valid, but it is guaranteed that the domain part has only one label.
    AtLeastTwoLabelsMust,
    /// May not be valid, but it is guaranteed that the domain part has at least two labels.
    AtLeastTwoLabelsDisallow,
    /// May not be valid, but missing a port is guaranteed.
    PortMust,
    /// May not be valid, and the port part seems to exist.
    PortDisallow,
}

impl Display for DomainError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Invalid => f.write_str("invalid domain"),
            Self::IPv4Must => f.write_str("must use an IPv4"),
            Self::IPv4Disallow => f.write_str("must not use an IPv4"),
            Self::LocalMust => f.write_str("must be local"),
            Self::LocalDisallow => f.write_str("must not be local"),
            Self::AtLeastTwoLabelsMust => f.write_str("must have at least two labels"),
            Self::AtLeastTwoLabelsDisallow => f.write_str("must have only one label"),
            Self::PortMust => f.write_str("port not found"),
            Self::PortDisallow => f.write_str("port not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for DomainError {}
