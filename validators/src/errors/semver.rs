use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `semver` and `semver_req` validator.
#[derive(Debug)]
pub struct SemverError(pub semver::Error);

impl From<semver::Error> for SemverError {
    #[inline]
    fn from(error: semver::Error) -> Self {
        Self(error)
    }
}

impl Display for SemverError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "std")]
impl Error for SemverError {}
