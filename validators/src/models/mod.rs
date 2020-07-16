#[cfg(all(feature = "std", feature = "idna"))]
mod host;
#[cfg(all(feature = "std", feature = "idna"))]
pub use host::*;

#[cfg(any(feature = "http_url", feature = "http_ftp_url"))]
mod protocol;
#[cfg(any(feature = "http_url", feature = "http_ftp_url"))]
pub use protocol::*;
