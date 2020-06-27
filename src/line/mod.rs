extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::str::{FromStr, Utf8Error};

lazy_static! {
    static ref LINE_RE: Regex = Regex::new("^[^\x00-\x08\x0A-\x1F\x7F]*$").unwrap();
}

#[derive(Debug, PartialEq, Clone)]
pub enum LineError {
    IncorrectFormat,
    UTF8Error(Utf8Error),
}

impl Display for LineError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for LineError {}

impl From<Utf8Error> for LineError {
    #[inline]
    fn from(err: Utf8Error) -> Self {
        LineError::UTF8Error(err)
    }
}

pub type LineResult = Result<Line, LineError>;

#[derive(Debug, PartialEq)]
pub struct LineValidator {}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Line {
    line: String,
}

impl Line {
    #[inline]
    pub fn get_line(&self) -> &str {
        &self.line
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.line
    }

    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn from_string_unchecked(line: String) -> Line {
        Line {
            line,
        }
    }
}

impl Deref for Line {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.line
    }
}

impl Validated for Line {}

impl Debug for Line {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(Line, f, self, let .0 = self.line);
    }
}

impl Display for Line {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.line)?;
        Ok(())
    }
}

impl LineValidator {
    #[inline]
    pub fn is_line(&self, line: &str) -> bool {
        self.parse_inner(line).is_ok()
    }

    #[inline]
    pub fn parse_string(&self, line: String) -> LineResult {
        let mut line_inner = self.parse_inner(&line)?;

        line_inner.line = line;

        Ok(line_inner)
    }

    #[inline]
    pub fn parse_str(&self, line: &str) -> LineResult {
        let mut line_inner = self.parse_inner(line)?;

        line_inner.line.push_str(line);

        Ok(line_inner)
    }

    #[inline]
    fn parse_inner(&self, line: &str) -> LineResult {
        if LINE_RE.is_match(line) {
            Ok(Line {
                line: String::new(),
            })
        } else {
            Err(LineError::IncorrectFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_methods() {
        let line = "abc123 ABC中文".to_string();

        let bv = LineValidator {};

        let line = bv.parse_string(line).unwrap();

        assert_eq!("abc123 ABC中文", line.get_line());
    }

    #[test]
    fn test_line_lv1() {
        let line = "abc123 ABC中文".to_string();

        let bv = LineValidator {};

        bv.parse_string(line).unwrap();
    }
}

// Line's wrapper struct is itself
impl ValidatedWrapper for Line {
    type Error = LineError;

    #[inline]
    fn from_string(line: String) -> Result<Self, Self::Error> {
        Line::from_string(line)
    }

    #[inline]
    fn from_str(line: &str) -> Result<Self, Self::Error> {
        Line::from_str(line)
    }
}

impl Line {
    #[inline]
    pub fn from_string(line: String) -> Result<Self, LineError> {
        Line::create_validator().parse_string(line)
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(line: &str) -> Result<Self, LineError> {
        Line::create_validator().parse_str(line)
    }

    #[inline]
    fn create_validator() -> LineValidator {
        LineValidator {}
    }
}

impl FromStr for Line {
    type Err = LineError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Line::from_str(s)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for Line {
    type Error = LineError;

    #[inline]
    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Line::from_string(param.url_decode()?)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for Line {
    type Error = LineError;

    #[inline]
    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        Line::from_string(form_value.url_decode()?)
    }
}

#[cfg(feature = "serdely")]
struct StringVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = Line;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Line string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        Line::from_str(v).map_err(|err| E::custom(err.to_string()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error, {
        Line::from_string(v).map_err(|err| E::custom(err.to_string()))
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for Line {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>, {
        deserializer.deserialize_str(StringVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for Line {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer, {
        serializer.serialize_str(&self.line)
    }
}
