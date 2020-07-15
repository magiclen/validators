use core::fmt::{self, Display, Formatter};
use core::num::ParseIntError;

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum SignedIntegerError {
    ParseIntError(ParseIntError),
    TooLarge,
    TooSmall,
}

impl From<ParseIntError> for SignedIntegerError {
    #[inline]
    fn from(error: ParseIntError) -> Self {
        SignedIntegerError::ParseIntError(error)
    }
}

impl Display for SignedIntegerError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            SignedIntegerError::ParseIntError(error) => Display::fmt(error, f),
            SignedIntegerError::TooLarge => f.write_str("integer is too large"),
            SignedIntegerError::TooSmall => f.write_str("integer is too small"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for SignedIntegerError {}
