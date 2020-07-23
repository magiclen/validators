use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum LineError {
    Invalid,
    /// may not be valid but it is guaranteed that this line is too long
    TooLong,
    /// may not be valid but it is guaranteed that this line is too short
    TooShort,
}

impl Display for LineError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            LineError::Invalid => f.write_str("invalid line"),
            LineError::TooLong => f.write_str("line is too long"),
            LineError::TooShort => f.write_str("line is too short"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for LineError {}
