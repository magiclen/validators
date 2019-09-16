/*!
# Validators

This crate provides many validators for validating data from users and modeling them to structs without much extra effort.

All validators are separated into different modules and unified for two main types: **XXX** and **XXXValidator** where **XXX** is a type that you want to validate.

The former is a struct or a enum, and the latter is a struct which can be considered as a generator of the former.

A **XXXValidator** struct usually contains some values of `ValidatorOption` in order to use different rules to check data.

For example, the mod `domain` has `Domain` and `DomainValidator` structs. If we want to create a `Domain` instance, we need to create a `DomainValidator` instance first.

When initialing a `DomainValidator`, we can choose to make this `DomainValidator` **allow** or **not allow** the input to have or **must** have a port number.

```rust
extern crate validators;

use validators::ValidatorOption;
use validators::domain::{Domain, DomainValidator};

let domain = "tool.magiclen.org:8080".to_string();

let dv = DomainValidator {
    port: ValidatorOption::Allow,
    localhost: ValidatorOption::NotAllow,
};

let domain = dv.parse_string(domain).unwrap();

assert_eq!("tool.magiclen.org:8080", domain.get_full_domain());
assert_eq!("tool.magiclen.org", domain.get_full_domain_without_port());
assert_eq!("org", domain.get_top_level_domain().unwrap());
assert_eq!("tool", domain.get_sub_domain().unwrap());
assert_eq!("magiclen", domain.get_domain());
assert_eq!(8080, domain.get_port().unwrap());
```

If you want the **XXX** model to be stricter, you can use its wrapper type which is something like **XXXWithPort** or **XXXWithoutPort**.

For instance, `Domain` has some wrappers, such as **DomainLocalhostableWithPort**, **DomainLocalhostableAllowPort** and **DomainLocalhostableWithoutPort**.

```rust
extern crate validators;

use validators::domain::{DomainLocalhostableWithPort};

let domain = "tool.magiclen.org:8080".to_string();

let domain = DomainLocalhostableWithPort::from_string(domain).unwrap();

assert_eq!("tool.magiclen.org:8080", domain.get_full_domain());
assert_eq!("tool.magiclen.org", domain.get_full_domain_without_port());
assert_eq!("org", domain.get_top_level_domain().unwrap());
assert_eq!("tool", domain.get_sub_domain().unwrap());
assert_eq!("magiclen", domain.get_domain());
assert_eq!(8080, domain.get_port()); // This function does not use `Option` as its return value, because the struct `DomainLocalhostableWithPort` has already made sure the input must have a port number!
```

This crate aims to use the simplest and slackest way (normally only use regular expressions) to validate data, in order to minimize the overhead.

Therefore, it may not be competent in some critical situations. Use it carefully. Check out the documentation to see more useful validators and wrapper types.

## Customization

This crate also provides macros to create customized validated structs for any strings, numbers and Vecs.

For example, to create a struct which only allows **"Hi"** or **"Hello"** restricted by a regular expression,

```rust
#[macro_use] extern crate validators;

validated_customized_regex_string!(Greet, "^(Hi|Hello)$");

let s = Greet::from_str("Hi").unwrap();
```

While a regex needs to be compiled before it operates, if you want to reuse a compiled regex, you can add a **ref** keyword, and pass a static Regex instance to the macro,

```rust
#[macro_use] extern crate validators;
#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

lazy_static! {
    static ref RE_GREET: Regex = {
        Regex::new("^(Hi|Hello)$").unwrap()
    };
}

validated_customized_regex_string!(Greet, ref RE_GREET);

let s = Greet::from_str("Hi").unwrap();
```

You can also make your struct public by adding a **pub** keyword,

```rust
#[macro_use] extern crate validators;

validated_customized_regex_string!(pub Greet, "^(Hi|Hello)$");

let s = Greet::from_str("Hi").unwrap();
```

For numbers limited in a range,

```rust
#[macro_use] extern crate validators;

validated_customized_ranged_number!(Score, u8, 0, 100);

let score = Score::from_str("80").unwrap();
```

For a Vec whose length is limited in a range,

```rust
#[macro_use] extern crate validators;

validated_customized_regex_string!(Name, "^[A-Z][a-zA-Z]*( [A-Z][a-zA-Z]*)*$");
validated_customized_ranged_length_vec!(Names, 1, 5);

let mut names = Vec::new();

names.push(Name::from_str("Ron").unwrap());
names.push(Name::from_str("Magic Len").unwrap());

let names = Names::from_vec(names).unwrap();
```

For a HashSet whose length is limited in a range,

```rust
#[macro_use] extern crate validators;

use std::collections::HashSet;

validated_customized_regex_string!(Name, "^[A-Z][a-zA-Z]*( [A-Z][a-zA-Z]*)*$");
validated_customized_ranged_length_hash_set!(Names, 1, 5);

let mut names = HashSet::new();

names.insert(Name::from_str("Ron").unwrap());
names.insert(Name::from_str("Magic Len").unwrap());

let names = Names::from_hash_set(names).unwrap();
```

All validated wrapper types and validated customized structs implement the `ValidatedWrapper` trait.

Read the documentation to know more helpful customized macros.

## Phone Number Support

This crate supports [phonenumber](https://crates.io/crates/phonenumber) crate. You can create validators for phone numbers by using the `validated_customized_phone_number` macro.

To use it, you have to enable **phone-number** feature for this crate.

```toml
[dependencies.validators]
version = "*"
features = ["phone-number"]
```

For example,

```rust,ignore
#[macro_use] extern crate validators;

use validators::PhoneNumberCountry;

validated_customized_phone_number!(P1, PhoneNumberCountry::TW);
validated_customized_phone_number!(pub P2, PhoneNumberCountry::CN, PhoneNumberCountry::US);

let phone_number = P1::from_str("0912345678").unwrap();
assert_eq!("0912345678", phone_number.get_full_phone_number());
assert!(phone_number.get_countries().contains(&PhoneNumberCountry::TW));

let phone_number = P2::from_str("626-555-1212").unwrap();
assert_eq!("626-555-1212", phone_number.get_full_phone_number());
assert!(phone_number.get_countries().contains(&PhoneNumberCountry::US));
```

## Rocket Support

This crate supports [Rocket](https://rocket.rs/) framework. All validated wrapper types and validated customized structs implement the `FromFormValue` and `FromParam` traits.

To use with Rocket support, you have to enable **rocketly** feature for this crate.

```toml
[dependencies.validators]
version = "*"
features = ["rocketly"]
```

For example,

```rust,ignore
#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate validators;

extern crate rocket;

use rocket::request::Form;

use validators::http_url::HttpUrlUnlocalableWithProtocol;
use validators::email::Email;

validated_customized_ranged_number!(PersonID, u8, 0, 100);
validated_customized_regex_string!(Name, r"^[\S ]{1,80}$");
validated_customized_ranged_number!(PersonAge, u8, 0, 130);

#[derive(Debug, FromForm)]
struct ContactModel {
    name: Name,
    age: Option<PersonAge>,
    email: Email,
    url: Option<HttpUrlUnlocalableWithProtocol>
}

#[post("/contact/<id>", data = "<model>")]
fn contact(id: PersonID, model: Form<ContactModel>) -> &'static str {
    println!("{}", id);
    println!("{:?}", model.get());
    "do something..."
}
```

## Serde Support

Serde is a framework for serializing and deserializing Rust data structures efficiently and generically. And again, this crate supports [Serde](https://crates.io/crates/serde) framework.

All validated wrapper types and validated customized structs implement the `Serialize` and `Deserialize` traits.

To use with Serde support, you have to enable **serdely** feature for this crate.

```toml
[dependencies.validators]
version = "*"
features = ["serdely"]
```

For example,

```rust,ignore
#[macro_use] extern crate validators;
#[macro_use] extern crate serde_json;

validated_customized_regex_string!(Name, "^[A-Z][a-zA-Z]*( [A-Z][a-zA-Z]*)*$");
validated_customized_ranged_length_vec!(Names, 1, 5);

let mut names = Vec::new();

names.push(Name::from_str("Ron").unwrap());
names.push(Name::from_str("Magic Len").unwrap());

let names = Names::from_vec(names).unwrap();

assert_eq!("[\"Ron\",\"Magic Len\"]", json!(names).to_string());
```

Also, the `json`, `json_array` and `json_object` modules are available.
*/

#![cfg_attr(feature = "nightly", feature(ip))]

#[doc(hidden)]
pub extern crate regex;

#[macro_use]
pub extern crate lazy_static;

#[macro_use]
pub extern crate debug_helper;

#[cfg(feature = "rocketly")]
pub extern crate rocket;

#[cfg(feature = "serdely")]
#[macro_use]
pub extern crate serde;

#[cfg(feature = "phone-number")]
pub extern crate phonenumber;

pub extern crate num_traits;

#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;

mod validator_option;

pub use validator_option::*;

use std::fmt::{Debug, Display};

pub trait Validated: Display + PartialEq + Clone + Debug {}

pub trait ValidatedWrapper: Validated {
    type Error: Display + PartialEq + Clone + Debug;

    fn from_string(from_string_input: String) -> Result<Self, Self::Error>;

    fn from_str(from_str_input: &str) -> Result<Self, Self::Error>;
}

pub mod base32;
pub mod base64;
pub mod base64_url;
pub mod boolean;
pub mod domain;
pub mod email;
pub mod host;
pub mod http_ftp_url;
pub mod http_url;
pub mod integer;
pub mod ipv4;
pub mod ipv6;
#[cfg(feature = "serdely")]
pub mod json;
#[cfg(feature = "serdely")]
pub mod json_array;
#[cfg(feature = "serdely")]
pub mod json_object;
pub mod line;
pub mod mac_address;
pub mod number;
pub mod short_crypt_qr_code_alphanumeric;
pub mod short_crypt_url_component;
pub mod text;
pub mod uri;
pub mod uuid;
pub mod version;

mod macro_validated_customized_hash_set;
mod macro_validated_customized_number;
#[cfg(feature = "phone-number")]
mod macro_validated_customized_phone_number;
mod macro_validated_customized_string;
mod macro_validated_customized_vec;

pub use self::macro_validated_customized_hash_set::*;
pub use self::macro_validated_customized_number::*;
#[cfg(feature = "phone-number")]
pub use self::macro_validated_customized_phone_number::*;
pub use self::macro_validated_customized_string::*;
pub use self::macro_validated_customized_vec::*;
