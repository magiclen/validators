use alloc::{string::String, vec::Vec};
use core::{
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use super::Result;
use crate::traits::*;

impl<T: Debug, E: Debug> Debug for Result<T, E> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T, E> From<core::result::Result<T, E>> for Result<T, E> {
    #[inline]
    fn from(result: core::result::Result<T, E>) -> Self {
        Self::new(result)
    }
}

impl<T, E> From<Result<T, E>> for core::result::Result<T, E> {
    #[inline]
    fn from(result: Result<T, E>) -> Self {
        result.0
    }
}

impl<T, E> Deref for Result<T, E> {
    type Target = core::result::Result<T, E>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, E> DerefMut for Result<T, E> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// ------

impl<T: ValidateString<Error = E>, E> FromStr for Result<T, E> {
    type Err = E;

    #[inline]
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(Self::new(T::parse_str(s)))
    }
}

impl<T: ValidateString<Error = E>, E> From<String> for Result<T, E> {
    #[inline]
    fn from(value: String) -> Self {
        Self::new(T::parse_string(value))
    }
}

impl<T: ValidateString<Error = E>, E> From<&str> for Result<T, E> {
    #[inline]
    fn from(value: &str) -> Self {
        Self::new(T::parse_str(value))
    }
}

impl<T: ValidateBoolean<Error = E>, E> From<bool> for Result<T, E> {
    #[inline]
    fn from(value: bool) -> Self {
        Self::new(T::parse_bool(value))
    }
}

impl<T: ValidateBytes<Error = E>, E> From<Vec<u8>> for Result<T, E> {
    #[inline]
    fn from(value: Vec<u8>) -> Self {
        Self::new(T::parse_vec_u8(value))
    }
}

impl<T: ValidateBytes<Error = E>, E> From<&[u8]> for Result<T, E> {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self::new(T::parse_u8_slice(value))
    }
}

impl<T: ValidateChar<Error = E>, E> From<char> for Result<T, E> {
    #[inline]
    fn from(value: char) -> Self {
        Self::new(T::parse_char(value))
    }
}

#[cfg(feature = "serde_json")]
impl<T: ValidateJsonValue<Error = E>, E> From<serde_json::Value> for Result<T, E> {
    #[inline]
    fn from(value: serde_json::Value) -> Self {
        Self::new(T::parse_json_value(value))
    }
}

impl<T: ValidateNumber<Error = E>, E> From<f32> for Result<T, E> {
    #[inline]
    fn from(value: f32) -> Self {
        Self::new(T::parse_f32(value))
    }
}

impl<T: ValidateNumber<Error = E>, E> From<f64> for Result<T, E> {
    #[inline]
    fn from(value: f64) -> Self {
        Self::new(T::parse_f64(value))
    }
}

impl<T: ValidateSignedInteger<Error = E>, E> From<i8> for Result<T, E> {
    #[inline]
    fn from(value: i8) -> Self {
        Self::new(T::parse_i8(value))
    }
}

impl<T: ValidateSignedInteger<Error = E>, E> From<i16> for Result<T, E> {
    #[inline]
    fn from(value: i16) -> Self {
        Self::new(T::parse_i16(value))
    }
}

impl<T: ValidateSignedInteger<Error = E>, E> From<i32> for Result<T, E> {
    #[inline]
    fn from(value: i32) -> Self {
        Self::new(T::parse_i32(value))
    }
}

impl<T: ValidateSignedInteger<Error = E>, E> From<i64> for Result<T, E> {
    #[inline]
    fn from(value: i64) -> Self {
        Self::new(T::parse_i64(value))
    }
}

impl<T: ValidateSignedInteger<Error = E>, E> From<i128> for Result<T, E> {
    #[inline]
    fn from(value: i128) -> Self {
        Self::new(T::parse_i128(value))
    }
}

impl<T: ValidateSignedInteger<Error = E>, E> From<isize> for Result<T, E> {
    #[inline]
    fn from(value: isize) -> Self {
        Self::new(T::parse_isize(value))
    }
}

impl<T: ValidateUnsignedInteger<Error = E>, E> From<u8> for Result<T, E> {
    #[inline]
    fn from(value: u8) -> Self {
        Self::new(T::parse_u8(value))
    }
}

impl<T: ValidateUnsignedInteger<Error = E>, E> From<u16> for Result<T, E> {
    #[inline]
    fn from(value: u16) -> Self {
        Self::new(T::parse_u16(value))
    }
}

impl<T: ValidateUnsignedInteger<Error = E>, E> From<u32> for Result<T, E> {
    #[inline]
    fn from(value: u32) -> Self {
        Self::new(T::parse_u32(value))
    }
}

impl<T: ValidateUnsignedInteger<Error = E>, E> From<u64> for Result<T, E> {
    #[inline]
    fn from(value: u64) -> Self {
        Self::new(T::parse_u64(value))
    }
}

impl<T: ValidateUnsignedInteger<Error = E>, E> From<u128> for Result<T, E> {
    #[inline]
    fn from(value: u128) -> Self {
        Self::new(T::parse_u128(value))
    }
}

impl<T: ValidateUnsignedInteger<Error = E>, E> From<usize> for Result<T, E> {
    #[inline]
    fn from(value: usize) -> Self {
        Self::new(T::parse_usize(value))
    }
}
