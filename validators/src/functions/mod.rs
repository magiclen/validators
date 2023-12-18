#[cfg(any(
    feature = "domain",
    feature = "email",
    feature = "host",
    feature = "http_url",
    feature = "http_ftp_url",
    feature = "ip",
    feature = "ipv4",
    feature = "ipv6"
))]
mod ipv4;
#[cfg(any(
    feature = "domain",
    feature = "email",
    feature = "host",
    feature = "http_url",
    feature = "http_ftp_url",
    feature = "ip",
    feature = "ipv4",
    feature = "ipv6"
))]
pub use ipv4::*;

#[cfg(any(
    feature = "email",
    feature = "host",
    feature = "http_url",
    feature = "http_ftp_url",
    feature = "ip",
    feature = "ipv6"
))]
mod ipv6;
#[cfg(any(
    feature = "email",
    feature = "host",
    feature = "http_url",
    feature = "http_ftp_url",
    feature = "ip",
    feature = "ipv6"
))]
pub use ipv6::*;

#[cfg(feature = "ip")]
mod ip;
#[cfg(feature = "ip")]
pub use ip::*;

#[cfg(any(
    feature = "domain",
    feature = "email",
    feature = "host",
    feature = "http_url",
    feature = "http_ftp_url",
))]
mod domain;
#[cfg(any(
    feature = "domain",
    feature = "email",
    feature = "host",
    feature = "http_url",
    feature = "http_ftp_url",
))]
pub use domain::*;
