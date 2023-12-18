use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `base32` validator.
#[derive(Debug, Clone)]
pub enum Base32Error {
    /// Incorrect Base32-encoded data.
    Invalid,
    /// May not be valid, but the absence of padding is guaranteed.
    PaddingMust,
    /// May not be valid, but it appears that the padding part exists.
    PaddingDisallow,
}

impl Display for Base32Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Invalid => f.write_str("invalid Base32"),
            Self::PaddingMust => f.write_str("padding not found"),
            Self::PaddingDisallow => f.write_str("padding not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for Base32Error {}
