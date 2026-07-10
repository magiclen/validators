use core::fmt::{self, Display, Formatter};

use byte_unit::ParseError;

/// Error from the `bit` validator.
#[derive(Debug, Clone)]
pub enum BitError {
    ParseError(ParseError),
    TooLarge,
    TooSmall,
}

impl From<ParseError> for BitError {
    #[inline]
    fn from(error: ParseError) -> Self {
        Self::ParseError(error)
    }
}

impl Display for BitError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::ParseError(error) => Display::fmt(error, f),
            Self::TooLarge => f.write_str("bit is too large"),
            Self::TooSmall => f.write_str("bit is too small"),
        }
    }
}

impl core::error::Error for BitError {}
