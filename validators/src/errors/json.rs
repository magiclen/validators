use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

#[cfg(feature = "std")]
use std::str::Utf8Error;

use crate::serde_json;

#[derive(Debug)]
pub enum JSONError {
    SerdeJsonError(serde_json::Error),
    #[cfg(feature = "std")]
    Utf8Error(Utf8Error),
}

impl From<serde_json::Error> for JSONError {
    #[inline]
    fn from(error: serde_json::Error) -> Self {
        JSONError::SerdeJsonError(error)
    }
}

#[cfg(feature = "std")]
impl From<Utf8Error> for JSONError {
    #[inline]
    fn from(error: Utf8Error) -> Self {
        JSONError::Utf8Error(error)
    }
}

impl Display for JSONError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            JSONError::SerdeJsonError(error) => Display::fmt(error, f),
            #[cfg(feature = "std")]
            JSONError::Utf8Error(error) => Display::fmt(error, f),
        }
    }
}

#[cfg(feature = "std")]
impl Error for JSONError {}
