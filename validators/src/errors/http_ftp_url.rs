use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

use crate::url;

#[derive(Debug, Clone)]
pub enum HttpFtpURLError {
    ParseError(url::ParseError),
    // may not be valid but it is guaranteed that the scheme (protocol) is not `http`, `https` or `ftp`
    ProtocolError,
    LocalMust,
    LocalNotAllow,
}

impl From<url::ParseError> for HttpFtpURLError {
    #[inline]
    fn from(error: url::ParseError) -> Self {
        HttpFtpURLError::ParseError(error)
    }
}

impl Display for HttpFtpURLError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            HttpFtpURLError::ParseError(error) => Display::fmt(error, f),
            HttpFtpURLError::ProtocolError => {
                f.write_str("need to use `http`, `https` or `ftp` as a protocol")
            }
            HttpFtpURLError::LocalMust => f.write_str("must be local"),
            HttpFtpURLError::LocalNotAllow => f.write_str("must not be local"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for HttpFtpURLError {}
