extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::{FromStr, Utf8Error};

lazy_static! {
    static ref URI_RE: Regex = {
        Regex::new(r"^(?i)([a-z][a-z0-9+.-]+):(//([^@]+@)?([a-z0-9.\-_~]+)(:\d+)?)?((?:[a-z0-9-._~]|%[a-f0-9]|[!$&'()*+,;=:@])+(?:/(?:[a-z0-9-._~]|%[a-f0-9]|[!$&'()*+,;=:@])*)*|(?:/(?:[a-z0-9-._~]|%[a-f0-9]|[!$&'()*+,;=:@])+)*)?(\?(?:[a-z0-9-._~]|%[a-f0-9]|[!$&'()*+,;=:@]|[/?])+)?(\#(?:[a-z0-9-._~]|%[a-f0-9]|[!$&'()*+,;=:@]|[/?])+)?$").unwrap()
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum URIError {
    IncorrectFormat,
    UTF8Error(Utf8Error),
}

impl Display for URIError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for URIError {}

impl From<Utf8Error> for URIError {
    #[inline]
    fn from(err: Utf8Error) -> Self {
        URIError::UTF8Error(err)
    }
}

pub type URIResult = Result<URI, URIError>;

#[derive(Debug, PartialEq)]
pub struct URIValidator {}

#[derive(Clone)]
pub struct URI {
    full_uri: String,
    scheme: (usize, usize),
    authority: Option<(usize, usize)>,
    user_info: Option<(usize, usize)>,
    host: Option<(usize, usize)>,
    port: Option<u16>,
    path: Option<(usize, usize)>,
    query: Option<(usize, usize)>,
    fragment: Option<(usize, usize)>,
}

impl URI {
    #[inline]
    pub fn get_full_uri(&self) -> &str {
        &self.full_uri
    }

    #[inline]
    pub fn get_scheme(&self) -> &str {
        &self.full_uri[self.scheme.0..self.scheme.1]
    }

    #[inline]
    pub fn get_authority(&self) -> Option<&str> {
        if let Some(authority) = self.authority {
            Some(&self.full_uri[authority.0..authority.1])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_user_info(&self) -> Option<&str> {
        if let Some(user_info) = self.user_info {
            Some(&self.full_uri[user_info.0..user_info.1])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_host(&self) -> Option<&str> {
        if let Some(host) = self.host {
            Some(&self.full_uri[host.0..host.1])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_port(&self) -> Option<u16> {
        self.port
    }

    #[inline]
    pub fn get_path(&self) -> Option<&str> {
        if let Some(path) = self.path {
            Some(&self.full_uri[path.0..path.1])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_query(&self) -> Option<&str> {
        if let Some(query) = self.query {
            Some(&self.full_uri[query.0..query.1])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_fragment(&self) -> Option<&str> {
        if let Some(fragment) = self.fragment {
            Some(&self.full_uri[fragment.0..fragment.1])
        } else {
            None
        }
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.full_uri
    }
}

impl Deref for URI {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.full_uri
    }
}

impl Validated for URI {}

impl Debug for URI {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(URI, f, self, let .0 = self.full_uri);
    }
}

impl Display for URI {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.full_uri)?;
        Ok(())
    }
}

impl PartialEq for URI {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.full_uri.eq(&other.full_uri)
    }
}

impl Eq for URI {}

impl Hash for URI {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_uri.hash(state)
    }
}

impl URIValidator {
    #[inline]
    pub fn is_uri(&self, full_uri: &str) -> bool {
        self.parse_inner(full_uri).is_ok()
    }

    #[inline]
    pub fn parse_string(&self, full_uri: String) -> URIResult {
        let mut uri_inner = self.parse_inner(&full_uri)?;

        uri_inner.full_uri = full_uri;

        Ok(uri_inner)
    }

    #[inline]
    pub fn parse_str(&self, full_uri: &str) -> URIResult {
        let mut uri_inner = self.parse_inner(full_uri)?;

        uri_inner.full_uri.push_str(full_uri);

        Ok(uri_inner)
    }

    fn parse_inner(&self, full_uri: &str) -> URIResult {
        let c = match URI_RE.captures(full_uri) {
            Some(c) => c,
            None => return Err(URIError::IncorrectFormat),
        };

        let scheme = match c.get(1) {
            Some(cc) => (cc.start(), cc.end()),
            None => unreachable!(),
        };

        let authority = match c.get(2) {
            Some(cc) => Some((cc.start() + 2, cc.end())),
            None => None,
        };

        let user_info = match c.get(3) {
            Some(cc) => Some((cc.start(), cc.end() - 1)),
            None => None,
        };

        let host = match c.get(4) {
            Some(cc) => Some((cc.start(), cc.end())),
            None => None,
        };

        let port = match c.get(5) {
            Some(cc) => Some(full_uri[(cc.start() + 1)..cc.end()].parse().unwrap()),
            None => None,
        };

        let path = match c.get(6) {
            Some(cc) => Some((cc.start(), cc.end())),
            None => None,
        };

        let query = match c.get(7) {
            Some(cc) => Some((cc.start() + 1, cc.end())),
            None => None,
        };

        let fragment = match c.get(8) {
            Some(cc) => Some((cc.start() + 1, cc.end())),
            None => None,
        };

        Ok(URI {
            full_uri: String::new(),
            scheme,
            authority,
            user_info,
            host,
            port,
            path,
            query,
            fragment,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_methods() {
        let uri = "ssh://root@127.0.0.1:886/path/to?query=1#fragment".to_string();

        let uv = URIValidator {};

        let uri = uv.parse_string(uri).unwrap();

        assert_eq!("ssh://root@127.0.0.1:886/path/to?query=1#fragment", uri.get_full_uri());
        assert_eq!("ssh", uri.get_scheme());
        assert_eq!("root@127.0.0.1:886", uri.get_authority().unwrap());
        assert_eq!("root", uri.get_user_info().unwrap());
        assert_eq!("127.0.0.1", uri.get_host().unwrap());
        assert_eq!(886, uri.get_port().unwrap());
        assert_eq!("/path/to", uri.get_path().unwrap());
        assert_eq!("query=1", uri.get_query().unwrap());
        assert_eq!("fragment", uri.get_fragment().unwrap());
    }
}

// URI's wrapper struct is itself
impl ValidatedWrapper for URI {
    type Error = URIError;

    #[inline]
    fn from_string(full_uri: String) -> Result<Self, Self::Error> {
        URI::from_string(full_uri)
    }

    #[inline]
    fn from_str(full_uri: &str) -> Result<Self, Self::Error> {
        URI::from_str(full_uri)
    }
}

impl URI {
    #[inline]
    pub fn from_string(full_uri: String) -> Result<Self, URIError> {
        URI::create_validator().parse_string(full_uri)
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(full_uri: &str) -> Result<Self, URIError> {
        URI::create_validator().parse_str(full_uri)
    }

    #[inline]
    fn create_validator() -> URIValidator {
        URIValidator {}
    }
}

impl FromStr for URI {
    type Err = URIError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        URI::from_str(s)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for URI {
    type Error = URIError;

    #[inline]
    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        URI::from_string(form_value.url_decode()?)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for URI {
    type Error = URIError;

    #[inline]
    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        URI::from_string(param.url_decode()?)
    }
}

#[cfg(feature = "serdely")]
struct StringVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = URI;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Base64 string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        URI::from_str(v).map_err(|err| E::custom(err.to_string()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        URI::from_string(v).map_err(|err| E::custom(err.to_string()))
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for URI {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>, {
        deserializer.deserialize_string(StringVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for URI {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer, {
        serializer.serialize_str(&self.full_uri)
    }
}
