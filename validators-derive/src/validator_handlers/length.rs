use alloc::boxed::Box;

use crate::proc_macro::TokenStream;
use crate::syn::{Data, DeriveInput, Fields, Lit, Meta, NestedMeta, Path};

use crate::{panic, TypeEnum, Validator};

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::CollectionLength);
const VALIDATOR: Validator = Validator::length;

pub fn length_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() != 1 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                let data_type = data.fields.into_iter().next().unwrap().ty;

                let mut min: Option<usize> = None;
                let mut max: Option<usize> = None;

                let correct_usage_for_attribute = [
                    stringify!(#[validator(length(max = 100))]),
                    stringify!(#[validator(length(min = 1))]),
                    stringify!(#[validator(length(min = 5, max = 12))]),
                ];

                if let Meta::List(list) = meta {
                    let length = list.nested.len();

                    if (1..=2).contains(&length) {
                        let mut min_is_set = false;
                        let mut max_is_set = false;

                        for p in list.nested.iter() {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    if let Meta::NameValue(name_value) = meta {
                                        match &name_value.lit {
                                            Lit::Int(i) => {
                                                if let Some(ident) = meta.path().get_ident() {
                                                    if ident == "min" {
                                                        if min_is_set {
                                                            panic::reset_parameter("min");
                                                        }

                                                        min_is_set = true;

                                                        min = Some(
                                                            i.base10_digits().parse().unwrap(),
                                                        );
                                                    } else if ident == "max" {
                                                        if max_is_set {
                                                            panic::reset_parameter("max");
                                                        }

                                                        max_is_set = true;

                                                        max = Some(
                                                            i.base10_digits().parse().unwrap(),
                                                        );
                                                    } else {
                                                        panic::attribute_incorrect_format(
                                                            "length",
                                                            &correct_usage_for_attribute,
                                                        )
                                                    }
                                                } else {
                                                    panic::attribute_incorrect_format(
                                                        "length",
                                                        &correct_usage_for_attribute,
                                                    )
                                                }
                                            }
                                            _ => {
                                                panic::attribute_incorrect_format(
                                                    "length",
                                                    &correct_usage_for_attribute,
                                                )
                                            }
                                        }
                                    } else {
                                        panic::attribute_incorrect_format(
                                            "length",
                                            &correct_usage_for_attribute,
                                        )
                                    }
                                }
                                NestedMeta::Lit(_) => {
                                    panic::attribute_incorrect_format(
                                        "length",
                                        &correct_usage_for_attribute,
                                    )
                                }
                            }
                        }

                        if let Some(min) = min {
                            if let Some(max) = max {
                                if min > max {
                                    panic!("{} > {} (min > max)", min, max);
                                }
                            }
                        }
                    } else {
                        panic::attribute_incorrect_format("length", &correct_usage_for_attribute)
                    }
                } else {
                    panic::attribute_incorrect_format("length", &correct_usage_for_attribute)
                }

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::LengthError }).unwrap();

                let min_expr = {
                    match min {
                        Some(min) => {
                            quote! {
                                Some(#min)
                            }
                        }
                        None => {
                            quote! {
                                None
                            }
                        }
                    }
                };

                let max_expr = {
                    match max {
                        Some(max) => {
                            quote! {
                                Some(#max)
                            }
                        }
                        None => {
                            quote! {
                                None
                            }
                        }
                    }
                };

                let parameters_impl = quote! {
                    impl #name {
                        pub(crate) const V_LENGTH_MIN: Option<usize> = #min_expr;
                        pub(crate) const V_LENGTH_MAX: Option<usize> = #max_expr;
                    }
                };

                let handle_range = {
                    match min {
                        Some(min) => {
                            let mut token_stream = quote! {
                                let length = v.len();

                                if length < #min {
                                    return Err(#error_path::TooSmall);
                                }
                            };

                            if let Some(max) = max {
                                token_stream.extend(quote! {
                                    if length > #max {
                                        return Err(#error_path::TooLarge);
                                    }
                                });
                            }

                            token_stream
                        }
                        None => {
                            match max {
                                Some(max) => {
                                    quote! {
                                        let length = v.len();

                                        if length > #max {
                                            return Err(#error_path::TooLarge);
                                        }
                                    }
                                }
                                None => {
                                    quote! {}
                                }
                            }
                        }
                    }
                };

                let v_parse_v = quote! {
                    #[allow(clippy::ptr_arg)]
                    #[inline]
                    fn v_parse_v(v: &#data_type) -> Result<(), #error_path> {
                        #handle_range

                        Ok(())
                    }
                };

                let parse_impl = quote! {
                    impl #name {
                        #v_parse_v
                    }
                };

                let validate_collection_impl = quote! {
                    impl ValidateLength<#data_type> for #name {
                        type Error = #error_path;
                        type Output = Self;

                        #[inline]
                        fn parse_collection(v: #data_type) -> Result<Self::Output, Self::Error> {
                            Self::v_parse_v(&v)?;

                            Ok(#name(v))
                        }

                        #[allow(clippy::ptr_arg)]
                        #[inline]
                        fn validate_collection(v: &#data_type) -> Result<(), Self::Error> {
                            Self::v_parse_v(v)?;

                            Ok(())
                        }
                    }
                };

                let collection_length_impl = quote! {
                    impl CollectionLength for #name {
                         #[inline]
                        fn len(&self) -> usize {
                            CollectionLength::len(&self.0)
                        }
                    }
                };

                let serde_impl = if cfg!(feature = "serde") {
                    quote! {
                        impl validators_prelude::Serialize for #name {
                            #[inline]
                            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                where
                                    S: validators_prelude::Serializer, {
                                validators_prelude::Serialize::serialize(&self.0, serializer)
                            }
                        }

                        impl<'de> validators_prelude::Deserialize<'de> for #name {
                            #[inline]
                            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                            where
                                D: validators_prelude::Deserializer<'de>, {
                                let v: #data_type = validators_prelude::Deserialize::deserialize(deserializer)?;

                                Self::v_parse_v(&v).map_err(validators_prelude::DeError::custom)?;

                                Ok(#name(v))
                            }
                        }
                    }
                } else {
                    quote! {}
                };

                let length_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_collection_impl

                    #collection_length_impl

                    #serde_impl
                };

                length_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
