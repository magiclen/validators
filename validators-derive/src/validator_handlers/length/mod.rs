mod length_attribute;

use length_attribute::LengthAttribute;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{common::type_enum::TypeEnum, panic};

pub(crate) struct LengthHandler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::CollectionLength);

impl ValidatorHandler for LengthHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        #[cfg(feature = "serde")]
        let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
        #[cfg(feature = "serde")]
        let deserializer = ident_generator.next("__ValidatorsDeserializer");
        #[cfg(feature = "serde")]
        let deserializer_name = ident_generator.next("__validators_deserializer");

        let type_attribute = LengthAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data
            && let Fields::Unnamed(_) = &data.fields
            && data.fields.len() == 1
        {
            let mut token_stream = proc_macro2::TokenStream::new();

            let data_type = data.fields.into_iter().next().unwrap().ty;

            let name = ast.ident;

            let error_path: Path = syn::parse2(quote! { validators_prelude::LengthError }).unwrap();

            #[cfg(feature = "test")]
            {
                let min_expr = {
                    match type_attribute.min {
                        Some(min) => {
                            quote! {
                                ::core::option::Option::Some(#min)
                            }
                        },
                        None => {
                            quote! {
                                ::core::option::Option::None
                            }
                        },
                    }
                };

                let max_expr = {
                    match type_attribute.max {
                        Some(max) => {
                            quote! {
                                ::core::option::Option::Some(#max)
                            }
                        },
                        None => {
                            quote! {
                                ::core::option::Option::None
                            }
                        },
                    }
                };

                token_stream.extend(quote! {
                    impl #name {
                        pub(crate) const V_LENGTH_MIN: ::core::option::Option<usize> = #min_expr;
                        pub(crate) const V_LENGTH_MAX: ::core::option::Option<usize> = #max_expr;
                    }
                });
            }

            let handle_range = {
                match type_attribute.min {
                    Some(min) => {
                        if let Some(max) = type_attribute.max {
                            if min == max {
                                quote! {
                                    let length = CollectionLength::len(v);

                                    match ::core::cmp::Ord::cmp(&length, &#min) {
                                        ::core::cmp::Ordering::Equal => (),
                                        ::core::cmp::Ordering::Less => return ::core::result::Result::Err(#error_path::TooSmall),
                                        ::core::cmp::Ordering::Greater => return ::core::result::Result::Err(#error_path::TooLarge),
                                    }
                                }
                            } else {
                                quote! {
                                    let length = CollectionLength::len(v);

                                    if length < #min {
                                        return ::core::result::Result::Err(#error_path::TooSmall);
                                    }

                                    if length > #max {
                                        return ::core::result::Result::Err(#error_path::TooLarge);
                                    }
                                }
                            }
                        } else {
                            quote! {
                                let length = CollectionLength::len(v);

                                if length < #min {
                                    return ::core::result::Result::Err(#error_path::TooSmall);
                                }
                            }
                        }
                    },
                    None => match type_attribute.max {
                        Some(max) => {
                            quote! {
                                let length = CollectionLength::len(v);

                                if length > #max {
                                    return ::core::result::Result::Err(#error_path::TooLarge);
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
                    fn v_parse_v(v: &#data_type) -> ::core::result::Result<(), #error_path> {
                        #handle_range

                        ::core::result::Result::Ok(())
                    }
                }
            });

            token_stream.extend(quote! {
                impl ValidateLength<#data_type> for #name {
                    type Error = #error_path;

                    #[inline]
                    fn parse_collection(v: #data_type) -> ::core::result::Result<Self, Self::Error> {
                        Self::v_parse_v(&v)?;

                        ::core::result::Result::Ok(Self(v))
                    }

                    #[allow(clippy::ptr_arg)]
                    #[inline]
                    fn validate_collection(v: &#data_type) -> ::core::result::Result<(), Self::Error> {
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
                    token_stream.extend(quote! {
                                impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                                    #[inline]
                                    fn deserialize<#deserializer>(#deserializer_name: #deserializer) -> ::core::result::Result<Self, #deserializer::Error>
                                    where
                                        #deserializer: validators_prelude::serde::Deserializer<'de>, {
                                        let v: #data_type = validators_prelude::serde::Deserialize::deserialize(#deserializer_name)?;

                                        Self::v_parse_v(&v).map_err(validators_prelude::serde::de::Error::custom)?;

                                        ::core::result::Result::Ok(Self(v))
                                    }
                                }
                            });
                }
            }

            return Ok(token_stream);
        }

        Err(panic::validator_for_specific_item(meta.path(), ITEM))
    }
}
