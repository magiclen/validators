/*!
# Validators Derive

This crate provides a procedural macro to define validators with optional parameters.

## Basic Usage

```toml
[dependencies]
validators-derive = "*"
validators = "*"
```

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

/*
#[derive(Validator)]
#[validator(validator_name)]
DEFINE_YOUR_STRUCT_HERE
*/
```

When you add the `#[validator(validator_name)]` attribute for your structs, one or more traits in the `validators::traits` are implemented. They can be used for validation and deserialization.

The struct used as a validator should have specific components according to its validator. For example, a **base32** validator must be `struct(String)` and a **base32_decoded** validator must be `struct(Vec<u8>)`.

The `#[validator(validator_name)]` attribute cannot be used for fields in any structs or enums. The reason that this crate uses a procedural macro to define a validator instead of using a struct with configuration is to make the configuration check have no overhead at runtime.

### No Std

Some validators such as **ip**, **ipv4**, and **ipv6** depend on std. If you don't need them, you can disable the default features to compile this crate and your validators without std.

```toml
[dependencies]
validators = "*"

[dependencies.validators-derive]
version = "*"
default-features = false
features = ["base32"]
```

### Serde Support

Enable the `serde` feature to let your validators support the serde framework.

```toml
[dependencies]
validators = "*"

[dependencies.validators-derive]
version = "*"
features = ["serde"]
```

### Rocket Support

Enable the `rocket` feature to let your validators support the Rocket framework.

```toml
[dependencies]
validators = "*"

[dependencies.validators-derive]
version = "*"
features = ["rocket"]
```

## Validators

### base32

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(base32(padding(Must)))]
pub struct MyBase32(String);

assert!(MyBase32::parse_string("GEZDGNBVGY3TQOI=").is_ok());
assert!(MyBase32::parse_string("GEZDGNBVGY3TQOI").is_err());
```

### base32_decoded

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(base32_decoded(padding(Must)))]
pub struct MyBase32Decoded(Vec<u8>);

assert_eq!(b"123456789", MyBase32Decoded::parse_string("GEZDGNBVGY3TQOI=").unwrap().0.as_slice());
```

### base64

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(base64(padding(Must)))]
pub struct MyBase64(String);

assert!(MyBase64::parse_string("MTIzNDU2Nzg5MA==").is_ok());
assert!(MyBase64::parse_string("MTIzNDU2Nzg5MA").is_err());
```

### base64_decoded

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(base64_decoded(padding(Must)))]
pub struct MyBase64Decoded(Vec<u8>);

assert_eq!(b"1234567890", MyBase64Decoded::parse_string("MTIzNDU2Nzg5MA==").unwrap().0.as_slice());
```

### base64_url

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(base64_url(padding(NotAllow)))]
pub struct MyBase64Url(String);

assert!(MyBase64Url::parse_string("PmR8hJhjgVNcB61zqhc_B2duZ7ld8Gy1GW2xSBVzeno").is_ok());
```

### base64_url_decoded

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(base64_url_decoded(padding(NotAllow)))]
pub struct MyBase64UrlDecoded(Vec<u8>);

assert_eq!([62, 100, 124, 132, 152, 99, 129, 83, 92, 7, 173, 115, 170, 23, 63, 7, 103, 110, 103, 185, 93, 240, 108, 181, 25, 109, 177, 72, 21, 115, 122, 122], MyBase64UrlDecoded::parse_string("PmR8hJhjgVNcB61zqhc_B2duZ7ld8Gy1GW2xSBVzeno").unwrap().0.as_slice());
```

### boolean

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(boolean)]
pub struct MyBoolean(bool);

assert_eq!(true, MyBoolean::parse_str("true").unwrap().0);
assert_eq!(false, MyBoolean::parse_str("f").unwrap().0);
assert_eq!(true, MyBoolean::parse_str("y").unwrap().0);
assert_eq!(false, MyBoolean::parse_str("no").unwrap().0);
assert_eq!(true, MyBoolean::parse_str("on").unwrap().0);
assert_eq!(false, MyBoolean::parse_str("off").unwrap().0);
assert_eq!(true, MyBoolean::parse_str("1").unwrap().0);

assert_eq!(true, MyBoolean::parse_char('t').unwrap().0);
assert_eq!(false, MyBoolean::parse_char('0').unwrap().0);

assert_eq!(true, MyBoolean::parse_isize(1).unwrap().0);
```

*/

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

extern crate validators_options;

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

mod panic;
mod support_validators;
mod syn_validator_options;
mod type_enum;
mod validator_handlers;

use alloc::string::ToString;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, Meta, NestedMeta};

use support_validators::Validator;
use syn_validator_options::*;
use type_enum::*;
use validator_handlers::*;
use validators_options::*;

fn derive_input_handler(ast: DeriveInput) -> TokenStream {
    for attr in ast.attrs.iter() {
        if let Some(attr_meta_name) = attr.path.get_ident() {
            if attr_meta_name == "validator" {
                let attr_meta = attr.parse_meta().unwrap();

                match attr_meta {
                    Meta::List(list) => {
                        if list.nested.len() == 1 {
                            for p in list.nested {
                                match p {
                                    NestedMeta::Meta(meta) => {
                                        let meta_name = meta.path().into_token_stream().to_string();

                                        match Validator::from_str(meta_name) {
                                            Validator::base32 => return base32_handler(ast, meta),
                                            Validator::base32_decoded => {
                                                return base32_decoded_handler(ast, meta)
                                            }
                                            Validator::base64 => return base64_handler(ast, meta),
                                            Validator::base64_decoded => {
                                                return base64_decoded_handler(ast, meta)
                                            }
                                            Validator::base64_url => {
                                                return base64_url_handler(ast, meta)
                                            }
                                            Validator::base64_url_decoded => {
                                                return base64_url_decoded_handler(ast, meta)
                                            }
                                            Validator::boolean => {
                                                return boolean_handler(ast, meta)
                                            }
                                        }
                                    }
                                    NestedMeta::Lit(_) => panic::validator_format_incorrect(),
                                }
                            }
                        } else {
                            panic::validator_format_incorrect()
                        }
                    }
                    _ => panic::validator_format_incorrect(),
                }
            }
        }
    }

    panic::derive_attribute_not_set_up_yet("Validator")
}

#[proc_macro_derive(Validator, attributes(validator))]
pub fn validator_derive(input: TokenStream) -> TokenStream {
    derive_input_handler(syn::parse(input).unwrap())
}
