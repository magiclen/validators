extern crate proc_macro2;

extern crate phonenumber;

use core::fmt::Write;
use core::str::FromStr;

use alloc::boxed::Box;

use std::collections::HashSet;

use crate::proc_macro::TokenStream;
use crate::syn::{Data, DeriveInput, Fields, Index, Meta, NestedMeta, Path};

use crate::{panic, TypeEnum, Validator};

use proc_macro2::TokenStream as TokenStream2;

use phonenumber::country::Id;

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::Serde);
const ITEM_MAP: Struct = Struct(TypeEnum::HashMapPhoneNumber);
const VALIDATOR: Validator = Validator::phone;

pub fn phone_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                let mut countries = HashSet::new();

                let correct_usage_for_attribute = [
                    stringify!(#[validator(phone)]),
                    stringify!(#[validator(phone(TW))]),
                    stringify!(#[validator(phone(US, TW))]),
                ];

                match meta {
                    Meta::Path(_) => (),
                    Meta::List(list) => {
                        for p in list.nested.iter() {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    if let Meta::Path(path) = meta {
                                        if let Some(ident) = path.get_ident() {
                                            let country = ident.to_string();

                                            let id = Id::from_str(country.as_str()).unwrap();

                                            if !countries.insert(id) {
                                                panic!("the country `{}` of the phone validator is repeated", id.as_ref());
                                            }
                                        } else {
                                            panic::attribute_incorrect_format(
                                                "phone",
                                                &correct_usage_for_attribute,
                                            )
                                        }
                                    } else {
                                        panic::attribute_incorrect_format(
                                            "phone",
                                            &correct_usage_for_attribute,
                                        )
                                    }
                                }
                                NestedMeta::Lit(_) => {
                                    panic::attribute_incorrect_format(
                                        "phone",
                                        &correct_usage_for_attribute,
                                    )
                                }
                            }
                        }
                    }
                    _ => panic::attribute_incorrect_format("phone", &correct_usage_for_attribute),
                }

                let countries_length = countries.len();

                if data.fields.len() != 1 {
                    if countries_length > 1 {
                        panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM_MAP));
                    } else {
                        panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                    }
                }

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::PhoneError }).unwrap();

                let (parameters_impl, v_parse_str) = {
                    match countries_length {
                        0 => {
                            (quote! {}, quote! {
                                #[inline]
                                fn v_parse_str(s: &str) -> Result<validators_prelude::phonenumber::PhoneNumber, #error_path> {
                                    let phonenumber = validators_prelude::phonenumber::parse(None, s)?;

                                    if phonenumber.is_valid() {
                                        Ok(phonenumber)
                                    } else {
                                        Err(#error_path::Invalid)
                                    }
                                }
                            })
                        }
                        1 => {
                            let c =
                                TokenStream2::from_str(countries.iter().next().unwrap().as_ref())
                                    .unwrap();

                            (
                                quote! {
                                    impl #name {
                                        pub(crate) const V_COUNTRY: validators_prelude::phonenumber::country::Id = validators_prelude::phonenumber::country::Id::#c;
                                    }
                                },
                                quote! {
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
                                },
                            )
                        }
                        _ => {
                            let countries: Vec<_> = countries
                                .iter()
                                .map(|id| TokenStream2::from_str(id.as_ref()).unwrap())
                                .collect();
                            let index = Index::from(countries_length);

                            let parameters_impl = quote! {
                                impl #name {
                                    pub(crate) const V_COUNTRIES: [validators_prelude::phonenumber::country::Id; #index] = [#(validators_prelude::phonenumber::country::Id::#countries, )*];
                                }
                            };

                            (parameters_impl, quote! {
                                #[inline]
                                fn v_parse_str(s: &str) -> Result<::std::collections::HashMap<validators_prelude::phonenumber::country::Id, validators_prelude::phonenumber::PhoneNumber>, #error_path> {
                                    let mut map = ::std::collections::HashMap::with_capacity(2);

                                    #(
                                        let phonenumber = validators_prelude::phonenumber::parse(Some(validators_prelude::phonenumber::country::Id::#countries), s)?;

                                        if let Some(id) = phonenumber.country().id() {
                                            if id == validators_prelude::phonenumber::country::Id::#countries && phonenumber.is_valid() {
                                                map.insert(validators_prelude::phonenumber::country::Id::#countries, phonenumber);
                                            }
                                        }
                                    )*

                                    if map.is_empty() {
                                        Err(#error_path::Invalid)
                                    } else {
                                        Ok(map)
                                    }
                                }
                            })
                        }
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
                    let expect = {
                        let mut s = String::new();

                        if countries_length == 0 {
                            s.push_str("an international phone number");
                        } else {
                            s.push_str("a phone number in ");

                            s.write_fmt(format_args!("{:?}", countries)).unwrap();
                        }

                        s
                    };

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
                                struct ValidatingVisitor;

                                impl<'de> validators_prelude::Visitor<'de> for ValidatingVisitor {
                                    type Value = #name;

                                    #[inline]
                                    fn expecting(&self, f: &mut validators_prelude::Formatter) -> Result<(), validators_prelude::fmt::Error> {
                                        f.write_str(#expect)
                                    }

                                    #[inline]
                                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateString>::parse_str(v).map_err(validators_prelude::DeError::custom)
                                    }
                                }

                                deserializer.deserialize_str(ValidatingVisitor)
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

                let phone_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #serde_impl

                    #rocket_impl
                };

                phone_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
