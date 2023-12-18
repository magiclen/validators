#[cfg(not(any(
    feature = "base32",
    feature = "base32_decoded",
    feature = "base64",
    feature = "base64_decoded",
    feature = "base64_url",
    feature = "base64_url_decoded",
    feature = "bit",
    feature = "boolean",
    feature = "byte",
    feature = "domain",
    feature = "email",
    feature = "host",
    feature = "http_url",
    feature = "http_ftp_url",
    feature = "ip",
    feature = "ipv4",
    feature = "ipv6",
    feature = "json",
    feature = "length",
    feature = "line",
    feature = "mac_address",
    feature = "number",
    feature = "phone",
    feature = "regex",
    feature = "semver",
    feature = "semver_req",
    feature = "signed_integer",
    feature = "text",
    feature = "unsigned_integer",
    feature = "url",
    feature = "uuid",
)))]
compile_error!("at least one of the validator features must be enabled");

use enum_ordinalize::Ordinalize;
use syn::Path;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ordinalize)]
#[ordinalize(impl_trait = false)]
#[ordinalize(variants(pub(crate) const VARIANTS))]
pub(crate) enum Validator {
    #[cfg(feature = "base32")]
    base32,
    #[cfg(feature = "base32_decoded")]
    base32_decoded,
    #[cfg(feature = "base64")]
    base64,
    #[cfg(feature = "base64_decoded")]
    base64_decoded,
    #[cfg(feature = "base64_url")]
    base64_url,
    #[cfg(feature = "base64_url_decoded")]
    base64_url_decoded,
    #[cfg(feature = "bit")]
    bit,
    #[cfg(feature = "boolean")]
    boolean,
    #[cfg(feature = "byte")]
    byte,
    #[cfg(feature = "domain")]
    domain,
    #[cfg(feature = "email")]
    email,
    #[cfg(feature = "host")]
    host,
    #[cfg(feature = "http_url")]
    http_url,
    #[cfg(feature = "http_ftp_url")]
    http_ftp_url,
    #[cfg(feature = "ip")]
    ip,
    #[cfg(feature = "ipv4")]
    ipv4,
    #[cfg(feature = "ipv6")]
    ipv6,
    #[cfg(feature = "json")]
    json,
    #[cfg(feature = "length")]
    length,
    #[cfg(feature = "line")]
    line,
    #[cfg(feature = "mac_address")]
    mac_address,
    #[cfg(feature = "number")]
    number,
    #[cfg(feature = "phone")]
    phone,
    #[cfg(feature = "regex")]
    regex,
    #[cfg(feature = "semver")]
    semver,
    #[cfg(feature = "semver_req")]
    semver_req,
    #[cfg(feature = "signed_integer")]
    signed_integer,
    #[cfg(feature = "text")]
    text,
    #[cfg(feature = "unsigned_integer")]
    unsigned_integer,
    #[cfg(feature = "url")]
    url,
    #[cfg(feature = "uuid")]
    uuid,

    _Nothing,
}

impl Validator {
    #[inline]
    pub(crate) fn from_path(path: &Path) -> Option<Self> {
        let ident_string = match path.get_ident() {
            Some(ident) => ident.to_string(),
            None => return None,
        };

        match ident_string.as_str() {
            #[cfg(feature = "base32")]
            "base32" => Some(Self::base32),
            #[cfg(feature = "base32_decoded")]
            "base32_decoded" => Some(Self::base32_decoded),
            #[cfg(feature = "base64")]
            "base64" => Some(Self::base64),
            #[cfg(feature = "base64_decoded")]
            "base64_decoded" => Some(Self::base64_decoded),
            #[cfg(feature = "base64_url")]
            "base64_url" => Some(Self::base64_url),
            #[cfg(feature = "base64_url_decoded")]
            "base64_url_decoded" => Some(Self::base64_url_decoded),
            #[cfg(feature = "bit")]
            "bit" => Some(Self::bit),
            #[cfg(feature = "boolean")]
            "boolean" => Some(Self::boolean),
            #[cfg(feature = "byte")]
            "byte" => Some(Self::byte),
            #[cfg(feature = "domain")]
            "domain" => Some(Self::domain),
            #[cfg(feature = "email")]
            "email" => Some(Self::email),
            #[cfg(feature = "host")]
            "host" => Some(Self::host),
            #[cfg(feature = "http_url")]
            "http_url" => Some(Self::http_url),
            #[cfg(feature = "http_ftp_url")]
            "http_ftp_url" => Some(Self::http_ftp_url),
            #[cfg(feature = "ip")]
            "ip" => Some(Self::ip),
            #[cfg(feature = "ipv4")]
            "ipv4" => Some(Self::ipv4),
            #[cfg(feature = "ipv6")]
            "ipv6" => Some(Self::ipv6),
            #[cfg(feature = "json")]
            "json" => Some(Self::json),
            #[cfg(feature = "length")]
            "length" => Some(Self::length),
            #[cfg(feature = "line")]
            "line" => Some(Self::line),
            #[cfg(feature = "mac_address")]
            "mac_address" => Some(Self::mac_address),
            #[cfg(feature = "number")]
            "number" => Some(Self::number),
            #[cfg(feature = "phone")]
            "phone" => Some(Self::phone),
            #[cfg(feature = "regex")]
            "regex" => Some(Self::regex),
            #[cfg(feature = "semver")]
            "semver" => Some(Self::semver),
            #[cfg(feature = "semver_req")]
            "semver_req" => Some(Self::semver_req),
            #[cfg(feature = "signed_integer")]
            "signed_integer" => Some(Self::signed_integer),
            #[cfg(feature = "text")]
            "text" => Some(Self::text),
            #[cfg(feature = "unsigned_integer")]
            "unsigned_integer" => Some(Self::unsigned_integer),
            #[cfg(feature = "url")]
            "url" => Some(Self::url),
            #[cfg(feature = "uuid")]
            "uuid" => Some(Self::uuid),
            _ => None,
        }
    }
}
