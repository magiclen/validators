extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::{FromStr, Utf8Error};

lazy_static! {
    static ref VERSION_RE: Regex =
        { Regex::new(r"^(\d)+(\.(\d)+)?(\.(\d)+)?(-([^\x00-\x1F\x7F]+))?$").unwrap() };
}

#[derive(Debug, PartialEq, Clone)]
pub enum VersionError {
    IncorrectFormat,
    UTF8Error(Utf8Error),
}

impl Display for VersionError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for VersionError {}

impl From<Utf8Error> for VersionError {
    #[inline]
    fn from(err: Utf8Error) -> Self {
        VersionError::UTF8Error(err)
    }
}

pub type VersionResult = Result<Version, VersionError>;

#[derive(Debug, PartialEq)]
pub struct VersionValidator {}

#[derive(Clone)]
pub struct Version {
    full_version: String,
    major: u16,
    minor: Option<u16>,
    patch: Option<u16>,
    label: Option<usize>,
}

impl Version {
    #[inline]
    pub fn get_full_version(&self) -> &str {
        &self.full_version
    }

    #[inline]
    pub fn get_full_version_without_label(&self) -> &str {
        if let Some(label) = self.label {
            &self.full_version[..(label - 1)]
        } else {
            &self.full_version
        }
    }

    #[inline]
    pub fn get_label(&self) -> Option<&str> {
        if let Some(label) = self.label {
            Some(&self.full_version[label..])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_major(&self) -> u16 {
        self.major
    }

    #[inline]
    pub fn get_minor(&self) -> Option<u16> {
        self.minor
    }

    #[inline]
    pub fn get_patch(&self) -> Option<u16> {
        self.patch
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.full_version
    }
}

impl Deref for Version {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.full_version
    }
}

impl Validated for Version {}

impl Debug for Version {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(Version, f, self, let .0 = self.full_version);
    }
}

impl Display for Version {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.full_version)?;
        Ok(())
    }
}

impl PartialEq for Version {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.full_version.eq(&other.full_version)
    }
}

impl Eq for Version {}

impl Hash for Version {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_version.hash(state)
    }
}

impl VersionValidator {
    #[inline]
    pub fn is_short_crypt_url_component_url(&self, short_crypt_url_component_url: &str) -> bool {
        self.parse_inner(short_crypt_url_component_url).is_ok()
    }

    #[inline]
    pub fn parse_string(&self, full_version: String) -> VersionResult {
        let mut full_version_inner = self.parse_inner(&full_version)?;

        full_version_inner.full_version = full_version;

        Ok(full_version_inner)
    }

    #[inline]
    pub fn parse_str(&self, full_version: &str) -> VersionResult {
        let mut full_version_inner = self.parse_inner(full_version)?;

        full_version_inner.full_version.push_str(full_version);

        Ok(full_version_inner)
    }

    fn parse_inner(&self, full_version: &str) -> VersionResult {
        let c = match VERSION_RE.captures(full_version) {
            Some(c) => c,
            None => return Err(VersionError::IncorrectFormat),
        };

        let major = match c.get(1) {
            Some(m) => {
                match m.as_str().parse::<u16>() {
                    Ok(m) => m,
                    Err(_) => {
                        return Err(VersionError::IncorrectFormat);
                    }
                }
            }
            None => unreachable!(),
        };

        let minor = match c.get(3) {
            Some(m) => {
                match m.as_str().parse::<u16>() {
                    Ok(m) => Some(m),
                    Err(_) => {
                        return Err(VersionError::IncorrectFormat);
                    }
                }
            }
            None => None,
        };

        let patch = match c.get(5) {
            Some(m) => {
                match m.as_str().parse::<u16>() {
                    Ok(m) => Some(m),
                    Err(_) => {
                        return Err(VersionError::IncorrectFormat);
                    }
                }
            }
            None => None,
        };

        let label = match c.get(7) {
            Some(m) => Some(m.start()),
            None => None,
        };

        Ok(Version {
            full_version: String::new(),
            major,
            minor,
            patch,
            label,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_methods() {
        let version = "1.2.3-alpha.1".to_string();

        let vv = VersionValidator {};

        let version = vv.parse_string(version).unwrap();

        assert_eq!("1.2.3-alpha.1", version.get_full_version());
        assert_eq!("1.2.3", version.get_full_version_without_label());
        assert_eq!(1, version.get_major());
        assert_eq!(2, version.get_minor().unwrap());
        assert_eq!(3, version.get_patch().unwrap());
        assert_eq!("alpha.1", version.get_label().unwrap());
    }

    #[test]
    fn test_version_lv1() {
        let version = "0.1.2".to_string();

        let vv = VersionValidator {};

        vv.parse_string(version).unwrap();
    }
}

// Version's wrapper struct is itself
impl ValidatedWrapper for Version {
    type Error = VersionError;

    #[inline]
    fn from_string(full_version: String) -> Result<Self, Self::Error> {
        Version::from_string(full_version)
    }

    #[inline]
    fn from_str(full_version: &str) -> Result<Self, Self::Error> {
        Version::from_str(full_version)
    }
}

impl Version {
    #[inline]
    pub fn from_string(full_version: String) -> Result<Self, VersionError> {
        Version::create_validator().parse_string(full_version)
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(full_version: &str) -> Result<Self, VersionError> {
        Version::create_validator().parse_str(full_version)
    }

    #[inline]
    fn create_validator() -> VersionValidator {
        VersionValidator {}
    }
}

impl FromStr for Version {
    type Err = VersionError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Version::from_str(s)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for Version {
    type Error = VersionError;

    #[inline]
    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Version::from_string(form_value.url_decode()?)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for Version {
    type Error = VersionError;

    #[inline]
    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Version::from_string(param.url_decode()?)
    }
}

#[cfg(feature = "serdely")]
struct StringVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = Version;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a version string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        Version::from_str(v).map_err(|err| E::custom(err.to_string()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        Version::from_string(v).map_err(|err| E::custom(err.to_string()))
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for Version {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>, {
        deserializer.deserialize_string(StringVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for Version {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer, {
        serializer.serialize_str(&self.full_version)
    }
}
