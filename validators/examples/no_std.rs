//! A build check for the code that `#[derive(Validator)]` generates without `std`.
//!
//! The CI builds this example for `thumbv7em-none-eabi`, which has no `std`.

#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use validators::{
    byte_unit::{Bit, Byte},
    models::Host,
    prelude::*,
    semver::{Version, VersionReq},
};

#[derive(Validator)]
#[validator(base32)]
struct Base32(String);

#[derive(Validator)]
#[validator(base32_decoded)]
struct Base32Decoded(Vec<u8>);

#[derive(Validator)]
#[validator(base64(padding(Must)))]
struct Base64(String);

#[derive(Validator)]
#[validator(base64_decoded(padding(Disallow)))]
struct Base64Decoded(Vec<u8>);

#[derive(Validator)]
#[validator(base64_url)]
struct Base64Url(String);

#[derive(Validator)]
#[validator(base64_url_decoded)]
struct Base64UrlDecoded(Vec<u8>);

#[derive(Validator)]
#[validator(bit(range(min = 1)))]
struct BitSize(Bit);

#[derive(Validator)]
#[validator(boolean)]
struct Boolean(bool);

#[derive(Validator)]
#[validator(byte(range(max = 1024)))]
struct ByteSize(Byte);

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), port(Allow), at_least_two_labels(Must)))]
struct Domain {
    domain:   String,
    is_ipv4:  bool,
    is_local: bool,
    port:     Option<u16>,
}

#[derive(Validator)]
#[validator(email)]
struct Email {
    local_part:                 String,
    need_quoted:                bool,
    domain_part:                Host,
    comment_before_local_part:  Option<String>,
    comment_after_local_part:   Option<String>,
    comment_before_domain_part: Option<String>,
    comment_after_domain_part:  Option<String>,
}

#[derive(Validator)]
#[validator(hex_color)]
struct HexColor(u32);

#[derive(Validator)]
#[validator(hex_decoded)]
struct HexDecoded(Vec<u8>);

#[derive(Validator)]
#[validator(host(local(Allow), port(Must), at_least_two_labels(Must)))]
struct HostWithPort {
    host:     Host,
    is_local: bool,
    port:     u16,
}

#[derive(Validator)]
#[validator(ip(local(Disallow), port(Allow)))]
struct Ip {
    ip:   IpAddr,
    port: Option<u16>,
}

#[derive(Validator)]
#[validator(ipv4(port(Must)))]
struct Ipv4 {
    ipv4: Ipv4Addr,
    port: u16,
}

#[derive(Validator)]
#[validator(ipv6(port(Disallow)))]
struct Ipv6(Ipv6Addr);

#[derive(Validator)]
#[validator(json)]
struct Json(i64);

#[derive(Validator)]
#[validator(length(min = 1))]
struct Length(Vec<u8>);

#[derive(Validator)]
#[validator(line(char_length(trimmed_min = 1, max = 100)))]
struct Line(String);

#[derive(Validator)]
#[validator(mac_address)]
struct MacAddress(u64);

#[derive(Validator)]
#[validator(number(range(Inside(min = 0, max = 1))))]
struct Number(f64);

#[derive(Validator)]
#[validator(semver)]
struct SemVer(Version);

#[derive(Validator)]
#[validator(semver_req)]
struct SemVerReq(VersionReq);

#[derive(Validator)]
#[validator(signed_integer(range(Inside(min = -1, max = 1))))]
struct SignedInteger(i32);

#[derive(Validator)]
#[validator(text(byte_length(min = 1)))]
struct Text(String);

#[derive(Validator)]
#[validator(unsigned_integer)]
struct UnsignedInteger(u128);

#[derive(Validator)]
#[validator(uuid)]
struct Uuid(u128);
