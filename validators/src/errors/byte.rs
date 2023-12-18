use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

use byte_unit::ParseError;

/// Error from the `bit` validator.
#[derive(Debug, Clone)]
pub enum ByteError {
    ParseError(ParseError),
    TooLarge,
    TooSmall,
}

impl From<ParseError> for ByteError {
    #[inline]
    fn from(error: ParseError) -> Self {
        Self::ParseError(error)
    }
}

impl Display for ByteError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::ParseError(error) => Display::fmt(error, f),
            Self::TooLarge => f.write_str("byte is too large"),
            Self::TooSmall => f.write_str("byte is too small"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for ByteError {}
