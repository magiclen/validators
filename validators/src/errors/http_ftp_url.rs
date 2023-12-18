use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

use crate::url;

/// Error from the `http_ftp_url` validator.
#[derive(Debug, Clone)]
pub enum HttpFtpURLError {
    ParseError(url::ParseError),
    /// May not be valid, but it is guaranteed that the scheme (protocol) is not `http`, `https` or `ftp`
    ProtocolError,
    LocalMust,
    LocalDisallow,
}

impl From<url::ParseError> for HttpFtpURLError {
    #[inline]
    fn from(error: url::ParseError) -> Self {
        Self::ParseError(error)
    }
}

impl Display for HttpFtpURLError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::ParseError(error) => Display::fmt(error, f),
            Self::ProtocolError => {
                f.write_str("need to use `http`, `https` or `ftp` as a protocol")
            },
            Self::LocalMust => f.write_str("must be local"),
            Self::LocalDisallow => f.write_str("must not be local"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for HttpFtpURLError {}
