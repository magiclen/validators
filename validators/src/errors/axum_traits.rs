use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::*;

/// Implement `IntoResponse` for every validator error type so that handlers can return these errors directly with the `?` operator.
macro_rules! impl_into_response {
    ($(($feature:literal, $Error:ident)),* $(,)*) => {
        $(
            #[cfg(feature = $feature)]
            impl IntoResponse for $Error {
                #[inline]
                fn into_response(self) -> Response {
                    (StatusCode::BAD_REQUEST, self.to_string()).into_response()
                }
            }
        )*
    };
}

impl_into_response! {
    ("base32", Base32Error),
    ("base32_decoded", Base32DecodedError),
    ("base64", Base64Error),
    ("base64_decoded", Base64DecodedError),
    ("base64_url", Base64UrlError),
    ("base64_url_decoded", Base64UrlDecodedError),
    ("bit", BitError),
    ("boolean", BooleanError),
    ("byte", ByteError),
    ("domain", DomainError),
    ("email", EmailError),
    ("host", HostError),
    ("http_url", HttpURLError),
    ("http_ftp_url", HttpFtpURLError),
    ("ip", IpError),
    ("ipv4", Ipv4Error),
    ("ipv6", Ipv6Error),
    ("json", JsonError),
    ("length", LengthError),
    ("line", LineError),
    ("mac_address", MacAddressError),
    ("number", NumberError),
    ("phone", PhoneError),
    ("regex", RegexError),
    ("signed_integer", SignedIntegerError),
    ("text", TextError),
    ("unsigned_integer", UnsignedIntegerError),
    ("url", UrlError),
    ("uuid", UuidError),
}

// `SemverError` is shared by the `semver` and `semver_req` validators, so it cannot use the single-feature macro above.
#[cfg(any(feature = "semver", feature = "semver_req"))]
impl IntoResponse for SemverError {
    #[inline]
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
