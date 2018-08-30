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

pub trait Validated: Display + PartialEq {}

pub mod domain;
pub mod email;
pub mod ipv4;

