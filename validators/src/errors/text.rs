use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum TextError {
    Invalid,
    /// may not be valid but it is guaranteed that this text is too long
    TooLong,
    /// may not be valid but it is guaranteed that this text is too short
    TooShort,
}

impl Display for TextError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            TextError::Invalid => f.write_str("invalid text"),
            TextError::TooLong => f.write_str("text is too long"),
            TextError::TooShort => f.write_str("text is too short"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for TextError {}
