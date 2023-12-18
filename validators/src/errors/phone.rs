use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

use phonenumber::ParseError;

/// Error from the `phone` validator.
#[derive(Debug)]
pub enum PhoneError {
    /// Fail to parse.
    Failure(ParseError),
    /// Parsed successfully, but is invalid according to the country.
    Invalid,
}

impl From<ParseError> for PhoneError {
    #[inline]
    fn from(error: ParseError) -> Self {
        Self::Failure(error)
    }
}

impl Display for PhoneError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Failure(error) => Display::fmt(error, f),
            Self::Invalid => f.write_str("invalid phone number"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for PhoneError {}
