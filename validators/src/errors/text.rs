use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `text` validator.
#[derive(Debug, Clone)]
pub enum TextError {
    Invalid,
    /// May not be valid, but it is guaranteed that this text is too long.
    TooLong,
    /// May not be valid, but it is guaranteed that this text is too short.
    TooShort,
}

impl Display for TextError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Invalid => f.write_str("invalid text"),
            Self::TooLong => f.write_str("text is too long"),
            Self::TooShort => f.write_str("text is too short"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for TextError {}
