use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `mac_address` validator.
#[derive(Debug, Clone)]
pub enum MacAddressError {
    /// Including the violation of the case rule.
    Invalid,
    /// May not be valid, but missing separators is guaranteed.
    SeparatorMust,
    /// May not be valid, but separators seem to exist.
    SeparatorDisallow,
}

impl Display for MacAddressError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Invalid => f.write_str("invalid mac address"),
            Self::SeparatorMust => f.write_str("separators not found"),
            Self::SeparatorDisallow => f.write_str("separators not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for MacAddressError {}
