use super::ValidatedWrapper;

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::Utf8Error;

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedVecError {
    Overflow,
    Underflow,
    NotSupport,
    UTF8Error(Utf8Error),
}

impl Display for ValidatedCustomizedVecError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ValidatedCustomizedVecError {}

pub trait ValidatedVecWrapper<T: ValidatedWrapper>: ValidatedWrapper {
    fn from_vec(v: Vec<T>) -> Result<Self, ValidatedCustomizedVecError>;
}

#[cfg(feature = "serdely")]
pub struct VecVisitor<V, T>(pub Vec<V>, pub Vec<T>);

#[cfg(feature = "serdely")]
impl<'de, V: ValidatedVecWrapper<T>, T: ValidatedWrapper + serde::Deserialize<'de>>
    serde::de::Visitor<'de> for VecVisitor<V, T>
{
    type Value = V;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("an array({})", stringify!($name)))
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>, {
        let mut v = Vec::<T>::new();

        while let Some(e) = seq.next_element()? {
            v.push(e);
        }

        Ok(V::from_vec(v).map_err(|err| serde::de::Error::custom(err.to_string()))?)
    }
}

#[cfg(feature = "serdely")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_se_de {
    ($name:ident) => {
        impl<'de, T: $crate::ValidatedWrapper + $crate::serde::Deserialize<'de>>
            $crate::serde::Deserialize<'de> for $name<T>
        {
            #[inline]
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: $crate::serde::Deserializer<'de>, {
                deserializer
                    .deserialize_seq($crate::VecVisitor(Vec::<$name<T>>::new(), Vec::<T>::new()))
            }
        }

        impl<T: $crate::ValidatedWrapper + $crate::serde::Serialize> $crate::serde::Serialize
            for $name<T>
        {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer, {
                serializer.collect_seq(self.as_vec().iter())
            }
        }
    };
}

#[cfg(not(feature = "serdely"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_se_de {
    ($name:ident) => {};
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_from_form_value {
    ($name:ident) => {
        impl<'a, T: $crate::ValidatedWrapper> $crate::rocket::request::FromFormValue<'a>
            for $name<T>
        {
            type Error = $crate::ValidatedCustomizedVecError;

            #[inline]
            fn from_form_value(
                form_value: &'a $crate::rocket::http::RawStr,
            ) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string(
                    form_value
                        .url_decode()
                        .map_err(|err| $crate::ValidatedCustomizedVecError::UTF8Error(err))?,
                )
            }
        }

        impl<'a, T: $crate::ValidatedWrapper> $crate::rocket::request::FromParam<'a> for $name<T> {
            type Error = $crate::ValidatedCustomizedVecError;

            #[inline]
            fn from_param(
                param: &'a $crate::rocket::http::RawStr,
            ) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string(
                    param
                        .url_decode()
                        .map_err(|err| $crate::ValidatedCustomizedVecError::UTF8Error(err))?,
                )
            }
        }
    };
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_from_form_value {
    ($name:ident) => {};
}

#[macro_export]
macro_rules! validated_customized_vec_struct {
    ( $name:ident, $field:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_vec_input:ident $from_vec:block ) => {
        impl<T: $crate::ValidatedWrapper> ::std::fmt::Debug for $name<T> {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                $crate::debug_helper::impl_debug_for_tuple_struct!($name, f, self, let .0 = self.$field);
            }
        }

        impl<T: $crate::ValidatedWrapper> ::std::fmt::Display for $name<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("[")?;

                let len = self.$field.len();

                if len > 0 {
                    for n in self.$field.iter().take(len - 1) {
                        ::std::fmt::Display::fmt(n, f)?;


                        f.write_str(", ")?;
                    }

                    ::std::fmt::Display::fmt(&self.$field[len - 1], f)?;
                }

                f.write_str("]")?;

                Ok(())
            }
        }

        impl<T: $crate::ValidatedWrapper> ::std::cmp::PartialEq for $name<T> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.$field.eq(&other.$field)
            }
        }

        impl<T: $crate::ValidatedWrapper> ::std::cmp::Eq for $name<T> {}

        impl<T: $crate::ValidatedWrapper> ::std::ops::Deref for $name<T> {
            type Target = Vec<T>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl<T: $crate::ValidatedWrapper> $crate::Validated for $name<T> {}

        impl<T: $crate::ValidatedWrapper> $crate::ValidatedWrapper for $name<T> {
            type Error = $crate::ValidatedCustomizedVecError;

            #[inline]
            fn from_string($from_string_input: String) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string($from_string_input)
            }

            #[inline]
            fn from_str($from_str_input: &str) -> ::std::result::Result<Self, Self::Error> {
                $name::from_str($from_str_input)
            }
        }

        impl<T: $crate::ValidatedWrapper> $crate::ValidatedVecWrapper<T> for $name<T> {
            #[inline]
            fn from_vec($from_vec_input: Vec<T>) -> ::std::result::Result<Self, $crate::ValidatedCustomizedVecError> {
                $name::from_vec($from_vec_input)
            }
        }

        impl<T: $crate::ValidatedWrapper> $name<T> {
            #[inline]
            pub fn as_vec(&self) -> &Vec<T> {
                &self.$field
            }

            #[inline]
            pub fn into_vec(self) -> Vec<T> {
                self.$field
            }

            #[inline]
            pub fn from_string($from_string_input: String) -> ::std::result::Result<Self, $crate::ValidatedCustomizedVecError> {
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name {$field})
            }

            #[inline]
            pub fn from_str($from_str_input: &str) -> ::std::result::Result<Self, $crate::ValidatedCustomizedVecError> {
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name {$field})
            }

            #[inline]
            pub fn from_vec($from_vec_input: Vec<T>) -> ::std::result::Result<Self, $crate::ValidatedCustomizedVecError> {
                let $field = match $from_vec {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name {$field})
            }

            #[inline]
            pub unsafe fn from_vec_unchecked($from_vec_input: Vec<T>) -> Self {
                $name {$field:$from_vec_input}
            }
        }

        impl<T: $crate::ValidatedWrapper> ::std::str::FromStr for $name<T> {
            type Err = $crate::ValidatedCustomizedVecError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, $crate::ValidatedCustomizedVecError> {
                $name::from_str(s)
            }
        }

        validated_customized_vec_struct_implement_from_form_value!($name);

        validated_customized_vec_struct_implement_se_de!($name);
    };
}

#[macro_export]
macro_rules! validated_customized_vec {
    ( $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_vec_input:ident $from_vec:block ) => {
        #[derive(Clone)]
        struct $name<T: $crate::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_vec_struct!($name, v, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $v:vis $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_vec_input:ident $from_vec:block ) => {
        #[derive(Clone)]
        $v struct $name<T: $crate::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_vec_struct!($name, v, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $v:vis $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!($v $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $v:vis $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!($v $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $v:vis $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!($v $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $v:vis $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!($v $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $v:vis $name:ident, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!($v $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $v:vis $name:ident, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!($v $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_length_vec_struct {
    ( $name:ident, $field:expr, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_vec_struct!($name, v,
        $from_string_input $from_string,
        $from_str_input $from_str,
        input {
            let len = input.len();

            if len > $max {
                Err($crate::ValidatedCustomizedVecError::Overflow)
            } else if len < $min {
                Err($crate::ValidatedCustomizedVecError::Underflow)
            } else {
                Ok(input)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_length_vec {
    ( $name:ident, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        #[derive(Clone)]
        struct $name<T: $crate::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_ranged_length_vec_struct!($name, v, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $min:expr, $max:expr, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!($name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $min:expr, $max:expr, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block) => {
        validated_customized_ranged_length_vec!($name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $min:expr, $max:expr) => {
        validated_customized_ranged_length_vec!($name, $min, $max,
        _input {Err($crate::ValidatedCustomizedVecError::NotSupport)},
        _input {Err($crate::ValidatedCustomizedVecError::NotSupport)});
    };
    ( $name:ident, $equal:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!($name, $equal, $equal, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $equal:expr) => {
        validated_customized_ranged_length_vec!($name, $equal, $equal);
    };
    ( $v:vis $name:ident, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        #[derive(Clone)]
        $v struct $name<T: $crate::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_ranged_length_vec_struct!($name, v, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $v:vis $name:ident, $min:expr, $max:expr, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!($v $name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $v:vis $name:ident, $min:expr, $max:expr, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block) => {
        validated_customized_ranged_length_vec!($v $name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $v:vis $name:ident, $min:expr, $max:expr) => {
        validated_customized_ranged_length_vec!($v $name, $min, $max,
        _input {Err($crate::ValidatedCustomizedVecError::NotSupport)},
        _input {Err($crate::ValidatedCustomizedVecError::NotSupport)});
    };
    ( $v:vis $name:ident, $equal:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!($v $name, $equal, $equal, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $v:vis $name:ident, $equal:expr) => {
        validated_customized_ranged_length_vec!($v $name, $equal, $equal);
    };
}
