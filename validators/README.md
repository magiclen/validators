Validators
====================

[![CI](https://github.com/magiclen/validators/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/validators/actions/workflows/ci.yml)

This is a library for validating and modeling user input and this crate provides models, function, traits, errors and other dependencies.

## Basic Usage

```toml
[dependencies]
validators = "*"
```

```rust
use validators::prelude::*;

/*
#[derive(Validator)]
#[validator(validator_name)]
DEFINE_YOUR_STRUCT_HERE
*/
```

When you add the `#[validator(validator_name)]` attribute for your structs, one or more traits in the `validators::traits` are implemented. They can be used for validation and deserialization.

The struct used as a validator should have specific components according to its validator and the parameters of that validator. For example, a **base32** validator must be `struct(String)` and a **base32_decoded** validator must be `struct(Vec<u8>)`.

The `#[validator(validator_name)]` attribute cannot be used on fields in any structs or enums. The reason that this crate uses a procedural macro to define a validator (i.e. a struct) instead of providing built-in structs for each configuration is to make the configurable validations have no overhead at runtime and also to increase the compilation speed.

### No Std

Some validators such as **ip**, **ipv4**, and **ipv6** depend on std. If you don't need them, you can disable the default features to compile this crate and your validators without std.

```toml
[dependencies.validators]
version = "*"
default-features = false
features = ["derive", "base32"]
```

### Serde Support

Enable the `serde` feature to let your validators support the serde framework.

```toml
[dependencies.validators]
version = "*"
features = ["serde"]
```

### Rocket Support

Enable the `rocket` feature to let your validators support the Rocket framework.

```toml
[dependencies.validators]
version = "*"
features = ["rocket"]
```

## Validators

### base32

traits: `ValidateString`, `ValidateBytes`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(base32(padding(Must)))]
pub struct Base32WithPadding(String);

assert!(Base32WithPadding::parse_string("GEZDGNBVGY3TQOI=").is_ok());
assert!(Base32WithPadding::parse_string("GEZDGNBVGY3TQOI").is_err());
```

### base32_decoded

traits: `ValidateString`, `ValidateBytes`, `CollectionLength`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(base32_decoded(padding(Must)))]
pub struct Base32WithPaddingDecoded(Vec<u8>);

assert_eq!(b"123456789", Base32WithPaddingDecoded::parse_string("GEZDGNBVGY3TQOI=").unwrap().0.as_slice());
```

### base64

traits: `ValidateString`, `ValidateBytes`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(base64(padding(Must)))]
pub struct Base64WithPadding(String);

assert!(Base64WithPadding::parse_string("MTIzNDU2Nzg5MA==").is_ok());
assert!(Base64WithPadding::parse_string("MTIzNDU2Nzg5MA").is_err());
```

### base64_decoded

traits: `ValidateString`, `ValidateBytes`, `CollectionLength`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(base64_decoded(padding(Must)))]
pub struct Base64WithPaddingDecoded(Vec<u8>);

assert_eq!(b"1234567890", Base64WithPaddingDecoded::parse_string("MTIzNDU2Nzg5MA==").unwrap().0.as_slice());
```

### base64_url

traits: `ValidateString`, `ValidateBytes`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(base64_url(padding(NotAllow)))]
pub struct Base64WithoutPaddingUrl(String);

assert!(Base64WithoutPaddingUrl::parse_string("PmR8hJhjgVNcB61zqhc_B2duZ7ld8Gy1GW2xSBVzeno").is_ok());
```

### base64_url_decoded

traits: `ValidateString`, `ValidateBytes`, `CollectionLength`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(base64_url_decoded(padding(NotAllow)))]
pub struct Base64WithoutPaddingUrlDecoded(Vec<u8>);

assert_eq!([62, 100, 124, 132, 152, 99, 129, 83, 92, 7, 173, 115, 170, 23, 63, 7, 103, 110, 103, 185, 93, 240, 108, 181, 25, 109, 177, 72, 21, 115, 122, 122], Base64WithoutPaddingUrlDecoded::parse_string("PmR8hJhjgVNcB61zqhc_B2duZ7ld8Gy1GW2xSBVzeno").unwrap().0.as_slice());
```

### boolean

traits: `ValidateString`, `ValidateChar`, `ValidateSignedInteger`, `ValidateUnignedInteger`

```rust
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

traits: `ValidateString`

additional methods: `is_fully_qualified`, `get_domain_non_fully_qualified`, `to_uri_authority_string`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), at_least_two_labels(Allow), port(NotAllow)))]
pub struct DomainWithoutPort(pub String);

assert!(DomainWithoutPort::parse_string("example.com").is_ok());
assert_eq!("xn--fiq228c.com", DomainWithoutPort::parse_string("中文.com").unwrap().0);

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), at_least_two_labels(Allow), port(Allow)))]
pub struct DomainAllowPort {
    pub domain: String,
    port: Option<u16>,
}

assert_eq!(Some(8080), DomainAllowPort::parse_string("example.com:8080").unwrap().port);
```

### email

traits: `ValidateString`

additional methods: `to_email_string`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(email(comment(Allow), ip(Allow), local(Allow), at_least_two_labels(Allow), non_ascii(Allow)))]
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
#[validator(email(comment(NotAllow), ip(Allow), local(Allow), at_least_two_labels(Allow), non_ascii(Allow)))]
pub struct EmailNotAllowComment {
    pub local_part: String,
    pub need_quoted: bool,
    pub domain_part: validators::models::Host,
}

assert!(EmailNotAllowComment::parse_string("(john)joke@example.com").is_err());
```

### host

traits: `ValidateString`

additional methods: `to_uri_authority_string`

```rust
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

### http_url

traits: `ValidateString`

```rust
use validators::prelude::*;
use validators_prelude::url;

#[derive(Validator)]
#[validator(http_url(local(Allow)))]
pub struct HttpURL {
    url: url::Url,
    is_https: bool,
}

assert!(HttpURL::parse_string("https://example.org/").is_ok());
assert!(HttpURL::parse_string("http://example.org/").is_ok());
assert!(HttpURL::parse_string("ftp://example.org/").is_err());
```

### http_ftp_url

traits: `ValidateString`

```rust
use validators::prelude::*;
use validators_prelude::url;

#[derive(Validator)]
#[validator(http_ftp_url(local(Allow)))]
pub struct HttpFtpURL {
    url: url::Url,
    protocol: validators::models::Protocol,
}

assert!(HttpFtpURL::parse_string("https://example.org/").is_ok());
assert!(HttpFtpURL::parse_string("http://example.org/").is_ok());
assert!(HttpFtpURL::parse_string("ftp://example.org/").is_ok());
```

### ip

traits: `ValidateString`

additional methods: `to_uri_authority_string`

```rust
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

traits: `ValidateString`

additional methods: `to_uri_authority_string`

```rust
use std::net::Ipv4Addr;

use validators::prelude::*;

#[derive(Validator)]
#[validator(ipv4(local(Allow), port(NotAllow)))]
pub struct IPv4WithoutPort(pub Ipv4Addr);

assert!(IPv4WithoutPort::parse_string("127.0.0.1").is_ok());
```

### ipv6

traits: `ValidateString`

additional methods: `to_uri_authority_string`

```rust
use std::net::Ipv6Addr;

use validators::prelude::*;

#[derive(Validator)]
#[validator(ipv6(local(Allow), port(NotAllow)))]
pub struct IPv6WithoutPort(pub Ipv6Addr);

assert!(IPv6WithoutPort::parse_string("::ffff:c000:0280").is_ok());
assert!(IPv6WithoutPort::parse_string("[::ffff:c000:0280]").is_ok());
```

### json

traits: `ValidateString`, `ValidateSignedInteger`, `ValidateUnignedInteger`, `ValidateNumber`, `ValidateBoolean`

additional methods: `to_minified_json_string`, `to_beautified_json_string`

```rust
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

### length

traits: `ValidateLength`, `CollectionLength`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(length(min = 1, max = 3))]
pub struct NonEmptyNotTooLongVec(pub Vec<u8>);

assert!(NonEmptyNotTooLongVec::parse_collection(vec![]).is_err());
assert!(NonEmptyNotTooLongVec::parse_collection(vec![0]).is_ok());
assert!(NonEmptyNotTooLongVec::parse_collection(vec![0, 1, 2, 3]).is_err());
```

### line

traits: `ValidateString`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(line(char_length(trimmed_min = 1, min = 1, max = 1000)))] // `byte_length` can also be used
pub struct LineNotAllowEmpty(pub String);

assert!(LineNotAllowEmpty::parse_string("123").is_ok());
assert!(LineNotAllowEmpty::parse_string("123\0").is_err());
assert!(LineNotAllowEmpty::parse_string("123\n456").is_err());
assert!(LineNotAllowEmpty::parse_string("   ").is_err());
```

### mac_address

traits: `ValidateString`

additional methods: `to_mac_address_string`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(mac_address(case(Upper), separator(Allow(colon))))]
pub struct MacAddress(pub u64);

assert!(MacAddress::parse_string("080027B246C3").is_ok());
assert!(MacAddress::parse_string("08:00:27:B2:46:C3").is_ok());
```

The default value of the `separator` option is `Allow(colon)`.

### number

traits: `ValidateString`, `ValidateNumber`

```rust
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

### phone

traits: `ValidateString`

```rust
use validators::prelude::*;
use validators_prelude::phonenumber;

use std::collections::HashMap;

#[derive(Validator)]
#[validator(phone)]
pub struct InternationalPhone(pub phonenumber::PhoneNumber);

#[derive(Validator)]
#[validator(phone(TW))]
pub struct TWPhone(pub phonenumber::PhoneNumber);

#[derive(Validator)]
#[validator(phone(TW, US))]
pub struct TWorUSPhone(
    pub HashMap<phonenumber::country::Id, phonenumber::PhoneNumber>,
);

assert!(InternationalPhone::parse_string("+886912345678").is_ok());
assert!(InternationalPhone::parse_string("0912345678").is_err());
assert!(InternationalPhone::parse_string("+14155552671").is_ok());

assert!(TWPhone::parse_string("+886912345678").is_ok());
assert!(TWPhone::parse_string("0912345678").is_ok());
assert!(TWPhone::parse_string("+14155552671").is_err());

assert!(TWorUSPhone::parse_string("+886912345678").is_ok());
assert!(TWorUSPhone::parse_string("0912345678").is_ok());
assert!(TWorUSPhone::parse_string("+14155552671").is_ok());
```

### regex

traits: `ValidateString`

```rust
use validators::prelude::*;
use validators_prelude::regex;

use lazy_static::lazy_static;
use once_cell::sync::Lazy;

lazy_static! {
    static ref RE_NON_ZERO_NUMBERS: regex::Regex = regex::Regex::new("^[1-9]+$").unwrap();
}

static RE_POKER: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new("^([AJQK1-9]|10)$").unwrap()
});

#[derive(Validator)]
#[validator(regex("^[0-9a-fA-F]+$"))]
pub struct Hex(pub String); // this compiles the regex every time

#[derive(Validator)]
#[validator(regex(RE_NON_ZERO_NUMBERS))]
pub struct NonZeroNumbers(pub String);

#[derive(Validator)]
#[validator(regex(RE_POKER))]
pub struct Poker(pub String);

assert!(Hex::parse_string("1Ab").is_ok());
assert!(Hex::parse_string("1AG").is_err());

assert!(NonZeroNumbers::parse_string("12345").is_ok());
assert!(NonZeroNumbers::parse_string("012345").is_err());

assert!(Poker::parse_string("1").is_ok());
assert!(Poker::parse_string("10").is_ok());
assert!(Poker::parse_string("J").is_ok());
assert!(Poker::parse_string("0").is_err());
```

### semver

traits: `ValidateString`

```rust
use validators::prelude::*;
use validators_prelude::semver;

#[derive(Validator)]
#[validator(semver)]
pub struct SemVer(semver::Version);

assert!(SemVer::parse_string("0.0.0").is_ok());
assert!(SemVer::parse_string("0.0.0-beta.1").is_ok());
```

### semver_req

traits: `ValidateString`

```rust
use validators::prelude::*;
use validators_prelude::semver;

#[derive(Validator)]
#[validator(semver_req)]
pub struct SemVerReq(semver::VersionReq);

assert!(SemVerReq::parse_string("0.0.0").is_ok());
assert!(SemVerReq::parse_string(">= 0.4").is_ok());
```

### signed_integer

traits: `ValidateString`, `ValidateSignedInteger`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(signed_integer(range(Inside(min = -1, max = 100))))]
pub struct Score(i8);

assert!(Score::parse_string("0").is_ok());
assert!(Score::parse_string("-2").is_err());
assert!(Score::parse_i8(4).is_ok());

#[derive(Validator)]
#[validator(signed_integer(range(Outside(min = 0, max = 0))))]
pub struct NonZeroShort(i16);

assert!(NonZeroShort::parse_i8(4).is_ok());
assert!(NonZeroShort::parse_i8(-4).is_ok());
assert!(NonZeroShort::parse_i8(0).is_err());
```

### text

traits: `ValidateString`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(text(char_length(trimmed_min = 1, min = 1, max = 1000)))] // `byte_length` can also be used
pub struct TextNotAllowEmpty(pub String);

assert!(TextNotAllowEmpty::parse_string("123").is_ok());
assert!(TextNotAllowEmpty::parse_string("123\0").is_err());
assert!(TextNotAllowEmpty::parse_string("123\n456").is_ok());
assert!(TextNotAllowEmpty::parse_string("   ").is_err());
```

### unsigned_integer

traits: `ValidateString`, `ValidateUnignedInteger`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(unsigned_integer(range(Inside(min = 1, max = 100))))]
pub struct Count(u8);

assert!(Count::parse_string("5").is_ok());
assert!(Count::parse_string("0").is_err());
assert!(Count::parse_u8(4).is_ok());
```

### url

traits: `ValidateString`

```rust
use validators::prelude::*;
use validators_prelude::url;

#[derive(Validator)]
#[validator(url)]
pub struct URL(pub url::Url);

assert!(URL::parse_string("https://example.org/").is_ok());
assert!(URL::parse_string("https:example.org").is_ok());
assert!(URL::parse_string("example:").is_ok());
```

### uuid

traits: `ValidateString`

additional methods: `to_uuid_string`

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(uuid(case(Upper), separator(Allow(hyphen))))]
pub struct UUID(pub u128);

assert!(UUID::parse_string("A866664AF9D34DDE89CB182015FA4F41").is_ok());
assert!(UUID::parse_string("A866664A-F9D3-4DDE-89CB-182015FA4F41").is_ok());
```

The default value of the `separator` option is `Allow(hyphen)`.

## Crates.io

https://crates.io/crates/validators

## Documentation

https://docs.rs/validators

## License

[MIT](LICENSE)