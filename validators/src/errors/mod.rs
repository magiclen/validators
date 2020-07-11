#[cfg(any(feature = "base32", feature = "base32_decoded"))]
pub mod base32;
#[cfg(feature = "base32_decoded")]
pub mod base32_decoded;

#[cfg(any(feature = "base64", feature = "base64_decoded"))]
pub mod base64;
#[cfg(feature = "base64_decoded")]
pub mod base64_decoded;

#[cfg(any(feature = "base64_url", feature = "base64_url_decoded"))]
pub mod base64_url;
#[cfg(feature = "base64_url_decoded")]
pub mod base64_url_decoded;

#[cfg(feature = "boolean")]
pub mod boolean;

#[cfg(feature = "domain")]
pub mod domain;
