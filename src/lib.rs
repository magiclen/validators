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
//! assert_eq!(8080, domain.get_port()); // This function does not used `Option` for its return value, because the struct `DomainLocalhostableWithPort` makes sure the input must have a port number!
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

