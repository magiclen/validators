#![cfg(feature = "serdely")]
extern crate serde_json;

use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};
use std::str::Utf8Error;
use std::ops::Deref;
use std::ops::DerefMut;
use std::hash::{Hash, Hasher};

use self::serde_json::Value;
use self::serde_json::Map;

#[derive(Debug, PartialEq, Clone)]
pub enum JSONObjectError {
    IncorrectJSONObject,
    UTF8Error(Utf8Error),
}

impl Display for JSONObjectError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for JSONObjectError {}

pub type JSONObjectResult = Result<JSONObject, JSONObjectError>;

#[derive(Debug, PartialEq)]
pub struct JSONObjectValidator {}

#[derive(Clone)]
pub struct JSONObject {
    value: Value,
    full_json_object: String,
}

impl JSONObject {
    pub fn get_json_value(&self) -> &Value {
        &self.value
    }

    pub fn get_full_json_object(&self) -> &str {
        &self.full_json_object
    }

    pub fn into_map(self) -> Map<String, Value> {
        match self.value {
            Value::Object(map) => map,
            _ => unreachable!()
        }
    }

    pub fn into_value(self) -> Value {
        self.value
    }

    pub fn into_string(self) -> String {
        self.full_json_object
    }
}

impl Deref for JSONObject {
    type Target = Map<String, Value>;

    fn deref(&self) -> &Self::Target {
        self.value.as_object().unwrap()
    }
}

impl DerefMut for JSONObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_object_mut().unwrap()
    }
}

impl Validated for JSONObject {}

impl Debug for JSONObject {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

impl Display for JSONObject {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl PartialEq for JSONObject {
    fn eq(&self, other: &Self) -> bool {
        self.full_json_object.eq(&other.full_json_object)
    }

    fn ne(&self, other: &Self) -> bool {
        self.full_json_object.ne(&other.full_json_object)
    }
}

impl Eq for JSONObject {}

impl Hash for JSONObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_json_object.hash(state);
    }
}

impl JSONObjectValidator {
    pub fn is_json_object(&self, full_json_object: &str) -> bool {
        self.parse_inner(full_json_object).is_ok()
    }

    pub fn parse_string(&self, full_json_object: String) -> JSONObjectResult {
        let mut json_object_inner = self.parse_inner(&full_json_object)?;

        json_object_inner.full_json_object = full_json_object;

        Ok(json_object_inner)
    }

    pub fn parse_str(&self, full_json_object: &str) -> JSONObjectResult {
        let mut json_object_inner = self.parse_inner(full_json_object)?;

        json_object_inner.full_json_object.push_str(full_json_object);

        Ok(json_object_inner)
    }

    fn parse_inner(&self, full_json_object: &str) -> JSONObjectResult {
        let json_object: Map<String, Value> = match serde_json::from_str(full_json_object) {
            Ok(json_object) => json_object,
            Err(_) => return Err(JSONObjectError::IncorrectJSONObject)
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

    fn from_string(json_object: String) -> Result<Self, Self::Error> {
        JSONObject::from_string(json_object)
    }

    fn from_str(json_object: &str) -> Result<Self, Self::Error> {
        JSONObject::from_str(json_object)
    }
}

impl JSONObject {
    pub fn from_string(full_json_object: String) -> Result<Self, JSONObjectError> {
        JSONObject::create_validator().parse_string(full_json_object)
    }

    pub fn from_str(full_json_object: &str) -> Result<Self, JSONObjectError> {
        JSONObject::create_validator().parse_str(full_json_object)
    }

    fn create_validator() -> JSONObjectValidator {
        JSONObjectValidator {}
    }
}


#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for JSONObject {
    type Error = JSONObjectError;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        JSONObject::from_string(form_value.url_decode().map_err(|err| JSONObjectError::UTF8Error(err))?)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for JSONObject {
    type Error = JSONObjectError;

    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        JSONObject::from_string(param.url_decode().map_err(|err| JSONObjectError::UTF8Error(err))?)
    }
}

struct StringVisitor;

impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = JSONObject;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an JSONObject string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
        JSONObject::from_str(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: ::serde::de::Error {
        JSONObject::from_string(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

impl<'de> ::serde::Deserialize<'de> for JSONObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        deserializer.deserialize_string(StringVisitor)
    }
}

impl ::serde::Serialize for JSONObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        self.value.serialize(serializer)
    }
}