#[cfg(any(feature = "base32", feature = "base32_decoded"))]
mod base32;
#[cfg(any(feature = "base32", feature = "base32_decoded"))]
pub use base32::Base32Error;

#[cfg(feature = "base32_decoded")]
mod base32_decoded;
#[cfg(feature = "base32_decoded")]
pub use base32_decoded::Base32DecodedError;

#[cfg(any(feature = "base64", feature = "base64_decoded"))]
mod base64;
#[cfg(any(feature = "base64", feature = "base64_decoded"))]
pub use base64::Base64Error;

#[cfg(feature = "base64_decoded")]
mod base64_decoded;
#[cfg(feature = "base64_decoded")]
pub use base64_decoded::Base64DecodedError;

#[cfg(any(feature = "base64_url", feature = "base64_url_decoded"))]
mod base64_url;
#[cfg(any(feature = "base64_url", feature = "base64_url_decoded"))]
pub use base64_url::Base64UrlError;

#[cfg(feature = "base64_url_decoded")]
mod base64_url_decoded;
#[cfg(feature = "base64_url_decoded")]
pub use base64_url_decoded::Base64UrlDecodedError;

#[cfg(feature = "boolean")]
mod boolean;
#[cfg(feature = "boolean")]
pub use boolean::BooleanError;

#[cfg(feature = "domain")]
mod domain;
#[cfg(feature = "domain")]
pub use domain::DomainError;

#[cfg(feature = "email")]
mod email;
#[cfg(feature = "email")]
pub use email::EmailError;

#[cfg(feature = "host")]
mod host;
#[cfg(feature = "host")]
pub use host::HostError;

#[cfg(feature = "ip")]
mod ip;
#[cfg(feature = "ip")]
pub use ip::IPError;

#[cfg(feature = "ipv4")]
mod ipv4;
#[cfg(feature = "ipv4")]
pub use ipv4::IPv4Error;

#[cfg(feature = "ipv6")]
mod ipv6;
#[cfg(feature = "ipv6")]
pub use ipv6::IPv6Error;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "json")]
pub use json::JSONError;

#[cfg(feature = "line")]
mod line;
#[cfg(feature = "line")]
pub use line::LineError;

#[cfg(feature = "mac_address")]
mod mac_address;
#[cfg(feature = "mac_address")]
pub use mac_address::MacAddressError;

#[cfg(feature = "number")]
mod number;
#[cfg(feature = "number")]
pub use number::NumberError;

#[cfg(feature = "regex")]
mod regex;
#[cfg(feature = "regex")]
pub use self::regex::RegexError;

#[cfg(feature = "semver")]
mod semver;
#[cfg(feature = "semver")]
pub use self::semver::SemVerError;

#[cfg(feature = "semver_req")]
mod semver_req;
#[cfg(feature = "semver_req")]
pub use semver_req::SemVerReqError;

#[cfg(feature = "signed_integer")]
mod signed_integer;
#[cfg(feature = "signed_integer")]
pub use signed_integer::SignedIntegerError;

#[cfg(feature = "text")]
mod text;
#[cfg(feature = "text")]
pub use text::TextError;

#[cfg(feature = "unsigned_integer")]
mod unsigned_integer;
#[cfg(feature = "unsigned_integer")]
pub use unsigned_integer::UnsignedIntegerError;

#[cfg(feature = "url")]
mod url;
#[cfg(feature = "url")]
pub use self::url::URLError;

#[cfg(feature = "uuid")]
mod uuid;
#[cfg(feature = "uuid")]
pub use uuid::UUIDError;
