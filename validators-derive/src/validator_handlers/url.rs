use alloc::boxed::Box;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use crate::{panic, TypeEnum, Validator};

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::Url);
const VALIDATOR: Validator = Validator::url;

pub fn url_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() != 1 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                let correct_usage_for_attribute = [stringify!(#[validator(url)])];

                match meta {
                    Meta::Path(_) => (),
                    _ => panic::attribute_incorrect_format("url", &correct_usage_for_attribute),
                }

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::URLError }).unwrap();

                let parameters_impl = quote! {};

                let v_parse_str = quote! {
                    #[inline]
                    fn v_parse_str(s: &str) -> Result<validators_prelude::url::Url, #error_path> {
                        Ok(validators_prelude::url::Url::parse(s)?)
                    }
                };

                let parse_impl = quote! {
                    impl #name {
                        #v_parse_str
                    }
                };

                let validate_string_impl = quote! {
                    impl ValidateString for #name {
                        type Error = #error_path;
                        type Output = Self;

                        #[inline]
                        fn parse_string<S: Into<validators_prelude::String>>(s: S) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_str(s.into().as_str())?))
                        }

                        #[inline]
                        fn parse_str<S: AsRef<str>>(s: S) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_str(s.as_ref())?))
                        }

                        #[inline]
                        fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error> {
                            Self::v_parse_str(s.as_ref())?;

                            Ok(())
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
                                Ok(#name(validators_prelude::url::Url::deserialize(deserializer)?))
                            }
                        }
                    }
                } else {
                    quote! {}
                };

                let rocket_impl = if cfg!(feature = "rocket") {
                    quote! {
                        impl<'a> validators_prelude::FromFormField<'a> for #name {
                            #[inline]
                            fn from_value(v: validators_prelude::ValueField<'a>) -> validators_prelude::FormResult<'a, Self> {
                                Ok(<#name as ValidateString>::parse_str(v.value).map_err(validators_prelude::FormError::custom)?)
                            }
                        }

                        impl<'a> validators_prelude::FromParam<'a> for #name {
                            type Error = #error_path;

                            #[inline]
                            fn from_param(v: &'a str) -> Result<Self, Self::Error> {
                                <#name as ValidateString>::parse_str(v)
                            }
                        }
                    }
                } else {
                    quote! {}
                };

                let url_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #serde_impl

                    #rocket_impl
                };

                url_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        },
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
