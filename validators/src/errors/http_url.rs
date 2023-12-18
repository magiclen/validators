use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

use crate::url;

/// Error from the `http_url` validator.
#[derive(Debug, Clone)]
pub enum HttpURLError {
    ParseError(url::ParseError),
    /// May not be valid, but it is guaranteed that the scheme (protocol) is not `http` or `https`.
    ProtocolError,
    LocalMust,
    LocalDisallow,
}

impl From<url::ParseError> for HttpURLError {
    #[inline]
    fn from(error: url::ParseError) -> Self {
        HttpURLError::ParseError(error)
    }
}

impl Display for HttpURLError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::ParseError(error) => Display::fmt(error, f),
            Self::ProtocolError => f.write_str("need to use `http` or `https` as a protocol"),
            Self::LocalMust => f.write_str("must be local"),
            Self::LocalDisallow => f.write_str("must not be local"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for HttpURLError {}
