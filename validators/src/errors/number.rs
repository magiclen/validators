use core::{
    fmt::{self, Display, Formatter},
    num::ParseFloatError,
};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `number` validator.
#[derive(Debug, Clone)]
pub enum NumberError {
    ParseFloatError(ParseFloatError),
    TooLarge,
    TooSmall,
    Forbidden,
    NaNMust,
    NaNDisallow,
}

impl From<ParseFloatError> for NumberError {
    #[inline]
    fn from(error: ParseFloatError) -> Self {
        Self::ParseFloatError(error)
    }
}

impl Display for NumberError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::ParseFloatError(error) => Display::fmt(error, f),
            Self::TooLarge => f.write_str("number is too large"),
            Self::TooSmall => f.write_str("number is too small"),
            Self::Forbidden => f.write_str("number is forbidden"),
            Self::NaNMust => f.write_str("must be NaN"),
            Self::NaNDisallow => f.write_str("must not be NaN"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for NumberError {}
