extern crate proc_macro2;

use core::fmt::Write;

use alloc::boxed::Box;
use alloc::string::{String, ToString};

use crate::proc_macro::TokenStream;
use crate::quote::ToTokens;
use crate::syn::{Data, DeriveInput, Fields, Meta, NestedMeta, Path};

use crate::{panic, SynOption, TypeEnum, Validator, ValidatorOption, ValidatorRangeOption};

use proc_macro2::TokenStream as TokenStream2;

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::Number);
const VALIDATOR: Validator = Validator::number;

enum NumberType {
    F32,
    F64,
}

pub fn number_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() != 1 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                let data_type = data.fields.into_iter().next().unwrap().ty;

                let number_type = {
                    let data_type_name = data_type.to_token_stream().to_string();

                    match data_type_name.as_str() {
                        "f32" => NumberType::F32,
                        "f64" => NumberType::F64,
                        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
                    }
                };

                let mut nan = ValidatorOption::new();
                let mut range_f32 = ValidatorRangeOption::<f32>::new();
                let mut range = ValidatorRangeOption::<f64>::new();
                let mut conflict = ValidatorOption::NotAllow;

                let correct_usage_for_attribute = [stringify!(#[validator(number)])];

                let correct_usage_for_nan = [
                    stringify!(#[validator(number(nan(Must)))]),
                    stringify!(#[validator(number(nan(Allow)))]),
                    stringify!(#[validator(number(nan(NotAllow)))]),
                ];

                let correct_usage_for_range = [
                    stringify!(#[validator(number(range(Limited(max = 100.0))))]),
                    stringify!(#[validator(number(range(Limited(min = 0.0))))]),
                    stringify!(#[validator(number(range(Limited(min = 5.1, max = 200))))]),
                ];

                let correct_usage_for_conflict = [
                    stringify!(#[validator(number(conflict(Allow)))]),
                    stringify!(#[validator(number(conflict(NotAllow)))]),
                ];

                match meta {
                    Meta::Path(_) => (),
                    Meta::List(list) => {
                        let mut nan_is_set = false;
                        let mut range_is_set = false;
                        let mut conflict_is_set = false;

                        for p in list.nested.iter() {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    let meta_name = meta.path().into_token_stream().to_string();

                                    match meta_name.as_str() {
                                        "nan" => {
                                            nan = ValidatorOption::from_meta(
                                                meta_name.as_str(),
                                                meta,
                                                &mut nan_is_set,
                                                &correct_usage_for_nan,
                                            );
                                        }
                                        "range" => {
                                            match number_type {
                                                NumberType::F32 => {
                                                    range_f32 = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                                NumberType::F64 => {
                                                    range = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                            }
                                        }
                                        "conflict" => {
                                            conflict = ValidatorOption::from_meta(
                                                meta_name.as_str(),
                                                meta,
                                                &mut conflict_is_set,
                                                &correct_usage_for_conflict,
                                            );

                                            if conflict == ValidatorOption::Must {
                                                panic::parameter_incorrect_format(
                                                    meta_name.as_str(),
                                                    &correct_usage_for_conflict,
                                                );
                                            }
                                        }
                                        _ => panic::unknown_parameter("number", meta_name.as_str()),
                                    }
                                }
                                NestedMeta::Lit(_) => {
                                    panic::attribute_incorrect_format(
                                        "number",
                                        &correct_usage_for_attribute,
                                    )
                                }
                            }
                        }
                    }
                    Meta::NameValue(_) => {
                        panic::attribute_incorrect_format("number", &correct_usage_for_attribute)
                    }
                }

                // merge
                let (range_expr, cast) = match number_type {
                    NumberType::F32 => {
                        let expr = range_f32.to_expr();

                        if let ValidatorRangeOption::Limited {
                            min,
                            max,
                        } = range_f32
                        {
                            range = ValidatorRangeOption::Limited {
                                min: min.map(|f| f as f64),
                                max: max.map(|f| f as f64),
                            }
                        }

                        (expr, Some(quote! {as f32}))
                    }
                    NumberType::F64 => (range.to_expr(), None),
                };

                let mut _meta_is_conflict = false;

                if nan.must() && range.limited().is_some() {
                    if conflict.not_allow() {
                        panic!("`nan(Must)` and `range(Limited)` cannot be used together.");
                    }

                    _meta_is_conflict = true;
                }

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::number::NumberError }).unwrap();

                let nan_path = nan.to_expr();

                let parameters_impl = quote! {
                    impl #name {
                        pub(crate) const V_NAN: validators_prelude::ValidatorOption = #nan_path;
                        pub(crate) const V_RANGE: validators_prelude::ValidatorRangeOption<#data_type> = #range_expr;
                    }
                };

                let v_parse_str = quote! {
                    #[inline]
                    fn v_parse_str(s: &str) -> Result<#data_type, #error_path> {
                        let f = s.parse::<#data_type>()?;

                        Self::v_parse_f(f)?;

                        Ok(f)
                    }
                };

                let handle_range = {
                    match range {
                        ValidatorRangeOption::Limited {
                            min,
                            max,
                        } => {
                            let mut token_stream = TokenStream2::new();

                            if let Some(min) = min {
                                token_stream.extend(quote! {
                                    if f < #min #cast {
                                        return Err(#error_path::TooSmall);
                                    }
                                });
                            }

                            if let Some(max) = max {
                                token_stream.extend(quote! {
                                    if f > #max #cast {
                                        return Err(#error_path::TooLarge);
                                    }
                                });
                            }

                            token_stream
                        }
                        ValidatorRangeOption::NotLimited => quote! {},
                    }
                };

                let handle_nan = {
                    match nan {
                        ValidatorOption::Allow => quote! {},
                        ValidatorOption::Must => {
                            quote! {
                                if !f.is_nan() {
                                    return Err(#error_path::NaNMust);
                                }
                            }
                        }
                        ValidatorOption::NotAllow => {
                            quote! {
                                if f.is_nan() {
                                    return Err(#error_path::NaNNotAllow);
                                }
                            }
                        }
                    }
                };

                let v_parse_f = quote! {
                    #[inline]
                    fn v_parse_f(f: #data_type) -> Result<(), #error_path> {
                        #handle_range

                        #handle_nan

                        Ok(())
                    }
                };

                let parse_impl = quote! {
                    impl #name {
                        #v_parse_str

                        #v_parse_f
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

                let validate_number_impl = {
                    match number_type {
                        NumberType::F32 => {
                            quote! {
                                impl ValidateNumber for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[inline]
                                    fn parse_f64(f: f64) -> Result<Self::Output, Self::Error> {
                                        unimplemented!("should use `parse_f32`");
                                    }

                                    #[inline]
                                    fn validate_f64(f: f64) -> Result<(), Self::Error> {
                                        unimplemented!("should use `validate_f32`");
                                    }

                                    #[inline]
                                    fn parse_f32(f: f32) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_f(f)?;

                                        Ok(#name(f))
                                    }

                                    #[inline]
                                    fn validate_f32(f: f32) -> Result<(), Self::Error> {
                                        Self::v_parse_f(f)?;

                                        Ok(())
                                    }
                                }
                            }
                        }
                        NumberType::F64 => {
                            quote! {
                                impl ValidateNumber for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[inline]
                                    fn parse_f64(f: f64) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_f(f)?;

                                        Ok(#name(f))
                                    }

                                    #[inline]
                                    fn validate_f64(f: f64) -> Result<(), Self::Error> {
                                        Self::v_parse_f(f)?;

                                        Ok(())
                                    }
                                }
                            }
                        }
                    }
                };

                let serde_impl = if cfg!(feature = "serde") {
                    let expect = {
                        let mut s = String::from("a number");

                        match range {
                            ValidatorRangeOption::Limited {
                                min,
                                max,
                            } => {
                                s.push_str(" in ");

                                if let Some(min) = min {
                                    s.write_fmt(format_args!("{:.1}", min)).unwrap();
                                }

                                s.push_str("..");

                                if let Some(max) = max {
                                    s.write_fmt(format_args!("{:.1}", max)).unwrap();
                                }
                            }
                            ValidatorRangeOption::NotLimited => (),
                        }

                        match nan {
                            ValidatorOption::Allow => (),
                            ValidatorOption::Must => {
                                s.push_str(" which must be NaN");
                            }
                            ValidatorOption::NotAllow => {
                                s.push_str(" which must not be NaN");
                            }
                        }

                        s
                    };

                    let handle_serialize = {
                        match number_type {
                            NumberType::F32 => {
                                quote! {
                                    serializer.serialize_f32(self.0)
                                }
                            }
                            NumberType::F64 => {
                                quote! {
                                    serializer.serialize_f64(self.0)
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
                                #handle_serialize
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

                                    #[inline]
                                    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateNumber>::parse_f64(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    #[inline]
                                    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateNumber>::parse_f32(v).map_err(validators_prelude::DeError::custom)
                                    }
                                }

                                deserializer.deserialize_any(ValidatingVisitor)
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

                let number_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #validate_number_impl

                    #serde_impl

                    #rocket_impl
                };

                number_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
