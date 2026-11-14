use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{attributes::basic_attribute::BasicAttribute, type_enum::TypeEnum},
    panic,
};

pub(crate) struct BooleanHandler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::Boolean);

impl ValidatorHandler for BooleanHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        #[cfg(feature = "serde")]
        let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
        #[cfg(feature = "serde")]
        let deserializer = ident_generator.next("__ValidatorsDeserializer");
        #[cfg(feature = "serde")]
        let deserializer_name = ident_generator.next("__validators_deserializer");
        #[cfg(feature = "serde")]
        let visitor = ident_generator.next("__ValidatorsVisitor");

        #[allow(unused_variables)]
        let type_attribute = BasicAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data
            && let Fields::Unnamed(_) = &data.fields
            && data.fields.len() == 1
        {
            let mut token_stream = proc_macro2::TokenStream::new();

            let name = ast.ident;

            let error_path: Path =
                syn::parse2(quote! { validators_prelude::BooleanError }).unwrap();

            token_stream.extend(quote! {
                        impl #name {
                            fn v_parse_str(s: &str) -> ::core::result::Result<bool, #error_path> {
                                let bytes = s.as_bytes();

                                match bytes.len() {
                                    // A one-byte string is ASCII, so it can be checked as a character.
                                    1 => Self::v_parse_char(bytes[0] as char),
                                    2 if bytes.eq_ignore_ascii_case(b"on") => ::core::result::Result::Ok(true),
                                    2 if bytes.eq_ignore_ascii_case(b"no") => ::core::result::Result::Ok(false),
                                    3 if bytes.eq_ignore_ascii_case(b"yes") => ::core::result::Result::Ok(true),
                                    3 if bytes.eq_ignore_ascii_case(b"off") => ::core::result::Result::Ok(false),
                                    4 if bytes.eq_ignore_ascii_case(b"true") => ::core::result::Result::Ok(true),
                                    5 if bytes.eq_ignore_ascii_case(b"false") => ::core::result::Result::Ok(false),
                                    _ => ::core::result::Result::Err(#error_path),
                                }
                            }

                            #[inline]
                            fn v_parse_char(c: char) -> ::core::result::Result<bool, #error_path> {
                                match c {
                                    't' | 'T' | '1' | 'y' | 'Y' => ::core::result::Result::Ok(true),
                                    'f' | 'F' | '0' | 'n' | 'N' => ::core::result::Result::Ok(false),
                                    _ => ::core::result::Result::Err(#error_path),
                                }
                            }

                            #[inline]
                            fn v_parse_i128(i: i128) -> ::core::result::Result<bool, #error_path> {
                                match i {
                                    1 => ::core::result::Result::Ok(true),
                                    0 => ::core::result::Result::Ok(false),
                                    _ => ::core::result::Result::Err(#error_path),
                                }
                            }

                            #[inline]
                            fn v_parse_u128(u: u128) -> ::core::result::Result<bool, #error_path> {
                                match u {
                                    1 => ::core::result::Result::Ok(true),
                                    0 => ::core::result::Result::Ok(false),
                                    _ => ::core::result::Result::Err(#error_path),
                                }
                            }

                            #[inline]
                            fn v_parse_bool(b: bool) -> ::core::result::Result<bool, #error_path> {
                                ::core::result::Result::Ok(b)
                            }
                        }
                    });

            token_stream.extend(quote! {
                        impl ValidateString for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                                ::core::result::Result::Ok(Self(Self::v_parse_str(s.into().as_str())?))
                            }

                            #[inline]
                            fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                                ::core::result::Result::Ok(Self(Self::v_parse_str(s.as_ref())?))
                            }

                            #[inline]
                            fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
                                Self::v_parse_str(s.as_ref())?;

                                ::core::result::Result::Ok(())
                            }
                        }

                        impl ValidateChar for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_char(c: char) -> ::core::result::Result<Self, Self::Error> {
                                ::core::result::Result::Ok(Self(Self::v_parse_char(c)?))
                            }

                            #[inline]
                            fn validate_char(c: char) -> ::core::result::Result<(), Self::Error> {
                                Self::v_parse_char(c)?;

                                ::core::result::Result::Ok(())
                            }
                        }

                        impl ValidateSignedInteger for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_i128(i: i128) -> ::core::result::Result<Self, Self::Error> {
                                ::core::result::Result::Ok(Self(Self::v_parse_i128(i)?))
                            }

                            #[inline]
                            fn validate_i128(i: i128) -> ::core::result::Result<(), Self::Error> {
                                Self::v_parse_i128(i)?;

                                ::core::result::Result::Ok(())
                            }
                        }

                        impl ValidateUnsignedInteger for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_u128(u: u128) -> ::core::result::Result<Self, Self::Error> {
                                ::core::result::Result::Ok(Self(Self::v_parse_u128(u)?))
                            }

                            #[inline]
                            fn validate_u128(u: u128) -> ::core::result::Result<(), Self::Error> {
                                Self::v_parse_u128(u)?;

                                ::core::result::Result::Ok(())
                            }
                        }

                        impl ValidateBoolean for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_bool(b: bool) -> ::core::result::Result<Self, Self::Error> {
                                ::core::result::Result::Ok(Self(Self::v_parse_bool(b)?))
                            }

                            #[inline]
                            fn validate_bool(b: bool) -> ::core::result::Result<(), Self::Error> {
                                Self::v_parse_bool(b)?;

                                ::core::result::Result::Ok(())
                            }
                        }
                    });

            #[cfg(feature = "serde")]
            {
                if type_attribute.serde_options.serialize {
                    token_stream.extend(quote! {
                        impl validators_prelude::serde::Serialize for #name {
                            #[inline]
                            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                                where
                                    S: validators_prelude::serde::Serializer, {
                                serializer.serialize_bool(self.0)
                            }
                        }
                    });
                }

                if type_attribute.serde_options.deserialize {
                    let expect = "a boolean, a boolean string like \"true\", \"false\", \"0\", \
                                  \"1\", \"on\", \"off\", \"yes\", \"no\", or a number `0` or `1`";

                    token_stream.extend(quote! {
                                impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                                    #[inline]
                                    fn deserialize<#deserializer>(#deserializer_name: #deserializer) -> ::core::result::Result<Self, #deserializer::Error>
                                    where
                                        #deserializer: validators_prelude::serde::Deserializer<'de>, {
                                        struct #visitor;

                                        impl<'de> validators_prelude::serde::de::Visitor<'de> for #visitor {
                                            type Value = #name;

                                            #[inline]
                                            fn expecting(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                                f.write_str(#expect)
                                            }

                                            #[inline]
                                            fn visit_str<E>(self, v: &str) -> ::core::result::Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <Self::Value as ValidateString>::parse_str(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_string<E>(self, v: validators_prelude::String) -> ::core::result::Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <Self::Value as ValidateString>::parse_string(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            fn visit_bool<E>(self, v: bool) -> ::core::result::Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <Self::Value as ValidateBoolean>::parse_bool(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_char<E>(self, v: char) -> ::core::result::Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <Self::Value as ValidateChar>::parse_char(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_i64<E>(self, v: i64) -> ::core::result::Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                 <Self::Value as ValidateSignedInteger>::parse_i64(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_u64<E>(self, v: u64) -> ::core::result::Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <Self::Value as ValidateUnsignedInteger>::parse_u64(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_i128<E>(self, v: i128) -> ::core::result::Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error,
                                            {
                                                <Self::Value as ValidateSignedInteger>::parse_i128(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_u128<E>(self, v: u128) -> ::core::result::Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error,
                                            {
                                                <Self::Value as ValidateUnsignedInteger>::parse_u128(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }
                                        }

                                        if #deserializer_name.is_human_readable() {
                                            #deserializer_name.deserialize_any(#visitor)
                                        } else {
                                            #deserializer_name.deserialize_bool(#visitor)
                                        }
                                    }
                                }
                            });
                }
            }

            #[cfg(feature = "rocket")]
            {
                if type_attribute.rocket_options.from_form_field {
                    crate::common::rocket::impl_from_form_field(&mut token_stream, &name);
                }

                if type_attribute.rocket_options.from_param {
                    crate::common::rocket::impl_from_param(&mut token_stream, &name, &error_path);
                }
            }

            return Ok(token_stream);
        }

        Err(panic::validator_for_specific_item(meta.path(), ITEM))
    }
}
