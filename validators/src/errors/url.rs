use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

use crate::url;

/// Error from the `url` validator.
#[derive(Debug, Clone)]
pub struct UrlError(pub url::ParseError);

impl From<url::ParseError> for UrlError {
    #[inline]
    fn from(error: url::ParseError) -> Self {
        Self(error)
    }
}

impl Display for UrlError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "std")]
impl Error for UrlError {}
