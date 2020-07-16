#[cfg(all(feature = "std", feature = "idna"))]
mod host;
#[cfg(all(feature = "std", feature = "idna"))]
pub use host::*;

#[cfg(feature = "http_ftp_url")]
mod protocol;
#[cfg(feature = "http_ftp_url")]
pub use protocol::*;
