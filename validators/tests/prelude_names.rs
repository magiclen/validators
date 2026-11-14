#![cfg(feature = "derive")]
#![allow(dead_code)]

// The generated code uses full paths, so names in the user's scope cannot shadow `Ok`, `Err`, `Some`, `None`, or `Result`.

use validators::prelude::*;

enum Status {
    Ok,
    Err,
    Some,
    None,
}

// These variants shadow the prelude names in this file, like `use anyhow::Ok` does.
#[allow(unused_imports)]
use Status::*;

type Result<T> = core::result::Result<T, ()>;

#[cfg(feature = "base32")]
#[derive(Validator)]
#[validator(base32)]
struct Base32(String);

#[cfg(feature = "base32_decoded")]
#[derive(Validator)]
#[validator(base32_decoded)]
struct Base32Decoded(Vec<u8>);

#[cfg(feature = "base64")]
#[derive(Validator)]
#[validator(base64(padding(Must)))]
struct Base64(String);

#[cfg(feature = "base64_decoded")]
#[derive(Validator)]
#[validator(base64_decoded(padding(Disallow)))]
struct Base64Decoded(Vec<u8>);

#[cfg(feature = "base64_url")]
#[derive(Validator)]
#[validator(base64_url)]
struct Base64Url(String);

#[cfg(feature = "base64_url_decoded")]
#[derive(Validator)]
#[validator(base64_url_decoded)]
struct Base64UrlDecoded(Vec<u8>);

#[cfg(feature = "bit")]
#[derive(Validator)]
#[validator(bit(range(min = 1)))]
struct Bit(validators::byte_unit::Bit);

#[cfg(feature = "boolean")]
#[derive(Validator)]
#[validator(boolean)]
struct Boolean(bool);

#[cfg(feature = "byte")]
#[derive(Validator)]
#[validator(byte(range(max = 1024)))]
struct Byte(validators::byte_unit::Byte);

#[cfg(feature = "domain")]
#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), port(Must), at_least_two_labels(Must)))]
struct Domain {
    domain:   String,
    is_ipv4:  bool,
    is_local: bool,
    port:     u16,
}

#[cfg(feature = "email")]
#[derive(Validator)]
#[validator(email)]
struct Email {
    local_part:                 String,
    need_quoted:                bool,
    domain_part:                validators::models::Host,
    comment_before_local_part:  Option<String>,
    comment_after_local_part:   Option<String>,
    comment_before_domain_part: Option<String>,
    comment_after_domain_part:  Option<String>,
}

#[cfg(feature = "hex_color")]
#[derive(Validator)]
#[validator(hex_color)]
struct HexColor(u32);

#[cfg(feature = "hex_decoded")]
#[derive(Validator)]
#[validator(hex_decoded)]
struct HexDecoded(Vec<u8>);

#[cfg(feature = "host")]
#[derive(Validator)]
#[validator(host(local(Allow), port(Allow), at_least_two_labels(Must)))]
struct Host {
    host:     validators::models::Host,
    is_local: bool,
    port:     Option<u16>,
}

#[cfg(feature = "http_url")]
#[derive(Validator)]
#[validator(http_url(local(Disallow)))]
struct HttpUrl {
    url:      validators::url::Url,
    is_https: bool,
}

#[cfg(feature = "http_ftp_url")]
#[derive(Validator)]
#[validator(http_ftp_url)]
struct HttpFtpUrl {
    url:      validators::url::Url,
    protocol: validators::models::Protocol,
}

#[cfg(feature = "ip")]
#[derive(Validator)]
#[validator(ip(local(Disallow), port(Allow)))]
struct Ip {
    ip:   core::net::IpAddr,
    port: Option<u16>,
}

#[cfg(feature = "ipv4")]
#[derive(Validator)]
#[validator(ipv4(port(Must)))]
struct Ipv4 {
    ipv4: core::net::Ipv4Addr,
    port: u16,
}

#[cfg(feature = "ipv6")]
#[derive(Validator)]
#[validator(ipv6(port(Disallow)))]
struct Ipv6(core::net::Ipv6Addr);

#[cfg(feature = "json")]
#[derive(Validator)]
#[validator(json)]
struct Json(i64);

#[cfg(feature = "length")]
#[derive(Validator)]
#[validator(length(min = 1))]
struct Length(Vec<u8>);

#[cfg(feature = "line")]
#[derive(Validator)]
#[validator(line(char_length(trimmed_min = 1, max = 100)))]
struct Line(String);

#[cfg(feature = "mac_address")]
#[derive(Validator)]
#[validator(mac_address)]
struct MacAddress(u64);

#[cfg(feature = "number")]
#[derive(Validator)]
#[validator(number(range(Inside(min = 0, max = 1))))]
struct Number(f64);

#[cfg(feature = "phone")]
#[derive(Validator)]
#[validator(phone(countries(TW, US)))]
struct Phone(
    std::collections::HashMap<
        validators::phonenumber::country::Id,
        validators::phonenumber::PhoneNumber,
    >,
);

#[cfg(feature = "regex")]
#[derive(Validator)]
#[validator(regex(regex("^[a-z]+$")))]
struct Regex(String);

#[cfg(feature = "semver")]
#[derive(Validator)]
#[validator(semver)]
struct SemVer(validators::semver::Version);

#[cfg(feature = "semver_req")]
#[derive(Validator)]
#[validator(semver_req)]
struct SemVerReq(validators::semver::VersionReq);

#[cfg(feature = "signed_integer")]
#[derive(Validator)]
#[validator(signed_integer(range(Inside(min = -1, max = 1))))]
struct SignedInteger(i32);

#[cfg(feature = "text")]
#[derive(Validator)]
#[validator(text(byte_length(min = 1)))]
struct Text(String);

#[cfg(feature = "unsigned_integer")]
#[derive(Validator)]
#[validator(unsigned_integer(range(Inside(min = 1, max = 100))))]
struct UnsignedInteger(u8);

#[cfg(feature = "url")]
#[derive(Validator)]
#[validator(url)]
struct Url(validators::url::Url);

#[cfg(feature = "uuid")]
#[derive(Validator)]
#[validator(uuid)]
struct Uuid(u128);

#[test]
fn shadowed_prelude_names() {
    #[cfg(feature = "base32")]
    assert!(Base32::parse_str("GEZDGNBVGY3TQOI=").is_ok());
    #[cfg(feature = "base32_decoded")]
    assert!(Base32Decoded::parse_str("GEZDGNBVGY3TQOI=").is_ok());
    #[cfg(feature = "base64")]
    assert!(Base64::parse_str("MTIzNDU2Nzg5MA==").is_ok());
    #[cfg(feature = "base64_decoded")]
    assert!(Base64Decoded::parse_str("MTIzNDU2Nzg5MA").is_ok());
    #[cfg(feature = "base64_url")]
    assert!(Base64Url::parse_str("5LmN5pqW6YKE5a-S5pmC5YCZ").is_ok());
    #[cfg(feature = "base64_url_decoded")]
    assert!(Base64UrlDecoded::parse_str("5LmN5pqW6YKE5a-S5pmC5YCZ").is_ok());
    #[cfg(feature = "bit")]
    assert!(Bit::parse_str("1kb").is_ok());
    #[cfg(feature = "boolean")]
    assert!(Boolean::parse_str("true").is_ok());
    #[cfg(feature = "byte")]
    assert!(Byte::parse_str("1KB").is_ok());
    #[cfg(feature = "domain")]
    assert!(Domain::parse_str("example.com:8080").is_ok());
    #[cfg(feature = "email")]
    assert!(Email::parse_str("(john)joke@example.com").is_ok());
    #[cfg(feature = "hex_color")]
    assert!(HexColor::parse_str("#abc").is_ok());
    #[cfg(feature = "hex_decoded")]
    assert!(HexDecoded::parse_str("00ABFF").is_ok());
    #[cfg(feature = "host")]
    assert!(Host::parse_str("[::1]:8080").is_ok());
    #[cfg(feature = "http_url")]
    assert!(HttpUrl::parse_str("https://example.org/").is_ok());
    #[cfg(feature = "http_ftp_url")]
    assert!(HttpFtpUrl::parse_str("ftp://example.org/").is_ok());
    #[cfg(feature = "ip")]
    assert!(Ip::parse_str("8.8.8.8:53").is_ok());
    #[cfg(feature = "ipv4")]
    assert!(Ipv4::parse_str("127.0.0.1:8080").is_ok());
    #[cfg(feature = "ipv6")]
    assert!(Ipv6::parse_str("::1").is_ok());
    #[cfg(feature = "json")]
    assert!(Json::parse_str("123").is_ok());
    #[cfg(feature = "length")]
    assert!(Length::parse_collection(vec![0]).is_ok());
    #[cfg(feature = "line")]
    assert!(Line::parse_str("123").is_ok());
    #[cfg(feature = "mac_address")]
    assert!(MacAddress::parse_str("08:00:27:B2:46:C3").is_ok());
    #[cfg(feature = "number")]
    assert!(Number::parse_str("0.5").is_ok());
    #[cfg(feature = "phone")]
    assert!(Phone::parse_str("+886912345678").is_ok());
    #[cfg(feature = "regex")]
    assert!(Regex::parse_str("abc").is_ok());
    #[cfg(feature = "semver")]
    assert!(SemVer::parse_str("1.0.0").is_ok());
    #[cfg(feature = "semver_req")]
    assert!(SemVerReq::parse_str(">= 0.4").is_ok());
    #[cfg(feature = "signed_integer")]
    assert!(SignedInteger::parse_str("-1").is_ok());
    #[cfg(feature = "text")]
    assert!(Text::parse_str("123\n456").is_ok());
    #[cfg(feature = "unsigned_integer")]
    assert!(UnsignedInteger::parse_str("5").is_ok());
    #[cfg(feature = "url")]
    assert!(Url::parse_str("https://example.org/").is_ok());
    #[cfg(feature = "uuid")]
    assert!(Uuid::parse_str("A866664A-F9D3-4DDE-89CB-182015FA4F41").is_ok());
}
