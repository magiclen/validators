use core::{
    fmt::{self, Display, Formatter},
    num::ParseIntError,
};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `unsigned_integer` validator.
#[derive(Debug, Clone)]
pub enum UnsignedIntegerError {
    ParseIntError(ParseIntError),
    TooLarge,
    TooSmall,
    Forbidden,
}

impl From<ParseIntError> for UnsignedIntegerError {
    #[inline]
    fn from(error: ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

impl Display for UnsignedIntegerError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::ParseIntError(error) => Display::fmt(error, f),
            Self::TooLarge => f.write_str("integer is too large"),
            Self::TooSmall => f.write_str("integer is too small"),
            Self::Forbidden => f.write_str("integer is forbidden"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for UnsignedIntegerError {}
