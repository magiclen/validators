use core::fmt::{self, Display, Formatter};

/// Error from the `json` validator.
#[derive(Debug)]
pub enum JsonError {
    SerdeJsonError(serde_json::Error),
    /// This crate no longer returns this variant.
    /// It is kept for compatibility.
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

impl core::error::Error for JsonError {}
