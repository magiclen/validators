mod byte_attribute;

use byte_attribute::ByteAttribute;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{range::range_equal, type_enum::TypeEnum},
    panic,
};

pub(crate) struct ByteHandler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::Byte);

impl ValidatorHandler for ByteHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        #[cfg(feature = "serde")]
        let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
        #[cfg(feature = "serde")]
        let deserializer = ident_generator.next("__ValidatorsDeserializer");
        #[cfg(feature = "serde")]
        let deserializer_name = ident_generator.next("__validators_deserializer");

        let type_attribute = ByteAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data
            && let Fields::Unnamed(_) = &data.fields
            && data.fields.len() == 1
        {
            let mut token_stream = proc_macro2::TokenStream::new();

            let name = ast.ident;

            let error_path: Path = syn::parse2(quote! { validators_prelude::ByteError }).unwrap();

            #[cfg(feature = "test")]
            {
                let v_range = &type_attribute.range;
                let v_ignore_case = &type_attribute.ignore_case;

                token_stream.extend(quote! {
                    impl #name {
                        pub(crate) const V_RANGE: validators_prelude::RangeOption<u128> = #v_range;
                        pub(crate) const V_IGNORE_CASE: bool = #v_ignore_case;
                    }
                });
            }

            let equal = range_equal(
                type_attribute.range.min,
                type_attribute.range.max,
                type_attribute.range.inclusive,
            );

            // Compare `u128` values, so older `byte-unit` versions without `PartialOrd<u128>` also work.
            let handle_range = if equal {
                let min = type_attribute.range.min.unwrap();

                quote! {
                    match ::core::cmp::Ord::cmp(&v.as_u128(), &#min) {
                        ::core::cmp::Ordering::Equal => (),
                        ::core::cmp::Ordering::Less => return ::core::result::Result::Err(#error_path::TooSmall),
                        ::core::cmp::Ordering::Greater => return ::core::result::Result::Err(#error_path::TooLarge),
                    }
                }
            } else {
                let mut token_stream = proc_macro2::TokenStream::new();

                if let Some(min) = type_attribute.range.min {
                    token_stream.extend(quote! {
                        if v.as_u128() < #min {
                            return ::core::result::Result::Err(#error_path::TooSmall);
                        }
                    });
                }

                if let Some(max) = type_attribute.range.max {
                    token_stream.extend(if type_attribute.range.inclusive {
                        quote! {
                            if v.as_u128() > #max {
                                return ::core::result::Result::Err(#error_path::TooLarge);
                            }
                        }
                    } else {
                        quote! {
                            if v.as_u128() >= #max {
                                return ::core::result::Result::Err(#error_path::TooLarge);
                            }
                        }
                    });
                }

                token_stream
            };

            let ignore_case = type_attribute.ignore_case;

            token_stream.extend(quote! {
                        impl #name {
                            fn v_parse_str(s: &str) -> ::core::result::Result<validators_prelude::byte_unit::Byte, #error_path> {
                                let v = validators_prelude::byte_unit::Byte::parse_str(s, #ignore_case)?;

                                Self::v_parse_v(v)?;

                                ::core::result::Result::Ok(v)
                            }

                            fn v_parse_v(v: validators_prelude::byte_unit::Byte) -> ::core::result::Result<(), #error_path> {
                                #handle_range

                                ::core::result::Result::Ok(())
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
                    });

            token_stream.extend(quote! {
                        impl ValidateUnsignedInteger for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_u128(u: u128) -> ::core::result::Result<Self, Self::Error> {
                                let v = validators_prelude::byte_unit::Byte::from_u128(u).ok_or(#error_path::TooLarge)?;

                                Self::v_parse_v(v)?;

                                ::core::result::Result::Ok(Self(v))
                            }

                            #[inline]
                            fn validate_u128(u: u128) -> ::core::result::Result<(), Self::Error> {
                                let v = validators_prelude::byte_unit::Byte::from_u128(u).ok_or(#error_path::TooLarge)?;

                                Self::v_parse_v(v)?;

                                ::core::result::Result::Ok(())
                            }

                            #[inline]
                            fn parse_u64(u: u64) -> ::core::result::Result<Self, Self::Error> {
                                let v = validators_prelude::byte_unit::Byte::from_u64(u);

                                Self::v_parse_v(v)?;

                                ::core::result::Result::Ok(Self(v))
                            }

                            #[inline]
                            fn validate_u64(u: u64) -> ::core::result::Result<(), Self::Error> {
                                let v = validators_prelude::byte_unit::Byte::from_u64(u);

                                Self::v_parse_v(v)?;

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
                                validators_prelude::serde::Serialize::serialize(&self.0, serializer)
                            }
                        }
                    });
                }

                if type_attribute.serde_options.deserialize {
                    let expect =
                        "a string such as \"123\", \"123KiB\", \"50.84 MB\", or a positive integer";

                    token_stream.extend(quote! {
                                impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                                    #[inline]
                                    fn deserialize<#deserializer>(#deserializer_name: #deserializer) -> ::core::result::Result<Self, #deserializer::Error>
                                    where
                                        #deserializer: validators_prelude::serde::Deserializer<'de>, {
                                        validators_prelude::deserialize_byte_unit(#deserializer_name, #expect, #error_path::TooSmall)?
                                            .map_err(validators_prelude::serde::de::Error::custom)
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
