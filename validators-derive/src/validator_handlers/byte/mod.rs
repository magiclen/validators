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

#[derive(Debug)]
#[allow(dead_code)] // Used for parsing
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::Bit);

impl ValidatorHandler for ByteHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = ByteAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() == 1 {
                    let mut token_stream = proc_macro2::TokenStream::new();

                    let name = ast.ident;

                    let error_path: Path =
                        syn::parse2(quote! { validators_prelude::ByteError }).unwrap();

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

                    let handle_range = if equal {
                        let min = type_attribute.range.min.unwrap();

                        quote! {
                            match ::core::cmp::PartialOrd::partial_cmp(&v, &#min) {
                                Some(::core::cmp::Ordering::Equal) => (),
                                Some(::core::cmp::Ordering::Less) => return Err(#error_path::TooSmall),
                                Some(::core::cmp::Ordering::Greater) => return Err(#error_path::TooLarge),
                                None => unreachable!(),
                            }
                        }
                    } else {
                        let mut token_stream = proc_macro2::TokenStream::new();

                        if let Some(min) = type_attribute.range.min {
                            token_stream.extend(quote! {
                                if v < #min {
                                    return Err(#error_path::TooSmall);
                                }
                            });
                        }

                        if let Some(max) = type_attribute.range.max {
                            token_stream.extend(if type_attribute.range.inclusive {
                                quote! {
                                    if v > #max {
                                        return Err(#error_path::TooLarge);
                                    }
                                }
                            } else {
                                quote! {
                                    if v >= #max {
                                        return Err(#error_path::TooLarge);
                                    }
                                }
                            });
                        }

                        token_stream
                    };

                    let ignore_case = type_attribute.ignore_case;

                    token_stream.extend(quote! {
                        impl #name {
                            fn v_parse_str(s: &str) -> Result<validators_prelude::byte_unit::Byte, #error_path> {
                                let v = validators_prelude::byte_unit::Byte::parse_str(s, #ignore_case)?;

                                Self::v_parse_v(v)?;

                                Ok(v)
                            }

                            fn v_parse_v(v: validators_prelude::byte_unit::Byte) -> Result<(), #error_path> {
                                #handle_range

                                Ok(())
                            }
                        }
                    });

                    token_stream.extend(quote! {
                        impl ValidateString for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_string<S: Into<validators_prelude::String>>(s: S) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_str(s.into().as_str())?))
                            }

                            #[inline]
                            fn parse_str<S: AsRef<str>>(s: S) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_str(s.as_ref())?))
                            }

                            #[inline]
                            fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error> {
                                Self::v_parse_str(s.as_ref())?;

                                Ok(())
                            }
                        }
                    });

                    token_stream.extend(quote! {
                        impl ValidateUnsignedInteger for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_u128(u: u128) -> Result<Self, Self::Error> {
                                let v = validators_prelude::byte_unit::Byte::from_u128(u).ok_or(#error_path::TooLarge)?;

                                Self::v_parse_v(v)?;

                                Ok(Self(v))
                            }

                            #[inline]
                            fn validate_u128(u: u128) -> Result<(), Self::Error> {
                                let v = validators_prelude::byte_unit::Byte::from_u128(u).ok_or(#error_path::TooLarge)?;

                                Self::v_parse_v(v)?;

                                Ok(())
                            }

                            #[inline]
                            fn parse_u64(u: u64) -> Result<Self, Self::Error> {
                                let v = validators_prelude::byte_unit::Byte::from_u64(u);

                                Self::v_parse_v(v)?;

                                Ok(Self(v))
                            }

                            #[inline]
                            fn validate_u64(u: u64) -> Result<(), Self::Error> {
                                let v = validators_prelude::byte_unit::Byte::from_u64(u);

                                Self::v_parse_v(v)?;

                                Ok(())
                            }
                        }
                    });

                    #[cfg(feature = "serde")]
                    {
                        if type_attribute.serde_options.serialize {
                            token_stream.extend(quote! {
                                impl validators_prelude::serde::Serialize for #name {
                                    #[inline]
                                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                        where
                                            S: validators_prelude::serde::Serializer, {
                                        validators_prelude::serde::Serialize::serialize(&self.0, serializer)
                                    }
                                }
                            });
                        }

                        if type_attribute.serde_options.deserialize {
                            let expect = "a string such as \"123\", \"123Kib\", \"50.84 Mb\", or \
                                          a positive integer";

                            token_stream.extend(quote! {
                                impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                                    #[inline]
                                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                                    where
                                        D: validators_prelude::serde::Deserializer<'de>, {
                                        struct MyVisitor;

                                        impl<'de> validators_prelude::serde::de::Visitor<'de> for MyVisitor {
                                            type Value = #name;

                                            #[inline]
                                            fn expecting(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                                f.write_str(#expect)
                                            }

                                            #[inline]
                                            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateString>::parse_str(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateUnsignedInteger>::parse_u128(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }
                                        }

                                        if deserializer.is_human_readable() {
                                            deserializer.deserialize_str(MyVisitor)
                                        } else {
                                            deserializer.deserialize_u128(MyVisitor)
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
                            crate::common::rocket::impl_from_param(
                                &mut token_stream,
                                &name,
                                &error_path,
                            );
                        }
                    }

                    return Ok(token_stream);
                }
            }
        }

        Err(panic::validator_for_specific_item(meta.path().get_ident().unwrap(), ITEM))
    }
}
