use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `line` validator.
#[derive(Debug, Clone)]
pub enum LineError {
    Invalid,
    /// May not be valid, but it is guaranteed that this line is too long.
    TooLong,
    /// May not be valid, but it is guaranteed that this line is too short.
    TooShort,
}

impl Display for LineError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Invalid => f.write_str("invalid line"),
            Self::TooLong => f.write_str("line is too long"),
            Self::TooShort => f.write_str("line is too short"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for LineError {}
