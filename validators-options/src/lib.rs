/*!
# Validator Options

This crate is used with the [`validators-derive`](https://crates.io/crates/validators-derive) crate and the [`validators`](https://crates.io/crates/validators) crate.
*/

#![no_std]

#[macro_use]
extern crate educe;

#[macro_use]
extern crate enum_ordinalize;

/// Options for validators.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Educe, Ordinalize)]
#[educe(Default)]
pub enum ValidatorOption {
    Must,
    #[educe(Default)]
    Allow,
    NotAllow,
}

impl ValidatorOption {
    #[doc(hidden)]
    #[inline]
    pub const fn new() -> ValidatorOption {
        ValidatorOption::Allow
    }

    #[inline]
    pub fn allow(&self) -> bool {
        match self {
            ValidatorOption::Must => true,
            ValidatorOption::Allow => true,
            ValidatorOption::NotAllow => false,
        }
    }

    #[inline]
    pub fn not_allow(&self) -> bool {
        match self {
            ValidatorOption::Must => false,
            ValidatorOption::Allow => false,
            ValidatorOption::NotAllow => true,
        }
    }

    #[inline]
    pub fn must(&self) -> bool {
        match self {
            ValidatorOption::Must => true,
            ValidatorOption::Allow => false,
            ValidatorOption::NotAllow => false,
        }
    }
}

/// A special kind of options for validators related to the case of characters.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Educe, Ordinalize)]
#[educe(Default)]
pub enum ValidatorCaseOption {
    #[educe(Default)]
    Any,
    Upper,
    Lower,
}

impl ValidatorCaseOption {
    #[doc(hidden)]
    #[inline]
    pub const fn new() -> ValidatorCaseOption {
        ValidatorCaseOption::Any
    }

    #[inline]
    pub fn any(&self) -> bool {
        match self {
            ValidatorCaseOption::Any => true,
            ValidatorCaseOption::Upper => false,
            ValidatorCaseOption::Lower => false,
        }
    }

    #[inline]
    pub fn upper(&self) -> bool {
        match self {
            ValidatorCaseOption::Any => true,
            ValidatorCaseOption::Upper => true,
            ValidatorCaseOption::Lower => false,
        }
    }

    #[inline]
    pub fn lower(&self) -> bool {
        match self {
            ValidatorCaseOption::Any => true,
            ValidatorCaseOption::Upper => false,
            ValidatorCaseOption::Lower => true,
        }
    }
}

/// A special kind of options for validators related to separators.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValidatorSeparatorOption {
    Must(u8),
    Allow(u8),
    NotAllow,
}

impl ValidatorSeparatorOption {
    #[inline]
    pub fn allow(&self) -> Option<u8> {
        match self {
            ValidatorSeparatorOption::Must(e) => Some(*e),
            ValidatorSeparatorOption::Allow(e) => Some(*e),
            ValidatorSeparatorOption::NotAllow => None,
        }
    }

    #[inline]
    pub fn not_allow(&self) -> bool {
        match self {
            ValidatorSeparatorOption::Must(_) => false,
            ValidatorSeparatorOption::Allow(_) => false,
            ValidatorSeparatorOption::NotAllow => true,
        }
    }

    #[inline]
    pub fn must(&self) -> Option<u8> {
        match self {
            ValidatorSeparatorOption::Must(e) => Some(*e),
            ValidatorSeparatorOption::Allow(_) => None,
            ValidatorSeparatorOption::NotAllow => None,
        }
    }
}
