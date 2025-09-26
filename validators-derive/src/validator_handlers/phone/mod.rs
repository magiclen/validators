mod phone_attribute;

use std::str::FromStr;

use phone_attribute::PhoneAttribute;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{common::type_enum::TypeEnum, panic};

pub(crate) struct PhoneHandler;

#[derive(Debug)]
#[allow(dead_code)] // Used for parsing
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::String);
const ITEM_MAP: Struct = Struct(TypeEnum::HashMapPhoneNumber);

impl ValidatorHandler for PhoneHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = PhoneAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() == 1 {
                    let mut token_stream = proc_macro2::TokenStream::new();

                    let name = ast.ident;

                    let error_path: Path =
                        syn::parse2(quote! { validators_prelude::PhoneError }).unwrap();

                    #[cfg(feature = "test")]
                    {
                        let c: Vec<proc_macro2::TokenStream> = type_attribute
                            .countries
                            .iter()
                            .map(|id| proc_macro2::TokenStream::from_str(id.as_ref()).unwrap())
                            .collect();
                        let size = c.len();

                        token_stream.extend(quote! {
                            impl #name {
                                pub(crate) const V_COUNTRIES: [validators_prelude::phonenumber::country::Id; #size] = [#(validators_prelude::phonenumber::country::Id::#c, )*];
                            }
                        });
                    }

                    token_stream.extend(match type_attribute.countries.len() {
                        0 => {
                            quote! {
                                impl #name {
                                    #[inline]
                                    fn v_parse_str(s: &str) -> Result<validators_prelude::phonenumber::PhoneNumber, #error_path> {
                                        let phonenumber = validators_prelude::phonenumber::parse(None, s)?;

                                        if phonenumber.is_valid() {
                                            Ok(phonenumber)
                                        } else {
                                            Err(#error_path::Invalid)
                                        }
                                    }
                                }
                            }
                        },
                        1 => {
                            let c = proc_macro2::TokenStream::from_str(type_attribute.countries.iter().next().unwrap().as_ref())
                                    .unwrap();

                            quote! {
                                impl #name {
                                    #[inline]
                                    fn v_parse_str(s: &str) -> Result<validators_prelude::phonenumber::PhoneNumber, #error_path> {
                                        let phonenumber = validators_prelude::phonenumber::parse(Some(validators_prelude::phonenumber::country::Id::#c), s)?;

                                        if let Some(id) = phonenumber.country().id() {
                                            if id == validators_prelude::phonenumber::country::Id::#c && phonenumber.is_valid() {
                                                Ok(phonenumber)
                                            } else {
                                                Err(#error_path::Invalid)
                                            }
                                        } else {
                                            Err(#error_path::Invalid)
                                        }
                                    }
                                }
                            }
                        },
                        _ => {
                            let c: Vec<proc_macro2::TokenStream> = type_attribute.countries
                                .iter()
                                .map(|id| proc_macro2::TokenStream::from_str(id.as_ref()).unwrap())
                                .collect();

                            quote! {
                                impl #name {
                                    #[inline]
                                    fn v_parse_str(s: &str) -> Result<::std::collections::HashMap<validators_prelude::phonenumber::country::Id, validators_prelude::phonenumber::PhoneNumber>, #error_path> {
                                        let mut map = ::std::collections::HashMap::with_capacity(2);

                                        #(
                                            let phonenumber = validators_prelude::phonenumber::parse(Some(validators_prelude::phonenumber::country::Id::#c), s)?;

                                            if let Some(id) = phonenumber.country().id() {
                                                if id == validators_prelude::phonenumber::country::Id::#c && phonenumber.is_valid() {
                                                    map.insert(validators_prelude::phonenumber::country::Id::#c, phonenumber);
                                                }
                                            }
                                        )*

                                        if map.is_empty() {
                                            Err(#error_path::Invalid)
                                        } else {
                                            Ok(map)
                                        }
                                    }
                                }
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
                            use std::fmt::Write;

                            let expect = {
                                let mut s = String::new();

                                if type_attribute.countries.is_empty() {
                                    s.push_str("an international phone number");
                                } else {
                                    s.push_str("a phone number in ");

                                    s.write_fmt(format_args!("{:?}", type_attribute.countries))
                                        .unwrap();
                                }

                                s
                            };

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
                                        }

                                        deserializer.deserialize_str(MyVisitor)
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

        if type_attribute.countries.len() > 1 {
            Err(panic::validator_for_specific_item(meta.path().get_ident().unwrap(), ITEM_MAP))
        } else {
            Err(panic::validator_for_specific_item(meta.path().get_ident().unwrap(), ITEM))
        }
    }
}
