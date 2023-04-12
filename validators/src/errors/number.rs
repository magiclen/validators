use core::{
    fmt::{self, Display, Formatter},
    num::ParseFloatError,
};
#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum NumberError {
    ParseFloatError(ParseFloatError),
    TooLarge,
    TooSmall,
    Forbidden,
    NaNMust,
    NaNNotAllow,
}

impl From<ParseFloatError> for NumberError {
    #[inline]
    fn from(error: ParseFloatError) -> Self {
        NumberError::ParseFloatError(error)
    }
}

impl Display for NumberError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            NumberError::ParseFloatError(error) => Display::fmt(error, f),
            NumberError::TooLarge => f.write_str("number is too large"),
            NumberError::TooSmall => f.write_str("number is too small"),
            NumberError::Forbidden => f.write_str("number is forbidden"),
            NumberError::NaNMust => f.write_str("must be NaN"),
            NumberError::NaNNotAllow => f.write_str("must not be NaN"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for NumberError {}
