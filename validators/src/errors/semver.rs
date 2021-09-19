use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

use crate::semver;

#[derive(Debug)]
pub struct SemVerError(pub semver::Error);

impl From<semver::Error> for SemVerError {
    #[inline]
    fn from(error: semver::Error) -> Self {
        SemVerError(error)
    }
}

impl Display for SemVerError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "std")]
impl Error for SemVerError {}
