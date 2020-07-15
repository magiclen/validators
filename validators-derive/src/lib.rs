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

The struct used as a validator should have specific components according to its validator and the parameters of that validator. For example, a **base32** validator must be `struct(String)` and a **base32_decoded** validator must be `struct(Vec<u8>)`.

The `#[validator(validator_name)]` attribute cannot be used on fields in any structs or enums. The reason that this crate uses a procedural macro to define a validator (i.e. a struct) instead of providing built-in structs for each configuration is to make the configurable validations have no overhead at runtime and also to insrease the compilation speed.

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
pub struct Base32WithPadding(String);

assert!(Base32WithPadding::parse_string("GEZDGNBVGY3TQOI=").is_ok());
assert!(Base32WithPadding::parse_string("GEZDGNBVGY3TQOI").is_err());
```

### base32_decoded

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(base32_decoded(padding(Must)))]
pub struct Base32WithPaddingDecoded(Vec<u8>);

assert_eq!(b"123456789", Base32WithPaddingDecoded::parse_string("GEZDGNBVGY3TQOI=").unwrap().0.as_slice());
```

### base64

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(base64(padding(Must)))]
pub struct Base64WithPadding(String);

assert!(Base64WithPadding::parse_string("MTIzNDU2Nzg5MA==").is_ok());
assert!(Base64WithPadding::parse_string("MTIzNDU2Nzg5MA").is_err());
```

### base64_decoded

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(base64_decoded(padding(Must)))]
pub struct Base64WithPaddingDecoded(Vec<u8>);

assert_eq!(b"1234567890", Base64WithPaddingDecoded::parse_string("MTIzNDU2Nzg5MA==").unwrap().0.as_slice());
```

### base64_url

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(base64_url(padding(NotAllow)))]
pub struct Base64WithoutPaddingUrl(String);

assert!(Base64WithoutPaddingUrl::parse_string("PmR8hJhjgVNcB61zqhc_B2duZ7ld8Gy1GW2xSBVzeno").is_ok());
```

### base64_url_decoded

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(base64_url_decoded(padding(NotAllow)))]
pub struct Base64WithoutPaddingUrlDecoded(Vec<u8>);

assert_eq!([62, 100, 124, 132, 152, 99, 129, 83, 92, 7, 173, 115, 170, 23, 63, 7, 103, 110, 103, 185, 93, 240, 108, 181, 25, 109, 177, 72, 21, 115, 122, 122], Base64WithoutPaddingUrlDecoded::parse_string("PmR8hJhjgVNcB61zqhc_B2duZ7ld8Gy1GW2xSBVzeno").unwrap().0.as_slice());
```

### boolean

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(boolean)]
pub struct Boolean(bool);

assert_eq!(true, Boolean::parse_str("true").unwrap().0);
assert_eq!(false, Boolean::parse_str("f").unwrap().0);
assert_eq!(true, Boolean::parse_str("y").unwrap().0);
assert_eq!(false, Boolean::parse_str("no").unwrap().0);
assert_eq!(true, Boolean::parse_str("on").unwrap().0);
assert_eq!(false, Boolean::parse_str("off").unwrap().0);
assert_eq!(true, Boolean::parse_str("1").unwrap().0);

assert_eq!(true, Boolean::parse_char('t').unwrap().0);
assert_eq!(false, Boolean::parse_char('0').unwrap().0);

assert_eq!(true, Boolean::parse_isize(1).unwrap().0);
```

### domain

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), at_least_two_labels(Allow), port(NotAllow)))]
pub struct DomainWithoutPort {
    pub domain: String,
    is_ipv4: bool,
}

assert!(DomainWithoutPort::parse_string("example.com").is_ok());
assert_eq!("xn--fiq228c.com", DomainWithoutPort::parse_string("中文.com").unwrap().domain);

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), at_least_two_labels(Allow), port(Allow)))]
pub struct DomainAllowPort {
    pub domain: String,
    is_ipv4: bool,
    port: Option<u16>,
}

assert_eq!(Some(8080), DomainAllowPort::parse_string("example.com:8080").unwrap().port);
```

### email

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(email(comment(Allow), ip(Allow), local(Allow), at_least_two_labels(Allow)))]
pub struct EmailAllowComment {
    pub local_part: String,
    pub need_quoted: bool,
    pub domain_part: validators::models::Host,
    pub comment_before_local_part: Option<String>,
    pub comment_after_local_part: Option<String>,
    pub comment_before_domain_part: Option<String>,
    pub comment_after_domain_part: Option<String>,
}

assert!(EmailAllowComment::parse_string("(john)joke@example.com").is_ok());

#[derive(Validator)]
#[validator(email(comment(NotAllow), ip(Allow), local(Allow), at_least_two_labels(Allow)))]
pub struct EmailNotAllowComment {
    pub local_part: String,
    pub need_quoted: bool,
    pub domain_part: validators::models::Host,
}

assert!(EmailNotAllowComment::parse_string("(john)joke@example.com").is_err());
```

### host

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(host(local(Allow), at_least_two_labels(Must), port(Allow)))]
pub struct HostMustAtLeastTwoLabelsAllowPort {
    pub host: validators::models::Host,
    pub port: Option<u16>,
    pub is_local: bool,
}

assert!(HostMustAtLeastTwoLabelsAllowPort::parse_string("example.com:8000").is_ok());
assert!(HostMustAtLeastTwoLabelsAllowPort::parse_string("example").is_err());
```

### ip

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use std::net::IpAddr;

use validators::prelude::*;

#[derive(Validator)]
#[validator(ip(local(Allow), port(Allow)))]
pub struct IPAllowPort {
    pub ip: IpAddr,
    pub port: Option<u16>,
}

assert!(IPAllowPort::parse_string("127.0.0.1").is_ok());
assert!(IPAllowPort::parse_string("[::ffff:c000:0280]:8000").is_ok());
```

### ipv4

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use std::net::Ipv4Addr;

use validators::prelude::*;

#[derive(Validator)]
#[validator(ipv4(local(Allow), port(NotAllow)))]
pub struct IPv4WithoutPort(pub Ipv4Addr);

assert!(IPv4WithoutPort::parse_string("127.0.0.1").is_ok());
```

### ipv6

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use std::net::Ipv6Addr;

use validators::prelude::*;

#[derive(Validator)]
#[validator(ipv6(local(Allow), port(NotAllow)))]
pub struct IPv6WithoutPort(pub Ipv6Addr);

assert!(IPv6WithoutPort::parse_string("::ffff:c000:0280").is_ok());
assert!(IPv6WithoutPort::parse_string("[::ffff:c000:0280]").is_ok());
```

### json

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(json)]
pub struct JSONString(pub String);

#[derive(Validator)]
#[validator(json)]
pub struct JSONNumber(pub f64);

#[derive(Validator)]
#[validator(json)]
pub struct JSONBoolean(pub bool);

assert!(JSONString::parse_string("123").is_err());
assert!(JSONString::parse_string("\"123\"").is_ok());
assert!(JSONNumber::parse_u64(123).is_ok());
assert!(JSONBoolean::parse_bool(false).is_ok());
```

### line

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(line(empty(NotAllow)))]
pub struct LineNotAllowEmpty(pub String);

assert!(LineNotAllowEmpty::parse_string("123").is_ok());
assert!(LineNotAllowEmpty::parse_string("123\0").is_err());
assert!(LineNotAllowEmpty::parse_string("123\n456").is_err());
assert!(LineNotAllowEmpty::parse_string("   ").is_err());
```

### mac_address

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(mac_address(case(Upper), separator(Allow(colon))))]
pub struct MacAddress(pub u64);

assert!(MacAddress::parse_string("080027B246C3").is_ok());
assert!(MacAddress::parse_string("08:00:27:B2:46:C3").is_ok());
```

The default value of the `separator` option is `Allow(colon)`.

### number

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(number(nan(NotAllow), range(NotLimited)))]
pub struct Double(pub f64);

assert!(Double::parse_string("123.456").is_ok());
assert!(Double::parse_string("NaN").is_err());
assert!(Double::parse_f32(123.4).is_ok());

#[derive(Validator)]
#[validator(number(nan(Allow), range(Inside(min = 0, max = 1.0))))]
pub struct SinglePercentage(pub f32);

assert!(SinglePercentage::parse_string("0").is_ok());
assert!(SinglePercentage::parse_string("1").is_ok());
assert!(SinglePercentage::parse_string("1.1").is_err());
assert!(SinglePercentage::parse_string("NaN").is_ok());
```

### semver

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;
use validators_prelude::semver;

#[derive(Validator)]
#[validator(semver)]
pub struct SemVer(semver::Version);

assert!(SemVer::parse_string("0.0.0").is_ok());
assert!(SemVer::parse_string("0.0.0-beta.1").is_ok());
```

### semver_req

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;
use validators_prelude::semver;

#[derive(Validator)]
#[validator(semver_req)]
pub struct SemVerReq(semver::VersionReq);

assert!(SemVerReq::parse_string("0.0.0").is_ok());
assert!(SemVerReq::parse_string(">= 0.4").is_ok());
```

### signed_integer

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(signed_integer(range(Inside(min = -1, max = 100))))]
pub struct Score(i8);

assert!(Score::parse_string("0").is_ok());
assert!(Score::parse_string("-2").is_err());
assert!(Score::parse_i8(4).is_ok());
```

### text

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(text(empty(NotAllow)))]
pub struct TextNotAllowEmpty(pub String);

assert!(TextNotAllowEmpty::parse_string("123").is_ok());
assert!(TextNotAllowEmpty::parse_string("123\0").is_err());
assert!(TextNotAllowEmpty::parse_string("123\n456").is_ok());
assert!(TextNotAllowEmpty::parse_string("   ").is_err());
```

### unsigned_integer

```rust
#[macro_use] extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[derive(Validator)]
#[validator(unsigned_integer(range(Inside(min = 1, max = 100))))]
pub struct Count(u8);

assert!(Count::parse_string("5").is_ok());
assert!(Count::parse_string("0").is_err());
assert!(Count::parse_u8(4).is_ok());

#[derive(Validator)]
#[validator(unsigned_integer(range(Outside(min = 0, max = 0))))]
pub struct NonZeroUnsignedShort(u16);

assert!(NonZeroUnsignedShort::parse_u8(4).is_ok());
assert!(NonZeroUnsignedShort::parse_u8(0).is_err());
```

*/

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
extern crate enum_ordinalize;

extern crate validators_options;

extern crate proc_macro;
extern crate syn;

#[allow(unused_imports)]
#[macro_use]
extern crate educe;

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
use type_enum::*;
use validator_handlers::*;

#[allow(unused_imports)]
use syn_validator_options::*;

#[allow(unused_imports)]
use validators_options::*;

fn derive_input_handler(ast: DeriveInput) -> TokenStream {
    for attr in ast.attrs.iter() {
        if let Some(attr_meta_name) = attr.path.get_ident() {
            if attr_meta_name == "validator" {
                let attr_meta = attr.parse_meta().unwrap();

                match attr_meta {
                    Meta::List(list) => {
                        if list.nested.len() == 1 {
                            let p = list.nested.into_iter().next().unwrap();

                            match p {
                                NestedMeta::Meta(meta) => {
                                    let meta_name = meta.path().into_token_stream().to_string();

                                    match Validator::from_str(meta_name) {
                                        #[cfg(feature = "base32")]
                                        Validator::base32 => {
                                            return base32::base32_handler(ast, meta)
                                        }
                                        #[cfg(feature = "base32_decoded")]
                                        Validator::base32_decoded => {
                                            return base32_decoded::base32_decoded_handler(
                                                ast, meta,
                                            )
                                        }
                                        #[cfg(feature = "base64")]
                                        Validator::base64 => {
                                            return base64::base64_handler(ast, meta)
                                        }
                                        #[cfg(feature = "base64_decoded")]
                                        Validator::base64_decoded => {
                                            return base64_decoded::base64_decoded_handler(
                                                ast, meta,
                                            )
                                        }
                                        #[cfg(feature = "base64_url")]
                                        Validator::base64_url => {
                                            return base64_url::base64_url_handler(ast, meta)
                                        }
                                        #[cfg(feature = "base64_url_decoded")]
                                        Validator::base64_url_decoded => {
                                            return base64_url_decoded::base64_url_decoded_handler(
                                                ast, meta,
                                            )
                                        }
                                        #[cfg(feature = "boolean")]
                                        Validator::boolean => {
                                            return boolean::boolean_handler(ast, meta)
                                        }
                                        #[cfg(feature = "domain")]
                                        Validator::domain => {
                                            return domain::domain_handler(ast, meta)
                                        }
                                        #[cfg(feature = "email")]
                                        Validator::email => return email::email_handler(ast, meta),
                                        #[cfg(feature = "host")]
                                        Validator::host => return host::host_handler(ast, meta),
                                        #[cfg(feature = "ip")]
                                        Validator::ip => return ip::ip_handler(ast, meta),
                                        #[cfg(feature = "ipv4")]
                                        Validator::ipv4 => return ipv4::ipv4_handler(ast, meta),
                                        #[cfg(feature = "ipv6")]
                                        Validator::ipv6 => return ipv6::ipv6_handler(ast, meta),
                                        #[cfg(feature = "json")]
                                        Validator::json => return json::json_handler(ast, meta),
                                        #[cfg(feature = "line")]
                                        Validator::line => return line::line_handler(ast, meta),
                                        #[cfg(feature = "mac_address")]
                                        Validator::mac_address => {
                                            return mac_address::mac_address_handler(ast, meta)
                                        }
                                        #[cfg(feature = "number")]
                                        Validator::number => {
                                            return number::number_handler(ast, meta)
                                        }
                                        #[cfg(feature = "semver")]
                                        Validator::semver => {
                                            return semver::semver_handler(ast, meta)
                                        }
                                        #[cfg(feature = "semver_req")]
                                        Validator::semver_req => {
                                            return semver_req::semver_req_handler(ast, meta)
                                        }
                                        #[cfg(feature = "signed_integer")]
                                        Validator::signed_integer => {
                                            return signed_integer::signed_integer_handler(
                                                ast, meta,
                                            )
                                        }
                                        #[cfg(feature = "text")]
                                        Validator::text => return text::text_handler(ast, meta),
                                        #[cfg(feature = "unsigned_integer")]
                                        Validator::unsigned_integer => {
                                            return unsigned_integer::unsigned_integer_handler(
                                                ast, meta,
                                            )
                                        }
                                    }
                                }
                                NestedMeta::Lit(_) => panic::validator_format_incorrect(),
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
