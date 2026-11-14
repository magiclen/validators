use core::{
    fmt::{self, Display, Formatter},
    num::{IntErrorKind, ParseIntError},
};

/// Error from the `unsigned_integer` validator.
#[derive(Debug, Clone)]
pub enum UnsignedIntegerError {
    /// The input is not an unsigned integer.
    /// An integer that is too large for the type is reported as `TooLarge` instead.
    ParseIntError(ParseIntError),
    TooLarge,
    TooSmall,
    Forbidden,
}

impl From<ParseIntError> for UnsignedIntegerError {
    #[inline]
    fn from(error: ParseIntError) -> Self {
        // An integer that is too large for the type is reported like a number out of the range.
        match error.kind() {
            IntErrorKind::PosOverflow => Self::TooLarge,
            _ => Self::ParseIntError(error),
        }
    }
}

impl Display for UnsignedIntegerError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::ParseIntError(error) => Display::fmt(error, f),
            Self::TooLarge => f.write_str("integer is too large"),
            Self::TooSmall => f.write_str("integer is too small"),
            Self::Forbidden => f.write_str("integer is forbidden"),
        }
    }
}

impl core::error::Error for UnsignedIntegerError {}
