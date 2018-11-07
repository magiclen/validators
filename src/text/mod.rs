extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};

lazy_static! {
    static ref TEXT_RE: Regex = {
        Regex::new("^([^\x00-\x1F\x7F]|\n|\r\n|\n\r)*$").unwrap()
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum TextError {
    IncorrectFormat,
}

impl Display for TextError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for TextError {}

pub type TextResult = Result<Text, TextError>;

#[derive(Debug, PartialEq)]
pub struct TextValidator {}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn into_string(self) -> String {
        self.text
    }

    pub unsafe fn from_string_unchecked(text: String) -> Text {
        Text {
            text
        }
    }
}

impl Validated for Text {}

impl Debug for Text {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Text({})", self.text))?;
        Ok(())
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.text)?;
        Ok(())
    }
}

impl TextValidator {
    pub fn is_text(&self, text: &str) -> bool {
        self.parse_inner(text).is_ok()
    }

    pub fn parse_string(&self, text: String) -> TextResult {
        let mut text_inner = self.parse_inner(&text)?;

        text_inner.text = text;

        Ok(text_inner)
    }

    pub fn parse_str(&self, text: &str) -> TextResult {
        let mut text_inner = self.parse_inner(text)?;

        text_inner.text.push_str(text);

        Ok(text_inner)
    }

    fn parse_inner(&self, text: &str) -> TextResult {
        if TEXT_RE.is_match(text) {
            Ok(Text {
                text: String::new(),
            })
        } else {
            Err(TextError::IncorrectFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_methods() {
        let text = "abc123ABC中文\r\n測試 123\n\rQQQ\n".to_string();

        let bv = TextValidator {};

        let text = bv.parse_string(text).unwrap();

        assert_eq!("abc123ABC中文\r\n測試 123\n\rQQQ\n", text.get_text());
    }

    #[test]
    fn test_text_lv1() {
        let text = "abc123ABC中文\n測試 123\n\n".to_string();

        let bv = TextValidator {};

        bv.parse_string(text).unwrap();
    }
}

// Text's wrapper struct is itself
impl ValidatedWrapper for Text {
    type Error = TextError;

    fn from_string(text: String) -> Result<Self, Self::Error> {
        Text::from_string(text)
    }

    fn from_str(text: &str) -> Result<Self, Self::Error> {
        Text::from_str(text)
    }
}

impl Text {
    pub fn from_string(text: String) -> Result<Self, TextError> {
        Text::create_validator().parse_string(text)
    }

    pub fn from_str(text: &str) -> Result<Self, TextError> {
        Text::create_validator().parse_str(text)
    }

    fn create_validator() -> TextValidator {
        TextValidator {}
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for Text {
    type Error = TextError;

    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Text::from_str(param)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for Text {
    type Error = TextError;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Text::from_str(form_value)
    }
}

#[cfg(feature = "serdely")]
struct StringVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = Text;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Text string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
        Text::from_str(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: ::serde::de::Error {
        Text::from_string(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for Text {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        deserializer.deserialize_str(StringVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for Text {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        serializer.serialize_str(&self.text)
    }
}