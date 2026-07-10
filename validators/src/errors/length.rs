use core::fmt::{self, Display, Formatter};

/// Error from the `length` validator.
#[derive(Debug, Clone)]
pub enum LengthError {
    TooLarge,
    TooSmall,
}

impl Display for LengthError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::TooLarge => f.write_str("collection is too large"),
            Self::TooSmall => f.write_str("collection is too small"),
        }
    }
}

impl core::error::Error for LengthError {}
