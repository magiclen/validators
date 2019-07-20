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

#[derive(Debug, PartialEq, Clone)]
pub enum JSONError {
    IncorrectJSON,
    UTF8Error(Utf8Error),
}

impl Display for JSONError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for JSONError {}

pub type JSONResult = Result<JSON, JSONError>;

#[derive(Debug, PartialEq)]
pub struct JSONValidator {}

#[derive(Clone)]
pub struct JSON {
    value: Value,
    full_json: String,
}

impl JSON {
    pub fn get_json_value(&self) -> &Value {
        &self.value
    }

    pub fn get_full_json(&self) -> &str {
        &self.full_json
    }

    pub fn into_value(self) -> Value {
        self.value
    }

    pub fn into_string(self) -> String {
        self.full_json
    }
}

impl Deref for JSON {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for JSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Validated for JSON {}

impl Debug for JSON {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(JSON, f, self, let .0 = self.value);
    }
}

impl Display for JSON {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl PartialEq for JSON {
    fn eq(&self, other: &Self) -> bool {
        self.full_json.eq(&other.full_json)
    }

    fn ne(&self, other: &Self) -> bool {
        self.full_json.ne(&other.full_json)
    }
}

impl Eq for JSON {}

impl Hash for JSON {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_json.hash(state);
    }
}

impl JSONValidator {
    pub fn is_json(&self, full_json: &str) -> bool {
        self.parse_inner(full_json).is_ok()
    }

    pub fn parse_string(&self, full_json: String) -> JSONResult {
        let mut json_inner = self.parse_inner(&full_json)?;

        json_inner.full_json = full_json;

        Ok(json_inner)
    }

    pub fn parse_str(&self, full_json: &str) -> JSONResult {
        let mut json_inner = self.parse_inner(full_json)?;

        json_inner.full_json.push_str(full_json);

        Ok(json_inner)
    }

    fn parse_inner(&self, full_json: &str) -> JSONResult {
        let value: Value = match serde_json::from_str(full_json) {
            Ok(json) => json,
            Err(_) => return Err(JSONError::IncorrectJSON)
        };

        Ok(JSON {
            value,
            full_json: String::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_lv1() {
        let json = "123".to_string();

        let jo = JSONValidator {};

        jo.parse_string(json).unwrap();
    }

    #[test]
    fn test_json_lv2() {
        let json = "\"123\"".to_string();

        let jo = JSONValidator {};

        jo.parse_string(json).unwrap();
    }

    #[test]
    fn test_json_lv3() {
        let json = "{\"id\": 1, \"name\": \"Magic Len\"}".to_string();

        let jo = JSONValidator {};

        jo.parse_string(json).unwrap();
    }

    #[test]
    fn test_json_lv4() {
        let json = "[1, \"Magic Len\"]".to_string();

        let jo = JSONValidator {};

        jo.parse_string(json).unwrap();
    }
}

// JSON's wrapper struct is itself
impl ValidatedWrapper for JSON {
    type Error = JSONError;

    fn from_string(json: String) -> Result<Self, Self::Error> {
        JSON::from_string(json)
    }

    fn from_str(json: &str) -> Result<Self, Self::Error> {
        JSON::from_str(json)
    }
}

impl JSON {
    pub fn from_string(full_json: String) -> Result<Self, JSONError> {
        JSON::create_validator().parse_string(full_json)
    }

    pub fn from_str(full_json: &str) -> Result<Self, JSONError> {
        JSON::create_validator().parse_str(full_json)
    }

    fn create_validator() -> JSONValidator {
        JSONValidator {}
    }
}


#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for JSON {
    type Error = JSONError;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        JSON::from_string(form_value.url_decode().map_err(|err| JSONError::UTF8Error(err))?)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for JSON {
    type Error = JSONError;

    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        JSON::from_string(param.url_decode().map_err(|err| JSONError::UTF8Error(err))?)
    }
}

struct StringVisitor;

impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = JSON;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an JSON string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
        JSON::from_str(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: ::serde::de::Error {
        JSON::from_string(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

impl<'de> ::serde::Deserialize<'de> for JSON {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        deserializer.deserialize_string(StringVisitor)
    }
}

impl ::serde::Serialize for JSON {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        self.value.serialize(serializer)
    }
}