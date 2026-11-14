# Validators 0.26.1 to 0.27.0

Highlights from `v0.26.1` through `3222ff3`, plus subsequent email buffer reuse and the MSRV update.

## Breaking changes

- **MSRV:** Both crates now require Rust 1.95, up from 1.89. Rocket's `ref-swap 0.1.3` dependency uses `AtomicPtr::try_update`, which became stable in Rust 1.95. See the [runtime manifest](validators/Cargo.toml#L6) and [derive manifest](validators-derive/Cargo.toml#L6).

Serde output changes (JSON examples):

| Validator | Previous output | New output |
| --- | --- | --- |
| [`json`](validators-derive/src/validator_handlers/json/mod.rs) | JSON text inside a string, such as `"123"` | The inner value, such as `123` |
| [Decoded Base32/Base64/Base64url](validators-derive/src/validator_handlers/base_xx.rs) | Decoded byte arrays, such as `[65]` | Encoded strings, such as `"QQ=="`; non-empty byte arrays are still accepted as input |
| [`phone`](validators-derive/src/validator_handlers/phone/mod.rs) | Underlying phonenumber structures | E.164/RFC 3966 strings, or country-to-string maps for multi-country validators; carrier codes are not retained |
| Non-finite [`number`](validators-derive/src/validator_handlers/number/mod.rs) values | `null` | `"NaN"`, `"inf"`, or `"-inf"` |

Update stored-data schemas and consumers that depend on the previous representations.

- **Other output changes:** [`DomainHandler`](validators-derive/src/validator_handlers/domain/mod.rs) preserves a trailing root dot in Serde output. [`MacAddressHandler`](validators-derive/src/validator_handlers/mac_address/mod.rs) formats only the low 48 bits, including compact output.
- **Stricter validation:** Reject non-canonical encodings, concatenated padded [Base32/64](validators/src/encoded_bytes.rs), partial [MAC](validators-derive/src/validator_handlers/mac_address/mod.rs)/[UUID](validators-derive/src/validator_handlers/uuid/mod.rs) separators, `+` ports, and numeric-ending [domains](validators-derive/src/validator_handlers/domain/mod.rs) except supported dotted-decimal IPv4. Tighten [email](validators-derive/src/validator_handlers/email/mod.rs) syntax and normalized length limits, and reject additional controls in email and [line/text](validators-derive/src/validator_handlers/utf8.rs).
- **Local-address rules:** `*.localhost` and more special-purpose IP ranges are local, affecting `local(Must/Disallow)`. See [`is_local_domain`](validators/src/functions/domain.rs), [`is_local_ipv4`](validators/src/functions/ipv4.rs), and [`is_local_ipv6`](validators/src/functions/ipv6.rs).
- **Attribute checks:** Reject empty range or line/text length constraints, non-finite/reversed range bounds, `inclusive` without `max`, arguments to `Unlimited`, and non-ASCII separators. Omit unrestricted constraints or use `range(Unlimited)` where supported. See [`Range::check_bounds`](validators-derive/src/common/range.rs), [`RangeOption::from_meta`](validators-derive/src/common/range_option.rs), [`Length::from_meta`](validators-derive/src/common/length.rs), and [`SeparatorOption::parse_separator`](validators-derive/src/common/separator_option.rs).
- **Error variants:** Integer overflow now returns `TooLarge`/`TooSmall`; bit/byte storage overflow returns `TooLarge`. Malformed URL, port, bracket, and encoding input can return different error variants. Review error matching. See [`SignedIntegerError`](validators/src/errors/signed_integer.rs), [`UnsignedIntegerError`](validators/src/errors/unsigned_integer.rs), [`BitError`](validators/src/errors/bit.rs), [`ByteError`](validators/src/errors/byte.rs), and [validator handlers](validators-derive/src/validator_handlers/mod.rs).
- **JSON Result errors:** `Result<T, JsonError>` propagates deserializer failures as outer errors instead of capturing `InvalidJsonValueError`. See its [`Deserialize` implementation](validators/src/result/serde_traits.rs#L842).
- **Feature selection:** Domain/email/host/IP features no longer implicitly enable `std`; HTTP validators no longer enable `str-utils`. Enable these features explicitly when needed. See [feature definitions](validators/Cargo.toml#L94).

## Added

- Add [`hex_decoded`](validators-derive/src/validator_handlers/hex_decoded/mod.rs) and [`hex_color`](validators-derive/src/validator_handlers/hex_color/mod.rs), including Serde/Rocket/Axum support and `ToHexColorString`.
- Add [`ends_in_a_number`](validators/src/functions/domain.rs); extend [`CollectionLength`](validators/src/traits/collection_length.rs) to more collections/custom hashers and [Result traits](validators/src/result/built_in_traits.rs) to any marker `K`.
- Support more `macro_rules!` attribute fragments, negative range endpoints, and trailing commas; improve diagnostics and generated-name collision handling. See [`derive_input_handler`](validators-derive/src/lib.rs) and [`IdentGenerator`](validators-derive/src/common/ident_generator.rs).

## Fixed

- Fix recursive deserialization in [`SemverHandler`](validators-derive/src/validator_handlers/semver/mod.rs), [`SemverReqHandler`](validators-derive/src/validator_handlers/semver_req/mod.rs), and [`UrlHandler`](validators-derive/src/validator_handlers/url/mod.rs), and support concrete binary Serde representations through the [shared deserializers](validators/src/result/serde_traits.rs).
- Fix quoted/empty local parts, escaped comments, IDNA/IPv4 detection, and IPv6 formatting in [`EmailHandler`](validators-derive/src/validator_handlers/email/mod.rs); correct [URL scheme/error handling](validators-derive/src/validator_handlers/http_xx_url.rs) and [host IPv6 brackets](validators-derive/src/validator_handlers/host/mod.rs).
- Fix [phone country fallback](validators-derive/src/validator_handlers/phone/mod.rs), [equal-bound collection length generation](validators-derive/src/validator_handlers/length/mod.rs), [short MAC input panics](validators-derive/src/validator_handlers/mac_address/mod.rs), and [integer-to-f32 double rounding](validators-derive/src/validator_handlers/number/mod.rs).

## Improved

- Reduce allocations with borrowed parsing, owned-buffer reuse, allocation-free IPv6 length counting, cached literal regexes, and direct Serde formatting. The post-baseline email `parse_string` optimization in [`EmailHandler`](validators-derive/src/validator_handlers/email/mod.rs#L1046) saves one allocation for prefix local parts; measured IP-literal cases need no allocation beyond the owned input. See also [regex caching](validators-derive/src/validator_handlers/regex/regex_attribute.rs).
- Share encoding, UTF-8, HTTP, hex, and struct-shape handling; expand regression tests and `no_std` CI coverage. See [handlers](validators-derive/src/validator_handlers/mod.rs), [`fields_match`](validators-derive/src/common/mod.rs), and [CI](.github/workflows/ci.yml).
- Update `syn` to 3, `proc-macro2` to 1.0.91, and `educe` to 0.8; pin `validators-derive` to `=0.27.0`. See the [derive manifest](validators-derive/Cargo.toml#L18) and [runtime manifest](validators/Cargo.toml#L16).
