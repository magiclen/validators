#[cfg(not(any(
    feature = "base32",
    feature = "base32_decoded",
    feature = "base64",
    feature = "base64_decoded",
    feature = "base64_url",
    feature = "base64_url_decoded",
    feature = "boolean",
)))]
compile_error!("at least one of the validator features must be enabled");

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Validator {
    base32,
    base32_decoded,
    base64,
    base64_decoded,
    base64_url,
    base64_url_decoded,
    boolean,
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
            _ => {
                panic!("Unsupported validator `{}`. Available validators are {:?}", s, [
                    #[cfg(feature = "base32")]
                    Validator::base32,
                    #[cfg(feature = "base32_decoded")]
                    Validator::base32_decoded,
                    #[cfg(feature = "base64")]
                    Validator::base64,
                    #[cfg(feature = "base64_decoded")]
                    Validator::base64_decoded,
                    #[cfg(feature = "base64_url")]
                    Validator::base64_url,
                    #[cfg(feature = "base64_url_decoded")]
                    Validator::base64_url_decoded,
                    #[cfg(feature = "boolean")]
                    Validator::boolean,
                ])
            }
        }
    }
}