use super::ValidatedWrapper;

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::Utf8Error;

use num_traits::ToPrimitive;

#[cfg(feature = "serdely")]
use num_traits::NumCast;

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedNumberError {
    RegexError(regex::Error),
    ParseError(String),
    UnpreciseError,
    OutRange,
    NotMatch,
    UTF8Error(Utf8Error),
}

impl Display for ValidatedCustomizedNumberError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ValidatedCustomizedNumberError {}

pub trait ValidatedNumberWrapper<T: ToPrimitive>: ValidatedWrapper {
    fn from_number(n: T) -> Result<Self, ValidatedCustomizedNumberError>;
}

#[cfg(feature = "serdely")]
pub struct NumberVisitor<V, T>(pub Vec<V>, pub Vec<T>);

#[cfg(feature = "serdely")]
impl<'de, V, T> serde::de::Visitor<'de> for NumberVisitor<V, T>
where
    V: ValidatedWrapper + ValidatedNumberWrapper<T> + NumCast,
    T: ToPrimitive,
{
    type Value = V;

    serde_if_integer128! {
       #[inline] fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E> where E: serde::de::Error {
            NumCast::from(v).ok_or_else( || E::custom("UnpreciseError".to_string()))
        }
    }

    serde_if_integer128! {
       #[inline] fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E> where E: serde::de::Error {
            NumCast::from(v).ok_or_else( || E::custom("UnpreciseError".to_string()))
        }
    }

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("a string({})", stringify!($name)))
    }

    #[inline]
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {
        NumCast::from(v).ok_or_else(|| E::custom("UnpreciseError".to_string()))
    }

    #[inline]
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {
        NumCast::from(v).ok_or_else(|| E::custom("UnpreciseError".to_string()))
    }

    #[inline]
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {
        NumCast::from(v).ok_or_else(|| E::custom("UnpreciseError".to_string()))
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {
        NumCast::from(v).ok_or_else(|| E::custom("UnpreciseError".to_string()))
    }

    #[inline]
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {
        NumCast::from(v).ok_or_else(|| E::custom("UnpreciseError".to_string()))
    }

    #[inline]
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {
        NumCast::from(v).ok_or_else(|| E::custom("UnpreciseError".to_string()))
    }

    #[inline]
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {
        NumCast::from(v).ok_or_else(|| E::custom("UnpreciseError".to_string()))
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {
        NumCast::from(v).ok_or_else(|| E::custom("UnpreciseError".to_string()))
    }

    #[inline]
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {
        NumCast::from(v).ok_or_else(|| E::custom("UnpreciseError".to_string()))
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {
        NumCast::from(v).ok_or_else(|| E::custom("UnpreciseError".to_string()))
    }
}

#[cfg(feature = "serdely")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_se_de {
    ($name:ident, $t:ident) => {
        impl<'de> $crate::serde::Deserialize<'de> for $name {
            #[inline]
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: $crate::serde::Deserializer<'de>, {
                let v = $crate::NumberVisitor(Vec::<$name>::new(), Vec::<$t>::new());

                match stringify!($t) {
                    "u8" => deserializer.deserialize_u8(v),
                    "u16" => deserializer.deserialize_u16(v),
                    "u32" => deserializer.deserialize_u32(v),
                    "u64" => deserializer.deserialize_u64(v),
                    "u128" => deserializer.deserialize_u128(v),
                    "i8" => deserializer.deserialize_i8(v),
                    "i16" => deserializer.deserialize_i16(v),
                    "i32" => deserializer.deserialize_i32(v),
                    "i64" => deserializer.deserialize_i64(v),
                    "i128" => deserializer.deserialize_i128(v),
                    "f32" => deserializer.deserialize_f32(v),
                    "f64" => deserializer.deserialize_f64(v),
                    _ => unreachable!(),
                }
            }
        }

        impl $crate::serde::Serialize for $name {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer, {
                match stringify!($t) {
                    "u8" => serializer.serialize_u8(self.get_number() as u8),
                    "u16" => serializer.serialize_u16(self.get_number() as u16),
                    "u32" => serializer.serialize_u32(self.get_number() as u32),
                    "u64" => serializer.serialize_u64(self.get_number() as u64),
                    "u128" => serializer.serialize_u128(self.get_number() as u128),
                    "i8" => serializer.serialize_i8(self.get_number() as i8),
                    "i16" => serializer.serialize_i16(self.get_number() as i16),
                    "i32" => serializer.serialize_i32(self.get_number() as i32),
                    "i64" => serializer.serialize_i64(self.get_number() as i64),
                    "i128" => serializer.serialize_i128(self.get_number() as i128),
                    "f32" => serializer.serialize_f32(self.get_number() as f32),
                    "f64" => serializer.serialize_f64(self.get_number() as f64),
                    _ => unreachable!(),
                }
            }
        }
    };
}

#[cfg(not(feature = "serdely"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_se_de {
    ($name:ident, $t:expr) => {};
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_from_form_value {
    ($name:ident) => {
        impl<'a> $crate::rocket::request::FromFormValue<'a> for $name {
            type Error = $crate::ValidatedCustomizedNumberError;

            #[inline]
            fn from_form_value(
                form_value: &'a $crate::rocket::http::RawStr,
            ) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string(
                    form_value
                        .url_decode()
                        .map_err(|err| $crate::ValidatedCustomizedNumberError::UTF8Error(err))?,
                )
            }
        }

        impl<'a> $crate::rocket::request::FromParam<'a> for $name {
            type Error = $crate::ValidatedCustomizedNumberError;

            #[inline]
            fn from_param(
                param: &'a $crate::rocket::http::RawStr,
            ) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string(
                    param
                        .url_decode()
                        .map_err(|err| $crate::ValidatedCustomizedNumberError::UTF8Error(err))?,
                )
            }
        }
    };
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_from_form_value {
    ($name:ident) => {};
}

#[macro_export]
macro_rules! validated_customized_number_struct {
    ( $name:ident, $field:ident, $t:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        impl ::std::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                $crate::debug_helper::impl_debug_for_tuple_struct!($name, f, self, let .0 = self.$field);
            }
        }

        impl ::std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}", self.$field))?;
                Ok(())
            }
        }

        impl ::std::cmp::PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.$field.eq(&other.$field)
            }
        }

        impl ::std::cmp::Eq for $name {}

        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H){
                match stringify!($t) {
                    "u8" => {
                        state.write_u8(self.get_number() as u8);
                    },
                    "u16" => {
                        state.write_u16(self.get_number() as u16);
                    },
                    "u32" => {
                        state.write_u32(self.get_number() as u32);
                    },
                    "u64" => {
                        state.write_u64(self.get_number() as u64);
                    },
                    "u128" => {
                        state.write_u128(self.get_number() as u128);
                    },
                    "i8" => {
                        state.write_i8(self.get_number() as i8);
                    },
                    "i16" => {
                        state.write_i16(self.get_number() as i16);
                    },
                    "i32" => {
                        state.write_i32(self.get_number() as i32);
                    },
                    "i64" => {
                        state.write_i64(self.get_number() as i64);
                    },
                    "i128" => {
                        state.write_i128(self.get_number() as i128);
                    },
                    "f32" => {
                        let bytes: [u8; 4] = unsafe{ ::std::mem::transmute(self.get_number() as f32) };
                        state.write(&bytes);
                    },
                    "f64" => {
                        let bytes: [u8; 8] = unsafe{ ::std::mem::transmute(self.get_number() as f64) };
                        state.write(&bytes);
                    },
                    _ => unreachable!()
                }
            }
        }

        impl $crate::Validated for $name {}

        impl $crate::num_traits::ToPrimitive for $name {
            #[inline]
            fn to_isize(&self) -> Option<isize> {
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_i8(&self) -> Option<i8> {
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_i16(&self) -> Option<i16> {
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_i32(&self) -> Option<i32> {
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_i64(&self) -> Option<i64> {
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_i128(&self) -> Option<i128> {
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_usize(&self) -> Option<usize> {
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_u8(&self) -> Option<u8> {
                println!("123");
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_u16(&self) -> Option<u16> {
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_u32(&self) -> Option<u32> {
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_u64(&self) -> Option<u64> {
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_u128(&self) -> Option<u128> {
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_f32(&self) -> Option<f32> {
                $crate::num_traits::NumCast::from(self.get_number())
            }

            #[inline]
            fn to_f64(&self) -> Option<f64> {
                $crate::num_traits::NumCast::from(self.get_number())
            }
        }

        impl $crate::num_traits::NumCast for $name {
            #[inline]
            fn from<T: $crate::num_traits::ToPrimitive>(n: T) -> Option<$name> {
                $crate::ValidatedNumberWrapper::from_number(n).ok()
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $t;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl $crate::ValidatedWrapper for $name {
            type Error = $crate::ValidatedCustomizedNumberError;

            #[inline]
            fn from_string($from_string_input: String) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string($from_string_input)
            }

            #[inline]
            fn from_str($from_str_input: &str) -> ::std::result::Result<Self, Self::Error> {
                $name::from_str($from_str_input)
            }
        }

        impl<T: $crate::num_traits::ToPrimitive> $crate::ValidatedNumberWrapper<T> for $name {
            #[inline]
            fn from_number($from_number_input: T) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                $name::from_number($crate::num_traits::NumCast::from($from_number_input).ok_or($crate::ValidatedCustomizedNumberError::UnpreciseError)?)
            }
        }

        impl $name {
            #[inline]
            pub fn get_number(&self) -> $t {
                self.$field
            }

            #[inline]
            #[allow(clippy::float_cmp)]
            pub fn is_zero(&self) -> bool {
                self.$field == 0 as $t
            }

            #[inline]
            pub fn is_positive(&self) -> bool {
                self.$field > 0 as $t
            }

            #[inline]
            pub fn is_negative(&self) -> bool {
                self.$field < 0 as $t
            }

            #[inline]
            #[allow(clippy::float_cmp)]
            pub fn is_integer(&self) -> bool {
                self.$field as u128 as $t == self.$field
            }

            #[inline]
            pub fn from_string($from_string_input: String) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name {$field})
            }

            #[inline]
            pub fn from_str($from_str_input: &str) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name {$field})
            }

            #[inline]
            pub fn from_number($from_number_input: $t) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let $field = match $from_number {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name {$field})
            }

            #[inline]
            #[allow(clippy::float_cmp)]
            pub fn from_f64($from_number_input: f64) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as f64 != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            #[allow(clippy::float_cmp)]
            pub fn from_f32($from_number_input: f32) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as f32 != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub fn from_isize($from_number_input: isize) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as isize != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub fn from_i8($from_number_input: i8) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as i8 != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub fn from_i16($from_number_input: i16) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as i16 != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub fn from_i32($from_number_input: i32) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as i32 != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub fn from_i64($from_number_input: i64) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as i64 != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub fn from_i128($from_number_input: i128) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as i128 != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub fn from_usize($from_number_input: usize) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as usize != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub fn from_u8($from_number_input: u8) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as u8 != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub fn from_u16($from_number_input: u16) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as u16 != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub fn from_u32($from_number_input: u32) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as u32 != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub fn from_u64($from_number_input: u64) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as u64 != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub fn from_u128($from_number_input: u128) -> ::std::result::Result<Self, $crate::ValidatedCustomizedNumberError> {
                let v = $from_number_input as $t;

                if v as u128 != $from_number_input {
                    return Err($crate::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            #[inline]
            pub unsafe fn from_number_unchecked($from_number_input: $t) -> Self {
                $name {$field:$from_number_input}
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::ValidatedCustomizedNumberError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, $crate::ValidatedCustomizedNumberError> {
                $name::from_str(s)
            }
        }

        validated_customized_number_struct_implement_from_form_value!($name);

        validated_customized_number_struct_implement_se_de!($name, $t);
    };
    ( $name:ident, $field:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
}

#[macro_export]
macro_rules! validated_customized_number {
    ( $name:ident, $t:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        #[derive(Clone)]
        struct $name {
            n: $t
        }

        validated_customized_number_struct!($name, n, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $v:vis $name:ident, $t:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        #[derive(Clone)]
        $v struct $name {
            n: $t
        }

        validated_customized_number_struct!($name, n, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $v:vis $name:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!($v $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $v:vis $name:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!($v $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $v:vis $name:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!($v $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $v:vis $name:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!($v $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $v:vis $name:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!($v $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $v:vis $name:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!($v $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
}

#[macro_export]
macro_rules! validated_customized_regex_number_struct {
    ( $name:ident, $field:ident, $t:ident, $re:expr ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let re = $crate::regex::Regex::new($re).map_err(|err| $crate::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                let value = input.parse::<$t>().map_err(|err| $crate::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

                if $crate::number::precise(&value.to_string(), &input) {
                    Ok(value)
                } else {
                    Err($crate::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err($crate::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let re = $crate::regex::Regex::new($re).map_err(|err| $crate::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(input) {
                let value = input.parse::<$t>().map_err(|err| $crate::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

                if $crate::number::precise(&value.to_string(), input) {
                    Ok(value)
                } else {
                    Err($crate::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err($crate::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let input = input.to_string();

            let re = $crate::regex::Regex::new($re).map_err(|err| $crate::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                let value = input.parse::<$t>().map_err(|err| $crate::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

                if $crate::number::precise(&value.to_string(), &input) {
                    Ok(value)
                } else {
                    Err($crate::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err($crate::ValidatedCustomizedNumberError::NotMatch)
            }
        });
    };
    ( $name:ident, $field:ident, $t:ident, ref $re:expr ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let re: &$crate::regex::Regex = &$re;

            if re.is_match(&input) {
                let value = input.parse::<$t>().map_err(|err| $crate::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

                if $crate::number::precise(&value.to_string(), &input) {
                    Ok(value)
                } else {
                    Err($crate::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err($crate::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let re: &$crate::regex::Regex = &$re;

            if re.is_match(input) {
                let value = input.parse::<$t>().map_err(|err| $crate::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

                if $crate::number::precise(&value.to_string(), input) {
                    Ok(value)
                } else {
                    Err($crate::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err($crate::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let input = input.to_string();

            let re: &$crate::regex::Regex = &$re;

            if re.is_match(&input) {
                let value = input.parse::<$t>().map_err(|err| $crate::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

                if $crate::number::precise(&value.to_string(), &input) {
                    Ok(value)
                } else {
                    Err($crate::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err($crate::ValidatedCustomizedNumberError::NotMatch)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_regex_number {
    ( $name:ident, $t:ident, $re:expr ) => {
        #[derive(Clone)]
        struct $name {
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, $re);
    };
    ( $v:vis $name:ident, $t:ident, $re:expr ) => {
        #[derive(Clone)]
        $v struct $name {
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, $re);
    };
    ( $name:ident, $t:ident, ref $re:expr ) => {
        #[derive(Clone)]
        struct $name {
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, ref $re);
    };
    ( $v:vis $name:ident, $t:ident, ref $re:expr ) => {
        #[derive(Clone)]
        $v struct $name {
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, ref $re);
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_number_struct {
    ( $name:ident, $field:ident, $t:ident, $min:expr, $max:expr ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let value = input.parse::<$t>().map_err(|err| $crate::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if value >= $min && value <= $max {
                if $crate::number::precise(&value.to_string(), &input) {
                    Ok(value)
                } else {
                    Err($crate::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err($crate::ValidatedCustomizedNumberError::OutRange)
            }
        },
        input {
            let value = input.parse::<$t>().map_err(|err| $crate::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if value >= $min && value <= $max {
                if $crate::number::precise(&value.to_string(), input) {
                    Ok(value)
                } else {
                    Err($crate::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err($crate::ValidatedCustomizedNumberError::OutRange)
            }
        },
        input {
            if input >= $min && input <= $max {
                Ok(input)
            } else{
                Err($crate::ValidatedCustomizedNumberError::OutRange)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_number {
    ( $name:ident, $t:ident, $min:expr, $max:expr ) => {
        #[derive(Clone)]
        struct $name {
            n: $t
        }

        validated_customized_ranged_number_struct!($name, n, $t, $min, $max);
    };
    ( $v:vis $name:ident, $t:ident, $min:expr, $max:expr ) => {
        #[derive(Clone)]
        $v struct $name {
            n: $t
        }

        validated_customized_ranged_number_struct!($name, n, $t, $min, $max);
    };
}

#[macro_export]
macro_rules! validated_customized_primitive_number_struct {
    ( $name:ident, $field:ident, $t:ident ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let value = input.parse::<$t>().map_err(|err|$crate::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if $crate::number::precise(&value.to_string(), &input) {
                    Ok(value)
                } else {
                    Err($crate::ValidatedCustomizedNumberError::UnpreciseError)
                }
        },
        input {
            let value = input.parse::<$t>().map_err(|err|$crate::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if $crate::number::precise(&value.to_string(), input) {
                    Ok(value)
                } else {
                    Err($crate::ValidatedCustomizedNumberError::UnpreciseError)
                }
        },
        input {
            Ok(input)
        });
    };
}

#[macro_export]
macro_rules! validated_customized_primitive_number {
    ( $name:ident, $t:ident ) => {
        #[derive(Clone)]
        struct $name {
            n: $t
        }

        validated_customized_primitive_number_struct!($name, n, $t);
    };
    ( $v:vis $name:ident, $t:ident ) => {
        #[derive(Clone)]
        $v struct $name {
            n: $t
        }

        validated_customized_primitive_number_struct!($name, n, $t);
    };
}
