use core::{
    fmt::{self, Display, Formatter},
    num::ParseIntError,
};

/// Error from the `signed_integer` validator.
#[derive(Debug, Clone)]
pub enum SignedIntegerError {
    ParseIntError(ParseIntError),
    TooLarge,
    TooSmall,
    Forbidden,
}

impl From<ParseIntError> for SignedIntegerError {
    #[inline]
    fn from(error: ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

impl Display for SignedIntegerError {
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

impl core::error::Error for SignedIntegerError {}
