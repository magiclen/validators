extern crate serde_json;

use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug)]
pub struct JSONError(pub serde_json::Error);

impl From<serde_json::Error> for JSONError {
    #[inline]
    fn from(error: serde_json::Error) -> Self {
        JSONError(error)
    }
}

impl Display for JSONError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "std")]
impl Error for JSONError {}
