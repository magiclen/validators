use super::ValidatedWrapper;

use std::error::Error;
use std::str::Utf8Error;
use std::fmt::{self, Display, Debug, Formatter};

use number_as::Number;

#[cfg(feature = "serdely")]
use number_as::NumberAs;

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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ValidatedCustomizedNumberError {}

pub trait ValidatedNumberWrapper<T: Number>: ValidatedWrapper {
    fn from_number(n: T) -> Result<Self, ValidatedCustomizedNumberError>;
}

#[cfg(feature = "serdely")]
pub struct NumberVisitor<V, T>(pub Vec<V>, pub Vec<T>);

#[cfg(feature = "serdely")]
impl<'de, V, T> serde::de::Visitor<'de> for NumberVisitor<V, T> where V: ValidatedWrapper + ValidatedNumberWrapper<T>,
                                                                      T: Number,
                                                                      u8: NumberAs<T>,
                                                                      u16: NumberAs<T>,
                                                                      u32: NumberAs<T>,
                                                                      u64: NumberAs<T>,
                                                                      u128: NumberAs<T>,
                                                                      i8: NumberAs<T>,
                                                                      i16: NumberAs<T>,
                                                                      i32: NumberAs<T>,
                                                                      i64: NumberAs<T>,
                                                                      i128: NumberAs<T>,
                                                                      f32: NumberAs<T>,
                                                                      f64: NumberAs<T> {
    type Value = V;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("a string({})", stringify!($name)))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    serde_if_integer128! {
        fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E> where E: serde::de::Error {
            V::from_number(v.number_as()).map_err(|err| {
                E::custom(err.to_string())
            })
        }
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    serde_if_integer128! {
        fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E> where E: serde::de::Error {
            V::from_number(v.number_as()).map_err(|err| {
                E::custom(err.to_string())
            })
        }
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

#[cfg(feature = "serdely")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_se_de {
    ( $name:ident, $t:ident ) => {
        impl<'de> ::validators::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::validators::serde::Deserializer<'de> {
                let v = ::validators::NumberVisitor(Vec::<$name>::new(), Vec::<$t>::new());

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
                    _ => unreachable!()
                }
            }
        }

        impl ::validators::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::validators::serde::Serializer {
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
                    _ => unreachable!()
                }
            }
        }
    }
}

#[cfg(not(feature = "serdely"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_se_de {
    ( $name:ident, $t:expr ) => {

    }
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a> ::validators::rocket::request::FromFormValue<'a> for $name {
            type Error = ::validators::ValidatedCustomizedNumberError;

            fn from_form_value(form_value: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_string(form_value.url_decode().map_err(|err| ::validators::ValidatedCustomizedNumberError::UTF8Error(err))?)
            }
        }

        impl<'a> ::validators::rocket::request::FromParam<'a> for $name {
            type Error = ::validators::ValidatedCustomizedNumberError;

            fn from_param(param: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error> {
                $name::from_string(param.url_decode().map_err(|err| ::validators::ValidatedCustomizedNumberError::UTF8Error(err))?)
            }
        }
    }
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_from_form_value {
    ( $name:ident ) => {

    }
}

#[macro_export]
macro_rules! validated_customized_number_struct {
    ( $name:ident, $field:ident, $t:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.$field))?;
                Ok(())
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}", self.$field))?;
                Ok(())
            }
        }

        impl ::std::cmp::Eq for $name {}

        impl ::std::hash::Hash for $name{
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

        impl ::validators::Validated for $name {}

        impl ::std::ops::Deref for $name {
            type Target = $t;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl ::validators::ValidatedWrapper for $name {
            type Error = ::validators::ValidatedCustomizedNumberError;

            fn from_string($from_string_input: String) -> Result<Self, Self::Error>{
                $name::from_string($from_string_input)
            }

            fn from_str($from_str_input: &str) -> Result<Self, Self::Error>{
                $name::from_str($from_str_input)
            }
        }

        impl<T: ::validators::number_as::Number> ::validators::ValidatedNumberWrapper<T> for $name {
            fn from_number($from_number_input: T) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                $name::from_number($from_number_input.number_as())
            }
        }

        impl $name {
            pub fn get_number(&self) -> $t {
                self.$field
            }

            pub fn is_zero(&self) -> bool {
                self.$field == 0 as $t
            }

            pub fn is_positive(&self) -> bool {
                self.$field > 0 as $t
            }

            pub fn is_negative(&self) -> bool {
                self.$field < 0 as $t
            }

            pub fn is_integer(&self) -> bool {
                self.$field as u128 as $t == self.$field
            }

            pub fn from_string($from_string_input: String) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub fn from_str($from_str_input: &str) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub fn from_number($from_number_input: $t) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let $field = match $from_number {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub fn from_f64($from_number_input: f64) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let v = $from_number_input as $t;

                if v as f64 != $from_number_input {
                    return Err(::validators::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            pub fn from_f32($from_number_input: f32) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let v = $from_number_input as $t;

                if v as f32 != $from_number_input {
                    return Err(::validators::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            pub fn from_i8($from_number_input: i8) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let v = $from_number_input as $t;

                if v as i8 != $from_number_input {
                    return Err(::validators::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            pub fn from_i16($from_number_input: i16) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let v = $from_number_input as $t;

                if v as i16 != $from_number_input {
                    return Err(::validators::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            pub fn from_i32($from_number_input: i32) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let v = $from_number_input as $t;

                if v as i32 != $from_number_input {
                    return Err(::validators::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            pub fn from_i64($from_number_input: i64) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let v = $from_number_input as $t;

                if v as i64 != $from_number_input {
                    return Err(::validators::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            pub fn from_i128($from_number_input: i128) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let v = $from_number_input as $t;

                if v as i128 != $from_number_input {
                    return Err(::validators::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            pub fn from_u8($from_number_input: u8) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let v = $from_number_input as $t;

                if v as u8 != $from_number_input {
                    return Err(::validators::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            pub fn from_u16($from_number_input: u16) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let v = $from_number_input as $t;

                if v as u16 != $from_number_input {
                    return Err(::validators::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            pub fn from_u32($from_number_input: u32) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let v = $from_number_input as $t;

                if v as u32 != $from_number_input {
                    return Err(::validators::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            pub fn from_u64($from_number_input: u64) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let v = $from_number_input as $t;

                if v as u64 != $from_number_input {
                    return Err(::validators::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            pub fn from_u128($from_number_input: u128) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let v = $from_number_input as $t;

                if v as u128 != $from_number_input {
                    return Err(::validators::ValidatedCustomizedNumberError::UnpreciseError);
                }

                Self::from_number(v)
            }

            pub unsafe fn from_number_unchecked($from_number_input: $t) -> Self{
                $name{$field:$from_number_input}
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
        #[derive(Clone, PartialEq)]
        struct $name{
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
    ( pub $name:ident, $t:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        #[derive(Clone, PartialEq)]
        pub struct $name{
            n: $t
        }

        validated_customized_number_struct!($name, n, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
}

#[macro_export]
macro_rules! validated_customized_regex_number_struct {
    ( $name:ident, $field:ident, $t:ident, $re:expr ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let re = ::validators::regex::Regex::new($re).map_err(|err| ::validators::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                let value = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

                if ::validators::number::precise(&value.to_string(), &input) {
                    Ok(value)
                } else {
                    Err(::validators::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let re = ::validators::regex::Regex::new($re).map_err(|err| ::validators::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(input) {
                let value = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

                if ::validators::number::precise(&value.to_string(), input) {
                    Ok(value)
                } else {
                    Err(::validators::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let input = input.to_string();

            let re = ::validators::regex::Regex::new($re).map_err(|err| ::validators::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                let value = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

                if ::validators::number::precise(&value.to_string(), &input) {
                    Ok(value)
                } else {
                    Err(::validators::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        });
    };
    ( $name:ident, $field:ident, $t:ident, ref $re:expr ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let re: &::validators::regex::Regex = &$re;

            if re.is_match(&input) {
                let value = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

                if ::validators::number::precise(&value.to_string(), &input) {
                    Ok(value)
                } else {
                    Err(::validators::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let re: &::validators::regex::Regex = &$re;

            if re.is_match(input) {
                let value = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

                if ::validators::number::precise(&value.to_string(), input) {
                    Ok(value)
                } else {
                    Err(::validators::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let input = input.to_string();

            let re: &::validators::regex::Regex = &$re;

            if re.is_match(&input) {
                let value = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

                if ::validators::number::precise(&value.to_string(), &input) {
                    Ok(value)
                } else {
                    Err(::validators::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_regex_number {
    ( $name:ident, $t:ident, $re:expr ) => {
        #[derive(Clone, PartialEq)]
        struct $name{
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, $re);
    };
    ( pub $name:ident, $t:ident, $re:expr ) => {
        #[derive(Clone, PartialEq)]
        pub struct $name{
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, $re);
    };
    ( $name:ident, $t:ident, ref $re:expr ) => {
        #[derive(Clone, PartialEq)]
        struct $name{
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, ref $re);
    };
    ( pub $name:ident, $t:ident, ref $re:expr ) => {
        #[derive(Clone, PartialEq)]
        pub struct $name{
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
            let value = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if value >= $min && value <= $max {
                if ::validators::number::precise(&value.to_string(), &input) {
                    Ok(value)
                } else {
                    Err(::validators::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err(::validators::ValidatedCustomizedNumberError::OutRange)
            }
        },
        input {
            let value = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if value >= $min && value <= $max {
                if ::validators::number::precise(&value.to_string(), input) {
                    Ok(value)
                } else {
                    Err(::validators::ValidatedCustomizedNumberError::UnpreciseError)
                }
            } else{
                Err(::validators::ValidatedCustomizedNumberError::OutRange)
            }
        },
        input {
            if input >= $min && input <= $max {
                Ok(input)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::OutRange)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_number {
    ( $name:ident, $t:ident, $min:expr, $max:expr ) => {
        #[derive(Clone, PartialEq)]
        struct $name{
            n: $t
        }

        validated_customized_ranged_number_struct!($name, n, $t, $min, $max);
    };
    ( pub $name:ident, $t:ident, $min:expr, $max:expr ) => {
        #[derive(Clone, PartialEq)]
        pub struct $name{
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
            let value = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if ::validators::number::precise(&value.to_string(), &input) {
                    Ok(value)
                } else {
                    Err(::validators::ValidatedCustomizedNumberError::UnpreciseError)
                }
        },
        input {
            let value = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if ::validators::number::precise(&value.to_string(), input) {
                    Ok(value)
                } else {
                    Err(::validators::ValidatedCustomizedNumberError::UnpreciseError)
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
        #[derive(Clone, PartialEq)]
        struct $name{
            n: $t
        }

        validated_customized_primitive_number_struct!($name, n, $t);
    };
    ( pub $name:ident, $t:ident ) => {
        #[derive(Clone, PartialEq)]
        pub struct $name{
            n: $t
        }

        validated_customized_primitive_number_struct!($name, n, $t);
    };
}