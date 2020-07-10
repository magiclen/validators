use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum Base32DecodedError {
    /// the fallback variant
    Invalid,
    /// may not be valid but missing the padding part is guaranteed
    PaddingMust,
    /// may not be valid and the padding part seems to exist
    PaddingNotAllow,
    /// may be valid but errors happen when decoding (missing padding? having padding?)
    Decode,
}

impl Display for Base32DecodedError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Base32DecodedError::Invalid => f.write_str("invalid Base32"),
            Base32DecodedError::PaddingMust => f.write_str("padding not found"),
            Base32DecodedError::PaddingNotAllow => f.write_str("padding not allowed"),
            Base32DecodedError::Decode => f.write_str("decoded incorrectly"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for Base32DecodedError {}
