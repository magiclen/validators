use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

use crate::semver;

#[derive(Debug, Clone)]
pub struct SemVerReqError(pub semver::ReqParseError);

impl From<semver::ReqParseError> for SemVerReqError {
    #[inline]
    fn from(error: semver::ReqParseError) -> Self {
        SemVerReqError(error)
    }
}

impl Display for SemVerReqError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "std")]
impl Error for SemVerReqError {}
