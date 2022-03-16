use alloc::boxed::Box;
use alloc::string::String;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit, Meta, NestedMeta, Path};

use crate::{panic, TypeEnum, Validator};

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::String);
const VALIDATOR: Validator = Validator::regex;

enum Regex {
    String(String),
    Ref(Box<Path>),
}

pub fn regex_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() != 1 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                let regex;

                let correct_usage_for_attribute = [
                    stringify!(#[validator(regex("[0-9a-fA-F]"))]),
                    stringify!(#[validator(regex(REGEX_VARIABLE))]),
                ];

                match meta {
                    Meta::List(list) => {
                        if list.nested.len() != 1 {
                            panic::attribute_incorrect_format(
                                "regex",
                                &correct_usage_for_attribute,
                            );
                        }

                        let p = list.nested.into_iter().next().unwrap();

                        match p {
                            NestedMeta::Meta(meta) => {
                                if let Meta::Path(path) = meta {
                                    regex = Regex::Ref(Box::new(path));
                                } else {
                                    panic::attribute_incorrect_format(
                                        "regex",
                                        &correct_usage_for_attribute,
                                    );
                                }
                            }
                            NestedMeta::Lit(lit) => {
                                if let Lit::Str(lit) = lit {
                                    let s = lit.value();

                                    regex_dep::Regex::new(&s).unwrap();

                                    regex = Regex::String(s);
                                } else {
                                    panic::attribute_incorrect_format(
                                        "regex",
                                        &correct_usage_for_attribute,
                                    );
                                }
                            }
                        }
                    }
                    _ => panic::attribute_incorrect_format("regex", &correct_usage_for_attribute),
                }

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::RegexError }).unwrap();

                let parameters_impl = {
                    match &regex {
                        Regex::String(regex) => {
                            quote! {
                                impl #name {
                                    pub(crate) const V_REGEX: &'static str = #regex;
                                }
                            }
                        }
                        Regex::Ref(_) => {
                            // unable to be used as a const
                            quote! {}
                        }
                    }
                };

                let get_regex = {
                    match &regex {
                        Regex::String(regex) => {
                            quote! {
                                validators_prelude::regex::Regex::new(#regex).unwrap()
                            }
                        }
                        Regex::Ref(path) => {
                            quote! {
                                #path
                            }
                        }
                    }
                };

                let v_parse_str = quote! {
                    #[inline]
                    fn v_parse_str(s: &str) -> Result<(), #error_path> {
                        if !#get_regex.is_match(s) {
                            return Err(#error_path);
                        }

                        Ok(())
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
                            let s = s.into();

                            Self::v_parse_str(s.as_str())?;

                            Ok(#name(s))
                        }

                        #[inline]
                        fn parse_str<S: AsRef<str>>(s: S) -> Result<Self::Output, Self::Error> {
                            let s = s.as_ref();

                            Self::v_parse_str(s)?;

                            Ok(#name(validators_prelude::String::from(s)))
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
                        match &regex {
                            Regex::String(regex) => {
                                let regex = alloc::format!(
                                    "a string matched by a regular expression: {}",
                                    regex
                                );

                                quote! {
                                    f.write_str(#regex)
                                }
                            }
                            Regex::Ref(path) => {
                                quote! {
                                    f.write_str("a string matched by a regular expression: ")?;
                                    f.write_fmt(format_args!("{}", #path.to_string()))
                                }
                            }
                        }
                    };

                    quote! {
                        impl validators_prelude::Serialize for #name {
                            #[inline]
                            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                where
                                    S: validators_prelude::Serializer, {
                                serializer.serialize_str(self.0.as_str())
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
                                        #expect
                                    }

                                    #[inline]
                                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateString>::parse_str(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    #[inline]
                                    fn visit_string<E>(self, v: validators_prelude::String) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateString>::parse_string(v).map_err(validators_prelude::DeError::custom)
                                    }
                                }

                                deserializer.deserialize_string(ValidatingVisitor)
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

                let regex_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #serde_impl

                    #rocket_impl
                };

                regex_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
