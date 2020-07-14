#[cfg(not(any(
    feature = "base32",
    feature = "base32_decoded",
    feature = "base64",
    feature = "base64_decoded",
    feature = "base64_url",
    feature = "base64_url_decoded",
    feature = "boolean",
    feature = "domain",
    feature = "email",
    feature = "host",
    feature = "ip",
    feature = "ipv4",
    feature = "ipv6",
    feature = "json",
    feature = "line",
    feature = "text",
)))]
compile_error!("at least one of the validator features must be enabled");

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Ordinalize)]
pub enum Validator {
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
    #[cfg(feature = "boolean")]
    boolean,
    #[cfg(feature = "domain")]
    domain,
    #[cfg(feature = "email")]
    email,
    #[cfg(feature = "host")]
    host,
    #[cfg(feature = "ip")]
    ip,
    #[cfg(feature = "ipv4")]
    ipv4,
    #[cfg(feature = "ipv6")]
    ipv6,
    #[cfg(feature = "json")]
    json,
    #[cfg(feature = "line")]
    line,
    #[cfg(feature = "text")]
    text,
}

impl Validator {
    #[inline]
    pub fn from_str<S: AsRef<str>>(s: S) -> Validator {
        let s = s.as_ref();

        match s {
            #[cfg(feature = "base32")]
            "base32" => Validator::base32,
            #[cfg(feature = "base32_decoded")]
            "base32_decoded" => Validator::base32_decoded,
            #[cfg(feature = "base64")]
            "base64" => Validator::base64,
            #[cfg(feature = "base64_decoded")]
            "base64_decoded" => Validator::base64_decoded,
            #[cfg(feature = "base64_url")]
            "base64_url" => Validator::base64_url,
            #[cfg(feature = "base64_url_decoded")]
            "base64_url_decoded" => Validator::base64_url_decoded,
            #[cfg(feature = "boolean")]
            "boolean" => Validator::boolean,
            #[cfg(feature = "domain")]
            "domain" => Validator::domain,
            #[cfg(feature = "email")]
            "email" => Validator::email,
            #[cfg(feature = "host")]
            "host" => Validator::host,
            #[cfg(feature = "ip")]
            "ip" => Validator::ip,
            #[cfg(feature = "ipv4")]
            "ipv4" => Validator::ipv4,
            #[cfg(feature = "ipv6")]
            "ipv6" => Validator::ipv6,
            #[cfg(feature = "json")]
            "json" => Validator::json,
            #[cfg(feature = "line")]
            "line" => Validator::line,
            #[cfg(feature = "text")]
            "text" => Validator::text,
            _ => {
                panic!(
                    "Unsupported validator `{}`. Available validators are {:?}",
                    s,
                    Validator::variants()
                )
            }
        }
    }
}
