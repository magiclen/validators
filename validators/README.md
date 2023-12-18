Validators
====================

[![CI](https://github.com/magiclen/validators/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/validators/actions/workflows/ci.yml)

This library is designed for validating and modeling user input. The crate includes models, functions, traits, errors, and other dependencies.

## Features

By default, every validators this crate supports will be enabled. You can disable all of them by turning off the default features and enable only the validators that you want to use by adding them to the `features` explicitly.

For example,

```toml
[dependencies.validators]
version = "*"
features = ["base64", "url", "uuid"]
default-features = false
```

Certain validators do not require the use of the `std` library. However, if needed, you can explicitly enable the `std` feature.

This library can support the Serde framework and the Rocket framework by enabling the `serde` and `rocket` features, respectively.

## Validators

```rust
use validators::prelude::*;

/*
#[derive(Validator)]
#[validator(validator_name)]
DEFINE_YOUR_STRUCT_HERE
*/
```

When you apply the `#[validator(validator_name)]` attribute to your structs, one or more traits from the `validators::traits` module are automatically implemented. These traits can then be utilized for validation and deserialization purposes.

The struct used as a validator should possess specific components corresponding to its validator type and its associated parameters. For instance, a `base32` validator must be a `struct(String)`, while a `base32_decoded` validator should be a `struct(Vec<u8>)`.

The `#[validator(validator_name)]` attribute cannot be applied to fields within any structs or enums. The decision to use a procedural macro for defining a validator (i.e., a struct) instead of offering built-in structs for each configuration is motivated by the aim to eliminate runtime overhead for configurable validations and to enhance compilation speed.

#### base32

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(base32(padding(Must)))]
pub struct Base32WithPadding(String);

assert!(Base32WithPadding::parse_string("GEZDGNBVGY3TQOI=").is_ok());
assert!(Base32WithPadding::parse_string("GEZDGNBVGY3TQOI").is_err());
assert_eq!("GEZDGNBVGY3TQOI=", Base32WithPadding::parse_string("GEZDGNBVGY3TQOI=").unwrap().0);
```

* Traits: `ValidateString`, `ValidateBytes`
* By default, `padding = Allow`

#### base32_decoded

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(base32_decoded(padding(Must)))]
pub struct Base32WithPaddingDecoded(Vec<u8>);

assert!(Base32WithPaddingDecoded::parse_string("GEZDGNBVGY3TQOI=").is_ok());
assert!(Base32WithPaddingDecoded::parse_string("GEZDGNBVGY3TQOI").is_err());
assert_eq!(b"123456789", Base32WithPaddingDecoded::parse_string("GEZDGNBVGY3TQOI=").unwrap().0.as_slice());
```

* Traits: `ValidateString`, `ValidateBytes`, `CollectionLength`
* By default, `padding = Allow`

#### base64

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(base64(padding(Must)))]
pub struct Base64WithPadding(String);

assert!(Base64WithPadding::parse_string("MTIzNDU2Nzg5MA==").is_ok());
assert!(Base64WithPadding::parse_string("MTIzNDU2Nzg5MA").is_err());
assert_eq!("MTIzNDU2Nzg5MA==", Base64WithPadding::parse_string("MTIzNDU2Nzg5MA==").unwrap().0);
```

* Traits: `ValidateString`, `ValidateBytes`
* By default, `padding = Allow`

#### base64_decoded

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(base64_decoded(padding(Must)))]
pub struct Base64WithPaddingDecoded(Vec<u8>);

assert!(Base64WithPaddingDecoded::parse_string("MTIzNDU2Nzg5MA==").is_ok());
assert!(Base64WithPaddingDecoded::parse_string("MTIzNDU2Nzg5MA").is_err());
assert_eq!(b"1234567890", Base64WithPaddingDecoded::parse_string("MTIzNDU2Nzg5MA==").unwrap().0.as_slice());
```

* Traits: `ValidateString`, `ValidateBytes`, `CollectionLength`
* By default, `padding = Allow`

#### base64_url

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(base64_url(padding(Disallow)))]
pub struct Base64UrlWithoutPadding(String);

assert!(Base64UrlWithoutPadding::parse_string("5LmN5pqW6YKE5a-S5pmC5YCZ").is_ok());
assert!(Base64UrlWithoutPadding::parse_string("5LmN5pqW6YKE5a+S5pmC5YCZ").is_err());
assert_eq!("5LmN5pqW6YKE5a-S5pmC5YCZ", Base64UrlWithoutPadding::parse_string("5LmN5pqW6YKE5a-S5pmC5YCZ").unwrap().0);
```

* Traits: `ValidateString`, `ValidateBytes`
* By default, `padding = Allow`

#### base64_url_decoded

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(base64_url_decoded(padding(Disallow)))]
pub struct Base64UrlWithoutPaddingDecoded(Vec<u8>);

assert!(Base64UrlWithoutPaddingDecoded::parse_string("5LmN5pqW6YKE5a-S5pmC5YCZ").is_ok());
assert!(Base64UrlWithoutPaddingDecoded::parse_string("5LmN5pqW6YKE5a+S5pmC5YCZ").is_err());
assert_eq!("乍暖還寒時候".as_bytes(), Base64UrlWithoutPaddingDecoded::parse_string("5LmN5pqW6YKE5a-S5pmC5YCZ").unwrap().0.as_slice());
```

* Traits: `ValidateString`, `ValidateBytes`, `CollectionLength`
* By default, `padding = Allow`

#### bit

```rust
use validators::prelude::*;
use validators::byte_unit::Bit;

#[derive(Validator)]
#[validator(bit(range(min = 1)))]
pub struct AtLeastOneBit(Bit);

assert!(AtLeastOneBit::parse_string("1kb").is_ok());
assert!(AtLeastOneBit::parse_string("0b").is_err());
assert_eq!(1000u64, AtLeastOneBit::parse_string("1kb").unwrap().0);
```

* Traits: `ValidateString`, `ValidateUnsignedInteger`
* By default, the range is unlimited

#### boolean

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

* Traits: `ValidateString`, `ValidateChar`, `ValidateSignedInteger`, `ValidateUnsignedInteger`, `ValidateBoolean`

#### byte

```rust
use validators::prelude::*;
use validators::byte_unit::Byte;

#[derive(Validator)]
#[validator(byte(range(min = 1), ignore_case = false))]
pub struct AtLeastOneByte(Byte);

assert!(AtLeastOneByte::parse_string("1KB").is_ok());
assert!(AtLeastOneByte::parse_string("0B").is_err());
assert_eq!(1000u64, AtLeastOneByte::parse_string("1KB").unwrap().0);
```

* Traits: `ValidateString`, `ValidateUnsignedInteger`
* By default, the range is unlimited, `ignore_case = true`

#### domain

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), port(Disallow), at_least_two_labels(Allow)))]
pub struct DomainWithoutPort(pub String);

assert!(DomainWithoutPort::parse_string("example.com").is_ok());
assert!(DomainWithoutPort::parse_string("example.com.").is_ok());
assert_eq!("xn--fiq228c.com", DomainWithoutPort::parse_string("中文.com").unwrap().0);

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), port(Allow), at_least_two_labels(Allow)))]
pub struct DomainAllowPort {
    pub domain: String,
    port: Option<u16>,
}

assert_eq!(Some(8080), DomainAllowPort::parse_string("example.com:8080").unwrap().port);
```

* Traits: `ValidateString`, `QualifyDomain`, `ToUriAuthorityString`
* By default, `ipv4 = Allow, local = Allow, port = Allow, at_least_two_labels = Allow`

#### email

```rust
use validators::prelude::*;
use validators::models::Host;

#[derive(Validator)]
#[validator(email(comment(Allow), ip(Allow), local(Allow), at_least_two_labels(Allow), non_ascii(Allow)))]
pub struct EmailAllowComment {
    pub local_part: String,
    pub need_quoted: bool,
    pub domain_part: Host,
    pub comment_before_local_part: Option<String>,
    pub comment_after_local_part: Option<String>,
    pub comment_before_domain_part: Option<String>,
    pub comment_after_domain_part: Option<String>,
}

assert!(EmailAllowComment::parse_string("(john)joke@example.com").is_ok());

#[derive(Validator)]
#[validator(email(comment(Disallow), ip(Allow), local(Allow), at_least_two_labels(Allow), non_ascii(Allow)))]
pub struct EmailWithoutComment {
    pub local_part: String,
    pub need_quoted: bool,
    pub domain_part: Host,
}

assert!(EmailWithoutComment::parse_string("(john)joke@example.com").is_err());
```

* Traits: `ValidateString`, `ToEmailString`
* By default, `comment = Allow, ip = Allow, local = Allow, at_least_two_labels = Allow, non_ascii = Allow`

#### host

```rust
use validators::prelude::*;
use validators::models::Host;

#[derive(Validator)]
#[validator(host(local(Allow), port(Allow), at_least_two_labels(Must)))]
pub struct HostMustAtLeastTwoLabelsAllowPort {
    pub host: Host,
    pub port: Option<u16>,
    pub is_local: bool,
}

assert!(HostMustAtLeastTwoLabelsAllowPort::parse_string("example.com:8000").is_ok());
assert!(HostMustAtLeastTwoLabelsAllowPort::parse_string("example.com.").is_err());
assert!(HostMustAtLeastTwoLabelsAllowPort::parse_string("example").is_err());
```

* Traits: `ValidateString`, `ToUriAuthorityString`
* By default, `local = Allow, port = Allow, at_least_two_labels = Allow`

#### http_url

```rust
use validators::prelude::*;
use validators::url::Url;

#[derive(Validator)]
#[validator(http_url(local(Allow)))]
pub struct HttpURL {
    url: Url,
    is_https: bool,
}

assert!(HttpURL::parse_string("https://example.org/").is_ok());
assert!(HttpURL::parse_string("http://example.org/").is_ok());
assert!(HttpURL::parse_string("ftp://example.org/").is_err());
```

* Traits: `ValidateString`
* By default, `local = Allow`

#### http_ftp_url

```rust
use validators::prelude::*;
use validators::models::Protocol;
use validators::url::Url;

#[derive(Validator)]
#[validator(http_ftp_url(local(Allow)))]
pub struct HttpFtpURL {
    url: Url,
    protocol: Protocol,
}

assert!(HttpFtpURL::parse_string("https://example.org/").is_ok());
assert!(HttpFtpURL::parse_string("http://example.org/").is_ok());
assert!(HttpFtpURL::parse_string("ftp://example.org/").is_ok());
```

* Traits: `ValidateString`
* By default, `local = Allow`

#### ip

```rust
use std::net::IpAddr;

use validators::prelude::*;

#[derive(Validator)]
#[validator(ip(local(Allow), port(Allow)))]
pub struct IpAllowPort {
    pub ip: IpAddr,
    pub port: Option<u16>,
}

assert!(IpAllowPort::parse_string("127.0.0.1").is_ok());
assert!(IpAllowPort::parse_string("[::ffff:c000:0280]:8000").is_ok());
```

* Traits: `ValidateString`, `ToUriAuthorityString`
* By default, `local = Allow, port = Allow`

#### ipv4

```rust
use std::net::Ipv4Addr;

use validators::prelude::*;

#[derive(Validator)]
#[validator(ipv4(local(Allow), port(Allow)))]
pub struct Ipv4AllowPort {
    pub ipv4: Ipv4Addr,
    pub port: Option<u16>,
}

assert!(Ipv4AllowPort::parse_string("127.0.0.1").is_ok());
```

* Traits: `ValidateString`, `ToUriAuthorityString`
* By default, `local = Allow, port = Allow`

#### ipv6

```rust
use std::net::Ipv6Addr;

use validators::prelude::*;

#[derive(Validator)]
#[validator(ipv6(local(Allow), port(Allow)))]
pub struct Ipv6AllowPort {
    pub ipv6: Ipv6Addr,
    pub port: Option<u16>,
}

assert!(Ipv6AllowPort::parse_string("::ffff:c000:0280").is_ok());
assert!(Ipv6AllowPort::parse_string("[::ffff:c000:0280]").is_ok());
```

* Traits: `ValidateString`, `ToUriAuthorityString`
* By default, `local = Allow, port = Allow`

#### json

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

* Traits: `ValidateString`, `ValidateSignedInteger`, `ValidateUnsignedInteger`, `ValidateNumber`, `ValidateBoolean`, `ValidateJsonValue`

#### length

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(length(min = 1, max = 3))]
pub struct NonEmptyNotTooLongVec(pub Vec<u8>);

assert!(NonEmptyNotTooLongVec::parse_collection(vec![]).is_err());
assert!(NonEmptyNotTooLongVec::parse_collection(vec![0]).is_ok());
assert!(NonEmptyNotTooLongVec::parse_collection(vec![0, 1, 2, 3]).is_err());
```

* Traits: `ValidateLength`, `CollectionLength`
* By default, the length is unlimited

#### line

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

* Traits: `ValidateLength`
* By default, the length is unlimited

#### mac_address

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(mac_address(case(Upper), separator(Allow(b':'))))]
pub struct MacAddress(pub u64);

assert!(MacAddress::parse_string("080027B246C3").is_ok());
assert!(MacAddress::parse_string("08:00:27:B2:46:C3").is_ok());
```

* Traits: `ValidateString`, `ToMacAddressString`
* By default, `case = Any, separator(Allow(b':')`

#### number

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(number(nan(Disallow), range(Unlimited)))]
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

* Traits: `ValidateString`, `ValidateNumber`
* By default, `nan = Allow, range(Unlimited)`

#### phone

```rust
use std::collections::HashMap;

use validators::prelude::*;
use validators_prelude::phonenumber::PhoneNumber;

#[derive(Validator)]
#[validator(phone)]
pub struct InternationalPhone(pub phonenumber::PhoneNumber);

#[derive(Validator)]
#[validator(phone(countries(TW)))]
pub struct TWPhone(pub phonenumber::PhoneNumber);

#[derive(Validator)]
#[validator(phone(countries(TW, US)))]
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

* Traits: `ValidateString`
* By default, countries is unlimited

#### regex

```rust
use validators::prelude::*;
use validators_prelude::regex::Regex;

use once_cell::sync::Lazy;

static RE_NON_ZERO_NUMBERS: Lazy<Regex> = Lazy::new(|| {
    Regex::new("^[1-9]+$").unwrap()
});

static RE_POKER: Lazy<Regex> = Lazy::new(|| {
    Regex::new("^([AJQK1-9]|10)$").unwrap()
});

#[derive(Validator)]
#[validator(regex(regex("^[0-9a-fA-F]+$")))]
pub struct Hex(pub String); // this doesn't cache the `Regex` instance

#[derive(Validator)]
#[validator(regex(regex(RE_NON_ZERO_NUMBERS)))]
pub struct NonZeroNumbers(pub String);

#[derive(Validator)]
#[validator(regex(regex(RE_POKER)))]
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

* Traits: `ValidateString`
* The `regex` parameter must be set to a string literal or an expression

#### semver

```rust
use validators::prelude::*;
use validators_prelude::semver::Version;

#[derive(Validator)]
#[validator(semver)]
pub struct SemVer(Version);

assert!(SemVer::parse_string("0.0.0").is_ok());
assert!(SemVer::parse_string("0.0.0-beta.1").is_ok());
```

* Traits: `ValidateString`

#### semver_req

```rust
use validators::prelude::*;
use validators_prelude::semver::VersionReq;

#[derive(Validator)]
#[validator(semver_req)]
pub struct SemVerReq(VersionReq);

assert!(SemVerReq::parse_string("0.0.0").is_ok());
assert!(SemVerReq::parse_string(">= 0.4").is_ok());
```

* Traits: `ValidateString`

#### signed_integer

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

* Traits: `ValidateString`, `ValidateSignedInteger`
* By default, `range(Unlimited)`

#### text

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

* Traits: `ValidateLength`
* By default, the length is unlimited

#### unsigned_integer

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(unsigned_integer(range(Inside(min = 1, max = 100))))]
pub struct Count(u8);

assert!(Count::parse_string("5").is_ok());
assert!(Count::parse_string("0").is_err());
assert!(Count::parse_u8(4).is_ok());
```

* Traits: `ValidateString`, `ValidateUnsignedInteger`
* By default, `range(Unlimited)`

#### url

```rust
use validators::prelude::*;
use validators_prelude::url::Url;

#[derive(Validator)]
#[validator(url)]
pub struct URL(pub Url);

assert!(URL::parse_string("https://example.org/").is_ok());
assert!(URL::parse_string("https:example.org").is_ok());
assert!(URL::parse_string("example:").is_ok());
```

* Traits: `ValidateString`

#### uuid

```rust
use validators::prelude::*;

#[derive(Validator)]
#[validator(uuid(case(Upper), separator(Allow(b'-'))))]
pub struct UUID(pub u128);

assert!(UUID::parse_string("A866664AF9D34DDE89CB182015FA4F41").is_ok());
assert!(UUID::parse_string("A866664A-F9D3-4DDE-89CB-182015FA4F41").is_ok());
```

* Traits: `ValidateString`, `ToUuidString`
* By default, `case = Any, separator(Allow(b'-')`

## `validators::Result`

When incorporating your validator type into another type, you might desire to obtain the original error instance provided by the validator.

For example, when using the `number` validator with the `#[derive(FromForm)]` attribute from the Rocket framework,

```rust
use rocket::{FromForm, get};
use validators::prelude::*;

#[derive(Debug, Validator)]
#[validator(number(range(Outside(max = 0))))]
pub struct NonZeroNumber(f64);

#[derive(Debug, FromForm)]
struct User {
    id:   i32,
    number: NonZeroNumber,
}

#[get("/?<user..>")]
fn index(user: User) -> String {
    format!("{:?}", user)
}
```

You may want the number field of the `User` instance to be allowed to fail. You can modify the code as follows:

```rust
use rocket::{FromForm, get};

use validators::prelude::*;

#[derive(Debug, Validator)]
#[validator(number(range(Outside(max = 0))))]
pub struct NonZeroNumber(f64);

#[derive(Debug, FromForm)]
struct User {
    id:   i32,
    // number: Result<NonZeroNumber, validators::errors::NumberError>, // compile error
    number: validators::Result<NonZeroNumber, validators::errors::NumberError>,
}

#[get("/?<user..>")]
fn index(user: User) -> String {
    format!("{:?}", user)
}
```

## Crates.io

https://crates.io/crates/validators

## Documentation

https://docs.rs/validators

## License

[MIT](LICENSE)