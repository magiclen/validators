use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum UUIDError {
    /// including the violation of the case rule
    Invalid,
    /// may not be valid but missing separators is guaranteed
    SeparatorMust,
    /// may not be valid but separators seem to exist
    SeparatorNotAllow,
}

impl Display for UUIDError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            UUIDError::Invalid => f.write_str("invalid uuid"),
            UUIDError::SeparatorMust => f.write_str("separators not found"),
            UUIDError::SeparatorNotAllow => f.write_str("separators not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for UUIDError {}
