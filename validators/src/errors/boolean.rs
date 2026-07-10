use core::fmt::{self, Display, Formatter};

/// Error from the `boolean` validator.
#[derive(Debug, Clone)]
pub struct BooleanError;

impl Display for BooleanError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("invalid boolean")
    }
}

impl core::error::Error for BooleanError {}
