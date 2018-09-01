//! # Validators
//!
//! This crate provides many validators for validating data from users and modeling them to structs without much extra effort.
//!
//! All validators are separated into different modules and unified for two main types: **XXX** and **XXXValidator** where **XXX** is a type that you want to validate.
//! The former is a struct or a enum, and the latter is a struct which can be considered as a generator of the former.
//! A **XXXValidator** struct usually contains some values of `ValidatorOption` in order to use different rules to check data.
//!
//! For example, the mod `domain` has `Domain` and `DomainValidator` structs. If we want to create a `Domain` instance, we need to create a `DomainValidator` instance first.
//! When initialing a `DomainValidator`, we can choose to make this `DomainValidator` **allow** or **not allow** the input to have or **must** have a port number.
//!
//! ```
//! extern crate validators;
//!
//! use validators::ValidatorOption;
//! use validators::domain::{Domain, DomainValidator};
//!
//! let domain = "tool.magiclen.org:8080".to_string();
//!
//! let dv = DomainValidator {
//!     port: ValidatorOption::Allow,
//!     localhost: ValidatorOption::NotAllow,
//! };
//!
//! let domain = dv.parse_string(domain).unwrap();
//!
//! assert_eq!("tool.magiclen.org:8080", domain.get_full_domain());
//! assert_eq!("tool.magiclen.org", domain.get_full_domain_without_port());
//! assert_eq!("org", domain.get_top_level_domain().unwrap());
//! assert_eq!("tool", domain.get_sub_domain().unwrap());
//! assert_eq!("magiclen", domain.get_domain());
//! assert_eq!(8080, domain.get_port().unwrap());
//! ```
//!
//! If you want the **XXX** model to be stricter, you can use its wrapper type which is something like **XXXWithPort** or **XXXWithoutPort**.
//! For instance, `Domain` has some wrappers, such as **DomainLocalhostableWithPort**, **DomainLocalhostableAllowPort** and **DomainLocalhostableWithoutPort**.
//!
//! ```
//! extern crate validators;
//!
//! use validators::domain::{DomainLocalhostableWithPort};
//!
//! let domain = "tool.magiclen.org:8080".to_string();
//!
//! let domain = DomainLocalhostableWithPort::from_string(domain).unwrap();
//!
//! assert_eq!("tool.magiclen.org:8080", domain.get_full_domain());
//! assert_eq!("tool.magiclen.org", domain.get_full_domain_without_port());
//! assert_eq!("org", domain.get_top_level_domain().unwrap());
//! assert_eq!("tool", domain.get_sub_domain().unwrap());
//! assert_eq!("magiclen", domain.get_domain());
//! assert_eq!(8080, domain.get_port()); // This function does not use `Option` as its return value, because the struct `DomainLocalhostableWithPort` has already made sure the input must have a port number!
//! ```
//!
//! This crate aims to use the simplest and slackest way (normally only use regular expressions) to validate data, in order to minimize the overhead.
//! Therefore, it may not be competent in some critical situations. Use it carefully.

#![cfg_attr(feature = "nightly", feature(ip))]

#[doc(hidden)]
pub extern crate regex;

#[cfg(feature = "rocketly")]
#[doc(hidden)]
pub extern crate rocket;

use std::fmt::{Display, Debug};
use std::cmp::PartialEq;

pub enum ValidatorOption {
    Must,
    Allow,
    NotAllow,
}

impl ValidatorOption {
    pub fn allow(&self) -> bool {
        match self {
            ValidatorOption::Must => true,
            ValidatorOption::Allow => true,
            ValidatorOption::NotAllow => false
        }
    }

    pub fn not_allow(&self) -> bool {
        match self {
            ValidatorOption::Must => false,
            ValidatorOption::Allow => false,
            ValidatorOption::NotAllow => true
        }
    }

    pub fn must(&self) -> bool {
        match self {
            ValidatorOption::Must => true,
            ValidatorOption::Allow => false,
            ValidatorOption::NotAllow => false
        }
    }
}

pub trait Validated: Display + PartialEq + Clone + Debug {}

pub mod domain;
pub mod email;
pub mod ipv4;
pub mod ipv6;
pub mod host;
pub mod http_url;

// TODO -----ValidatedCustomizedString START-----

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedStringError {
    RegexError(regex::Error),
    NotMatch,
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_string_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a> ::validators::rocket::request::FromFormValue<'a> for $name {
            type Error = ::validators::ValidatedCustomizedStringError;

            fn from_form_value(form_value: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_str(form_value)
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
        impl Clone for $name {
            fn clone(&self) -> Self{
                let $field = self.$field.clone();

                $name{$field}
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.$field))?;
                Ok(())
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(&self.$field)?;
                Ok(())
            }
        }

        impl std::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.$field.eq(&other.$field)
            }

            fn ne(&self, other: &Self) -> bool {
                self.$field.ne(&other.$field)
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                self.$field.as_bytes()
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.$field.as_ref()
            }
        }

        impl ::validators::Validated for $name {}

        impl<'a> $name {
            fn as_str(&'a self) -> &'a str {
                &self.$field
            }

            fn from_string($from_string_input: String) -> Result<Self, ::validators::ValidatedCustomizedStringError>{
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            fn from_str($from_str_input: &str) -> Result<Self, ::validators::ValidatedCustomizedStringError>{
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }
        }

        validated_customized_string_struct_implement_from_form_value!($name);
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
        struct $name{
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
    ( pub $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        pub struct $name{
            s: String
        }

        validated_customized_string_struct!($name, s, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_string!(pub $name, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_string!(pub $name, $from_string_input $from_string, $from_str_input $from_str);
    };
}

#[macro_export]
macro_rules! validated_customized_regex_string_struct {
    ( $name:ident, $field:ident, $re:expr ) => {
        validated_customized_string_struct!($name, $field,
        input {
            let re = ::validators::regex::Regex::new($re).map_err(|err| ::validators::ValidatedCustomizedStringError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input)
            } else{
                Err(::validators::ValidatedCustomizedStringError::NotMatch)
            }
        },
        input {
            let re = ::validators::regex::Regex::new($re).map_err(|err| ::validators::ValidatedCustomizedStringError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input.to_string())
            } else{
                Err(::validators::ValidatedCustomizedStringError::NotMatch)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_regex_string {
    ( $name:ident, $re:expr ) => {
        struct $name{
            s: String
        }

        validated_customized_regex_string_struct!($name, s, $re);
    };
    ( pub $name:ident, $re:expr ) => {
        pub struct $name{
            s: String
        }

        validated_customized_regex_string_struct!($name, s, $re);
    };
}

// TODO -----ValidatedCustomizedString END-----

// TODO -----ValidatedCustomizedNumber START-----

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedNumberError {
    RegexError(regex::Error),
    ParseError(String),
    OutRange,
    NotMatch,
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a> ::validators::rocket::request::FromFormValue<'a> for $name {
            type Error = ::validators::ValidatedCustomizedNumberError;

            fn from_form_value(form_value: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_str(form_value)
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
    ( $name:ident, $field:ident, $t:ty, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        impl Clone for $name {
            fn clone(&self) -> Self{
                let $field = self.$field;

                $name{$field}
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.$field))?;
                Ok(())
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_fmt(format_args!("{}", self.$field))?;
                Ok(())
            }
        }

        impl std::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.$field == other.$field
            }

            fn ne(&self, other: &Self) -> bool {
                self.$field != other.$field
            }
        }

        impl ::validators::Validated for $name {}

        impl $name {
            fn get_number(&self) -> $t {
                self.$field
            }

            fn from_string($from_string_input: String) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            fn from_str($from_str_input: &str) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }
        }

        validated_customized_number_struct_implement_from_form_value!($name);
    };
    ( $name:ident, $field:ident, $t:ty, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ty, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ty, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ty, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ty, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ty, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
}

#[macro_export]
macro_rules! validated_customized_number {
    ( $name:ident, $t:ty, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        struct $name{
            n: $t
        }

        validated_customized_number_struct!($name, n, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ty, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ty, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ty, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ty, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ty, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ty, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_number_struct!($name, n, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ty, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
}

#[macro_export]
macro_rules! validated_customized_regex_number_struct {
    ( $name:ident, $field:ident, $t:ty, $re:expr ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let re = ::validators::regex::Regex::new($re).map_err(|err| ::validators::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let re = ::validators::regex::Regex::new($re).map_err(|err| ::validators::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let input = input.to_string();

            let re = ::validators::regex::Regex::new($re).map_err(|err| ::validators::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_regex_number {
    ( $name:ident, $t:ty, $re:expr ) => {
        struct $name{
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, $re);
    };
    ( pub $name:ident, $t:ty, $re:expr ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, $re);
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_number_struct {
    ( $name:ident, $field:ident, $t:ty, $min:expr, $max:expr ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let input = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if input >= $min && input <= $max {
                Ok(input)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::OutRange)
            }
        },
        input {
            let input = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if input >= $min && input <= $max {
                Ok(input)
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
    ( $name:ident, $t:ty, $min:expr, $max:expr ) => {
        struct $name{
            n: $t
        }

        validated_customized_ranged_number_struct!($name, n, $t, $min, $max);
    };
    ( pub $name:ident, $t:ty, $min:expr, $max:expr ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_ranged_number_struct!($name, n, $t, $min, $max);
    };
}

#[macro_export]
macro_rules! validated_customized_primitive_number_struct {
    ( $name:ident, $field:ident, $t:ty ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let input = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            Ok(input)
        },
        input {
            let input = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            Ok(input)
        },
        input {
            Ok(input)
        });
    };
}

#[macro_export]
macro_rules! validated_customized_primitive_number {
    ( $name:ident, $t:ty ) => {
        struct $name{
            n: $t
        }

        validated_customized_primitive_number_struct!($name, n, $t);
    };
    ( pub $name:ident, $t:ty ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_primitive_number_struct!($name, n, $t);
    };
}

// TODO -----ValidatedCustomizedNumber END-----