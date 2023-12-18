#[cfg(any(feature = "email", feature = "host"))]
mod host;
#[cfg(any(feature = "email", feature = "host"))]
pub use host::*;

#[cfg(feature = "http_ftp_url")]
mod protocol;
#[cfg(feature = "http_ftp_url")]
pub use protocol::*;
