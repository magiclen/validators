use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{attributes::basic_attribute::BasicAttribute, type_enum::TypeEnum},
    panic,
};

pub(crate) struct JsonHandler;

#[derive(Debug)]
#[allow(dead_code)] // Used for parsing
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::Serde);

impl ValidatorHandler for JsonHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        #[allow(unused_variables)]
        let type_attribute = BasicAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() == 1 {
                    let mut token_stream = proc_macro2::TokenStream::new();

                    let name = ast.ident;

                    let data_type = data.fields.into_iter().next().unwrap().ty;

                    let error_path: Path =
                        syn::parse2(quote! { validators_prelude::JsonError }).unwrap();

                    token_stream.extend(quote! {
                        impl #name {
                            fn v_parse_str(s: &str) -> Result<#data_type, #error_path> {
                                Ok(validators_prelude::serde_json::from_str(s)?)
                            }

                            #[inline]
                            fn v_parse_i128(i: i128) -> Result<#data_type, #error_path> {
                                let value = validators_prelude::serde_json::to_value(i)?;

                                Ok(validators_prelude::serde_json::from_value(value)?)
                            }

                            #[inline]
                            fn v_parse_i64(i: i64) -> Result<#data_type, #error_path> {
                                let value = validators_prelude::serde_json::to_value(i)?;

                                Ok(validators_prelude::serde_json::from_value(value)?)
                            }

                            #[inline]
                            fn v_parse_u128(u: u128) -> Result<#data_type, #error_path> {
                                let value = validators_prelude::serde_json::to_value(u)?;

                                Ok(validators_prelude::serde_json::from_value(value)?)
                            }

                            #[inline]
                            fn v_parse_u64(u: u64) -> Result<#data_type, #error_path> {
                                let value = validators_prelude::serde_json::to_value(u)?;

                                Ok(validators_prelude::serde_json::from_value(value)?)
                            }

                            #[inline]
                            fn v_parse_f64(f: f64) -> Result<#data_type, #error_path> {
                                let value = validators_prelude::serde_json::to_value(f)?;

                                Ok(validators_prelude::serde_json::from_value(value)?)
                            }

                            #[inline]
                            fn v_parse_bool(b: bool) -> Result<#data_type, #error_path> {
                                let value = validators_prelude::serde_json::to_value(b)?;

                                Ok(validators_prelude::serde_json::from_value(value)?)
                            }

                            #[inline]
                            fn v_parse_json_value(v: validators_prelude::serde_json::Value) -> Result<#data_type, #error_path> {
                                Ok(validators_prelude::serde_json::from_value(v)?)
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

                        impl ValidateSignedInteger for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_i128(i: i128) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_i128(i)?))
                            }

                            #[inline]
                            fn parse_i64(i: i64) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_i64(i)?))
                            }

                            #[inline]
                            fn validate_i128(i: i128) -> Result<(), Self::Error> {
                                Self::v_parse_i128(i)?;

                                Ok(())
                            }

                            #[inline]
                            fn validate_i64(i: i64) -> Result<(), Self::Error> {
                                Self::v_parse_i64(i)?;

                                Ok(())
                            }
                        }

                        impl ValidateUnsignedInteger for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_u128(u: u128) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_u128(u)?))
                            }

                            #[inline]
                            fn parse_u64(u: u64) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_u64(u)?))
                            }

                            #[inline]
                            fn validate_u128(u: u128) -> Result<(), Self::Error> {
                                Self::v_parse_u128(u)?;

                                Ok(())
                            }

                            #[inline]
                            fn validate_u64(u: u64) -> Result<(), Self::Error> {
                                Self::v_parse_u64(u)?;

                                Ok(())
                            }
                        }

                        impl ValidateNumber for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_f64(f: f64) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_f64(f)?))
                            }

                            #[inline]
                            fn validate_f64(f: f64) -> Result<(), Self::Error> {
                                Self::v_parse_f64(f)?;

                                Ok(())
                            }
                        }

                        impl ValidateBoolean for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_bool(b: bool) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_bool(b)?))
                            }

                            #[inline]
                            fn validate_bool(b: bool) -> Result<(), Self::Error> {
                                Self::v_parse_bool(b)?;

                                Ok(())
                            }
                        }

                        impl ValidateJsonValue for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_json_value(v: validators_prelude::serde_json::Value) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_json_value(v)?))
                            }

                            #[inline]
                            fn validate_json_value(v: validators_prelude::serde_json::Value) -> Result<(), Self::Error> {
                                Self::v_parse_json_value(v)?;

                                Ok(())
                            }
                        }
                    });

                    token_stream.extend(quote! {
                        impl ToJsonString for #name {
                            fn to_minified_json_string(&self) -> String {
                                validators_prelude::serde_json::to_string(&self.0).unwrap()
                            }

                            fn to_beautified_json_string(&self) -> String {
                                validators_prelude::serde_json::to_string_pretty(&self.0).unwrap()
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
                                        serializer.serialize_str(&ToJsonString::to_minified_json_string(self))
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
                                        let value = validators_prelude::serde::Deserialize::deserialize(deserializer)?;

                                        Ok(Self(validators_prelude::serde_json::from_value(value).map_err(validators_prelude::serde::de::Error::custom)?))
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
