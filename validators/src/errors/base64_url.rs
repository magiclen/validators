use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `base64_url` validator.
#[derive(Debug, Clone)]
pub enum Base64UrlError {
    /// Incorrect Base64-url-encoded data.
    Invalid,
    /// May not be valid, but the absence of padding is guaranteed.
    PaddingMust,
    /// May not be valid, but it appears that the padding part exists.
    PaddingDisallow,
}

impl Display for Base64UrlError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Invalid => f.write_str("invalid Base64-url"),
            Self::PaddingMust => f.write_str("padding not found"),
            Self::PaddingDisallow => f.write_str("padding not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for Base64UrlError {}
