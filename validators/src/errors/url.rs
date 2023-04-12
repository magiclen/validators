use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

use crate::url;

#[derive(Debug, Clone)]
pub struct URLError(pub url::ParseError);

impl From<url::ParseError> for URLError {
    #[inline]
    fn from(error: url::ParseError) -> Self {
        URLError(error)
    }
}

impl Display for URLError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "std")]
impl Error for URLError {}
