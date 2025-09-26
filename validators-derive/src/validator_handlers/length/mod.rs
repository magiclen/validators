mod length_attribute;

use length_attribute::LengthAttribute;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{common::type_enum::TypeEnum, panic};

pub(crate) struct LengthHandler;

#[derive(Debug)]
#[allow(dead_code)] // Used for parsing
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::CollectionLength);

impl ValidatorHandler for LengthHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = LengthAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() == 1 {
                    let mut token_stream = proc_macro2::TokenStream::new();

                    let data_type = data.fields.into_iter().next().unwrap().ty;

                    let name = ast.ident;

                    let error_path: Path =
                        syn::parse2(quote! { validators_prelude::LengthError }).unwrap();

                    #[cfg(feature = "test")]
                    {
                        let min_expr = {
                            match type_attribute.min {
                                Some(min) => {
                                    quote! {
                                        Some(#min)
                                    }
                                },
                                None => {
                                    quote! {
                                        None
                                    }
                                },
                            }
                        };

                        let max_expr = {
                            match type_attribute.max {
                                Some(max) => {
                                    quote! {
                                        Some(#max)
                                    }
                                },
                                None => {
                                    quote! {
                                        None
                                    }
                                },
                            }
                        };

                        token_stream.extend(quote! {
                            impl #name {
                                pub(crate) const V_LENGTH_MIN: Option<usize> = #min_expr;
                                pub(crate) const V_LENGTH_MAX: Option<usize> = #max_expr;
                            }
                        });
                    }

                    let handle_range = {
                        match type_attribute.min {
                            Some(min) => {
                                if let Some(max) = type_attribute.max {
                                    if min == max {
                                        quote! {
                                            match ::core::cmp::Ord::cmp(&length, &#min) {
                                                ::core::cmp::Ordering::Equal => (),
                                                ::core::cmp::Ordering::Less => return Err(#error_path::TooSmall),
                                                ::core::cmp::Ordering::Greater => return Err(#error_path::TooLarge),
                                            }
                                        }
                                    } else {
                                        quote! {
                                            let length = CollectionLength::len(v);

                                            if length < #min {
                                                return Err(#error_path::TooSmall);
                                            }

                                            if length > #max {
                                                return Err(#error_path::TooLarge);
                                            }
                                        }
                                    }
                                } else {
                                    quote! {
                                        let length = CollectionLength::len(v);

                                        if length < #min {
                                            return Err(#error_path::TooSmall);
                                        }
                                    }
                                }
                            },
                            None => match type_attribute.max {
                                Some(max) => {
                                    quote! {
                                        let length = CollectionLength::len(v);

                                        if length > #max {
                                            return Err(#error_path::TooLarge);
                                        }
                                    }
                                },
                                None => {
                                    quote! {}
                                },
                            },
                        }
                    };

                    token_stream.extend(quote! {
                        impl #name {
                            #[allow(clippy::ptr_arg)]
                            #[inline]
                            fn v_parse_v(v: &#data_type) -> Result<(), #error_path> {
                                #handle_range

                                Ok(())
                            }
                        }
                    });

                    token_stream.extend(quote! {
                        impl ValidateLength<#data_type> for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_collection(v: #data_type) -> Result<Self, Self::Error> {
                                Self::v_parse_v(&v)?;

                                Ok(Self(v))
                            }

                            #[allow(clippy::ptr_arg)]
                            #[inline]
                            fn validate_collection(v: &#data_type) -> Result<(), Self::Error> {
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
                            token_stream.extend(quote! {
                                impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                                    #[inline]
                                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                                    where
                                        D: validators_prelude::serde::Deserializer<'de>, {
                                        let v: #data_type = validators_prelude::serde::Deserialize::deserialize(deserializer)?;

                                        Self::v_parse_v(&v).map_err(validators_prelude::serde::de::Error::custom)?;

                                        Ok(Self(v))
                                    }
                                }
                            });
                        }
                    }

                    return Ok(token_stream);
                }
            }
        }

        Err(panic::validator_for_specific_item(meta.path().get_ident().unwrap(), ITEM))
    }
}
