#[cfg(feature = "serdely")]
use super::ValidatedWrapper;

use std::error::Error;
use std::str::Utf8Error;
use std::fmt::{self, Display, Debug, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedStringError {
    RegexError(regex::Error),
    NotMatch,
    UTF8Error(Utf8Error),
}

impl Display for ValidatedCustomizedStringError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ValidatedCustomizedStringError {}

#[cfg(feature = "serdely")]
pub struct StringVisitor<V>(pub Vec<V>);

#[cfg(feature = "serdely")]
impl<'de, V: ValidatedWrapper> serde::de::Visitor<'de> for StringVisitor<V> {
    type Value = V;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("a string({})", stringify!($name)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_str(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_string(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

#[cfg(feature = "serdely")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_string_struct_implement_se_de {
     ( $name:ident ) => {
        impl<'de> $crate::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error> where D: $crate::serde::Deserializer<'de> {
                deserializer.deserialize_string($crate::StringVisitor(Vec::<$name>::new()))
            }
        }

        impl $crate::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> where S: $crate::serde::Serializer {
                serializer.serialize_str(self.as_str())
            }
        }
     }
}

#[cfg(not(feature = "serdely"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_string_struct_implement_se_de {
    ( $name:ident ) => {

    }
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_string_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a> $crate::rocket::request::FromFormValue<'a> for $name {
            type Error = $crate::ValidatedCustomizedStringError;

            fn from_form_value(form_value: &'a $crate::rocket::http::RawStr) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string(form_value.url_decode().map_err(|err| $crate::ValidatedCustomizedStringError::UTF8Error(err))?)
            }
        }

        impl<'a> $crate::rocket::request::FromParam<'a> for $name {
            type Error = $crate::ValidatedCustomizedStringError;

            fn from_param(param: &'a $crate::rocket::http::RawStr) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string(param.url_decode().map_err(|err| $crate::ValidatedCustomizedStringError::UTF8Error(err))?)
            }
        }
    }
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_string_struct_implement_from_form_value {
    ( $name:ident ) => {

    }
}

#[macro_export]
macro_rules! validated_customized_string_struct {
    ( $name:ident, $field:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                $crate::debug_helper::impl_debug_for_tuple_struct!($name, f, self, let .0 = self.$field);
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(&self.$field)?;
                Ok(())
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl $crate::Validated for $name {}

        impl $crate::ValidatedWrapper for $name {
            type Error = $crate::ValidatedCustomizedStringError;

            fn from_string($from_string_input: String) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string($from_string_input)
            }

            fn from_str($from_str_input: &str) -> ::std::result::Result<Self, Self::Error> {
                $name::from_str($from_str_input)
            }
        }

        impl<'a> $name {
            pub fn as_str(&'a self) -> &'a str {
                &self.$field
            }

            pub fn into_string(self) -> String {
                self.$field
            }

            pub fn from_string($from_string_input: String) -> ::std::result::Result<Self, $crate::ValidatedCustomizedStringError> {
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name {$field})
            }

            pub fn from_str($from_str_input: &str) -> ::std::result::Result<Self, $crate::ValidatedCustomizedStringError> {
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name {$field})
            }

            pub unsafe fn from_string_unchecked($from_string_input: String) -> Self {
                $name {$field:$from_string_input}
            }
        }

        validated_customized_string_struct_implement_from_form_value!($name);

        validated_customized_string_struct_implement_se_de!($name);
    };
    ( $name:ident, $field:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_string_struct!($name, $field, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $field:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_string_struct!($name, $field, $from_string_input $from_string, $from_str_input $from_str);
    };
}

#[macro_export]
macro_rules! validated_customized_string {
    ( $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct $name {
            s: String
        }

        validated_customized_string_struct!($name, s, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_string!($name, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_string!($name, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $v:vis $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        $v struct $name {
            s: String
        }

        validated_customized_string_struct!($name, s, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $v:vis $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_string!($v $name, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $v:vis $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_string!($v $name, $from_string_input $from_string, $from_str_input $from_str);
    };
}

#[macro_export]
macro_rules! validated_customized_regex_string_struct {
    ( $name:ident, $field:ident, $re:expr ) => {
        validated_customized_string_struct!($name, $field,
        input {
            let re = $crate::regex::Regex::new($re).map_err(|err| $crate::ValidatedCustomizedStringError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input)
            } else{
                Err($crate::ValidatedCustomizedStringError::NotMatch)
            }
        },
        input {
            let re = $crate::regex::Regex::new($re).map_err(|err| $crate::ValidatedCustomizedStringError::RegexError(err))?;

            if re.is_match(input) {
                Ok(input.to_string())
            } else{
                Err($crate::ValidatedCustomizedStringError::NotMatch)
            }
        });
    };
    ( $name:ident, $field:ident, ref $re:expr ) => {
        validated_customized_string_struct!($name, $field,
        input {
            let re: &$crate::regex::Regex = &$re;

            if re.is_match(&input) {
                Ok(input)
            } else{
                Err($crate::ValidatedCustomizedStringError::NotMatch)
            }
        },
        input {
            let re: &$crate::regex::Regex = &$re;

            if re.is_match(input) {
                Ok(input.to_string())
            } else{
                Err($crate::ValidatedCustomizedStringError::NotMatch)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_regex_string {
    ( $name:ident, $re:expr ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct $name {
            s: String
        }

        validated_customized_regex_string_struct!($name, s, $re);
    };
    ( $v:vis $name:ident, $re:expr ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        $v struct $name {
            s: String
        }

        validated_customized_regex_string_struct!($name, s, $re);
    };
    ( $name:ident, ref $re:expr ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct $name {
            s: String
        }

        validated_customized_regex_string_struct!($name, s, ref $re);
    };
    ( $v:vis $name:ident, ref $re:expr ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        $v struct $name {
            s: String
        }

        validated_customized_regex_string_struct!($name, s, ref $re);
    };
}