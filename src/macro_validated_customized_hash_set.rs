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
        impl<'de, T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash + $crate::serde::Deserialize<'de>> $crate::serde::Deserialize<'de> for $name<T> {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error> where D: $crate::serde::Deserializer<'de> {
                deserializer.deserialize_seq($crate::HashSetVisitor(Vec::<$name<T>>::new(), Vec::<T>::new()))
            }
        }

        impl<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash + $crate::serde::Serialize> $crate::serde::Serialize for $name<T> {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> where S: $crate::serde::Serializer {
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
        impl<'a, T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> $crate::rocket::request::FromFormValue<'a> for $name<T> {
            type Error = $crate::ValidatedCustomizedHashSetError;

            fn from_form_value(form_value: &'a $crate::rocket::http::RawStr) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string(form_value.url_decode().map_err(|err| $crate::ValidatedCustomizedHashSetError::UTF8Error(err))?)
            }
        }

        impl<'a, T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> $crate::rocket::request::FromParam<'a> for $name<T> {
            type Error = $crate::ValidatedCustomizedHashSetError;

            fn from_param(param: &'a $crate::rocket::http::RawStr) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string(param.url_decode().map_err(|err| $crate::ValidatedCustomizedHashSetError::UTF8Error(err))?)
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
        impl<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> ::std::fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                $crate::debug_helper::impl_debug_for_tuple_struct!($name, f, self, let .0 = self.$field);
            }
        }

        impl<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> ::std::fmt::Display for $name<T> {
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

        impl<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> ::std::hash::Hash for $name<T> {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H){
                for e in &self.$field {
                    e.hash(state)
                }
            }
        }

        impl<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> ::std::ops::Deref for $name<T> {
            type Target = ::std::collections::HashSet<T>;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> $crate::Validated for $name<T> {}

        impl<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> $crate::ValidatedWrapper for $name<T> {
            type Error = $crate::ValidatedCustomizedHashSetError;

            fn from_string($from_string_input: String) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string($from_string_input)
            }

            fn from_str($from_str_input: &str) -> ::std::result::Result<Self, Self::Error> {
                $name::from_str($from_str_input)
            }
        }

        impl<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> $crate::ValidatedHashSetWrapper<T> for $name<T> {
            fn from_hash_set($from_hash_set_input: ::std::collections::HashSet<T>) -> ::std::result::Result<Self, $crate::ValidatedCustomizedHashSetError> {
                $name::from_hash_set($from_hash_set_input)
            }
        }

        impl<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> $name<T> {
            pub fn as_hash_set(&self) -> &::std::collections::HashSet<T> {
                &self.$field
            }

            pub fn into_hash_set(self) -> ::std::collections::HashSet<T> {
                self.$field
            }

            pub fn from_string($from_string_input: String) -> ::std::result::Result<Self, $crate::ValidatedCustomizedHashSetError> {
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name {$field})
            }

            pub fn from_str($from_str_input: &str) -> ::std::result::Result<Self, $crate::ValidatedCustomizedHashSetError> {
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name {$field})
            }

            pub fn from_hash_set($from_hash_set_input: ::std::collections::HashSet<T>) -> ::std::result::Result<Self, $crate::ValidatedCustomizedHashSetError> {
                let $field = match $from_hash_set {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name {$field})
            }

            pub unsafe fn from_hash_set_unchecked($from_hash_set_input: ::std::collections::HashSet<T>) -> Self {
                $name {$field:$from_hash_set_input}
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
        struct $name<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> {
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
    ( $v:vis $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_hash_set_input:ident $from_hash_set:block ) => {
        #[derive(Clone, PartialEq, Eq)]
        $v struct $name<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> {
            v: ::std::collections::HashSet<T>
        }

        validated_customized_hash_set_struct!($name, v, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( $v:vis $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_hash_set $from_hash_set_input:ident $from_hash_set:block ) => {
        validated_customized_hash_set!($v $name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( $v:vis $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_hash_set $from_hash_set_input:ident $from_hash_set:block ) => {
        validated_customized_hash_set!($v $name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( $v:vis $name:ident, from_hash_set $from_hash_set_input:ident $from_hash_set:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_hash_set!($v $name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( $v:vis $name:ident, from_hash_set $from_hash_set_input:ident $from_hash_set:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_hash_set!($v $name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( $v:vis $name:ident, from_string $from_string_input:ident $from_string:block, from_hash_set $from_hash_set_input:ident $from_hash_set:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_hash_set!($v $name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
    };
    ( $v:vis $name:ident, from_str $from_str_input:ident $from_str:block, from_hash_set $from_hash_set_input:ident $from_hash_set:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_hash_set!($v $name, $from_string_input $from_string, $from_str_input $from_str, $from_hash_set_input $from_hash_set);
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
                Err($crate::ValidatedCustomizedHashSetError::Overflow)
            } else if len < $min {
                Err($crate::ValidatedCustomizedHashSetError::Underflow)
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
        struct $name<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> {
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
        _input {Err($crate::ValidatedCustomizedHashSetError::NotSupport)},
        _input {Err($crate::ValidatedCustomizedHashSetError::NotSupport)});
    };
    ( $name:ident, $equal:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_hash_set!($name, $equal, $equal, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $equal:expr) => {
        validated_customized_ranged_length_hash_set!($name, $equal, $equal);
    };
    ( $v:vis $name:ident, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        #[derive(Clone, PartialEq, Eq)]
        $v struct $name<T: $crate::ValidatedWrapper + Eq + ::std::hash::Hash> {
            v: ::std::collections::HashSet<T>
        }

        validated_customized_ranged_length_hash_set_struct!($name, v, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $v:vis $name:ident, $min:expr, $max:expr, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_hash_set!($v $name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $v:vis $name:ident, $min:expr, $max:expr, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block) => {
        validated_customized_ranged_length_hash_set!($v $name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $v:vis $name:ident, $min:expr, $max:expr) => {
        validated_customized_ranged_length_hash_set!($v $name, $min, $max,
        _input {Err($crate::ValidatedCustomizedHashSetError::NotSupport)},
        _input {Err($crate::ValidatedCustomizedHashSetError::NotSupport)});
    };
    ( $v:vis $name:ident, $equal:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_hash_set!($v $name, $equal, $equal, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $v:vis $name:ident, $equal:expr) => {
        validated_customized_ranged_length_hash_set!($v $name, $equal, $equal);
    };
}