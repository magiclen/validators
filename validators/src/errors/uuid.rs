use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `uuid` validator.
#[derive(Debug, Clone)]
pub enum UuidError {
    /// Including the violation of the case rule.
    Invalid,
    /// May not be valid, but missing separators is guaranteed.
    SeparatorMust,
    /// May not be valid, but separators seem to exist.
    SeparatorDisallow,
}

impl Display for UuidError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Invalid => f.write_str("invalid uuid"),
            Self::SeparatorMust => f.write_str("separators not found"),
            Self::SeparatorDisallow => f.write_str("separators not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for UuidError {}
