use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap};
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "serde_json")]
use crate::serde_json::{Map, Value};

/// Validate and deserialize strings.
pub trait ValidateString {
    type Error;
    type Output;

    fn parse_string<S: Into<String>>(s: S) -> Result<Self::Output, Self::Error>;
    fn parse_str<S: AsRef<str>>(s: S) -> Result<Self::Output, Self::Error>;
    fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error>;
}

/// Validate and deserialize bytes.
pub trait ValidateBytes {
    type Error;
    type Output;

    fn parse_vec_u8<V: Into<Vec<u8>>>(v: V) -> Result<Self::Output, Self::Error>;
    fn parse_u8_slice<V: AsRef<[u8]>>(v: V) -> Result<Self::Output, Self::Error>;
    fn validate_u8_slice<V: AsRef<[u8]>>(v: V) -> Result<(), Self::Error>;
}

/// Validate and deserialize characters.
pub trait ValidateChar {
    type Error;
    type Output;

    fn parse_char(c: char) -> Result<Self::Output, Self::Error>;
    fn validate_char(c: char) -> Result<(), Self::Error>;
}

/// Validate and deserialize signed integers.
pub trait ValidateSignedInteger {
    type Error;
    type Output;

    fn parse_i128(i: i128) -> Result<Self::Output, Self::Error>;

    fn validate_i128(i: i128) -> Result<(), Self::Error>;

    #[cfg(target_pointer_width = "128")]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self::Output, Self::Error> {
        Self::parse_i128(i128::from(i))
    }

    #[cfg(not(any(
        target_pointer_width = "128",
        target_pointer_width = "32",
        target_pointer_width = "16",
        target_pointer_width = "8"
    )))]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self::Output, Self::Error> {
        Self::parse_i64(i as i64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self::Output, Self::Error> {
        Self::parse_i32(i as i32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self::Output, Self::Error> {
        Self::parse_i16(i as i16)
    }

    #[cfg(target_pointer_width = "8")]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self::Output, Self::Error> {
        Self::parse_i8(i as i8)
    }

    #[inline]
    fn parse_i64(i: i64) -> Result<Self::Output, Self::Error> {
        Self::parse_i128(i128::from(i))
    }

    #[inline]
    fn parse_i32(i: i32) -> Result<Self::Output, Self::Error> {
        Self::parse_i64(i64::from(i))
    }

    #[inline]
    fn parse_i16(i: i16) -> Result<Self::Output, Self::Error> {
        Self::parse_i32(i32::from(i))
    }

    #[inline]
    fn parse_i8(i: i8) -> Result<Self::Output, Self::Error> {
        Self::parse_i16(i16::from(i))
    }

    #[cfg(target_pointer_width = "128")]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i128(i128::from(i))
    }

    #[cfg(not(any(
        target_pointer_width = "128",
        target_pointer_width = "32",
        target_pointer_width = "16",
        target_pointer_width = "8"
    )))]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i64(i as i64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i32(i as i32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i16(i as i16)
    }

    #[cfg(target_pointer_width = "8")]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i8(i as i8)
    }

    #[inline]
    fn validate_i64(i: i64) -> Result<(), Self::Error> {
        Self::validate_i128(i128::from(i))
    }

    #[inline]
    fn validate_i32(i: i32) -> Result<(), Self::Error> {
        Self::validate_i64(i64::from(i))
    }

    #[inline]
    fn validate_i16(i: i16) -> Result<(), Self::Error> {
        Self::validate_i32(i32::from(i))
    }

    #[inline]
    fn validate_i8(i: i8) -> Result<(), Self::Error> {
        Self::validate_i16(i16::from(i))
    }
}

/// Validate and deserialize unsigned integers.
pub trait ValidateUnsignedInteger {
    type Error;
    type Output;

    fn parse_u128(u: u128) -> Result<Self::Output, Self::Error>;

    fn validate_u128(u: u128) -> Result<(), Self::Error>;

    #[cfg(target_pointer_width = "128")]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self::Output, Self::Error> {
        Self::parse_u128(u128::from(u))
    }

    #[cfg(not(any(
        target_pointer_width = "128",
        target_pointer_width = "32",
        target_pointer_width = "16",
        target_pointer_width = "8"
    )))]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self::Output, Self::Error> {
        Self::parse_u64(u as u64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self::Output, Self::Error> {
        Self::parse_u32(u as u32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self::Output, Self::Error> {
        Self::parse_u16(u as u16)
    }

    #[cfg(target_pointer_width = "8")]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self::Output, Self::Error> {
        Self::parse_u8(u as u8)
    }

    #[inline]
    fn parse_u64(u: u64) -> Result<Self::Output, Self::Error> {
        Self::parse_u128(u128::from(u))
    }

    #[inline]
    fn parse_u32(u: u32) -> Result<Self::Output, Self::Error> {
        Self::parse_u64(u64::from(u))
    }

    #[inline]
    fn parse_u16(u: u16) -> Result<Self::Output, Self::Error> {
        Self::parse_u32(u32::from(u))
    }

    #[inline]
    fn parse_u8(u: u8) -> Result<Self::Output, Self::Error> {
        Self::parse_u16(u16::from(u))
    }

    #[cfg(target_pointer_width = "128")]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u128(u128::from(u))
    }

    #[cfg(not(any(
        target_pointer_width = "128",
        target_pointer_width = "32",
        target_pointer_width = "16",
        target_pointer_width = "8"
    )))]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u64(u as u64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u32(u as u32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u16(u as u16)
    }

    #[cfg(target_pointer_width = "8")]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u8(u as u8)
    }

    #[inline]
    fn validate_u64(u: u64) -> Result<(), Self::Error> {
        Self::validate_u128(u128::from(u))
    }

    #[inline]
    fn validate_u32(u: u32) -> Result<(), Self::Error> {
        Self::validate_u64(u64::from(u))
    }

    #[inline]
    fn validate_u16(u: u16) -> Result<(), Self::Error> {
        Self::validate_u32(u32::from(u))
    }

    #[inline]
    fn validate_u8(u: u8) -> Result<(), Self::Error> {
        Self::validate_u16(u16::from(u))
    }
}

/// Validate and deserialize (floating) numbers.
pub trait ValidateNumber {
    type Error;
    type Output;

    fn parse_f64(f: f64) -> Result<Self::Output, Self::Error>;

    fn validate_f64(f: f64) -> Result<(), Self::Error>;

    #[inline]
    fn parse_f32(f: f32) -> Result<Self::Output, Self::Error> {
        Self::parse_f64(f as f64)
    }

    #[inline]
    fn validate_f32(f: f32) -> Result<(), Self::Error> {
        Self::validate_f64(f as f64)
    }
}

/// Validate and deserialize booleans.
pub trait ValidateBoolean {
    type Error;
    type Output;

    fn parse_bool(b: bool) -> Result<Self::Output, Self::Error>;

    fn validate_bool(b: bool) -> Result<(), Self::Error>;
}

/// For types which should have the a `len` method.
#[allow(clippy::len_without_is_empty)]
pub trait CollectionLength {
    fn len(&self) -> usize;
}

impl<T> CollectionLength for Vec<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> CollectionLength for BinaryHeap<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> CollectionLength for BTreeSet<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<K, T> CollectionLength for BTreeMap<K, T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<T> CollectionLength for HashSet<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<K, T> CollectionLength for HashMap<K, T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "serde_json")]
impl CollectionLength for Map<String, Value> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

/// Validate the length of collections.
pub trait ValidateLength<T: CollectionLength> {
    type Error;
    type Output;

    fn parse_collection(v: T) -> Result<Self::Output, Self::Error>;

    fn validate_collection(v: &T) -> Result<(), Self::Error>;
}
