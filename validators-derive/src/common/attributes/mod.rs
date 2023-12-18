#[cfg(any(
    feature = "base32",
    feature = "base32_decoded",
    feature = "base64",
    feature = "base64_decoded",
    feature = "base64_url",
    feature = "base64_url_decoded",
))]
pub(crate) mod base_xx_attribute;
#[cfg(any(
    feature = "boolean",
    feature = "json",
    feature = "semver",
    feature = "semver_req",
    feature = "url"
))]
pub(crate) mod basic_attribute;
#[cfg(any(feature = "http_url", feature = "http_ftp_url"))]
pub(crate) mod http_xx_url_attribute;
#[cfg(any(feature = "ip", feature = "ipv4", feature = "ipv6"))]
pub(crate) mod ip_xx_attribute;
#[cfg(any(feature = "signed_integer", feature = "unsigned_integer"))]
pub(crate) mod range_attribute;
#[cfg(any(feature = "line", feature = "text"))]
pub(crate) mod utf8_attribute;
