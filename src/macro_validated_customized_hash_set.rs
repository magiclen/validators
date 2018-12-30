use super::ValidatedWrapper;

use std::error::Error;
use std::str::Utf8Error;
use std::fmt::{self, Display, Debug, Formatter};
use std::hash::Hash;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedHashSetError {
    Overflow,
    Underflow,
    NotSupport,
    UTF8Error(Utf8Error),
}

impl Display for ValidatedCustomizedHashSetError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ValidatedCustomizedHashSetError {}

pub trait ValidatedHashSetWrapper<T: ValidatedWrapper + Eq + Hash>: ValidatedWrapper {
    fn from_hash_set(s: HashSet<T>) -> Result<Self, ValidatedCustomizedHashSetError>;
}

#[cfg(feature = "serdely")]
pub struct HashSetVisitor<V, T>(pub Vec<V>, pub Vec<T>);

#[cfg(feature = "serdely")]
impl<'de, V: ValidatedHashSetWrapper<T>, T: ValidatedWrapper + Eq + Hash + serde::Deserialize<'de>> serde::de::Visitor<'de> for HashSetVisitor<V, T> {
    type Value = V;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("an array({})", stringify!($name)))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
        let mut v = HashSet::<T>::new();

        loop {
            match seq.next_element()? {
                Some(e) => {
                    v.insert(e);
                }
                None => { break; }
            }
        }

        Ok(V::from_hash_set(v).map_err(|err| {
            serde::de::Error::custom(err.to_string())
        })?)
    }
}

#[cfg(feature = "serdely")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_hash_set_struct_implement_se_de {
     ( $name:ident ) => {
        impl<'de, T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash + ::validators::serde::Deserialize<'de>> ::validators::serde::Deserialize<'de> for $name<T> {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: ::validators::serde::Deserializer<'de> {
                deserializer.deserialize_seq(::validators::HashSetVisitor(Vec::<$name<T>>::new(), Vec::<T>::new()))
            }
        }

        impl<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash + ::validators::serde::Serialize> ::validators::serde::Serialize for $name<T> {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: ::validators::serde::Serializer {
                serializer.collect_seq(self.as_hash_set().iter())
            }
        }
     }
}

#[cfg(not(feature = "serdely"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_hash_set_struct_implement_se_de {
    ( $name:ident ) => {

    }
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_hash_set_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a, T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> ::validators::rocket::request::FromFormValue<'a> for $name<T> {
            type Error = ::validators::ValidatedCustomizedHashSetError;

            fn from_form_value(form_value: &'a ::validators::rocket::http::RawStr) -> std::result::Result<Self, Self::Error> {
                $name::from_string(form_value.url_decode().map_err(|err| ::validators::ValidatedCustomizedHashSetError::UTF8Error(err))?)
            }
        }

        impl<'a, T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> ::validators::rocket::request::FromParam<'a> for $name<T> {
            type Error = ::validators::ValidatedCustomizedHashSetError;

            fn from_param(param: &'a ::validators::rocket::http::RawStr) -> std::result::Result<Self, Self::Error> {
                $name::from_string(param.url_decode().map_err(|err| ::validators::ValidatedCustomizedHashSetError::UTF8Error(err))?)
            }
        }

    }
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_hash_set_struct_implement_from_form_value {
    ( $name:ident ) => {

    }
}

#[macro_export]
macro_rules! validated_customized_hash_set_struct {
    ( $name:ident, $field:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_hash_set_input:ident $from_hash_set:block ) => {
        impl<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> ::std::fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}[", stringify!($name)))?;

                let len = self.$field.len();

                if len > 0 {
                     for n in self.$field.iter().take(1) {
                        ::std::fmt::Display::fmt(n, f)?;
                    }

                    for n in self.$field.iter().skip(1) {
                        f.write_str(", ")?;
                        ::std::fmt::Display::fmt(n, f)?;
                    }
                }

                f.write_str("]")?;

                Ok(())
            }
        }

        impl<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> ::std::fmt::Display for $name<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("[")?;

                let len = self.$field.len();

                if len > 0 {
                    for n in self.$field.iter().take(1) {
                        ::std::fmt::Display::fmt(n, f)?;
                    }

                    for n in self.$field.iter().skip(1) {
                        f.write_str(", ")?;
                        ::std::fmt::Display::fmt(n, f)?;
                    }
                }

                f.write_str("]")?;

                Ok(())
            }
        }

        impl<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> ::std::hash::Hash for $name<T> {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H){
                for e in &self.$field {
                    e.hash(state)
                }
            }
        }

        impl<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> ::std::ops::Deref for $name<T> {
            type Target = ::std::collections::HashSet<T>;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> ::validators::Validated for $name<T> {}

        impl<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> ::validators::ValidatedWrapper for $name<T> {
            type Error = ::validators::ValidatedCustomizedHashSetError;

            fn from_string($from_string_input: String) -> std::result::Result<Self, Self::Error> {
                $name::from_string($from_string_input)
            }

            fn from_str($from_str_input: &str) -> std::result::Result<Self, Self::Error> {
                $name::from_str($from_str_input)
            }
        }

        impl<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> ::validators::ValidatedHashSetWrapper<T> for $name<T> {
            fn from_hash_set($from_hash_set_input: ::std::collections::HashSet<T>) -> std::result::Result<Self, ::validators::ValidatedCustomizedHashSetError> {
                $name::from_hash_set($from_hash_set_input)
            }
        }

        impl<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> $name<T> {
            pub fn as_hash_set(&self) -> &::std::collections::HashSet<T> {
                &self.$field
            }

            pub fn into_hash_set(self) -> ::std::collections::HashSet<T> {
                self.$field
            }

            pub fn from_string($from_string_input: String) -> std::result::Result<Self, ::validators::ValidatedCustomizedHashSetError> {
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub fn from_str($from_str_input: &str) -> std::result::Result<Self, ::validators::ValidatedCustomizedHashSetError> {
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub fn from_hash_set($from_hash_set_input: ::std::collections::HashSet<T>) -> std::result::Result<Self, ::validators::ValidatedCustomizedHashSetError> {
                let $field = match $from_hash_set {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub unsafe fn from_hash_set_unchecked($from_hash_set_input: ::std::collections::HashSet<T>) -> Self {
                $name{$field:$from_hash_set_input}
            }
        }

         validated_customized_hash_set_struct_implement_from_form_value!($name);
         validated_customized_hash_set_struct_implement_se_de!($name);
    };
}

#[macro_export]
macro_rules! validated_customized_hash_set {
    ( $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_hash_set_input:ident $from_hash_set:block ) => {
        #[derive(Clone, PartialEq, Eq)]
        struct $name<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> {
            v: ::std::collections::HashSet<T>
        }

        validated_customized_hash_set_struct!($name, v, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_hash_set $from_hash_set_input:ident $from_hash_set:block ) => {
        validated_customized_hash_set!($name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_hash_set $from_hash_set_input:ident $from_hash_set:block ) => {
        validated_customized_hash_set!($name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( $name:ident, from_hash_set $from_hash_set_input:ident $from_hash_set:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_hash_set!($name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( $name:ident, from_hash_set $from_hash_set_input:ident $from_hash_set:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_hash_set!($name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( $name:ident, from_string $from_string_input:ident $from_string:block, from_hash_set $from_hash_set_input:ident $from_hash_set:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_hash_set!($name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( $name:ident, from_str $from_str_input:ident $from_str:block, from_hash_set $from_hash_set_input:ident $from_hash_set:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_hash_set!($name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( pub $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_hash_set_input:ident $from_hash_set:block ) => {
        #[derive(Clone, PartialEq, Eq)]
        pub struct $name<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> {
            v: ::std::collections::HashSet<T>
        }

        validated_customized_hash_set_struct!($name, v, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( pub $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_hash_set $from_hash_set_input:ident $from_hash_set:block ) => {
        validated_customized_hash_set!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( pub $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_hash_set $from_hash_set_input:ident $from_hash_set:block ) => {
        validated_customized_hash_set!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( pub $name:ident, from_hash_set $from_hash_set_input:ident $from_hash_set:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_hash_set!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( pub $name:ident, from_hash_set $from_hash_set_input:ident $from_hash_set:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_hash_set!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( pub $name:ident, from_string $from_string_input:ident $from_string:block, from_hash_set $from_hash_set_input:ident $from_hash_set:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_hash_set!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( pub $name:ident, from_str $from_str_input:ident $from_str:block, from_hash_set $from_hash_set_input:ident $from_hash_set:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_hash_set!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_length_hash_set_struct {
    ( $name:ident, $field:expr, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_hash_set_struct!($name, v,
        $from_string_input $from_string,
        $from_str_input $from_str,
        input {
            let len = input.len();

            if len > $max {
                Err(::validators::ValidatedCustomizedHashSetError::Overflow)
            } else if len < $min {
                Err(::validators::ValidatedCustomizedHashSetError::Underflow)
            } else {
                Ok(input)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_length_hash_set {
    ( $name:ident, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        #[derive(Clone, PartialEq, Eq)]
        struct $name<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> {
            v: ::std::collections::HashSet<T>
        }

        validated_customized_ranged_length_hash_set_struct!($name, v, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $min:expr, $max:expr, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_hash_set!($name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $min:expr, $max:expr, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block) => {
        validated_customized_ranged_length_hash_set!($name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $min:expr, $max:expr) => {
        validated_customized_ranged_length_hash_set!($name, $min, $max,
        _input {Err(::validators::ValidatedCustomizedHashSetError::NotSupport)},
        _input {Err(::validators::ValidatedCustomizedHashSetError::NotSupport)});
    };
    ( $name:ident, $equal:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_hash_set!($name, $equal, $equal, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $equal:expr) => {
        validated_customized_ranged_length_hash_set!($name, $equal, $equal);
    };
    ( pub $name:ident, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        #[derive(Clone, PartialEq, Eq)]
        pub struct $name<T: ::validators::ValidatedWrapper + Eq + ::std::hash::Hash> {
            v: ::std::collections::HashSet<T>
        }

        validated_customized_ranged_length_hash_set_struct!($name, v, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $min:expr, $max:expr, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_hash_set!(pub $name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $min:expr, $max:expr, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block) => {
        validated_customized_ranged_length_hash_set!(pub $name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $min:expr, $max:expr) => {
        validated_customized_ranged_length_hash_set!(pub $name, $min, $max,
        _input {Err(::validators::ValidatedCustomizedHashSetError::NotSupport)},
        _input {Err(::validators::ValidatedCustomizedHashSetError::NotSupport)});
    };
    ( pub $name:ident, $equal:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_hash_set!(pub $name, $equal, $equal, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $equal:expr) => {
        validated_customized_ranged_length_hash_set!(pub $name, $equal, $equal);
    };
}