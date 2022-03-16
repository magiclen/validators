use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug)]
pub enum JSONError {
    SerdeJsonError(serde_json::Error),
    /// The value is empty or is not supported in JSON.
    InvalidJsonValueError,
}

impl From<serde_json::Error> for JSONError {
    #[inline]
    fn from(error: serde_json::Error) -> Self {
        JSONError::SerdeJsonError(error)
    }
}

impl Display for JSONError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            JSONError::SerdeJsonError(error) => Display::fmt(error, f),
            JSONError::InvalidJsonValueError => f.write_str("invalid json value"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for JSONError {}
