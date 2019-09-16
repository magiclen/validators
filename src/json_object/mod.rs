#![cfg(feature = "serdely")]
extern crate serde_json;

use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ops::DerefMut;
use std::str::{FromStr, Utf8Error};

use self::serde_json::Map;
use self::serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum JSONObjectError {
    IncorrectJSONObject,
    UTF8Error(Utf8Error),
}

impl Display for JSONObjectError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for JSONObjectError {}

impl From<Utf8Error> for JSONObjectError {
    #[inline]
    fn from(err: Utf8Error) -> Self {
        JSONObjectError::UTF8Error(err)
    }
}

pub type JSONObjectResult = Result<JSONObject, JSONObjectError>;

#[derive(Debug, PartialEq)]
pub struct JSONObjectValidator {}

#[derive(Clone)]
pub struct JSONObject {
    value: Value,
    full_json_object: String,
}

impl JSONObject {
    #[inline]
    pub fn get_json_value(&self) -> &Value {
        &self.value
    }

    #[inline]
    pub fn get_full_json_object(&self) -> &str {
        &self.full_json_object
    }

    #[inline]
    pub fn into_map(self) -> Map<String, Value> {
        match self.value {
            Value::Object(map) => map,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn into_value(self) -> Value {
        self.value
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.full_json_object
    }
}

impl Deref for JSONObject {
    type Target = Map<String, Value>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.value.as_object().unwrap()
    }
}

impl DerefMut for JSONObject {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_object_mut().unwrap()
    }
}

impl Validated for JSONObject {}

impl Debug for JSONObject {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(JSONObject, f, self, let .0 = self.value);
    }
}

impl Display for JSONObject {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl PartialEq for JSONObject {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.full_json_object.eq(&other.full_json_object)
    }
}

impl Eq for JSONObject {}

impl Hash for JSONObject {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_json_object.hash(state);
    }
}

impl JSONObjectValidator {
    #[inline]
    pub fn is_json_object(&self, full_json_object: &str) -> bool {
        self.parse_inner(full_json_object).is_ok()
    }

    #[inline]
    pub fn parse_string(&self, full_json_object: String) -> JSONObjectResult {
        let mut json_object_inner = self.parse_inner(&full_json_object)?;

        json_object_inner.full_json_object = full_json_object;

        Ok(json_object_inner)
    }

    #[inline]
    pub fn parse_str(&self, full_json_object: &str) -> JSONObjectResult {
        let mut json_object_inner = self.parse_inner(full_json_object)?;

        json_object_inner.full_json_object.push_str(full_json_object);

        Ok(json_object_inner)
    }

    #[inline]
    fn parse_inner(&self, full_json_object: &str) -> JSONObjectResult {
        let json_object: Map<String, Value> = match serde_json::from_str(full_json_object) {
            Ok(json_object) => json_object,
            Err(_) => return Err(JSONObjectError::IncorrectJSONObject),
        };

        let value = Value::Object(json_object);

        Ok(JSONObject {
            value,
            full_json_object: String::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_object_lv1() {
        let json_object = "{\"id\": 1, \"name\": \"Magic Len\"}".to_string();

        let jo = JSONObjectValidator {};

        jo.parse_string(json_object).unwrap();
    }
}

// JSONObject's wrapper struct is itself
impl ValidatedWrapper for JSONObject {
    type Error = JSONObjectError;

    #[inline]
    fn from_string(json_object: String) -> Result<Self, Self::Error> {
        JSONObject::from_string(json_object)
    }

    #[inline]
    fn from_str(json_object: &str) -> Result<Self, Self::Error> {
        JSONObject::from_str(json_object)
    }
}

impl JSONObject {
    #[inline]
    pub fn from_string(full_json_object: String) -> Result<Self, JSONObjectError> {
        JSONObject::create_validator().parse_string(full_json_object)
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(full_json_object: &str) -> Result<Self, JSONObjectError> {
        JSONObject::create_validator().parse_str(full_json_object)
    }

    #[inline]
    fn create_validator() -> JSONObjectValidator {
        JSONObjectValidator {}
    }
}

impl FromStr for JSONObject {
    type Err = JSONObjectError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        JSONObject::from_str(s)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for JSONObject {
    type Error = JSONObjectError;

    #[inline]
    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        JSONObject::from_string(form_value.url_decode()?)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for JSONObject {
    type Error = JSONObjectError;

    #[inline]
    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        JSONObject::from_string(param.url_decode()?)
    }
}

struct StringVisitor;

impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = JSONObject;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an JSONObject string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        JSONObject::from_str(v).map_err(|err| E::custom(err.to_string()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        JSONObject::from_string(v).map_err(|err| E::custom(err.to_string()))
    }
}

impl<'de> ::serde::Deserialize<'de> for JSONObject {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>, {
        deserializer.deserialize_string(StringVisitor)
    }
}

impl ::serde::Serialize for JSONObject {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer, {
        self.value.serialize(serializer)
    }
}
