use core::fmt::Write;

use alloc::boxed::Box;
use alloc::string::{String, ToString};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, Meta, NestedMeta, Path};

use crate::{panic, SynOption, TypeEnum, Validator, ValidatorRangeOption};

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::SignedInteger);
const VALIDATOR: Validator = Validator::signed_integer;

enum SignedIntegerType {
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
}

pub fn signed_integer_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() != 1 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                let data_type = data.fields.into_iter().next().unwrap().ty;

                let signed_integer_type = {
                    let data_type_name = data_type.to_token_stream().to_string();

                    match data_type_name.as_str() {
                        "isize" => SignedIntegerType::Isize,
                        "i8" => SignedIntegerType::I8,
                        "i16" => SignedIntegerType::I16,
                        "i32" => SignedIntegerType::I32,
                        "i64" => SignedIntegerType::I64,
                        "i128" => SignedIntegerType::I128,
                        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
                    }
                };

                let mut range_isize = ValidatorRangeOption::<isize>::new();
                let mut range_i8 = ValidatorRangeOption::<i8>::new();
                let mut range_i16 = ValidatorRangeOption::<i16>::new();
                let mut range_i32 = ValidatorRangeOption::<i32>::new();
                let mut range_i64 = ValidatorRangeOption::<i64>::new();
                let mut range = ValidatorRangeOption::<i128>::new();

                let correct_usage_for_attribute = [stringify!(#[validator(signed_integer)])];

                let correct_usage_for_range = [
                    stringify!(#[validator(signed_integer(range(Inside(max = 100))))]),
                    stringify!(#[validator(signed_integer(range(Inside(min = 0))))]),
                    stringify!(#[validator(signed_integer(range(Inside(min = 5, max = 200))))]),
                ];

                match meta {
                    Meta::Path(_) => (),
                    Meta::List(list) => {
                        let mut range_is_set = false;

                        for p in list.nested.iter() {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    let meta_name = meta.path().into_token_stream().to_string();

                                    match meta_name.as_str() {
                                        "range" => {
                                            match signed_integer_type {
                                                SignedIntegerType::Isize => {
                                                    range_isize = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                                SignedIntegerType::I8 => {
                                                    range_i8 = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                                SignedIntegerType::I16 => {
                                                    range_i16 = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                                SignedIntegerType::I32 => {
                                                    range_i32 = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                                SignedIntegerType::I64 => {
                                                    range_i64 = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                                SignedIntegerType::I128 => {
                                                    range = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                            }
                                        }
                                        _ => {
                                            panic::unknown_parameter(
                                                "signed_integer",
                                                meta_name.as_str(),
                                            )
                                        }
                                    }
                                }
                                NestedMeta::Lit(_) => {
                                    panic::attribute_incorrect_format(
                                        "signed_integer",
                                        &correct_usage_for_attribute,
                                    )
                                }
                            }
                        }
                    }
                    Meta::NameValue(_) => {
                        panic::attribute_incorrect_format(
                            "signed_integer",
                            &correct_usage_for_attribute,
                        )
                    }
                }

                // merge
                let (range_expr, cast) = match signed_integer_type {
                    SignedIntegerType::Isize => {
                        let expr = range_isize.to_expr();

                        match range_isize {
                            ValidatorRangeOption::Inside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Inside {
                                    min: min.map(|i| i as i128),
                                    max: max.map(|i| i as i128),
                                }
                            }
                            ValidatorRangeOption::Outside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Outside {
                                    min: min.map(|i| i as i128),
                                    max: max.map(|i| i as i128),
                                }
                            }
                            ValidatorRangeOption::NotLimited => (),
                        }

                        (expr, Some(quote! {as isize}))
                    }
                    SignedIntegerType::I8 => {
                        let expr = range_i8.to_expr();

                        match range_i8 {
                            ValidatorRangeOption::Inside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Inside {
                                    min: min.map(|i| i as i128),
                                    max: max.map(|i| i as i128),
                                }
                            }
                            ValidatorRangeOption::Outside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Outside {
                                    min: min.map(|i| i as i128),
                                    max: max.map(|i| i as i128),
                                }
                            }
                            ValidatorRangeOption::NotLimited => (),
                        }

                        (expr, Some(quote! {as i8}))
                    }
                    SignedIntegerType::I16 => {
                        let expr = range_i16.to_expr();

                        match range_i16 {
                            ValidatorRangeOption::Inside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Inside {
                                    min: min.map(|i| i as i128),
                                    max: max.map(|i| i as i128),
                                }
                            }
                            ValidatorRangeOption::Outside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Outside {
                                    min: min.map(|i| i as i128),
                                    max: max.map(|i| i as i128),
                                }
                            }
                            ValidatorRangeOption::NotLimited => (),
                        }

                        (expr, Some(quote! {as i16}))
                    }
                    SignedIntegerType::I32 => {
                        let expr = range_i32.to_expr();

                        match range_i32 {
                            ValidatorRangeOption::Inside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Inside {
                                    min: min.map(|i| i as i128),
                                    max: max.map(|i| i as i128),
                                }
                            }
                            ValidatorRangeOption::Outside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Outside {
                                    min: min.map(|i| i as i128),
                                    max: max.map(|i| i as i128),
                                }
                            }
                            ValidatorRangeOption::NotLimited => (),
                        }

                        (expr, Some(quote! {as i32}))
                    }
                    SignedIntegerType::I64 => {
                        let expr = range_i64.to_expr();

                        match range_i64 {
                            ValidatorRangeOption::Inside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Inside {
                                    min: min.map(|i| i as i128),
                                    max: max.map(|i| i as i128),
                                }
                            }
                            ValidatorRangeOption::Outside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Outside {
                                    min: min.map(|i| i as i128),
                                    max: max.map(|i| i as i128),
                                }
                            }
                            ValidatorRangeOption::NotLimited => (),
                        }

                        (expr, Some(quote! {as i64}))
                    }
                    SignedIntegerType::I128 => (range.to_expr(), None),
                };

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::SignedIntegerError }).unwrap();

                let parameters_impl = quote! {
                    impl #name {
                        pub(crate) const V_RANGE: validators_prelude::ValidatorRangeOption<#data_type> = #range_expr;
                    }
                };

                let v_parse_str = quote! {
                    #[inline]
                    fn v_parse_str(s: &str) -> Result<#data_type, #error_path> {
                        let i = s.parse::<#data_type>()?;

                        Self::v_parse_i(i)?;

                        Ok(i)
                    }
                };

                let handle_range = {
                    match range {
                        ValidatorRangeOption::Inside {
                            min,
                            max,
                        } => {
                            let mut token_stream = TokenStream2::new();

                            if let Some(min) = min {
                                token_stream.extend(quote! {
                                    if i < #min #cast {
                                        return Err(#error_path::TooSmall);
                                    }
                                });
                            }

                            if let Some(max) = max {
                                token_stream.extend(quote! {
                                    if i > #max #cast {
                                        return Err(#error_path::TooLarge);
                                    }
                                });
                            }

                            token_stream
                        }
                        ValidatorRangeOption::Outside {
                            min,
                            max,
                        } => {
                            match min {
                                Some(min) => {
                                    match max {
                                        Some(max) => {
                                            if min == max {
                                                quote! {
                                                    if i == #min #cast {
                                                        return Err(#error_path::Forbidden);
                                                    }
                                                }
                                            } else {
                                                quote! {
                                                    if (#min #cast)..=(#max #cast).contains(&i) {
                                                        return Err(#error_path::Forbidden);
                                                    }
                                                }
                                            }
                                        }
                                        None => {
                                            quote! {
                                                if i >= #min #cast {
                                                    return Err(#error_path::Forbidden);
                                                }
                                            }
                                        }
                                    }
                                }
                                None => {
                                    match max {
                                        Some(max) => {
                                            quote! {
                                                if i <= #max #cast {
                                                    return Err(#error_path::Forbidden);
                                                }
                                            }
                                        }
                                        None => {
                                            quote! {}
                                        }
                                    }
                                }
                            }
                        }
                        ValidatorRangeOption::NotLimited => quote! {},
                    }
                };

                let v_parse_i = quote! {
                    #[inline]
                    fn v_parse_i(i: #data_type) -> Result<(), #error_path> {
                        #handle_range

                        Ok(())
                    }
                };

                let parse_impl = quote! {
                    impl #name {
                        #v_parse_str

                        #v_parse_i
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

                let validate_signed_integer_impl = {
                    match signed_integer_type {
                        SignedIntegerType::Isize => {
                            quote! {
                                impl ValidateSignedInteger for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[cfg(not(target_pointer_width = "128"))]
                                    #[inline]
                                    fn parse_i128(i: i128) -> Result<Self::Output, Self::Error> {
                                        if i > isize::MAX as i128 {
                                            Err(#error_path::TooLarge)
                                        } else if i < isize::MIN as i128 {
                                            Err(#error_path::TooSmall)
                                        } else {
                                            Self::parse_isize(i as isize)
                                        }
                                    }

                                    #[cfg(not(target_pointer_width = "128"))]
                                    #[inline]
                                    fn validate_i128(i: i128) -> Result<(), Self::Error> {
                                        if i > isize::MAX as i128 {
                                            Err(#error_path::TooLarge)
                                        } else if i < isize::MIN as i128 {
                                            Err(#error_path::TooSmall)
                                        } else {
                                            Self::validate_isize(i as isize)
                                        }
                                    }

                                    #[inline]
                                    fn parse_isize(i: isize) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(#name(i))
                                    }

                                    #[inline]
                                    fn validate_isize(i: isize) -> Result<(), Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(())
                                    }

                                    #[cfg(target_pointer_width = "8")]
                                    fn parse_i8(i: i8) -> Result<Self::Output, Self::Error> {
                                        Self::parse_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "16")]
                                    fn parse_i16(i: i16) -> Result<Self::Output, Self::Error> {
                                        Self::parse_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "32")]
                                    fn parse_i32(i: i32) -> Result<Self::Output, Self::Error> {
                                        Self::parse_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "64")]
                                    fn parse_i64(i: i64) -> Result<Self::Output, Self::Error> {
                                        Self::parse_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "128")]
                                    fn parse_i128(i: i128) -> Result<Self::Output, Self::Error> {
                                        Self::parse_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "8")]
                                    #[inline]
                                    fn validate_i8(i: i8) -> Result<(), Self::Error> {
                                        Self::validate_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "16")]
                                    #[inline]
                                    fn validate_i16(i: i16) -> Result<(), Self::Error> {
                                        Self::validate_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "32")]
                                    #[inline]
                                    fn validate_i32(i: i32) -> Result<(), Self::Error> {
                                        Self::validate_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "64")]
                                    #[inline]
                                    fn validate_i64(i: i64) -> Result<(), Self::Error> {
                                        Self::validate_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "128")]
                                    #[inline]
                                    fn validate_i128(i: i128) -> Result<(), Self::Error> {
                                        Self::validate_isize(i as isize)
                                    }
                                }
                            }
                        }
                        SignedIntegerType::I8 => {
                            quote! {
                                impl ValidateSignedInteger for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[inline]
                                    fn parse_i128(i: i128) -> Result<Self::Output, Self::Error> {
                                        if i > i8::MAX as i128 {
                                            Err(#error_path::TooLarge)
                                        } else if i < i8::MIN as i128 {
                                            Err(#error_path::TooSmall)
                                        } else {
                                            Self::parse_i8(i as i8)
                                        }
                                    }

                                    #[inline]
                                    fn validate_i128(i: i128) -> Result<(), Self::Error> {
                                        if i > i8::MAX as i128 {
                                            Err(#error_path::TooLarge)
                                        } else if i < i8::MIN as i128 {
                                            Err(#error_path::TooSmall)
                                        } else {
                                            Self::validate_i8(i as i8)
                                        }
                                    }

                                    #[inline]
                                    fn parse_i8(i: i8) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(#name(i))
                                    }

                                    #[inline]
                                    fn validate_i8(i: i8) -> Result<(), Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(())
                                    }
                                }
                            }
                        }
                        SignedIntegerType::I16 => {
                            quote! {
                                impl ValidateSignedInteger for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[inline]
                                    fn parse_i128(i: i128) -> Result<Self::Output, Self::Error> {
                                        if i > i16::MAX as i128 {
                                            Err(#error_path::TooLarge)
                                        } else if i < i16::MIN as i128 {
                                            Err(#error_path::TooSmall)
                                        } else {
                                            Self::parse_i16(i as i16)
                                        }
                                    }

                                    #[inline]
                                    fn validate_i128(i: i128) -> Result<(), Self::Error> {
                                        if i > i16::MAX as i128 {
                                            Err(#error_path::TooLarge)
                                        } else if i < i16::MIN as i128 {
                                            Err(#error_path::TooSmall)
                                        } else {
                                            Self::validate_i16(i as i16)
                                        }
                                    }

                                    #[inline]
                                    fn parse_i16(i: i16) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(#name(i))
                                    }

                                    #[inline]
                                    fn validate_i16(i: i16) -> Result<(), Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(())
                                    }
                                }
                            }
                        }
                        SignedIntegerType::I32 => {
                            quote! {
                                impl ValidateSignedInteger for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[inline]
                                    fn parse_i128(i: i128) -> Result<Self::Output, Self::Error> {
                                        if i > i32::MAX as i128 {
                                            Err(#error_path::TooLarge)
                                        } else if i < i32::MIN as i128 {
                                            Err(#error_path::TooSmall)
                                        } else {
                                            Self::parse_i32(i as i32)
                                        }
                                    }

                                    #[inline]
                                    fn validate_i128(i: i128) -> Result<(), Self::Error> {
                                        if i > i32::MAX as i128 {
                                            Err(#error_path::TooLarge)
                                        } else if i < i32::MIN as i128 {
                                            Err(#error_path::TooSmall)
                                        } else {
                                            Self::validate_i32(i as i32)
                                        }
                                    }

                                    #[inline]
                                    fn parse_i32(i: i32) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(#name(i))
                                    }

                                    #[inline]
                                    fn validate_i32(i: i32) -> Result<(), Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(())
                                    }
                                }
                            }
                        }
                        SignedIntegerType::I64 => {
                            quote! {
                                impl ValidateSignedInteger for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[inline]
                                    fn parse_i128(i: i128) -> Result<Self::Output, Self::Error> {
                                        if i > i64::MAX as i128 {
                                            Err(#error_path::TooLarge)
                                        } else if i < i64::MIN as i128 {
                                            Err(#error_path::TooSmall)
                                        } else {
                                            Self::parse_i64(i as i64)
                                        }
                                    }

                                    #[inline]
                                    fn validate_i128(i: i128) -> Result<(), Self::Error> {
                                        if i > i64::MAX as i128 {
                                            Err(#error_path::TooLarge)
                                        } else if i < i64::MIN as i128 {
                                            Err(#error_path::TooSmall)
                                        } else {
                                            Self::validate_i64(i as i64)
                                        }
                                    }

                                    #[inline]
                                    fn parse_i64(i: i64) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(#name(i))
                                    }

                                    #[inline]
                                    fn validate_i64(i: i64) -> Result<(), Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(())
                                    }
                                }
                            }
                        }
                        SignedIntegerType::I128 => {
                            quote! {
                                impl ValidateSignedInteger for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[inline]
                                    fn parse_i128(i: i128) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(#name(i))
                                    }

                                    #[inline]
                                    fn validate_i128(i: i128) -> Result<(), Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(())
                                    }
                                }
                            }
                        }
                    }
                };

                let serde_impl = if cfg!(feature = "serde") {
                    let expect = {
                        let mut s = String::from("a signed_integer");

                        match range {
                            ValidatorRangeOption::Inside {
                                min,
                                max,
                            } => {
                                s.push_str(" in ");

                                if let Some(min) = min {
                                    s.write_fmt(format_args!("{}", min)).unwrap();
                                }

                                s.push_str("..");

                                if let Some(max) = max {
                                    s.write_fmt(format_args!("={}", max)).unwrap();
                                }
                            }
                            ValidatorRangeOption::Outside {
                                min,
                                max,
                            } => {
                                s.push_str(" not in ");

                                if let Some(min) = min {
                                    s.write_fmt(format_args!("{}", min)).unwrap();
                                }

                                s.push_str("..");

                                if let Some(max) = max {
                                    s.write_fmt(format_args!("={}", max)).unwrap();
                                }
                            }
                            ValidatorRangeOption::NotLimited => (),
                        }

                        s
                    };

                    let handle_serialize = {
                        match signed_integer_type {
                            SignedIntegerType::Isize => {
                                quote! {
                                    #[cfg(target_pointer_width = "8")]
                                    {
                                        serializer.serialize_i8(self.0 as i8)
                                    }

                                    #[cfg(target_pointer_width = "16")]
                                    {
                                        serializer.serialize_i16(self.0 as i16)
                                    }

                                    #[cfg(target_pointer_width = "32")]
                                    {
                                        serializer.serialize_i32(self.0 as i32)
                                    }

                                    #[cfg(target_pointer_width = "64")]
                                    {
                                        serializer.serialize_i64(self.0 as i64)
                                    }

                                    #[cfg(target_pointer_width = "128")]
                                    {
                                        validators_prelude::serde_if_integer128! {
                                            return serializer.serialize_i128(self.0 as i128);
                                        }

                                        unreachable!("the `integer128` feature of the `serde` crate needs to be enabled")
                                    }
                                }
                            }
                            SignedIntegerType::I8 => {
                                quote! {
                                    serializer.serialize_i8(self.0)
                                }
                            }
                            SignedIntegerType::I16 => {
                                quote! {
                                    serializer.serialize_i16(self.0)
                                }
                            }
                            SignedIntegerType::I32 => {
                                quote! {
                                    serializer.serialize_i32(self.0)
                                }
                            }
                            SignedIntegerType::I64 => {
                                quote! {
                                    serializer.serialize_i64(self.0)
                                }
                            }
                            SignedIntegerType::I128 => {
                                quote! {
                                    validators_prelude::serde_if_integer128! {
                                        return serializer.serialize_i128(self.0);
                                    }

                                    unreachable!("the `integer128` feature of the `serde` crate needs to be enabled")
                                }
                            }
                        }
                    };

                    quote! {
                        impl validators_prelude::Serialize for #name {
                            #[allow(unreachable_code)]
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
                                    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateSignedInteger>::parse_i8(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    #[inline]
                                    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateSignedInteger>::parse_i16(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    #[inline]
                                    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateSignedInteger>::parse_i32(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    #[inline]
                                    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateSignedInteger>::parse_i64(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    validators_prelude::serde_if_integer128! {
                                        #[inline]
                                        fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
                                        where
                                            E: validators_prelude::DeError, {
                                            <#name as ValidateSignedInteger>::parse_i128(v).map_err(validators_prelude::DeError::custom)
                                        }
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

                let signed_integer_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #validate_signed_integer_impl

                    #serde_impl

                    #rocket_impl
                };

                signed_integer_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
