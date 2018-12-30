use super::ValidatedWrapper;

use std::error::Error;
use std::str::Utf8Error;
use std::fmt::{self, Display, Debug, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedVecError {
    Overflow,
    Underflow,
    NotSupport,
    UTF8Error(Utf8Error),
}

impl Display for ValidatedCustomizedVecError {
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
impl<'de, V: ValidatedVecWrapper<T>, T: ValidatedWrapper + serde::Deserialize<'de>> serde::de::Visitor<'de> for VecVisitor<V, T> {
    type Value = V;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("an array({})", stringify!($name)))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
        let mut v = Vec::<T>::new();

        loop {
            match seq.next_element()? {
                Some(e) => {
                    v.push(e);
                }
                None => { break; }
            }
        }

        Ok(V::from_vec(v).map_err(|err| {
            serde::de::Error::custom(err.to_string())
        })?)
    }
}

#[cfg(feature = "serdely")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_se_de {
     ( $name:ident ) => {
        impl<'de, T: ::validators::ValidatedWrapper + ::validators::serde::Deserialize<'de>> ::validators::serde::Deserialize<'de> for $name<T> {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: ::validators::serde::Deserializer<'de> {
                deserializer.deserialize_seq(::validators::VecVisitor(Vec::<$name<T>>::new(), Vec::<T>::new()))
            }
        }

        impl<T: ::validators::ValidatedWrapper + ::validators::serde::Serialize> ::validators::serde::Serialize for $name<T> {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: ::validators::serde::Serializer {
                serializer.collect_seq(self.as_vec().iter())
            }
        }
     }
}

#[cfg(not(feature = "serdely"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_se_de {
    ( $name:ident ) => {

    }
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a, T: ::validators::ValidatedWrapper> ::validators::rocket::request::FromFormValue<'a> for $name<T> {
            type Error = ::validators::ValidatedCustomizedVecError;

            fn from_form_value(form_value: &'a ::validators::rocket::http::RawStr) -> std::result::Result<Self, Self::Error>{
                $name::from_string(form_value.url_decode().map_err(|err| ::validators::ValidatedCustomizedVecError::UTF8Error(err))?)
            }
        }

        impl<'a, T: ::validators::ValidatedWrapper> ::validators::rocket::request::FromParam<'a> for $name<T> {
            type Error = ::validators::ValidatedCustomizedVecError;

            fn from_param(param: &'a ::validators::rocket::http::RawStr) -> std::result::Result<Self, Self::Error> {
                $name::from_string(param.url_decode().map_err(|err| ::validators::ValidatedCustomizedVecError::UTF8Error(err))?)
            }
        }

    }
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_from_form_value {
    ( $name:ident ) => {

    }
}

#[macro_export]
macro_rules! validated_customized_vec_struct {
    ( $name:ident, $field:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_vec_input:ident $from_vec:block ) => {
        impl<T: ::validators::ValidatedWrapper> ::std::fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}[", stringify!($name)))?;

                let len = self.$field.len();

                if len > 0 {
                    for n in self.$field.iter().take(len - 1) {
                        ::std::fmt::Debug::fmt(n, f)?;


                        f.write_str(", ")?;
                    }

                    ::std::fmt::Debug::fmt(&self.$field[len - 1], f)?;
                }

                f.write_str("]")?;

                Ok(())
            }
        }

        impl<T: ::validators::ValidatedWrapper> ::std::fmt::Display for $name<T> {
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

        impl<T: ::validators::ValidatedWrapper> ::std::cmp::Eq for $name<T> {}

        impl<T: ::validators::ValidatedWrapper> ::std::ops::Deref for $name<T> {
            type Target = Vec<T>;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl<T: ::validators::ValidatedWrapper> ::validators::Validated for $name<T> {}

        impl<T: ::validators::ValidatedWrapper> ::validators::ValidatedWrapper for $name<T> {
            type Error = ::validators::ValidatedCustomizedVecError;

            fn from_string($from_string_input: String) -> std::result::Result<Self, Self::Error>{
                $name::from_string($from_string_input)
            }

            fn from_str($from_str_input: &str) -> std::result::Result<Self, Self::Error>{
                $name::from_str($from_str_input)
            }
        }

        impl<T: ::validators::ValidatedWrapper> ::validators::ValidatedVecWrapper<T> for $name<T> {
            fn from_vec($from_vec_input: Vec<T>) -> std::result::Result<Self, ::validators::ValidatedCustomizedVecError>{
                $name::from_vec($from_vec_input)
            }
        }

        impl<T: ::validators::ValidatedWrapper> $name<T> {
            pub fn as_vec(&self) -> &Vec<T> {
                &self.$field
            }

            pub fn into_vec(self) -> Vec<T> {
                self.$field
            }

            pub fn from_string($from_string_input: String) -> std::result::Result<Self, ::validators::ValidatedCustomizedVecError>{
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub fn from_str($from_str_input: &str) -> std::result::Result<Self, ::validators::ValidatedCustomizedVecError>{
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub fn from_vec($from_vec_input: Vec<T>) -> std::result::Result<Self, ::validators::ValidatedCustomizedVecError>{
                let $field = match $from_vec {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub unsafe fn from_vec_unchecked($from_vec_input: Vec<T>) -> Self{
                $name{$field:$from_vec_input}
            }
        }

         validated_customized_vec_struct_implement_from_form_value!($name);
         validated_customized_vec_struct_implement_se_de!($name);
    };
}

#[macro_export]
macro_rules! validated_customized_vec {
    ( $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_vec_input:ident $from_vec:block ) => {
        #[derive(Clone, PartialEq)]
        struct $name<T: ::validators::ValidatedWrapper> {
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
    ( pub $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_vec_input:ident $from_vec:block ) => {
        #[derive(Clone, PartialEq)]
        pub struct $name<T: ::validators::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_vec_struct!($name, v, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
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
                Err(::validators::ValidatedCustomizedVecError::Overflow)
            } else if len < $min {
                Err(::validators::ValidatedCustomizedVecError::Underflow)
            } else {
                Ok(input)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_length_vec {
    ( $name:ident, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        #[derive(Clone, PartialEq)]
        struct $name<T: ::validators::ValidatedWrapper> {
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
        _input {Err(::validators::ValidatedCustomizedVecError::NotSupport)},
        _input {Err(::validators::ValidatedCustomizedVecError::NotSupport)});
    };
    ( $name:ident, $equal:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!($name, $equal, $equal, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $equal:expr) => {
        validated_customized_ranged_length_vec!($name, $equal, $equal);
    };
    ( pub $name:ident, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        #[derive(Clone, PartialEq)]
        pub struct $name<T: ::validators::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_ranged_length_vec_struct!($name, v, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $min:expr, $max:expr, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!(pub $name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $min:expr, $max:expr, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block) => {
        validated_customized_ranged_length_vec!(pub $name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $min:expr, $max:expr) => {
        validated_customized_ranged_length_vec!(pub $name, $min, $max,
        _input {Err(::validators::ValidatedCustomizedVecError::NotSupport)},
        _input {Err(::validators::ValidatedCustomizedVecError::NotSupport)});
    };
    ( pub $name:ident, $equal:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!(pub $name, $equal, $equal, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $equal:expr) => {
        validated_customized_ranged_length_vec!(pub $name, $equal, $equal);
    };
}