use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `length` validator.
#[derive(Debug, Clone)]
pub enum LengthError {
    TooLarge,
    TooSmall,
}

impl Display for LengthError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::TooLarge => f.write_str("collection is too large"),
            Self::TooSmall => f.write_str("collection is too small"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for LengthError {}
