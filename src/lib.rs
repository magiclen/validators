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
//! use validators::{ValidatorOption};
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

use std::fmt::Display;
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

pub trait Validated: Display + PartialEq + Clone {}

pub mod domain;
pub mod email;
pub mod ipv4;
pub mod ipv6;
pub mod host;
pub mod http_url;

pub trait ValidatedCustomizedString<'a>: Validated {
    type Error;

    fn as_str(&'a self) -> &'a str;

    fn from_string(s: String) -> Result<Self, Self::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Error>;
}

#[macro_export]
macro_rules! validated_customized_string_struct {
    ( $name:ident, $field:ident, $err:ty, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        impl Clone for $name {
            fn clone(&self) -> Self{
                let $field = self.$field.clone();

                $name{$field}
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(&self.$field)?;
                Ok(())
            }
        }

        impl PartialEq for $name {
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

        impl Validated for $name {}

        impl<'a> ValidatedCustomizedString<'a> for $name {
            type Error = $err;

            fn as_str(&'a self) -> &'a str {
                &self.$field
            }

            fn from_string($from_string_input: String) -> Result<Self, Self::Error>{
                let $field = match $from_string {
                    Ok(s)=>s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            fn from_str($from_str_input: &str) -> Result<Self, Self::Error>{
                let $field = match $from_str {
                    Ok(s)=>s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }
        }
    }
}

#[macro_export]
macro_rules! validated_customized_string {
    ( $name:ident, $err:ty, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        struct $name{
            s: String
        }

        validated_customized_string_struct!($name, s, $err, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $err:ty, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        pub struct $name{
            s: String
        }

        validated_customized_string_struct!($name, s, $err, $from_string_input $from_string, $from_str_input $from_str);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        validated_customized_string!(S1, (),
            input {
                Ok(input.to_string())
            },
            input {
                Ok(input.to_string())
            }
        );

        validated_customized_string!(pub S2, (),
            input {
                Ok(input.to_string())
            },
            input {
                Ok(input.to_string())
            }
        );
    }
}