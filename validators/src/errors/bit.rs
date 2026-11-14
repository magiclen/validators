use core::fmt::{self, Display, Formatter};

use byte_unit::{ParseError, ValueParseError};

/// Error from the `bit` validator.
#[derive(Debug, Clone)]
pub enum BitError {
    /// The input cannot be parsed.
    /// A parsed value that is too large to be stored is reported as `TooLarge` instead.
    ParseError(ParseError),
    TooLarge,
    TooSmall,
}

impl From<ParseError> for BitError {
    #[inline]
    fn from(error: ParseError) -> Self {
        match error {
            // A value that cannot be stored is reported like a number that is too large.
            ParseError::Value(ValueParseError::ExceededBounds(_)) => Self::TooLarge,
            _ => Self::ParseError(error),
        }
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
