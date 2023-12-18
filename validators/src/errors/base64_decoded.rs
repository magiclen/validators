use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `base64_decoded` validator.
#[derive(Debug, Clone)]
pub enum Base64DecodedError {
    /// Incorrect Base64-encoded data.
    Invalid,
    /// May not be valid, but the absence of padding is guaranteed.
    PaddingMust,
    /// May not be valid, but it appears that the padding part exists.
    PaddingDisallow,
    /// May be valid but errors happen when decoding (missing padding? having padding?).
    Decode,
}

impl Display for Base64DecodedError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Invalid => f.write_str("invalid Base64"),
            Self::PaddingMust => f.write_str("padding not found"),
            Self::PaddingDisallow => f.write_str("padding not allowed"),
            Self::Decode => f.write_str("decoded incorrectly"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for Base64DecodedError {}
