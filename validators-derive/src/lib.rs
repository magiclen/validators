/*!
# Validators Derive

The provided crate offers a procedural macro for defining validators, including optional parameters. See the [`validators`](https://crates.io/crates/validators) crate.
 */

#![cfg_attr(docsrs, feature(doc_cfg))]

mod common;
#[allow(unused)]
mod panic;
mod supported_validators;
mod validator_handlers;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, Meta,
    parse::{Parse, ParseStream},
    parse_macro_input,
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
                        return Err(panic::validator_only_one_at_a_time(path));
                    }

                    use_validator = Some((validator, meta));
                } else if supported_validators::is_validator_name(path) {
                    return Err(panic::validator_feature_not_enabled(path));
                } else {
                    return Err(panic::unsupported_validator(path));
                }
            } else {
                return Err(panic::validator_format_incorrect(attr));
            }
        }
    }

    if let Some((validator, meta)) = use_validator {
        // The generated `impl` blocks do not carry generic parameters.
        if !ast.generics.params.is_empty() {
            return Err(panic::generics_not_supported(&ast));
        }

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
            #[cfg(feature = "hex_color")]
            Validator::hex_color => {
                return validator_handlers::hex_color::HexColorHandler::meta_handler(ast, meta);
            },
            #[cfg(feature = "hex_decoded")]
            Validator::hex_decoded => {
                return validator_handlers::hex_decoded::HexDecodedHandler::meta_handler(ast, meta);
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
                return validator_handlers::utf8::LineHandler::meta_handler(ast, meta);
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
                return validator_handlers::utf8::TextHandler::meta_handler(ast, meta);
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

    Err(panic::derive_attribute_not_set_up_yet(&ast))
}

/// Put the generated items in an anonymous constant that imports the prelude names used by the generated code.
/// Then imports in the user's module, such as `use Status::*` where `Status` has an `Ok` variant, cannot shadow these names.
fn protect_prelude_names(
    type_name: &str,
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let imports = [
        ("Option", quote! { ::core::option::Option }),
        ("Some", quote! { ::core::option::Option::Some }),
        ("None", quote! { ::core::option::Option::None }),
        ("Ok", quote! { ::core::result::Result::Ok }),
        ("Err", quote! { ::core::result::Result::Err }),
    ]
    .into_iter()
    // An import with the same name as the type would hide the type, so skip it.
    .filter(|(name, _)| *name != type_name)
    .map(|(_, path)| {
        quote! {
            #[allow(unused_imports)]
            use #path;
        }
    });

    quote! {
        const _: () = {
            #(#imports)*

            #token_stream
        };
    }
}

#[proc_macro_derive(Validator, attributes(validator))]
pub fn validator_derive(input: TokenStream) -> TokenStream {
    struct MyDeriveInput(proc_macro2::TokenStream);

    impl Parse for MyDeriveInput {
        #[inline]
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let ast = input.parse::<DeriveInput>()?;
            let type_name = ast.ident.to_string();
            let token_stream = derive_input_handler(ast)?;

            Ok(Self(protect_prelude_names(&type_name, token_stream)))
        }
    }

    // Parse the token stream
    let derive_input = parse_macro_input!(input as MyDeriveInput);

    derive_input.0.into()
}

#[cfg(all(
    test,
    any(
        feature = "number",
        feature = "signed_integer",
        feature = "unsigned_integer",
        feature = "bit",
        feature = "byte",
        feature = "line",
        feature = "text",
        feature = "length",
        feature = "phone",
    )
))]
mod tests {
    use super::derive_input_handler;

    fn check_attribute(attribute: &str, data_type: &str, expected_error: Option<&str>) {
        let input =
            syn::parse_str(&format!("#[validator({attribute})] struct Value({data_type});"))
                .unwrap();
        let result = derive_input_handler(input);

        if let Some(expected) = expected_error {
            assert_eq!(expected, result.unwrap_err().to_string(), "{attribute}");
        } else {
            assert!(result.is_ok(), "{attribute}: {result:?}");
        }
    }

    #[cfg(any(feature = "number", feature = "signed_integer", feature = "unsigned_integer"))]
    #[test]
    fn numeric_range_parameters() {
        for (validator, data_type) in [
            #[cfg(feature = "number")]
            ("number", "f64"),
            #[cfg(feature = "signed_integer")]
            ("signed_integer", "i64"),
            #[cfg(feature = "unsigned_integer")]
            ("unsigned_integer", "u64"),
        ] {
            check_attribute(validator, data_type, None);
            check_attribute(&format!("{validator}(range(Unlimited))"), data_type, None);

            for direction in ["Inside", "Outside"] {
                check_attribute(
                    &format!("{validator}(range({direction}()))"),
                    data_type,
                    Some("expected at least one `min` or `max` bound"),
                );

                for flag in ["true", "false"] {
                    for bounds in
                        [format!("inclusive = {flag}"), format!("min = 0, inclusive = {flag}")]
                    {
                        check_attribute(
                            &format!("{validator}(range({direction}({bounds})))"),
                            data_type,
                            Some("`inclusive` requires a `max` bound"),
                        );
                    }
                }

                for bounds in [
                    "min = 0",
                    "max = 1",
                    "max = 1, inclusive = true",
                    "max = 1, inclusive = false",
                    "min = 0, max = 1, inclusive = false",
                ] {
                    check_attribute(
                        &format!("{validator}(range({direction}({bounds})))"),
                        data_type,
                        None,
                    );
                }

                #[cfg(feature = "full")]
                for expression in ["(..)", " = ..", "(0..)", " = 0..", "(..1)", "(..=1)", "(0..=1)"]
                {
                    check_attribute(
                        &format!("{validator}(range({direction}{expression}))"),
                        data_type,
                        if matches!(expression, "(..)" | " = ..") {
                            Some("expected at least one `min` or `max` bound")
                        } else {
                            None
                        },
                    );
                }
            }
        }
    }

    #[cfg(any(feature = "bit", feature = "byte"))]
    #[test]
    fn byte_unit_range_parameters() {
        for (validator, data_type) in [
            #[cfg(feature = "bit")]
            ("bit", "validators_prelude::byte_unit::Bit"),
            #[cfg(feature = "byte")]
            ("byte", "validators_prelude::byte_unit::Byte"),
        ] {
            check_attribute(validator, data_type, None);
            check_attribute(
                &format!("{validator}(range())"),
                data_type,
                Some("expected at least one `min` or `max` bound"),
            );

            for flag in ["true", "false"] {
                for bounds in
                    [format!("inclusive = {flag}"), format!("min = 0, inclusive = {flag}")]
                {
                    check_attribute(
                        &format!("{validator}(range({bounds}))"),
                        data_type,
                        Some("`inclusive` requires a `max` bound"),
                    );
                }
            }

            for bounds in [
                "min = 0",
                "max = 1",
                "max = 1, inclusive = true",
                "max = 1, inclusive = false",
                "min = 0, max = 1, inclusive = false",
            ] {
                check_attribute(&format!("{validator}(range({bounds}))"), data_type, None);
            }

            #[cfg(feature = "full")]
            for expression in ["(..)", " = ..", "(0..)", " = 0..", "(..1)", "(..=1)", "(0..=1)"] {
                check_attribute(
                    &format!("{validator}(range{expression})"),
                    data_type,
                    if matches!(expression, "(..)" | " = ..") {
                        Some("expected at least one `min` or `max` bound")
                    } else {
                        None
                    },
                );
            }
        }
    }

    #[cfg(any(feature = "line", feature = "text"))]
    #[test]
    fn string_length_parameters() {
        for validator in [
            #[cfg(feature = "line")]
            "line",
            #[cfg(feature = "text")]
            "text",
        ] {
            check_attribute(validator, "String", None);

            for length in ["char_length", "byte_length"] {
                check_attribute(
                    &format!("{validator}({length}())"),
                    "String",
                    Some("expected at least one `min`, `max`, or `trimmed_min` length bound"),
                );

                for bounds in ["min = 0", "max = 0", "trimmed_min = 0", "min = 1, max = 3"] {
                    check_attribute(&format!("{validator}({length}({bounds}))"), "String", None);
                }
            }
        }
    }

    #[cfg(feature = "length")]
    #[test]
    fn collection_length_defaults() {
        for attribute in ["length", "length()", "length(serde = false)", "length(min = 0)"] {
            check_attribute(attribute, "Vec<u8>", None);
        }
    }

    #[cfg(any(feature = "line", feature = "phone"))]
    #[test]
    fn trailing_commas() {
        #[cfg(all(feature = "line", feature = "serde"))]
        check_attribute("line(serde(Serialize, Deserialize,))", "String", None);

        #[cfg(all(feature = "line", feature = "rocket"))]
        check_attribute("line(rocket(FromFormField, FromParam,))", "String", None);

        #[cfg(feature = "phone")]
        check_attribute("phone(countries(TW, US,))", "HashMap<Id, PhoneNumber>", None);
    }
}
