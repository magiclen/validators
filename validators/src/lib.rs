/*!
# Validators

This crate provides models, function, traits, errors and other dependencies used with the [`validators-derive`](https://crates.io/crates/validators-derive) crate.
*/

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub extern crate validators_options;

#[cfg(feature = "data-encoding")]
pub extern crate data_encoding;

#[cfg(feature = "idna")]
pub extern crate idna;

#[cfg(feature = "serde_json")]
pub extern crate serde_json;

#[cfg(feature = "regex-dep")]
pub extern crate regex_dep as regex;

#[cfg(feature = "semver-dep")]
pub extern crate semver_dep as semver;

#[cfg(feature = "serde")]
#[allow(unused_imports)]
#[macro_use]
extern crate serde_dep as serde;

#[cfg(feature = "rocket")]
extern crate rocket_dep as rocket;

mod errors;
pub mod functions;
pub mod models;
pub mod traits;

pub use errors::*;

/**
A convenience module appropriate for glob imports

```rust
use validators::prelude::*;
```
*/
pub mod prelude {
    pub mod validators_prelude {
        pub use core::fmt::{self, Formatter};
        pub use core::str::from_utf8_unchecked;

        pub use alloc::borrow::Cow;
        pub use alloc::string::String;
        pub use alloc::vec::Vec;

        #[cfg(feature = "std")]
        pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

        #[cfg(feature = "data-encoding")]
        pub use crate::data_encoding;

        #[cfg(feature = "idna")]
        pub use crate::idna;

        #[cfg(feature = "serde_json")]
        pub use crate::serde_json;

        #[cfg(feature = "regex-dep")]
        pub use crate::regex;

        #[cfg(feature = "semver-dep")]
        pub use crate::semver;

        #[cfg(feature = "serde")]
        pub use crate::serde::serde_if_integer128;

        #[cfg(feature = "serde")]
        pub use crate::serde::ser::{Error as SeError, Serialize, Serializer};

        #[cfg(feature = "serde")]
        pub use crate::serde::de::{Deserialize, Deserializer, Error as DeError, Visitor};

        #[cfg(feature = "rocket")]
        pub use crate::rocket::request::{FromFormValue, FromParam};

        #[cfg(feature = "rocket")]
        pub use crate::rocket::http::{RawStr, Status};

        pub use crate::errors::*;
        pub use crate::functions::*;
        pub use crate::models::*;
        pub use crate::validators_options::*;

        pub use crate::alloc::format;
    }

    pub use crate::traits::*;
}
