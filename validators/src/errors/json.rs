use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `json` validator.
#[derive(Debug)]
pub enum JsonError {
    SerdeJsonError(serde_json::Error),
    /// The value is empty or is not supported in JSON.
    InvalidJsonValueError,
}

impl From<serde_json::Error> for JsonError {
    #[inline]
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeJsonError(error)
    }
}

impl Display for JsonError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::SerdeJsonError(error) => Display::fmt(error, f),
            Self::InvalidJsonValueError => f.write_str("invalid json value"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for JsonError {}
