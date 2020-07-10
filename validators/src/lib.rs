/*!
# Validators

This crate provides traits, errors and other dependencies used with the [`validators-derive`](https://crates.io/crates/validators-derive) crate.
*/

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub extern crate validators_options;

#[cfg(feature = "data-encoding")]
pub extern crate data_encoding;

#[cfg(feature = "serde")]
#[allow(unused_imports)]
#[macro_use]
extern crate serde_dep as serde;

#[cfg(feature = "rocket")]
extern crate rocket_dep as rocket;

mod errors;
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

        pub use alloc::string::String;
        pub use alloc::vec::Vec;

        #[cfg(feature = "data-encoding")]
        pub use crate::data_encoding;

        #[cfg(feature = "serde")]
        pub use crate::serde::serde_if_integer128;

        #[cfg(feature = "serde")]
        pub use crate::serde::ser::{Serialize, Serializer};

        #[cfg(feature = "serde")]
        pub use crate::serde::de::{Deserialize, Deserializer, Error as DeError, Visitor};

        #[cfg(feature = "rocket")]
        pub use crate::rocket::request::{FromFormValue, FromParam};

        #[cfg(feature = "rocket")]
        pub use crate::rocket::http::{RawStr, Status};

        pub use crate::errors::*;
        pub use crate::validators_options::*;
    }

    pub use crate::traits::*;
}
