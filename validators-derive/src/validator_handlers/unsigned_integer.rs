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

const ITEM: Struct = Struct(TypeEnum::UnsignedInteger);
const VALIDATOR: Validator = Validator::unsigned_integer;

enum UnsignedIntegerType {
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

pub fn unsigned_integer_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() != 1 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                let data_type = data.fields.into_iter().next().unwrap().ty;

                let unsigned_integer_type = {
                    let data_type_name = data_type.to_token_stream().to_string();

                    match data_type_name.as_str() {
                        "usize" => UnsignedIntegerType::Usize,
                        "u8" => UnsignedIntegerType::U8,
                        "u16" => UnsignedIntegerType::U16,
                        "u32" => UnsignedIntegerType::U32,
                        "u64" => UnsignedIntegerType::U64,
                        "u128" => UnsignedIntegerType::U128,
                        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
                    }
                };

                let mut range_usize = ValidatorRangeOption::<usize>::new();
                let mut range_u8 = ValidatorRangeOption::<u8>::new();
                let mut range_u16 = ValidatorRangeOption::<u16>::new();
                let mut range_u32 = ValidatorRangeOption::<u32>::new();
                let mut range_u64 = ValidatorRangeOption::<u64>::new();
                let mut range = ValidatorRangeOption::<u128>::new();

                let correct_usage_for_attribute = [stringify!(#[validator(unsigned_integer)])];

                let correct_usage_for_range = [
                    stringify!(#[validator(unsigned_integer(range(Inside(max = 100))))]),
                    stringify!(#[validator(unsigned_integer(range(Inside(min = 1))))]),
                    stringify!(#[validator(unsigned_integer(range(Inside(min = 5, max = 200))))]),
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
                                            match unsigned_integer_type {
                                                UnsignedIntegerType::Usize => {
                                                    range_usize = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                                UnsignedIntegerType::U8 => {
                                                    range_u8 = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                                UnsignedIntegerType::U16 => {
                                                    range_u16 = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                                UnsignedIntegerType::U32 => {
                                                    range_u32 = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                                UnsignedIntegerType::U64 => {
                                                    range_u64 = ValidatorRangeOption::from_meta(
                                                        meta_name.as_str(),
                                                        meta,
                                                        &mut range_is_set,
                                                        &correct_usage_for_range,
                                                    );
                                                }
                                                UnsignedIntegerType::U128 => {
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
                                                "unsigned_integer",
                                                meta_name.as_str(),
                                            )
                                        }
                                    }
                                }
                                NestedMeta::Lit(_) => {
                                    panic::attribute_incorrect_format(
                                        "unsigned_integer",
                                        &correct_usage_for_attribute,
                                    )
                                }
                            }
                        }
                    }
                    Meta::NameValue(_) => {
                        panic::attribute_incorrect_format(
                            "unsigned_integer",
                            &correct_usage_for_attribute,
                        )
                    }
                }

                // merge
                let (range_expr, cast) = match unsigned_integer_type {
                    UnsignedIntegerType::Usize => {
                        let expr = range_usize.to_expr();

                        match range_usize {
                            ValidatorRangeOption::Inside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Inside {
                                    min: min.map(|u| u as u128),
                                    max: max.map(|u| u as u128),
                                }
                            }
                            ValidatorRangeOption::Outside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Outside {
                                    min: min.map(|u| u as u128),
                                    max: max.map(|u| u as u128),
                                }
                            }
                            ValidatorRangeOption::NotLimited => (),
                        }

                        (expr, Some(quote! {as usize}))
                    }
                    UnsignedIntegerType::U8 => {
                        let expr = range_u8.to_expr();

                        match range_u8 {
                            ValidatorRangeOption::Inside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Inside {
                                    min: min.map(|u| u as u128),
                                    max: max.map(|u| u as u128),
                                }
                            }
                            ValidatorRangeOption::Outside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Outside {
                                    min: min.map(|u| u as u128),
                                    max: max.map(|u| u as u128),
                                }
                            }
                            ValidatorRangeOption::NotLimited => (),
                        }

                        (expr, Some(quote! {as u8}))
                    }
                    UnsignedIntegerType::U16 => {
                        let expr = range_u16.to_expr();

                        match range_u16 {
                            ValidatorRangeOption::Inside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Inside {
                                    min: min.map(|u| u as u128),
                                    max: max.map(|u| u as u128),
                                }
                            }
                            ValidatorRangeOption::Outside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Outside {
                                    min: min.map(|u| u as u128),
                                    max: max.map(|u| u as u128),
                                }
                            }
                            ValidatorRangeOption::NotLimited => (),
                        }

                        (expr, Some(quote! {as u16}))
                    }
                    UnsignedIntegerType::U32 => {
                        let expr = range_u32.to_expr();

                        match range_u32 {
                            ValidatorRangeOption::Inside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Inside {
                                    min: min.map(|u| u as u128),
                                    max: max.map(|u| u as u128),
                                }
                            }
                            ValidatorRangeOption::Outside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Outside {
                                    min: min.map(|u| u as u128),
                                    max: max.map(|u| u as u128),
                                }
                            }
                            ValidatorRangeOption::NotLimited => (),
                        }

                        (expr, Some(quote! {as u32}))
                    }
                    UnsignedIntegerType::U64 => {
                        let expr = range_u64.to_expr();

                        match range_u64 {
                            ValidatorRangeOption::Inside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Inside {
                                    min: min.map(|u| u as u128),
                                    max: max.map(|u| u as u128),
                                }
                            }
                            ValidatorRangeOption::Outside {
                                min,
                                max,
                            } => {
                                range = ValidatorRangeOption::Outside {
                                    min: min.map(|u| u as u128),
                                    max: max.map(|u| u as u128),
                                }
                            }
                            ValidatorRangeOption::NotLimited => (),
                        }

                        (expr, Some(quote! {as u64}))
                    }
                    UnsignedIntegerType::U128 => (range.to_expr(), None),
                };

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::UnsignedIntegerError }).unwrap();

                let parameters_impl = quote! {
                    impl #name {
                        pub(crate) const V_RANGE: validators_prelude::ValidatorRangeOption<#data_type> = #range_expr;
                    }
                };

                let v_parse_str = quote! {
                    #[inline]
                    fn v_parse_str(s: &str) -> Result<#data_type, #error_path> {
                        let u = s.parse::<#data_type>()?;

                        Self::v_parse_u(u)?;

                        Ok(u)
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
                                    if u < #min #cast {
                                        return Err(#error_path::TooSmall);
                                    }
                                });
                            }

                            if let Some(max) = max {
                                token_stream.extend(quote! {
                                    if u > #max #cast {
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
                                                    if u == #min #cast {
                                                        return Err(#error_path::Forbidden);
                                                    }
                                                }
                                            } else {
                                                quote! {
                                                    if (#min #cast)..=(#max #cast).contains(&u) {
                                                        return Err(#error_path::Forbidden);
                                                    }
                                                }
                                            }
                                        }
                                        None => {
                                            quote! {
                                                if u >= #min #cast {
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
                                                if u <= #max #cast {
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

                let v_parse_u = quote! {
                    #[inline]
                    fn v_parse_u(u: #data_type) -> Result<(), #error_path> {
                        #handle_range

                        Ok(())
                    }
                };

                let parse_impl = quote! {
                    impl #name {
                        #v_parse_str

                        #v_parse_u
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

                let validate_unsigned_integer_impl = {
                    match unsigned_integer_type {
                        UnsignedIntegerType::Usize => {
                            quote! {
                                impl ValidateUnsignedInteger for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[cfg(not(target_pointer_width = "128"))]
                                    #[inline]
                                    fn parse_u128(u: u128) -> Result<Self::Output, Self::Error> {
                                        if u > usize::MAX as u128 {
                                            Err(#error_path::TooLarge)
                                        } else {
                                            Self::parse_usize(u as usize)
                                        }
                                    }

                                    #[cfg(not(target_pointer_width = "128"))]
                                    #[inline]
                                    fn validate_u128(u: u128) -> Result<(), Self::Error> {
                                        if u > usize::MAX as u128 {
                                            Err(#error_path::TooLarge)
                                        } else {
                                            Self::validate_usize(u as usize)
                                        }
                                    }

                                    #[inline]
                                    fn parse_usize(u: usize) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(#name(u))
                                    }

                                    #[inline]
                                    fn validate_usize(u: usize) -> Result<(), Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(())
                                    }

                                    #[cfg(target_pointer_width = "8")]
                                    fn parse_u8(u: u8) -> Result<Self::Output, Self::Error> {
                                        Self::parse_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "16")]
                                    fn parse_u16(u: u16) -> Result<Self::Output, Self::Error> {
                                        Self::parse_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "32")]
                                    fn parse_u32(u: u32) -> Result<Self::Output, Self::Error> {
                                        Self::parse_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "64")]
                                    fn parse_u64(u: u64) -> Result<Self::Output, Self::Error> {
                                        Self::parse_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "128")]
                                    fn parse_u128(u: u128) -> Result<Self::Output, Self::Error> {
                                        Self::parse_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "8")]
                                    #[inline]
                                    fn validate_u8(u: u8) -> Result<(), Self::Error> {
                                        Self::validate_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "16")]
                                    #[inline]
                                    fn validate_u16(u: u16) -> Result<(), Self::Error> {
                                        Self::validate_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "32")]
                                    #[inline]
                                    fn validate_u32(u: u32) -> Result<(), Self::Error> {
                                        Self::validate_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "64")]
                                    #[inline]
                                    fn validate_u64(u: u64) -> Result<(), Self::Error> {
                                        Self::validate_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "128")]
                                    #[inline]
                                    fn validate_u128(u: u128) -> Result<(), Self::Error> {
                                        Self::validate_usize(u as usize)
                                    }
                                }
                            }
                        }
                        UnsignedIntegerType::U8 => {
                            quote! {
                                impl ValidateUnsignedInteger for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[inline]
                                    fn parse_u128(u: u128) -> Result<Self::Output, Self::Error> {
                                        if u > u8::MAX as u128 {
                                            Err(#error_path::TooLarge)
                                        } else {
                                            Self::parse_u8(u as u8)
                                        }
                                    }

                                    #[inline]
                                    fn validate_u128(u: u128) -> Result<(), Self::Error> {
                                        if u > u8::MAX as u128 {
                                            Err(#error_path::TooLarge)
                                        } else {
                                            Self::validate_u8(u as u8)
                                        }
                                    }

                                    #[inline]
                                    fn parse_u8(u: u8) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(#name(u))
                                    }

                                    #[inline]
                                    fn validate_u8(u: u8) -> Result<(), Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(())
                                    }
                                }
                            }
                        }
                        UnsignedIntegerType::U16 => {
                            quote! {
                                impl ValidateUnsignedInteger for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[inline]
                                    fn parse_u128(u: u128) -> Result<Self::Output, Self::Error> {
                                        if u > u16::MAX as u128 {
                                            Err(#error_path::TooLarge)
                                        } else {
                                            Self::parse_u16(u as u16)
                                        }
                                    }

                                    #[inline]
                                    fn validate_u128(u: u128) -> Result<(), Self::Error> {
                                        if u > u16::MAX as u128 {
                                            Err(#error_path::TooLarge)
                                        } else {
                                            Self::validate_u16(u as u16)
                                        }
                                    }

                                    #[inline]
                                    fn parse_u16(u: u16) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(#name(u))
                                    }

                                    #[inline]
                                    fn validate_u16(u: u16) -> Result<(), Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(())
                                    }
                                }
                            }
                        }
                        UnsignedIntegerType::U32 => {
                            quote! {
                                impl ValidateUnsignedInteger for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[inline]
                                    fn parse_u128(u: u128) -> Result<Self::Output, Self::Error> {
                                        if u > u32::MAX as u128 {
                                            Err(#error_path::TooLarge)
                                        } else {
                                            Self::parse_u32(u as u32)
                                        }
                                    }

                                    #[inline]
                                    fn validate_u128(u: u128) -> Result<(), Self::Error> {
                                        if u > u32::MAX as u128 {
                                            Err(#error_path::TooLarge)
                                        } else {
                                            Self::validate_u32(u as u32)
                                        }
                                    }

                                    #[inline]
                                    fn parse_u32(u: u32) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(#name(u))
                                    }

                                    #[inline]
                                    fn validate_u32(u: u32) -> Result<(), Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(())
                                    }
                                }
                            }
                        }
                        UnsignedIntegerType::U64 => {
                            quote! {
                                impl ValidateUnsignedInteger for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[inline]
                                    fn parse_u128(u: u128) -> Result<Self::Output, Self::Error> {
                                        if u > u64::MAX as u128 {
                                            Err(#error_path::TooLarge)
                                        } else {
                                            Self::parse_u64(u as u64)
                                        }
                                    }

                                    #[inline]
                                    fn validate_u128(u: u128) -> Result<(), Self::Error> {
                                        if u > u64::MAX as u128 {
                                            Err(#error_path::TooLarge)
                                        } else {
                                            Self::validate_u64(u as u64)
                                        }
                                    }

                                    #[inline]
                                    fn parse_u64(u: u64) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(#name(u))
                                    }

                                    #[inline]
                                    fn validate_u64(u: u64) -> Result<(), Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(())
                                    }
                                }
                            }
                        }
                        UnsignedIntegerType::U128 => {
                            quote! {
                                impl ValidateUnsignedInteger for #name {
                                    type Error = #error_path;
                                    type Output = Self;

                                    #[inline]
                                    fn parse_u128(u: u128) -> Result<Self::Output, Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(#name(u))
                                    }

                                    #[inline]
                                    fn validate_u128(u: u128) -> Result<(), Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(())
                                    }
                                }
                            }
                        }
                    }
                };

                let serde_impl = if cfg!(feature = "serde") {
                    let expect = {
                        let mut s = String::from("a unsigned_integer");

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
                        match unsigned_integer_type {
                            UnsignedIntegerType::Usize => {
                                quote! {
                                    #[cfg(target_pointer_width = "8")]
                                    {
                                        serializer.serialize_u8(self.0 as u8)
                                    }

                                    #[cfg(target_pointer_width = "16")]
                                    {
                                        serializer.serialize_u16(self.0 as u16)
                                    }

                                    #[cfg(target_pointer_width = "32")]
                                    {
                                        serializer.serialize_u32(self.0 as u32)
                                    }

                                    #[cfg(target_pointer_width = "64")]
                                    {
                                        serializer.serialize_u64(self.0 as u64)
                                    }

                                    #[cfg(target_pointer_width = "128")]
                                    {
                                        validators_prelude::serde_if_integer128! {
                                            return serializer.serialize_u128(self.0 as u128);
                                        }

                                        unreachable!("the `integer128` feature of the `serde` crate needs to be enabled")
                                    }
                                }
                            }
                            UnsignedIntegerType::U8 => {
                                quote! {
                                    serializer.serialize_u8(self.0)
                                }
                            }
                            UnsignedIntegerType::U16 => {
                                quote! {
                                    serializer.serialize_u16(self.0)
                                }
                            }
                            UnsignedIntegerType::U32 => {
                                quote! {
                                    serializer.serialize_u32(self.0)
                                }
                            }
                            UnsignedIntegerType::U64 => {
                                quote! {
                                    serializer.serialize_u64(self.0)
                                }
                            }
                            UnsignedIntegerType::U128 => {
                                quote! {
                                    validators_prelude::serde_if_integer128! {
                                        return serializer.serialize_u128(self.0);
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
                                    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateUnsignedInteger>::parse_u8(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    #[inline]
                                    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateUnsignedInteger>::parse_u16(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    #[inline]
                                    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateUnsignedInteger>::parse_u32(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    #[inline]
                                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateUnsignedInteger>::parse_u64(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    validators_prelude::serde_if_integer128! {
                                        #[inline]
                                        fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                                        where
                                            E: validators_prelude::DeError, {
                                            <#name as ValidateUnsignedInteger>::parse_u128(v).map_err(validators_prelude::DeError::custom)
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

                let unsigned_integer_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #validate_unsigned_integer_impl

                    #serde_impl

                    #rocket_impl
                };

                unsigned_integer_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
