use syn::{DeriveInput, Meta};

#[cfg(feature = "base32")]
pub(crate) mod base32;

#[cfg(feature = "base32_decoded")]
pub(crate) mod base32_decoded;

#[cfg(feature = "base64")]
pub(crate) mod base64;

#[cfg(feature = "base64_decoded")]
pub(crate) mod base64_decoded;

#[cfg(feature = "base64_url")]
pub(crate) mod base64_url;

#[cfg(feature = "base64_url_decoded")]
pub(crate) mod base64_url_decoded;

#[cfg(feature = "bit")]
pub(crate) mod bit;

#[cfg(feature = "boolean")]
pub(crate) mod boolean;

#[cfg(feature = "byte")]
pub(crate) mod byte;

#[cfg(feature = "domain")]
pub(crate) mod domain;

#[cfg(feature = "email")]
pub(crate) mod email;

#[cfg(feature = "host")]
pub(crate) mod host;

#[cfg(feature = "http_url")]
pub(crate) mod http_url;

#[cfg(feature = "http_ftp_url")]
pub(crate) mod http_ftp_url;

#[cfg(feature = "ip")]
pub(crate) mod ip;

#[cfg(feature = "ipv4")]
pub(crate) mod ipv4;

#[cfg(feature = "ipv6")]
pub(crate) mod ipv6;

#[cfg(feature = "json")]
pub(crate) mod json;

#[cfg(feature = "length")]
pub(crate) mod length;

#[cfg(feature = "line")]
pub(crate) mod line;

#[cfg(feature = "mac_address")]
pub(crate) mod mac_address;

#[cfg(feature = "number")]
pub(crate) mod number;

#[cfg(feature = "phone")]
pub(crate) mod phone;

#[cfg(feature = "regex")]
pub(crate) mod regex;

#[cfg(feature = "semver")]
pub(crate) mod semver;

#[cfg(feature = "semver_req")]
pub(crate) mod semver_req;

#[cfg(feature = "signed_integer")]
pub(crate) mod signed_integer;

#[cfg(feature = "text")]
pub(crate) mod text;

#[cfg(feature = "unsigned_integer")]
pub(crate) mod unsigned_integer;

#[cfg(feature = "url")]
pub(crate) mod url;

#[cfg(feature = "uuid")]
pub(crate) mod uuid;

pub(crate) trait ValidatorHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream>;
}
