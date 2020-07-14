use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum TextError {
    Invalid,
    /// may not be valid but it is guaranteed that this line is not empty after trimming
    EmptyMust,
    /// this line is empty after trimming
    EmptyNotAllow,
}

impl Display for TextError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            TextError::Invalid => f.write_str("invalid text"),
            TextError::EmptyMust => f.write_str("non-empty line after trimming"),
            TextError::EmptyNotAllow => f.write_str("empty line after trimming"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for TextError {}
