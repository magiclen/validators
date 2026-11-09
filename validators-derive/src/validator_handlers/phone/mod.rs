mod phone_attribute;

use std::str::FromStr;

use phone_attribute::PhoneAttribute;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{common::type_enum::TypeEnum, panic};

pub(crate) struct PhoneHandler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::String);
const ITEM_MAP: Struct = Struct(TypeEnum::HashMapPhoneNumber);

impl ValidatorHandler for PhoneHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = PhoneAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data
            && let Fields::Unnamed(_) = &data.fields
            && data.fields.len() == 1
        {
            let mut token_stream = proc_macro2::TokenStream::new();

            let name = ast.ident;

            let error_path: Path = syn::parse2(quote! { validators_prelude::PhoneError }).unwrap();
            let countries: Vec<_> = type_attribute
                .countries
                .iter()
                .map(|id| proc_macro2::TokenStream::from_str(id.as_ref()).unwrap())
                .collect();

            #[cfg(feature = "test")]
            {
                let size = countries.len();

                token_stream.extend(quote! {
                            impl #name {
                                pub(crate) const V_COUNTRIES: [validators_prelude::phonenumber::country::Id; #size] = [#(validators_prelude::phonenumber::country::Id::#countries, )*];
                            }
                        });
            }

            token_stream.extend(match countries.as_slice() {
                        [] => {
                            quote! {
                                impl #name {
                                    #[inline]
                                    fn v_parse_str(s: &str) -> ::core::result::Result<validators_prelude::phonenumber::PhoneNumber, #error_path> {
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
                        [country] => {
                            quote! {
                                impl #name {
                                    #[inline]
                                    fn v_parse_str(s: &str) -> ::core::result::Result<validators_prelude::phonenumber::PhoneNumber, #error_path> {
                                        let phonenumber = validators_prelude::phonenumber::parse(Some(validators_prelude::phonenumber::country::Id::#country), s)?;

                                        if let Some(id) = phonenumber.country().id() {
                                            if id == validators_prelude::phonenumber::country::Id::#country && phonenumber.is_valid() {
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
                            quote! {
                                impl #name {
                                    #[inline]
                                    fn v_parse_str(s: &str) -> ::core::result::Result<::std::collections::HashMap<validators_prelude::phonenumber::country::Id, validators_prelude::phonenumber::PhoneNumber>, #error_path> {
                                        let mut map = ::std::collections::HashMap::with_capacity(2);
                                        let mut failure = None;

                                        #(
                                            let country = validators_prelude::phonenumber::country::Id::#countries;

                                            match validators_prelude::phonenumber::parse(Some(country), s) {
                                                Ok(phonenumber) => {
                                                    if phonenumber.country().id() == Some(country) && phonenumber.is_valid() {
                                                        map.insert(country, phonenumber);
                                                    }

                                                    failure = Some(#error_path::Invalid);
                                                }
                                                Err(error) if failure.is_none() => failure = Some(error.into()),
                                                Err(_) => (),
                                            }
                                        )*

                                        if map.is_empty() {
                                            Err(failure.unwrap_or(#error_path::Invalid))
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
                            fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_str(s.into().as_str())?))
                            }

                            #[inline]
                            fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_str(s.as_ref())?))
                            }

                            #[inline]
                            fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
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
                            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                                where
                                    S: validators_prelude::serde::Serializer, {
                                validators_prelude::serde::Serialize::serialize(&self.0, serializer)
                            }
                        }
                    });
                }

                if type_attribute.serde_options.deserialize {
                    use std::fmt::Write;

                    let data_type = &data.fields.iter().next().unwrap().ty;
                    let is_valid_value = match countries.as_slice() {
                        [] => quote! { value.is_valid() },
                        [country] => quote! {
                            value.country().id() == Some(validators_prelude::phonenumber::country::Id::#country)
                                && value.is_valid()
                        },
                        _ => quote! {
                            !value.is_empty() && value.iter().all(|(&country, phonenumber)| {
                                matches!(country, #(validators_prelude::phonenumber::country::Id::#countries)|*)
                                    && phonenumber.country().id() == Some(country)
                                    && phonenumber.is_valid()
                            })
                        },
                    };

                    let expect = {
                        let mut s = String::new();

                        if type_attribute.countries.is_empty() {
                            s.push_str("an international phone number");
                        } else {
                            s.push_str("a phone number in ");

                            s.write_fmt(format_args!("{:?}", type_attribute.countries)).unwrap();
                        }

                        s
                    };

                    token_stream.extend(quote! {
                                impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                                    #[inline]
                                    fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
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
                                            fn visit_str<E>(self, v: &str) -> ::core::result::Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateString>::parse_str(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            fn visit_map<A>(self, map: A) -> ::core::result::Result<Self::Value, A::Error>
                                            where
                                                A: validators_prelude::serde::de::MapAccess<'de>, {
                                                let value: #data_type = validators_prelude::serde::Deserialize::deserialize(
                                                    validators_prelude::serde::de::value::MapAccessDeserializer::new(map),
                                                )?;

                                                if #is_valid_value {
                                                    Ok(#name(value))
                                                } else {
                                                    Err(validators_prelude::serde::de::Error::custom(#error_path::Invalid))
                                                }
                                            }
                                        }

                                        if deserializer.is_human_readable() {
                                            deserializer.deserialize_any(MyVisitor)
                                        } else {
                                            deserializer.deserialize_str(MyVisitor)
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

        if type_attribute.countries.len() > 1 {
            Err(panic::validator_for_specific_item(meta.path(), ITEM_MAP))
        } else {
            Err(panic::validator_for_specific_item(meta.path(), ITEM))
        }
    }
}
