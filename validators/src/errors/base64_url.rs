use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum Base64UrlError {
    /// the fallback variant
    Invalid,
    /// may not be valid but missing the padding part is guaranteed
    PaddingMust,
    /// may not be valid and the padding part seems to exist
    PaddingNotAllow,
}

impl Display for Base64UrlError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Base64UrlError::Invalid => f.write_str("invalid Base64-url"),
            Base64UrlError::PaddingMust => f.write_str("padding not found"),
            Base64UrlError::PaddingNotAllow => f.write_str("padding not allowed"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for Base64UrlError {}
