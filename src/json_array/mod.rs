#![cfg(feature = "serdely")]
extern crate serde_json;

use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};
use std::str::Utf8Error;
use std::ops::Deref;
use std::ops::DerefMut;

use self::serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum JSONArrayError {
    IncorrectJSONArray,
    UTF8Error(Utf8Error),
}

impl Display for JSONArrayError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for JSONArrayError {}

pub type JSONArrayResult = Result<JSONArray, JSONArrayError>;

#[derive(Debug, PartialEq)]
pub struct JSONArrayValidator {}

#[derive(Clone)]
pub struct JSONArray {
    value: Value
}

impl JSONArray {
    pub fn get_json_value(&self) -> &Value {
        &self.value
    }

    pub fn into_vec(self) -> Vec<Value> {
        match self.value {
            Value::Array(array) => array,
            _ => unreachable!()
        }
    }

    pub fn into_value(self) -> Value {
        self.value
    }
}

impl Deref for JSONArray {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        self.value.as_array().unwrap()
    }
}

impl DerefMut for JSONArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_array_mut().unwrap()
    }
}

impl Validated for JSONArray {}

impl Debug for JSONArray {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

impl Display for JSONArray {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl PartialEq for JSONArray {
    fn eq(&self, other: &Self) -> bool {
        self.get_json_value().eq(other.get_json_value())
    }

    fn ne(&self, other: &Self) -> bool {
        self.get_json_value().ne(other.get_json_value())
    }
}

impl JSONArrayValidator {
    pub fn is_json_array(&self, full_json_array: &str) -> bool {
        self.parse_inner(full_json_array).is_ok()
    }

    pub fn parse_string(&self, full_json_array: String) -> JSONArrayResult {
        let json_array_inner = self.parse_inner(&full_json_array)?;

        Ok(json_array_inner)
    }

    pub fn parse_str(&self, full_json_array: &str) -> JSONArrayResult {
        let json_array_inner = self.parse_inner(full_json_array)?;

        Ok(json_array_inner)
    }

    fn parse_inner(&self, full_json_array: &str) -> JSONArrayResult {
        let json_array: Vec<Value> = match serde_json::from_str(full_json_array) {
            Ok(json_array) => json_array,
            Err(_) => return Err(JSONArrayError::IncorrectJSONArray)
        };

        let value = Value::Array(json_array);

        Ok(JSONArray {
            value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_array_lv1() {
        let json_array = "[1, \"Magic Len\"]".to_string();

        let jo = JSONArrayValidator {};

        jo.parse_string(json_array).unwrap();
    }
}

// JSONArray's wrapper struct is itself
impl ValidatedWrapper for JSONArray {
    type Error = JSONArrayError;

    fn from_string(json_array: String) -> Result<Self, Self::Error> {
        JSONArray::from_string(json_array)
    }

    fn from_str(json_array: &str) -> Result<Self, Self::Error> {
        JSONArray::from_str(json_array)
    }
}

impl JSONArray {
    pub fn from_string(full_json_array: String) -> Result<Self, JSONArrayError> {
        JSONArray::create_validator().parse_string(full_json_array)
    }

    pub fn from_str(full_json_array: &str) -> Result<Self, JSONArrayError> {
        JSONArray::create_validator().parse_str(full_json_array)
    }

    fn create_validator() -> JSONArrayValidator {
        JSONArrayValidator {}
    }
}


#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for JSONArray {
    type Error = JSONArrayError;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        JSONArray::from_string(form_value.url_decode().map_err(|err| JSONArrayError::UTF8Error(err))?)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for JSONArray {
    type Error = JSONArrayError;

    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        JSONArray::from_string(param.url_decode().map_err(|err| JSONArrayError::UTF8Error(err))?)
    }
}

struct StringVisitor;

impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = JSONArray;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an JSONArray string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
        JSONArray::from_str(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: ::serde::de::Error {
        JSONArray::from_string(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

impl<'de> ::serde::Deserialize<'de> for JSONArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        deserializer.deserialize_string(StringVisitor)
    }
}

impl ::serde::Serialize for JSONArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        self.value.serialize(serializer)
    }
}