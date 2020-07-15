use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub struct RegexError;

impl Display for RegexError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("invalid format")
    }
}

#[cfg(feature = "std")]
impl Error for RegexError {}
