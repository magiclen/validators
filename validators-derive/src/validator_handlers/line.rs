use alloc::boxed::Box;
use alloc::string::ToString;

use crate::proc_macro::TokenStream;
use crate::quote::ToTokens;
use crate::syn::{Data, DeriveInput, Fields, Meta, NestedMeta, Path};

use crate::{panic, SynOption, TypeEnum, Validator, ValidatorOption};

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::String);
const VALIDATOR: Validator = Validator::line;

pub fn line_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() != 1 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                let mut empty = ValidatorOption::new();

                let correct_usage_for_attribute = [stringify!(#[validator(line)])];

                let correct_usage_for_empty = [
                    stringify!(#[validator(line(empty(Must)))]),
                    stringify!(#[validator(line(empty(Allow)))]),
                    stringify!(#[validator(line(empty(NotAllow)))]),
                ];

                match meta {
                    Meta::Path(_) => (),
                    Meta::List(list) => {
                        let mut empty_is_set = false;

                        for p in list.nested.iter() {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    let meta_name = meta.path().into_token_stream().to_string();

                                    match meta_name.as_str() {
                                        "empty" => {
                                            if let Some(validator_option) =
                                                ValidatorOption::from_meta(
                                                    meta_name.as_str(),
                                                    meta,
                                                    &mut empty_is_set,
                                                    &correct_usage_for_empty,
                                                )
                                            {
                                                empty = validator_option;
                                            }
                                        }
                                        _ => panic::unknown_parameter("line", meta_name.as_str()),
                                    }
                                }
                                NestedMeta::Lit(_) => {
                                    panic::attribute_incorrect_format(
                                        "line",
                                        &correct_usage_for_attribute,
                                    )
                                }
                            }
                        }
                    }
                    Meta::NameValue(_) => {
                        panic::attribute_incorrect_format("line", &correct_usage_for_attribute)
                    }
                }

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::line::LineError }).unwrap();

                let empty_path = empty.to_expr();

                let parameters_impl = quote! {
                    impl #name {
                        pub(crate) const V_EMPTY: validators_prelude::ValidatorOption = #empty_path;
                    }
                };

                let handle_str = {
                    match empty {
                        ValidatorOption::Allow => {
                            quote! {
                                for e in s.bytes() {
                                    match e {
                                        b'\x00'..=b'\x08' | b'\x0A'..=b'\x1F' | b'\x7F' => {
                                            return Err(#error_path::Invalid);
                                        }
                                        _ => (),
                                    }
                                }

                                Ok(())
                            }
                        }
                        ValidatorOption::Must => {
                            quote! {
                                for c in s.chars() {
                                    if !c.is_whitespace() {
                                        return Err(#error_path::EmptyMust);
                                    }
                                }

                                Ok(())
                            }
                        }
                        ValidatorOption::NotAllow => {
                            quote! {
                                let mut chars = s.chars();

                                while let Some(c) = chars.next() {
                                    if !c.is_whitespace() {
                                        match c {
                                            '\x00'..='\x08' | '\x0A'..='\x1F' | '\x7F' => {
                                                return Err(#error_path::Invalid);
                                            }
                                            _ => (),
                                        }

                                        while let Some(c) = chars.next() {
                                            match c {
                                                '\x00'..='\x08' | '\x0A'..='\x1F' | '\x7F' => {
                                                    return Err(#error_path::Invalid);
                                                }
                                                _ => (),
                                            }
                                        }

                                        return Ok(());
                                    }
                                }

                                Err(#error_path::EmptyNotAllow)
                            }
                        }
                    }
                };

                let v_parse_str = quote! {
                    #[inline]
                    fn v_parse_str(s: &str) -> Result<(), #error_path> {
                        #handle_str
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
                    let expect = match empty {
                        ValidatorOption::Allow => "a line",
                        ValidatorOption::Must => "a line which must be empty after trimming",
                        ValidatorOption::NotAllow => {
                            "a line which must not be empty after trimming"
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
                                        f.write_str(#expect)
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

                let line_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #serde_impl

                    #rocket_impl
                };

                line_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
