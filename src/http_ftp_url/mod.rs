extern crate regex;

use self::regex::Regex;
use super::{ValidatorOption, Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};
use std::str::Utf8Error;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use super::host::{Host, HostLocalable, HostError};
use super::http_url::{HttpUrl, HttpUrlError};

lazy_static! {
    static ref HTTP_FTP_URL_RE: Regex = {
        Regex::new(r"^(?i)((http|https|ftp):)?(//)?([^\x00-\x1F\x2F\x7F]+)(/[^\x00-\x1F\x23\x3F\x7F]*)?([?]([^\x00-\x1F\x23\x7F]*))?(#([^\x00-\x1F\x7F]*))?$").unwrap()
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum HttpFtpUrlError {
    IncorrectFormat,
    IncorrectHostFormat(HostError),
    LocalNotAllow,
    LocalNotFound,
    ProtocolNotAllow,
    ProtocolNotFound,
    UTF8Error(Utf8Error),
}

impl Display for HttpFtpUrlError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for HttpFtpUrlError {}

pub type HttpFtpUrlResult = Result<HttpFtpUrl, HttpFtpUrlError>;

#[derive(Debug, PartialEq)]
pub struct HttpFtpUrlValidator {
    pub local: ValidatorOption,
    pub protocol: ValidatorOption,
}

#[derive(Clone)]
pub struct HttpFtpUrl {
    pub(crate) protocol: usize,
    pub(crate) host: Host,
    pub(crate) host_index: usize,
    pub(crate) path: usize,
    pub(crate) query: usize,
    pub(crate) fragment: usize,
    pub(crate) full_http_ftp_url: String,
    pub(crate) full_http_ftp_url_len: usize,
    pub(crate) is_https: bool,
    pub(crate) is_http: bool,
    pub(crate) is_ftp: bool,
    pub(crate) is_local: bool,
    pub(crate) is_absolute: bool,
}

impl HttpFtpUrl {
    pub fn get_protocol(&self) -> Option<&str> {
        if self.protocol != self.full_http_ftp_url_len {
            if self.is_absolute {
                Some(&self.full_http_ftp_url[..(self.host_index - 3)])
            } else {
                Some(&self.full_http_ftp_url[..(self.host_index - 1)])
            }
        } else {
            None
        }
    }

    pub fn get_host(&self) -> &Host {
        &self.host
    }

    pub fn get_path(&self) -> Option<&str> {
        if self.path != self.full_http_ftp_url_len {
            if self.query != self.full_http_ftp_url_len {
                Some(&self.full_http_ftp_url[self.path..(self.query - 1)])
            } else {
                if self.fragment != self.full_http_ftp_url_len {
                    Some(&self.full_http_ftp_url[self.path..(self.fragment - 1)])
                } else {
                    Some(&self.full_http_ftp_url[self.path..])
                }
            }
        } else {
            None
        }
    }

    pub fn get_query(&self) -> Option<&str> {
        if self.query != self.full_http_ftp_url_len {
            if self.fragment != self.full_http_ftp_url_len {
                Some(&self.full_http_ftp_url[self.query..(self.fragment - 1)])
            } else {
                Some(&self.full_http_ftp_url[self.query..])
            }
        } else {
            None
        }
    }

    pub fn get_fragment(&self) -> Option<&str> {
        if self.fragment != self.full_http_ftp_url_len {
            Some(&self.full_http_ftp_url[self.fragment..])
        } else {
            None
        }
    }

    pub fn get_full_http_ftp_url(&self) -> &str {
        &self.full_http_ftp_url
    }

    pub fn get_full_http_ftp_url_without_query_and_fragment(&self) -> &str {
        if self.query != self.full_http_ftp_url_len {
            &self.full_http_ftp_url[..(self.query - 1)]
        } else {
            if self.fragment != self.full_http_ftp_url_len {
                &self.full_http_ftp_url[..(self.fragment - 1)]
            } else {
                &self.full_http_ftp_url
            }
        }
    }

    pub fn is_https(&self) -> bool {
        self.is_https
    }

    pub fn is_http(&self) -> bool {
        self.is_http
    }

    pub fn is_ftp(&self) -> bool {
        self.is_ftp
    }

    pub fn is_local(&self) -> bool {
        self.is_local
    }

    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }

    pub fn into_string(self) -> String {
        self.full_http_ftp_url
    }

    pub fn into_http_url(self) -> Result<HttpUrl, HttpUrlError> {
        if self.is_ftp {
            return Err(HttpUrlError::IncorrectFormat);
        }

        Ok(HttpUrl {
            protocol: self.protocol,
            host: self.host,
            host_index: self.host_index,
            path: self.path,
            query: self.query,
            fragment: self.fragment,
            full_http_url: self.full_http_ftp_url,
            full_http_url_len: self.full_http_ftp_url_len,
            is_https: self.is_https,
            is_local: self.is_local,
            is_absolute: self.is_absolute,
        })
    }
}

impl Deref for HttpFtpUrl {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.full_http_ftp_url
    }
}

impl Validated for HttpFtpUrl {}

impl Debug for HttpFtpUrl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let debug_text = format!("HttpFtpUrl({:?})", self.full_http_ftp_url);

        f.pad(&debug_text)
    }
}

impl Display for HttpFtpUrl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.full_http_ftp_url)?;
        Ok(())
    }
}

impl PartialEq for HttpFtpUrl {
    fn eq(&self, other: &Self) -> bool {
        self.full_http_ftp_url.eq(&other.full_http_ftp_url)
    }

    fn ne(&self, other: &Self) -> bool {
        self.full_http_ftp_url.ne(&other.full_http_ftp_url)
    }
}

impl Eq for HttpFtpUrl {}

impl Hash for HttpFtpUrl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_http_ftp_url.hash(state)
    }
}

impl HttpFtpUrlValidator {
    pub fn is_http_ftp_url(&self, full_http_ftp_url: &str) -> bool {
        self.parse_inner(full_http_ftp_url).is_ok()
    }

    pub fn parse_string(&self, full_http_ftp_url: String) -> HttpFtpUrlResult {
        let mut http_ftp_url_inner = self.parse_inner(&full_http_ftp_url)?;

        http_ftp_url_inner.full_http_ftp_url = full_http_ftp_url;

        Ok(http_ftp_url_inner)
    }

    pub fn parse_str(&self, full_http_ftp_url: &str) -> HttpFtpUrlResult {
        let mut http_ftp_url_inner = self.parse_inner(full_http_ftp_url)?;

        http_ftp_url_inner.full_http_ftp_url.push_str(full_http_ftp_url);

        Ok(http_ftp_url_inner)
    }

    fn parse_inner(&self, full_http_ftp_url: &str) -> HttpFtpUrlResult {
        let c = match HTTP_FTP_URL_RE.captures(&full_http_ftp_url) {
            Some(c) => c,
            None => return Err(HttpFtpUrlError::LocalNotFound)
        };

        let full_http_ftp_url_len = full_http_ftp_url.len();

        let is_local;
        let mut is_https = false;
        let mut is_http = false;
        let mut is_ftp = false;

        let protocol = match c.get(2) {
            Some(m) => {
                if self.protocol.not_allow() {
                    return Err(HttpFtpUrlError::ProtocolNotAllow);
                }

                let e = m.end();

                match &full_http_ftp_url[(e - 1)..e] {
                    "s" => {
                        is_https = true;
                    }
                    _ => {
                        match &full_http_ftp_url[..1] {
                            "h" => {
                                is_http = true;
                            }
                            _ => {
                                is_ftp = true;
                            }
                        }
                    }
                }

                0
            }
            None => {
                if self.protocol.must() {
                    return Err(HttpFtpUrlError::ProtocolNotFound);
                }

                full_http_ftp_url_len
            }
        };

        let is_absolute = c.get(3).is_some();

        let host;

        let host_index = match c.get(4) {
            Some(m) => {
                let host_localable = HostLocalable::from_str(&full_http_ftp_url[m.start()..m.end()]).map_err(|err| HttpFtpUrlError::IncorrectHostFormat(err))?;

                match self.local {
                    ValidatorOption::Must => {
                        if !host_localable.is_local() {
                            return Err(HttpFtpUrlError::LocalNotFound);
                        }
                    }
                    ValidatorOption::NotAllow => {
                        if host_localable.is_local() {
                            return Err(HttpFtpUrlError::LocalNotAllow);
                        }
                    }
                    _ => {}
                }

                is_local = host_localable.is_local();

                host = host_localable.into_host();

                m.start()
            }
            None => {
                unreachable!();
            }
        };

        let path = match c.get(5) {
            Some(m) => {
                m.start()
            }
            None => {
                full_http_ftp_url_len
            }
        };

        let query = match c.get(7) {
            Some(m) => {
                m.start()
            }
            None => {
                full_http_ftp_url_len
            }
        };

        let fragment = match c.get(9) {
            Some(m) => {
                m.start()
            }
            None => {
                full_http_ftp_url_len
            }
        };


        Ok(HttpFtpUrl {
            protocol,
            host,
            host_index,
            path,
            query,
            fragment,
            full_http_ftp_url: String::new(),
            full_http_ftp_url_len,
            is_https,
            is_http,
            is_ftp,
            is_local,
            is_absolute,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_ftp_url_methods() {
        let url = "ftp://magiclen.org:8080/path/to/something?a=1&b=2#12345".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        let http_ftp_url = huv.parse_string(url).unwrap();

        assert_eq!("ftp://magiclen.org:8080/path/to/something?a=1&b=2#12345", http_ftp_url.get_full_http_ftp_url());
        assert_eq!("ftp://magiclen.org:8080/path/to/something", http_ftp_url.get_full_http_ftp_url_without_query_and_fragment());
        assert_eq!("ftp", http_ftp_url.get_protocol().unwrap());
        assert_eq!("magiclen.org:8080", http_ftp_url.get_host().get_full_host());
        assert_eq!("/path/to/something", http_ftp_url.get_path().unwrap());
        assert_eq!("a=1&b=2", http_ftp_url.get_query().unwrap());
        assert_eq!("12345", http_ftp_url.get_fragment().unwrap());
        assert_eq!(false, http_ftp_url.is_local());
        assert_eq!(true, http_ftp_url.is_ftp());
        assert_eq!(true, http_ftp_url.is_absolute());
    }

    #[test]
    fn test_http_ftp_url_lv1_1() {
        let url = "http://magiclen.org".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_ftp_url_lv1_2() {
        let url = "http://localhost".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::Allow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_ftp_url_lv1_3() {
        let url = "http://127.0.0.1".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::Allow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_ftp_url_lv2() {
        let url = "//magiclen.org".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_ftp_url_lv3() {
        let url = "magiclen.org".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_ftp_url_lv4_1() {
        let url = "https://magiclen.org/path/to/something".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_ftp_url_lv4_2() {
        let url = "https://localhost/path/to/something".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::Allow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_ftp_url_lv4_3() {
        let url = "https://127.0.0.1/path/to/something".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::Allow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_ftp_url_lv5() {
        let url = "https://magiclen.org/path/to/something".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_ftp_url_lv6() {
        let url = "https://magiclen.org/path/to/something?a=1".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_ftp_url_lv7() {
        let url = "HTTPS://magiclen.org/path/to/something?a=1".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_ftp_url_lv8() {
        let url = "HttPS://magiclen.org/path/to/something?a=1&b=2#12345".to_string();

        let huv = HttpFtpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }
}

// TODO ----------

macro_rules! extend {
    ( $name:ident, $protocol:expr, $local:expr ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name(HttpFtpUrl);

        impl From<$name> for HttpFtpUrl {
            fn from(d: $name) -> Self {
                d.0
            }
        }

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0.full_http_ftp_url
            }
        }

        impl Validated for $name {}

        impl ValidatedWrapper for $name {
            type Error = HttpFtpUrlError;

            fn from_string(full_http_ftp_url: String) -> Result<Self, Self::Error> {
                $name::from_string(full_http_ftp_url)
            }

            fn from_str(full_http_ftp_url: &str) -> Result<Self, Self::Error> {
                $name::from_str(full_http_ftp_url)
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.0))?;
                Ok(())
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl $name {
            pub fn from_string(full_http_ftp_url: String) -> Result<$name, HttpFtpUrlError> {
                Ok($name($name::create_validator().parse_string(full_http_ftp_url)?))
            }

            pub fn from_str(full_http_ftp_url: &str) -> Result<$name, HttpFtpUrlError> {
                Ok($name($name::create_validator().parse_str(full_http_ftp_url)?))
            }

            pub fn from_http_ftp_url(http_ftp_url: HttpFtpUrl) -> Result<$name, HttpFtpUrlError> {
                 match $protocol {
                    ValidatorOption::Must => {
                        if http_ftp_url.protocol == http_ftp_url.full_http_ftp_url_len {
                            return Err(HttpFtpUrlError::ProtocolNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if http_ftp_url.protocol == http_ftp_url.full_http_ftp_url_len {
                            return Err(HttpFtpUrlError::ProtocolNotAllow)
                        }
                    }
                    _=>()
                }
                match $local {
                    ValidatorOption::Must => {
                        if !http_ftp_url.is_local {
                            return Err(HttpFtpUrlError::LocalNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if http_ftp_url.is_local {
                            return Err(HttpFtpUrlError::LocalNotAllow)
                        }
                    }
                    _=>()
                }

                Ok($name(http_ftp_url))
            }

            pub fn into_http_ftp_url(self) -> HttpFtpUrl {
                self.0
            }

            pub fn as_http_ftp_url(&self) -> &HttpFtpUrl {
                &self.0
            }

            fn create_validator() -> HttpFtpUrlValidator {
                HttpFtpUrlValidator {
                    protocol: $protocol,
                    local: $local,
                }
            }
        }

        impl $name {
            pub fn get_host(&self) -> &Host {
                &self.0.host
            }

            pub fn get_path(&self) -> Option<&str> {
                if self.0.path != self.0.full_http_ftp_url_len {
                    if self.0.query != self.0.full_http_ftp_url_len {
                        Some(&self.0.full_http_ftp_url[self.0.path..(self.0.query - 1)])
                    } else {
                        if self.0.fragment != self.0.full_http_ftp_url_len {
                            Some(&self.0.full_http_ftp_url[self.0.path..(self.0.fragment - 1)])
                        } else {
                            Some(&self.0.full_http_ftp_url[self.0.path..])
                        }
                    }
                } else {
                    None
                }
            }

            pub fn get_query(&self) -> Option<&str> {
                if self.0.query != self.0.full_http_ftp_url_len {
                    if self.0.fragment != self.0.full_http_ftp_url_len {
                        Some(&self.0.full_http_ftp_url[self.0.query..(self.0.fragment - 1)])
                    } else {
                        Some(&self.0.full_http_ftp_url[self.0.query..])
                    }
                } else {
                    None
                }
            }

            pub fn get_fragment(&self) -> Option<&str> {
                if self.0.fragment != self.0.full_http_ftp_url_len {
                    Some(&self.0.full_http_ftp_url[self.0.fragment..])
                } else {
                    None
                }
            }

            pub fn get_full_http_ftp_url(&self) -> &str {
                &self.0.full_http_ftp_url
            }

            pub fn get_full_http_ftp_url_without_query_and_fragment(&self) -> &str {
                if self.0.query != self.0.full_http_ftp_url_len {
                    &self.0.full_http_ftp_url[..(self.0.query - 1)]
                } else {
                    if self.0.fragment != self.0.full_http_ftp_url_len {
                        &self.0.full_http_ftp_url[..(self.0.fragment - 1)]
                    } else {
                        &self.0.full_http_ftp_url
                    }
                }
            }

            pub fn is_absolute(&self) -> bool {
                self.0.is_absolute
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromFormValue<'a> for $name {
            type Error = HttpFtpUrlError;

            fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
                $name::from_string(form_value.url_decode().map_err(|err| HttpFtpUrlError::UTF8Error(err))?)
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromParam<'a> for $name {
            type Error = HttpFtpUrlError;

            fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
                $name::from_string(param.url_decode().map_err(|err| HttpFtpUrlError::UTF8Error(err))?)
            }
        }

        #[cfg(feature = "serdely")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct StringVisitor;

                impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_fmt(format_args!("a HTTP URL({:?}) string", $name::create_validator()))
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        $name::from_str(v).map_err(|err| {
                            E::custom(err.to_string())
                        })
                    }

                    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        $name::from_string(v).map_err(|err| {
                            E::custom(err.to_string())
                        })
                    }
                }

                deserializer.deserialize_string(StringVisitor)
            }
        }

        #[cfg(feature = "serdely")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
                serializer.serialize_str(self.get_full_http_ftp_url())
            }
        }
    };
}

extend!(HttpFtpUrlLocalableWithProtocol, ValidatorOption::Must, ValidatorOption::Allow);

impl HttpFtpUrlLocalableWithProtocol {
    pub fn get_protocol(&self) -> &str {
        if self.0.is_absolute {
            &self.0.full_http_ftp_url[..(self.0.host_index - 3)]
        } else {
            &self.0.full_http_ftp_url[..(self.0.host_index - 1)]
        }
    }

    pub fn is_https(&self) -> bool {
        self.0.is_https
    }

    pub fn is_http(&self) -> bool {
        self.0.is_http
    }

    pub fn is_ftp(&self) -> bool {
        self.0.is_ftp
    }

    pub fn is_local(&self) -> bool {
        self.0.is_local
    }
}

extend!(HttpFtpUrlUnlocalableWithProtocol, ValidatorOption::Must, ValidatorOption::NotAllow);

impl HttpFtpUrlUnlocalableWithProtocol {
    pub fn get_protocol(&self) -> &str {
        if self.0.is_absolute {
            &self.0.full_http_ftp_url[..(self.0.host_index - 3)]
        } else {
            &self.0.full_http_ftp_url[..(self.0.host_index - 1)]
        }
    }

    pub fn is_https(&self) -> bool {
        self.0.is_https
    }

    pub fn is_http(&self) -> bool {
        self.0.is_http
    }

    pub fn is_ftp(&self) -> bool {
        self.0.is_ftp
    }
}

extend!(HttpFtpUrlLocalableWithoutProtocol, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl HttpFtpUrlLocalableWithoutProtocol {
    pub fn is_local(&self) -> bool {
        self.0.is_local
    }
}

extend!(HttpFtpUrlUnlocalableWithoutProtocol, ValidatorOption::NotAllow, ValidatorOption::NotAllow);

impl HttpFtpUrlUnlocalableWithoutProtocol {}