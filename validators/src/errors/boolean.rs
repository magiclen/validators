use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub struct BooleanError;

impl Display for BooleanError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("invalid boolean")
    }
}

#[cfg(feature = "std")]
impl Error for BooleanError {}
