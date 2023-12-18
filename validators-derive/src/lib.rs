/*!
# Validators Derive

The provided crate offers a procedural macro for defining validators, including optional parameters. See the [`validators`](https://crates.io/crates/validators) crate.
 */

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod common;
#[allow(unused)]
mod panic;
mod supported_validators;
mod validator_handlers;

use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    DeriveInput, Meta,
};
#[allow(unused)]
use validator_handlers::ValidatorHandler;

use crate::supported_validators::Validator;

fn derive_input_handler(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut use_validator: Option<(Validator, Meta)> = None;

    for attr in ast.attrs.iter() {
        let path = attr.path();

        if path.is_ident("validator") {
            if let Meta::List(list) = &attr.meta {
                let meta: Meta = list.parse_args()?;

                let path = meta.path();

                if let Some(validator) = Validator::from_path(path) {
                    if use_validator.is_some() {
                        return Err(panic::validator_only_one_at_a_time(path.span()));
                    }

                    use_validator = Some((validator, meta));
                } else {
                    return Err(panic::unsupported_validator(path));
                }
            } else {
                return Err(panic::validator_format_incorrect(path.span()));
            }
        }
    }

    if let Some((validator, meta)) = use_validator {
        match validator {
            #[cfg(feature = "base32")]
            Validator::base32 => {
                return validator_handlers::base32::Base32Handler::meta_handler(ast, meta);
            },
            #[cfg(feature = "base32_decoded")]
            Validator::base32_decoded => {
                return validator_handlers::base32_decoded::Base32DecodedHandler::meta_handler(
                    ast, meta,
                );
            },
            #[cfg(feature = "base64")]
            Validator::base64 => {
                return validator_handlers::base64::Base64Handler::meta_handler(ast, meta);
            },
            #[cfg(feature = "base64_decoded")]
            Validator::base64_decoded => {
                return validator_handlers::base64_decoded::Base64DecodedHandler::meta_handler(
                    ast, meta,
                );
            },
            #[cfg(feature = "base64_url")]
            Validator::base64_url => {
                return validator_handlers::base64_url::Base64UrlHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "base64_url_decoded")]
            Validator::base64_url_decoded => {
                return validator_handlers::base64_url_decoded::Base64UrlDecodedHandler::meta_handler(
                    ast, meta,
                );
            },
            #[cfg(feature = "bit")]
            Validator::bit => {
                return validator_handlers::bit::BitHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "boolean")]
            Validator::boolean => {
                return validator_handlers::boolean::BooleanHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "byte")]
            Validator::byte => {
                return validator_handlers::byte::ByteHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "domain")]
            Validator::domain => {
                return validator_handlers::domain::DomainHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "email")]
            Validator::email => {
                return validator_handlers::email::EmailHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "host")]
            Validator::host => {
                return validator_handlers::host::HostHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "http_url")]
            Validator::http_url => {
                return validator_handlers::http_url::HttpUrlHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "http_ftp_url")]
            Validator::http_ftp_url => {
                return validator_handlers::http_ftp_url::HttpFtpUrlHandler::meta_handler(
                    ast, meta,
                );
            },
            #[cfg(feature = "ip")]
            Validator::ip => {
                return validator_handlers::ip::IpHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "ipv4")]
            Validator::ipv4 => {
                return validator_handlers::ipv4::Ipv4Handler::meta_handler(ast, meta);
            },
            #[cfg(feature = "ipv6")]
            Validator::ipv6 => {
                return validator_handlers::ipv6::Ipv6Handler::meta_handler(ast, meta);
            },
            #[cfg(feature = "json")]
            Validator::json => {
                return validator_handlers::json::JsonHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "length")]
            Validator::length => {
                return validator_handlers::length::LengthHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "line")]
            Validator::line => {
                return validator_handlers::line::LineHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "mac_address")]
            Validator::mac_address => {
                return validator_handlers::mac_address::MacAddressHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "number")]
            Validator::number => {
                return validator_handlers::number::NumberHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "phone")]
            Validator::phone => {
                return validator_handlers::phone::PhoneHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "regex")]
            Validator::regex => {
                return validator_handlers::regex::RegexHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "semver")]
            Validator::semver => {
                return validator_handlers::semver::SemverHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "semver_req")]
            Validator::semver_req => {
                return validator_handlers::semver_req::SemverReqHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "signed_integer")]
            Validator::signed_integer => {
                return validator_handlers::signed_integer::SignedIntegerHandler::meta_handler(
                    ast, meta,
                );
            },
            #[cfg(feature = "text")]
            Validator::text => {
                return validator_handlers::text::TextHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "unsigned_integer")]
            Validator::unsigned_integer => {
                return validator_handlers::unsigned_integer::UnsignedIntegerHandler::meta_handler(
                    ast, meta,
                );
            },
            #[cfg(feature = "url")]
            Validator::url => {
                return validator_handlers::url::UrlHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "uuid")]
            Validator::uuid => {
                return validator_handlers::uuid::UuidHandler::meta_handler(ast, meta);
            },
            Validator::_Nothing => {
                // avoid unused warnings
                let _ = meta;
                unreachable!();
            },
        }
    }

    Err(panic::derive_attribute_not_set_up_yet())
}

#[proc_macro_derive(Validator, attributes(validator))]
pub fn validator_derive(input: TokenStream) -> TokenStream {
    struct MyDeriveInput(proc_macro2::TokenStream);

    impl Parse for MyDeriveInput {
        #[inline]
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let token_stream = derive_input_handler(input.parse::<DeriveInput>()?)?;

            Ok(Self(token_stream))
        }
    }

    // Parse the token stream
    let derive_input = parse_macro_input!(input as MyDeriveInput);

    derive_input.0.into()
}
