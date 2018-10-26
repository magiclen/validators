extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};

lazy_static! {
    static ref UUID_RE: Regex = {
        Regex::new("^[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}$").unwrap()
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum UUIDError {
    IncorrectFormat,
}

impl Display for UUIDError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for UUIDError {}

pub type UUIDResult = Result<UUID, UUIDError>;

#[derive(Debug, PartialEq)]
pub struct UUIDValidator {}

#[derive(Clone)]
pub struct UUID {
    uuid: String,
}

impl UUID {
    pub fn get_uuid(&self) -> &str {
        &self.uuid
    }
}

impl Validated for UUID {}

impl Debug for UUID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("UUID({})", self.uuid))?;
        Ok(())
    }
}

impl Display for UUID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.uuid)?;
        Ok(())
    }
}

impl PartialEq for UUID {
    fn eq(&self, other: &Self) -> bool {
        self.uuid.eq(&other.uuid)
    }

    fn ne(&self, other: &Self) -> bool {
        self.uuid.ne(&other.uuid)
    }
}

impl UUIDValidator {
    pub fn is_uuid(&self, uuid: &str) -> bool {
        self.parse_inner(uuid).is_ok()
    }

    pub fn parse_string(&self, uuid: String) -> UUIDResult {
        let mut uuid_inner = self.parse_inner(&uuid)?;

        uuid_inner.uuid = uuid;

        Ok(uuid_inner)
    }

    pub fn parse_str(&self, uuid: &str) -> UUIDResult {
        let mut uuid_inner = self.parse_inner(uuid)?;

        uuid_inner.uuid = uuid.to_string();

        Ok(uuid_inner)
    }

    fn parse_inner(&self, uuid: &str) -> UUIDResult {
        if UUID_RE.is_match(uuid) {
            Ok(UUID {
                uuid: String::new(),
            })
        } else {
            Err(UUIDError::IncorrectFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_methods() {
        let uuid = "80a6572b-ebb8-4bf8-94b8-5c198299d118".to_string();

        let uv = UUIDValidator {};

        let uuid = uv.parse_string(uuid).unwrap();

        assert_eq!("80a6572b-ebb8-4bf8-94b8-5c198299d118", uuid.get_uuid());
    }

    #[test]
    fn test_uuid_lv1() {
        let uuid = "80a6572b-ebb8-4bf8-94b8-5c198299d118".to_string();

        let bv = UUIDValidator {};

        bv.parse_string(uuid).unwrap();
    }
}

// UUID's wrapper struct is itself
impl ValidatedWrapper for UUID {
    type Error = UUIDError;

    fn from_string(uuid: String) -> Result<Self, Self::Error> {
        UUID::from_string(uuid)
    }

    fn from_str(uuid: &str) -> Result<Self, Self::Error> {
        UUID::from_str(uuid)
    }
}

impl UUID {
    pub fn from_string(uuid: String) -> Result<Self, UUIDError> {
        UUID::create_validator().parse_string(uuid)
    }

    pub fn from_str(uuid: &str) -> Result<Self, UUIDError> {
        UUID::create_validator().parse_str(uuid)
    }

    fn create_validator() -> UUIDValidator {
        UUIDValidator {}
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for UUID {
    type Error = UUIDError;

    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        UUID::from_str(param)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for UUID {
    type Error = UUIDError;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        UUID::from_str(form_value)
    }
}

#[cfg(feature = "serdely")]
struct StringVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = UUID;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a UUID string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
        UUID::from_str(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: ::serde::de::Error {
        UUID::from_string(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for UUID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        deserializer.deserialize_str(StringVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for UUID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        serializer.serialize_str(&self.uuid)
    }
}