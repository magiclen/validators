/*!
# Validators Derive

This crate provides a procedural macro to define validators with optional parameters. See the [`validators`](https://crates.io/crates/validators) crate.
 */

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
extern crate enum_ordinalize;

#[macro_use]
extern crate educe;

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
                                        #[cfg(feature = "http_url")]
                                        Validator::http_url => {
                                            return http_url::http_url_handler(ast, meta)
                                        }
                                        #[cfg(feature = "http_ftp_url")]
                                        Validator::http_ftp_url => {
                                            return http_ftp_url::http_ftp_url_handler(ast, meta)
                                        }
                                        #[cfg(feature = "ip")]
                                        Validator::ip => return ip::ip_handler(ast, meta),
                                        #[cfg(feature = "ipv4")]
                                        Validator::ipv4 => return ipv4::ipv4_handler(ast, meta),
                                        #[cfg(feature = "ipv6")]
                                        Validator::ipv6 => return ipv6::ipv6_handler(ast, meta),
                                        #[cfg(feature = "json")]
                                        Validator::json => return json::json_handler(ast, meta),
                                        #[cfg(feature = "length")]
                                        Validator::length => {
                                            return length::length_handler(ast, meta)
                                        }
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
                                        #[cfg(feature = "phone")]
                                        Validator::phone => return phone::phone_handler(ast, meta),
                                        #[cfg(feature = "regex")]
                                        Validator::regex => return regex::regex_handler(ast, meta),
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
                                        #[cfg(feature = "url")]
                                        Validator::url => return url::url_handler(ast, meta),
                                        #[cfg(feature = "uuid")]
                                        Validator::uuid => return uuid::uuid_handler(ast, meta),
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
