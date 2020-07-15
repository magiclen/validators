use alloc::boxed::Box;

use crate::proc_macro::TokenStream;
use crate::syn::{Data, DeriveInput, Fields, Meta, Path};

use crate::{panic, TypeEnum, Validator};

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::Serde);
const VALIDATOR: Validator = Validator::json;

pub fn json_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() != 1 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                let data_type = data.fields.into_iter().next().unwrap().ty;

                let correct_usage_for_attribute = [stringify!(#[validator(json)])];

                match meta {
                    Meta::Path(_) => (),
                    _ => panic::attribute_incorrect_format("json", &correct_usage_for_attribute),
                }

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::JSONError }).unwrap();

                let parameters_impl = quote! {};

                let v_parse_str = quote! {
                    #[inline]
                    pub(crate) fn v_parse_str(s: &str) -> Result<#data_type, #error_path> {
                        Ok(validators_prelude::serde_json::from_str(s)?)
                    }
                };

                let v_parse_i128 = quote! {
                    #[inline]
                    fn v_parse_i128(i: i128) -> Result<#data_type, #error_path> {
                        let value = validators_prelude::serde_json::to_value(i)?;

                        Ok(validators_prelude::serde_json::from_value(value)?)
                    }
                };

                let v_parse_i64 = quote! {
                    #[inline]
                    fn v_parse_i64(i: i64) -> Result<#data_type, #error_path> {
                        let value = validators_prelude::serde_json::to_value(i)?;

                        Ok(validators_prelude::serde_json::from_value(value)?)
                    }
                };

                let v_parse_u128 = quote! {
                    #[inline]
                    fn v_parse_u128(u: u128) -> Result<#data_type, #error_path> {
                        let value = validators_prelude::serde_json::to_value(u)?;

                        Ok(validators_prelude::serde_json::from_value(value)?)
                    }
                };

                let v_parse_u64 = quote! {
                    #[inline]
                    fn v_parse_u64(u: u64) -> Result<#data_type, #error_path> {
                        let value = validators_prelude::serde_json::to_value(u)?;

                        Ok(validators_prelude::serde_json::from_value(value)?)
                    }
                };

                let v_parse_f64 = quote! {
                    #[inline]
                    fn v_parse_f64(f: f64) -> Result<#data_type, #error_path> {
                        let value = validators_prelude::serde_json::to_value(f)?;

                        Ok(validators_prelude::serde_json::from_value(value)?)
                    }
                };

                let v_parse_bool = quote! {
                    #[inline]
                    fn v_parse_bool(b: bool) -> Result<#data_type, #error_path> {
                        let value = validators_prelude::serde_json::to_value(b)?;

                        Ok(validators_prelude::serde_json::from_value(value)?)
                    }
                };

                let parse_impl = quote! {
                    impl #name {
                        #v_parse_str

                        #v_parse_i128
                        #v_parse_i64

                        #v_parse_u128
                        #v_parse_u64

                        #v_parse_f64

                        #v_parse_bool
                    }
                };

                let other_functions = quote! {
                    impl #name {
                        #[inline]
                        pub fn to_minfied_json_string(&self) -> String {
                            validators_prelude::serde_json::to_string(&self.0).unwrap()
                        }

                        #[inline]
                        pub fn to_beautified_json_string(&self) -> String {
                            validators_prelude::serde_json::to_string_pretty(&self.0).unwrap()
                        }
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

                let validate_signed_integer_impl = quote! {
                    impl ValidateSignedInteger for #name {
                        type Error = #error_path;
                        type Output = Self;

                        #[inline]
                        fn parse_i128(i: i128) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_i128(i)?))
                        }

                        #[inline]
                        fn parse_i64(i: i64) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_i64(i)?))
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
                };

                let validate_unsigned_integer_impl = quote! {
                    impl ValidateUnsignedInteger for #name {
                        type Error = #error_path;
                        type Output = Self;

                        #[inline]
                        fn parse_u128(u: u128) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_u128(u)?))
                        }

                        #[inline]
                        fn parse_u64(u: u64) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_u64(u)?))
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
                };

                let validate_number_impl = quote! {
                    impl ValidateNumber for #name {
                        type Error = #error_path;
                        type Output = Self;

                        #[inline]
                        fn parse_f64(f: f64) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_f64(f)?))
                        }

                        #[inline]
                        fn validate_f64(f: f64) -> Result<(), Self::Error> {
                            Self::v_parse_f64(f)?;

                            Ok(())
                        }
                    }
                };

                let validate_boolean_impl = quote! {
                    impl ValidateBoolean for #name {
                        type Error = #error_path;
                        type Output = Self;

                        #[inline]
                        fn parse_bool(b: bool) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_bool(b)?))
                        }

                        #[inline]
                        fn validate_bool(b: bool) -> Result<(), Self::Error> {
                            Self::v_parse_bool(b)?;

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
                                serializer.serialize_str(&self.to_minfied_json_string())
                            }
                        }

                        impl<'de> validators_prelude::Deserialize<'de> for #name {
                            #[inline]
                            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                            where
                                D: validators_prelude::Deserializer<'de>, {
                                let value = validators_prelude::serde_json::Value::deserialize(deserializer)?;

                                Ok(#name(validators_prelude::serde_json::from_value(value).map_err(validators_prelude::DeError::custom)?))
                            }
                        }
                    }
                } else {
                    quote! {}
                };

                let rocket_impl = if cfg!(feature = "rocket") {
                    quote! {
                        impl<'a> validators_prelude::FromFormValue<'a> for #name {
                            type Error = #error_path;

                            #[inline]
                            fn from_form_value(v: &'a validators_prelude::RawStr) -> Result<Self, Self::Error> {
                                <#name as ValidateString>::parse_str(v)
                            }
                        }

                        impl<'a> validators_prelude::FromParam<'a> for #name {
                            type Error = #error_path;

                            #[inline]
                            fn from_param(v: &'a validators_prelude::RawStr) -> Result<Self, Self::Error> {
                                <#name as ValidateString>::parse_str(v)
                            }
                        }
                    }
                } else {
                    quote! {}
                };

                let json_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #validate_signed_integer_impl

                    #validate_unsigned_integer_impl

                    #validate_number_impl

                    #validate_boolean_impl

                    #other_functions

                    #serde_impl

                    #rocket_impl
                };

                json_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
