#![cfg(feature = "serdely")]
extern crate serde_json;

use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ops::DerefMut;
use std::str::{FromStr, Utf8Error};

use self::serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum JSONArrayError {
    IncorrectJSONArray,
    UTF8Error(Utf8Error),
}

impl Display for JSONArrayError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for JSONArrayError {}

impl From<Utf8Error> for JSONArrayError {
    #[inline]
    fn from(err: Utf8Error) -> Self {
        JSONArrayError::UTF8Error(err)
    }
}

pub type JSONArrayResult = Result<JSONArray, JSONArrayError>;

#[derive(Debug, PartialEq)]
pub struct JSONArrayValidator {}

#[derive(Clone)]
pub struct JSONArray {
    value: Value,
    full_json_array: String,
}

impl JSONArray {
    #[inline]
    pub fn get_json_value(&self) -> &Value {
        &self.value
    }

    #[inline]
    pub fn get_full_json_array(&self) -> &str {
        &self.full_json_array
    }

    #[inline]
    pub fn into_vec(self) -> Vec<Value> {
        match self.value {
            Value::Array(array) => array,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn into_value(self) -> Value {
        self.value
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.full_json_array
    }
}

impl Deref for JSONArray {
    type Target = Vec<Value>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.value.as_array().unwrap()
    }
}

impl DerefMut for JSONArray {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_array_mut().unwrap()
    }
}

impl Validated for JSONArray {}

impl Debug for JSONArray {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(JSONArray, f, self, let .0 = self.value);
    }
}

impl Display for JSONArray {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl PartialEq for JSONArray {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.full_json_array.eq(&other.full_json_array)
    }
}

impl Eq for JSONArray {}

impl Hash for JSONArray {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_json_array.hash(state)
    }
}

impl JSONArrayValidator {
    #[inline]
    pub fn is_json_array(&self, full_json_array: &str) -> bool {
        self.parse_inner(full_json_array).is_ok()
    }

    #[inline]
    pub fn parse_string(&self, full_json_array: String) -> JSONArrayResult {
        let mut json_array_inner = self.parse_inner(&full_json_array)?;

        json_array_inner.full_json_array = full_json_array;

        Ok(json_array_inner)
    }

    #[inline]
    pub fn parse_str(&self, full_json_array: &str) -> JSONArrayResult {
        let mut json_array_inner = self.parse_inner(full_json_array)?;

        json_array_inner.full_json_array.push_str(full_json_array);

        Ok(json_array_inner)
    }

    #[inline]
    fn parse_inner(&self, full_json_array: &str) -> JSONArrayResult {
        let json_array: Vec<Value> = match serde_json::from_str(full_json_array) {
            Ok(json_array) => json_array,
            Err(_) => return Err(JSONArrayError::IncorrectJSONArray),
        };

        let value = Value::Array(json_array);

        Ok(JSONArray {
            value,
            full_json_array: String::new(),
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

    #[inline]
    fn from_string(json_array: String) -> Result<Self, Self::Error> {
        JSONArray::from_string(json_array)
    }

    #[inline]
    fn from_str(json_array: &str) -> Result<Self, Self::Error> {
        JSONArray::from_str(json_array)
    }
}

impl JSONArray {
    #[inline]
    pub fn from_string(full_json_array: String) -> Result<Self, JSONArrayError> {
        JSONArray::create_validator().parse_string(full_json_array)
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(full_json_array: &str) -> Result<Self, JSONArrayError> {
        JSONArray::create_validator().parse_str(full_json_array)
    }

    fn create_validator() -> JSONArrayValidator {
        JSONArrayValidator {}
    }
}

impl FromStr for JSONArray {
    type Err = JSONArrayError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        JSONArray::from_str(s)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for JSONArray {
    type Error = JSONArrayError;

    #[inline]
    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        JSONArray::from_string(form_value.url_decode()?)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for JSONArray {
    type Error = JSONArrayError;

    #[inline]
    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        JSONArray::from_string(param.url_decode()?)
    }
}

struct StringVisitor;

impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = JSONArray;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an JSONArray string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        JSONArray::from_str(v).map_err(|err| E::custom(err.to_string()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        JSONArray::from_string(v).map_err(|err| E::custom(err.to_string()))
    }
}

impl<'de> ::serde::Deserialize<'de> for JSONArray {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>, {
        deserializer.deserialize_string(StringVisitor)
    }
}

impl ::serde::Serialize for JSONArray {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer, {
        self.value.serialize(serializer)
    }
}
