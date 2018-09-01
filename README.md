Validators
====================

[![Build Status](https://travis-ci.org/magiclen/validators.svg?branch=master)](https://travis-ci.org/magiclen/validators)
[![Build status](https://ci.appveyor.com/api/projects/status/ex5r8e8befa9oph7/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/validators/branch/master)

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
Therefore, it may not be competent in some critical situations. Use it carefully.

## Crates.io

https://crates.io/crates/validators

## Documentation

https://docs.rs/validators

## License

[MIT](LICENSE)