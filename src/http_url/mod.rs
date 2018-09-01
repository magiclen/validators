extern crate regex;

use self::regex::Regex;
use super::{ValidatorOption, Validated};

use std::fmt::{self, Display, Debug, Formatter};

use super::host::{Host, HostLocalable, HostError};

#[derive(Debug, PartialEq, Clone)]
pub enum HttpUrlError {
    IncorrectFormat,
    IncorrectHostFormat(HostError),
    LocalNotAllow,
    LocalNotFound,
    ProtocolNotAllow,
    ProtocolNotFound,
}

pub type HttpUrlResult = Result<HttpUrl, HttpUrlError>;

pub struct HttpUrlValidator {
    pub local: ValidatorOption,
    pub protocol: ValidatorOption,
}

#[derive(Clone)]
pub struct HttpUrl {
    protocol: usize,
    host: Host,
    host_index: usize,
    path: usize,
    query: usize,
    fragment: usize,
    full_http_url: String,
    full_http_url_len: usize,
    is_https: bool,
    is_local: bool,
    is_absolute: bool,
}

impl HttpUrl {
    pub fn get_protocol(&self) -> Option<&str> {
        if self.protocol != self.full_http_url_len {
            if self.is_absolute {
                Some(&self.full_http_url[..(self.host_index - 3)])
            } else {
                Some(&self.full_http_url[..(self.host_index - 1)])
            }
        } else {
            None
        }
    }

    pub fn get_host(&self) -> &Host {
        &self.host
    }

    pub fn get_path(&self) -> Option<&str> {
        if self.path != self.full_http_url_len {
            if self.query != self.full_http_url_len {
                Some(&self.full_http_url[self.path..(self.query - 1)])
            } else {
                if self.fragment != self.full_http_url_len {
                    Some(&self.full_http_url[self.path..(self.fragment - 1)])
                } else {
                    Some(&self.full_http_url[self.path..])
                }
            }
        } else {
            None
        }
    }

    pub fn get_query(&self) -> Option<&str> {
        if self.query != self.full_http_url_len {
            if self.fragment != self.full_http_url_len {
                Some(&self.full_http_url[self.query..(self.fragment - 1)])
            } else {
                Some(&self.full_http_url[self.query..])
            }
        } else {
            None
        }
    }

    pub fn get_fragment(&self) -> Option<&str> {
        if self.fragment != self.full_http_url_len {
            Some(&self.full_http_url[self.fragment..])
        } else {
            None
        }
    }

    pub fn get_full_http_url(&self) -> &str {
        &self.full_http_url
    }

    pub fn get_full_http_url_without_query_and_fragment(&self) -> &str {
        if self.query != self.full_http_url_len {
            &self.full_http_url[..(self.query - 1)]
        } else {
            if self.fragment != self.full_http_url_len {
                &self.full_http_url[..(self.fragment - 1)]
            } else {
                &self.full_http_url
            }
        }
    }

    pub fn is_https(&self) -> bool {
        self.is_https
    }

    pub fn is_local(&self) -> bool {
        self.is_local
    }
}

impl Validated for HttpUrl {}

impl Debug for HttpUrl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("HttpUrl({})", self.full_http_url))?;
        Ok(())
    }
}

impl Display for HttpUrl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.full_http_url)?;
        Ok(())
    }
}

impl PartialEq for HttpUrl {
    fn eq(&self, other: &Self) -> bool {
        self.full_http_url.eq(&other.full_http_url)
    }

    fn ne(&self, other: &Self) -> bool {
        self.full_http_url.ne(&other.full_http_url)
    }
}

impl HttpUrlValidator {
    pub fn is_http_url(&self, full_http_url: &str) -> bool {
        self.parse_inner(full_http_url).is_ok()
    }

    pub fn parse_string(&self, full_http_url: String) -> HttpUrlResult {
        let mut http_url_inner = self.parse_inner(&full_http_url)?;

        http_url_inner.full_http_url = full_http_url;

        Ok(http_url_inner)
    }

    pub fn parse_str(&self, full_http_url: &str) -> HttpUrlResult {
        let mut http_url_inner = self.parse_inner(full_http_url)?;

        http_url_inner.full_http_url = full_http_url.to_string();

        Ok(http_url_inner)
    }

    fn parse_inner(&self, full_http_url: &str) -> HttpUrlResult {
        let re = Regex::new(r"^((http|https):)?(//)?([\S&&[^/]]+)(/[\S&&[^?#]]*)?([?]([\S&&[^#]]*))?(#([\S]*))?$").unwrap();

        let c = match re.captures(&full_http_url) {
            Some(c) => c,
            None => return Err(HttpUrlError::LocalNotFound)
        };

        let full_http_url_len = full_http_url.len();

        let is_local;
        let mut is_https = false;

        let protocol = match c.get(2) {
            Some(m) => {
                if self.protocol.not_allow() {
                    return Err(HttpUrlError::ProtocolNotAllow);
                }

                let e = m.end();
                is_https = full_http_url[(e - 1)..e].eq("s");

                0
            }
            None => {
                if self.protocol.must() {
                    return Err(HttpUrlError::ProtocolNotFound);
                }

                full_http_url_len
            }
        };

        let is_absolute = c.get(3).is_some();

        let host;

        let host_index = match c.get(4) {
            Some(m) => {
                let host_localable = HostLocalable::from_str(&full_http_url[m.start()..m.end()]).map_err(|err| HttpUrlError::IncorrectHostFormat(err))?;

                match self.local {
                    ValidatorOption::Must => {
                        if !host_localable.is_local() {
                            return Err(HttpUrlError::LocalNotFound);
                        }
                    }
                    ValidatorOption::NotAllow => {
                        if host_localable.is_local() {
                            return Err(HttpUrlError::LocalNotAllow);
                        }
                    }
                    _ => {}
                }

                is_local = host_localable.is_local();

                host = host_localable.into_host();

                m.start()
            }
            None => {
                panic!("impossible");
            }
        };

        let path = match c.get(5) {
            Some(m) => {
                m.start()
            }
            None => {
                full_http_url_len
            }
        };

        let query = match c.get(7) {
            Some(m) => {
                m.start()
            }
            None => {
                full_http_url_len
            }
        };

        let fragment = match c.get(9) {
            Some(m) => {
                m.start()
            }
            None => {
                full_http_url_len
            }
        };


        Ok(HttpUrl {
            protocol,
            host,
            host_index,
            path,
            query,
            fragment,
            full_http_url: String::new(),
            full_http_url_len,
            is_https,
            is_local,
            is_absolute,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_url_methods() {
        let url = "https://magiclen.org:8080/path/to/something?a=1&b=2#12345".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        let http_url = huv.parse_string(url).unwrap();

        assert_eq!("https://magiclen.org:8080/path/to/something?a=1&b=2#12345", http_url.get_full_http_url());
        assert_eq!("https://magiclen.org:8080/path/to/something", http_url.get_full_http_url_without_query_and_fragment());
        assert_eq!("https", http_url.get_protocol().unwrap());
        assert_eq!("magiclen.org:8080", http_url.get_host().get_full_host());
        assert_eq!("/path/to/something", http_url.get_path().unwrap());
        assert_eq!("a=1&b=2", http_url.get_query().unwrap());
        assert_eq!("12345", http_url.get_fragment().unwrap());
        assert_eq!(false, http_url.is_local());
        assert_eq!(true, http_url.is_https());
    }

    #[test]
    fn test_http_url_lv1_1() {
        let url = "http://magiclen.org".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_url_lv1_2() {
        let url = "http://localhost".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::Allow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_url_lv1_3() {
        let url = "http://127.0.0.1".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::Allow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_url_lv2() {
        let url = "//magiclen.org".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_url_lv3() {
        let url = "magiclen.org".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_url_lv4_1() {
        let url = "https://magiclen.org/path/to/something".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_url_lv4_2() {
        let url = "https://localhost/path/to/something".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::Allow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_url_lv4_3() {
        let url = "https://127.0.0.1/path/to/something".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::Allow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_url_lv5() {
        let url = "https://magiclen.org/path/to/something".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_url_lv6() {
        let url = "https://magiclen.org/path/to/something?a=1".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_url_lv7() {
        let url = "https://magiclen.org/path/to/something?a=1".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }

    #[test]
    fn test_http_url_lv8() {
        let url = "https://magiclen.org/path/to/something?a=1&b=2#12345".to_string();

        let huv = HttpUrlValidator {
            local: ValidatorOption::NotAllow,
            protocol: ValidatorOption::Allow,
        };

        huv.parse_string(url).unwrap();
    }
}

macro_rules! extend {
    ( $name:ident, $protocol:expr, $local:expr ) => {
        #[derive(Clone)]
        pub struct $name(HttpUrl);

        impl From<$name> for HttpUrl {
            fn from(d: $name) -> Self {
                d.0
            }
        }

        impl Validated for $name {}

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

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }

            fn ne(&self, other: &Self) -> bool {
                self.0.ne(&other.0)
            }
        }

        impl PartialEq<HttpUrl> for $name {
            fn eq(&self, other: &HttpUrl) -> bool {
                self.0.eq(&other)
            }

            fn ne(&self, other: &HttpUrl) -> bool {
                self.0.ne(&other)
            }
        }

        impl $name {
            pub fn from_string(full_http_url: String) -> Result<$name, HttpUrlError> {
                let huv = HttpUrlValidator {
                    protocol: $protocol,
                    local: $local,
                };

                Ok($name(huv.parse_string(full_http_url)?))
            }

            pub fn from_str(full_http_url: &str) -> Result<$name, HttpUrlError> {
                let huv = HttpUrlValidator {
                    protocol: $protocol,
                    local: $local,
                };

                Ok($name(huv.parse_str(full_http_url)?))
            }

            pub fn from_http_url(http_url: HttpUrl) -> Result<$name, HttpUrlError> {
                 match $protocol {
                    ValidatorOption::Must => {
                        if http_url.protocol == http_url.full_http_url_len {
                            return Err(HttpUrlError::ProtocolNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if http_url.protocol == http_url.full_http_url_len {
                            return Err(HttpUrlError::ProtocolNotAllow)
                        }
                    }
                    _=>()
                }
                match $local {
                    ValidatorOption::Must => {
                        if !http_url.is_local {
                            return Err(HttpUrlError::LocalNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if http_url.is_local {
                            return Err(HttpUrlError::LocalNotAllow)
                        }
                    }
                    _=>()
                }

                Ok($name(http_url))
            }

            pub fn into_http_url(self) -> HttpUrl {
                self.0
            }

            pub fn as_http_url(&self) -> &HttpUrl {
                &self.0
            }
        }

        impl $name {
            pub fn get_host(&self) -> &Host {
                &self.0.host
            }

            pub fn get_path(&self) -> Option<&str> {
                if self.0.path != self.0.full_http_url_len {
                    if self.0.query != self.0.full_http_url_len {
                        Some(&self.0.full_http_url[self.0.path..(self.0.query - 1)])
                    } else {
                        if self.0.fragment != self.0.full_http_url_len {
                            Some(&self.0.full_http_url[self.0.path..(self.0.fragment - 1)])
                        } else {
                            Some(&self.0.full_http_url[self.0.path..])
                        }
                    }
                } else {
                    None
                }
            }

            pub fn get_query(&self) -> Option<&str> {
                if self.0.query != self.0.full_http_url_len {
                    if self.0.fragment != self.0.full_http_url_len {
                        Some(&self.0.full_http_url[self.0.query..(self.0.fragment - 1)])
                    } else {
                        Some(&self.0.full_http_url[self.0.query..])
                    }
                } else {
                    None
                }
            }

            pub fn get_fragment(&self) -> Option<&str> {
                if self.0.fragment != self.0.full_http_url_len {
                    Some(&self.0.full_http_url[self.0.fragment..])
                } else {
                    None
                }
            }

            pub fn get_full_http_url(&self) -> &str {
                &self.0.full_http_url
            }

            pub fn get_full_http_url_without_query_and_fragment(&self) -> &str {
                if self.0.query != self.0.full_http_url_len {
                    &self.0.full_http_url[..(self.0.query - 1)]
                } else {
                    if self.0.fragment != self.0.full_http_url_len {
                        &self.0.full_http_url[..(self.0.fragment - 1)]
                    } else {
                        &self.0.full_http_url
                    }
                }
            }

            pub fn is_https(&self) -> bool {
                self.0.is_https
            }
        }

         #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromFormValue<'a> for $name {
            type Error = HttpUrlError;

            fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_str(form_value)
            }
        }
    };
}

extend!(HttpUrlLocalableWithProtocol, ValidatorOption::Must, ValidatorOption::Allow);

impl HttpUrlLocalableWithProtocol {
    pub fn get_protocol(&self) -> &str {
        if self.0.is_absolute {
            &self.0.full_http_url[..(self.0.host_index - 3)]
        } else {
            &self.0.full_http_url[..(self.0.host_index - 1)]
        }
    }

    pub fn is_local(&self) -> bool {
        self.0.is_local
    }
}

extend!(HttpUrlUnlocalableWithProtocol, ValidatorOption::Must, ValidatorOption::NotAllow);

impl HttpUrlUnlocalableWithProtocol {
    pub fn get_protocol(&self) -> &str {
        if self.0.is_absolute {
            &self.0.full_http_url[..(self.0.host_index - 3)]
        } else {
            &self.0.full_http_url[..(self.0.host_index - 1)]
        }
    }
}