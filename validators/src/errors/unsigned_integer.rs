use core::{
    fmt::{self, Display, Formatter},
    num::ParseIntError,
};
#[cfg(feature = "std")]
use std::error::Error;

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
        UnsignedIntegerError::ParseIntError(error)
    }
}

impl Display for UnsignedIntegerError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            UnsignedIntegerError::ParseIntError(error) => Display::fmt(error, f),
            UnsignedIntegerError::TooLarge => f.write_str("integer is too large"),
            UnsignedIntegerError::TooSmall => f.write_str("integer is too small"),
            UnsignedIntegerError::Forbidden => f.write_str("integer is forbidden"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for UnsignedIntegerError {}
