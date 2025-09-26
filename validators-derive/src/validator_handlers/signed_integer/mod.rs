use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{
        attributes::range_attribute::RangeAttribute, range_option::RangeTokenStream,
        type_enum::TypeEnum,
    },
    panic,
};

pub(crate) struct SignedIntegerHandler;

#[derive(Debug)]
#[allow(dead_code)] // Used for parsing
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::SignedInteger);

enum SignedIntegerType {
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
}

impl ValidatorHandler for SignedIntegerHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        if let Data::Struct(data) = ast.data {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() == 1 {
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
                            _ => {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM,
                                ))
                            },
                        }
                    };

                    let type_attribute = match signed_integer_type {
                        SignedIntegerType::I128 => RangeAttribute::build_from_meta::<i128>(&meta)?,
                        SignedIntegerType::I64 => RangeAttribute::build_from_meta::<i64>(&meta)?,
                        SignedIntegerType::I32 => RangeAttribute::build_from_meta::<i32>(&meta)?,
                        SignedIntegerType::I16 => RangeAttribute::build_from_meta::<i16>(&meta)?,
                        SignedIntegerType::I8 => RangeAttribute::build_from_meta::<i8>(&meta)?,
                        SignedIntegerType::Isize => {
                            RangeAttribute::build_from_meta::<isize>(&meta)?
                        },
                    };

                    let mut token_stream = proc_macro2::TokenStream::new();

                    let name = ast.ident;

                    let error_path: Path =
                        syn::parse2(quote! { validators_prelude::SignedIntegerError }).unwrap();

                    #[cfg(feature = "test")]
                    {
                        let v_range = &type_attribute.range;

                        token_stream.extend(quote! {
                            impl #name {
                                pub(crate) const V_RANGE: validators_prelude::RangeOption<#data_type> = #v_range;
                            }
                        });
                    }

                    let handle_range = {
                        match &type_attribute.range {
                            RangeTokenStream::Inside {
                                min,
                                max,
                                inclusive,
                                equal,
                            } => {
                                if *equal {
                                    quote! {
                                        match ::core::cmp::Ord::cmp(&i, &#min) {
                                            ::core::cmp::Ordering::Equal => (),
                                            ::core::cmp::Ordering::Less => return Err(#error_path::TooSmall),
                                            ::core::cmp::Ordering::Greater => return Err(#error_path::TooLarge),
                                        }
                                    }
                                } else {
                                    let mut token_stream = proc_macro2::TokenStream::new();

                                    if let Some(min) = min {
                                        token_stream.extend(quote! {
                                            if i < #min {
                                                return Err(#error_path::TooSmall);
                                            }
                                        });
                                    }

                                    if let Some(max) = max {
                                        token_stream.extend(if *inclusive {
                                            quote! {
                                                if i > #max {
                                                    return Err(#error_path::TooLarge);
                                                }
                                            }
                                        } else {
                                            quote! {
                                                if i >= #max {
                                                    return Err(#error_path::TooLarge);
                                                }
                                            }
                                        });
                                    }

                                    token_stream
                                }
                            },
                            RangeTokenStream::Outside {
                                min,
                                max,
                                inclusive,
                                equal,
                            } => {
                                if *equal {
                                    quote! {
                                        if i == #min {
                                            return Err(#error_path::Forbidden);
                                        }
                                    }
                                } else {
                                    match min {
                                        Some(min) => match max {
                                            Some(max) => {
                                                if *inclusive {
                                                    quote! {
                                                        if (#min..=#max).contains(&i) {
                                                            return Err(#error_path::Forbidden);
                                                        }
                                                    }
                                                } else {
                                                    quote! {
                                                        if (#min..#max).contains(&i) {
                                                            return Err(#error_path::Forbidden);
                                                        }
                                                    }
                                                }
                                            },
                                            None => {
                                                quote! {
                                                    if i >= #min {
                                                        return Err(#error_path::Forbidden);
                                                    }
                                                }
                                            },
                                        },
                                        None => match max {
                                            Some(max) => {
                                                if *inclusive {
                                                    quote! {
                                                        if i <= #max {
                                                            return Err(#error_path::Forbidden);
                                                        }
                                                    }
                                                } else {
                                                    quote! {
                                                        if i < #max {
                                                            return Err(#error_path::Forbidden);
                                                        }
                                                    }
                                                }
                                            },
                                            None => {
                                                quote! {}
                                            },
                                        },
                                    }
                                }
                            },
                            RangeTokenStream::Unlimited => quote! {},
                        }
                    };

                    token_stream.extend(quote! {
                        impl #name {
                            fn v_parse_str(s: &str) -> Result<#data_type, #error_path> {
                                use ::core::str::FromStr;

                                let i = FromStr::from_str(s)?;

                                Self::v_parse_i(i)?;

                                Ok(i)
                            }

                            fn v_parse_i(i: #data_type) -> Result<(), #error_path> {
                                #handle_range

                                Ok(())
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

                    token_stream.extend(match signed_integer_type {
                        SignedIntegerType::I128 => {
                            quote! {
                                impl ValidateSignedInteger for #name {
                                    type Error = #error_path;

                                    #[inline]
                                    fn parse_i128(i: i128) -> Result<Self, Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(Self(i))
                                    }

                                    #[inline]
                                    fn validate_i128(i: i128) -> Result<(), Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(())
                                    }
                                }
                            }
                        },
                        SignedIntegerType::I64 => {
                            quote! {
                                impl ValidateSignedInteger for #name {
                                    type Error = #error_path;

                                    #[inline]
                                    fn parse_i128(i: i128) -> Result<Self, Self::Error> {
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
                                    fn parse_i64(i: i64) -> Result<Self, Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(Self(i))
                                    }

                                    #[inline]
                                    fn validate_i64(i: i64) -> Result<(), Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(())
                                    }
                                }
                            }
                        },
                        SignedIntegerType::I32 => {
                            quote! {
                                impl ValidateSignedInteger for #name {
                                    type Error = #error_path;

                                    #[inline]
                                    fn parse_i128(i: i128) -> Result<Self, Self::Error> {
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
                                    fn parse_i32(i: i32) -> Result<Self, Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(Self(i))
                                    }

                                    #[inline]
                                    fn validate_i32(i: i32) -> Result<(), Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(())
                                    }
                                }
                            }
                        },
                        SignedIntegerType::I16 => {
                            quote! {
                                impl ValidateSignedInteger for #name {
                                    type Error = #error_path;

                                    #[inline]
                                    fn parse_i128(i: i128) -> Result<Self, Self::Error> {
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
                                    fn parse_i16(i: i16) -> Result<Self, Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(Self(i))
                                    }

                                    #[inline]
                                    fn validate_i16(i: i16) -> Result<(), Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(())
                                    }
                                }
                            }
                        },
                        SignedIntegerType::I8 => {
                            quote! {
                                impl ValidateSignedInteger for #name {
                                    type Error = #error_path;

                                    #[inline]
                                    fn parse_i128(i: i128) -> Result<Self, Self::Error> {
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
                                    fn parse_i8(i: i8) -> Result<Self, Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(Self(i))
                                    }

                                    #[inline]
                                    fn validate_i8(i: i8) -> Result<(), Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(())
                                    }
                                }
                            }
                        },
                        SignedIntegerType::Isize => {
                            quote! {
                                impl ValidateSignedInteger for #name {
                                    type Error = #error_path;

                                    #[cfg(not(target_pointer_width = "128"))]
                                    #[inline]
                                    fn parse_i128(i: i128) -> Result<Self, Self::Error> {
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
                                    fn parse_isize(i: isize) -> Result<Self, Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(Self(i))
                                    }

                                    #[inline]
                                    fn validate_isize(i: isize) -> Result<(), Self::Error> {
                                        Self::v_parse_i(i)?;

                                        Ok(())
                                    }

                                    #[cfg(target_pointer_width = "8")]
                                    fn parse_i8(i: i8) -> Result<Self, Self::Error> {
                                        Self::parse_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "16")]
                                    fn parse_i16(i: i16) -> Result<Self, Self::Error> {
                                        Self::parse_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "32")]
                                    fn parse_i32(i: i32) -> Result<Self, Self::Error> {
                                        Self::parse_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "64")]
                                    fn parse_i64(i: i64) -> Result<Self, Self::Error> {
                                        Self::parse_isize(i as isize)
                                    }

                                    #[cfg(target_pointer_width = "128")]
                                    fn parse_i128(i: i128) -> Result<Self, Self::Error> {
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
                        },
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
                                let mut s = String::from("a signed integer");

                                match &type_attribute.range {
                                    RangeTokenStream::Inside {
                                        min,
                                        max,
                                        inclusive,
                                        equal: _,
                                    } => {
                                        s.push_str(" in ");

                                        if let Some(min) = min {
                                            s.write_fmt(format_args!("{min}")).unwrap();
                                        }

                                        s.push_str("..");

                                        if let Some(max) = max {
                                            if *inclusive {
                                                s.push('=');
                                            }

                                            s.write_fmt(format_args!("{max}")).unwrap();
                                        }
                                    },
                                    RangeTokenStream::Outside {
                                        min,
                                        max,
                                        inclusive,
                                        equal: _,
                                    } => {
                                        s.push_str(" not in ");

                                        if let Some(min) = min {
                                            s.write_fmt(format_args!("{min}")).unwrap();
                                        }

                                        s.push_str("..");

                                        if let Some(max) = max {
                                            if *inclusive {
                                                s.push('=');
                                            }

                                            s.write_fmt(format_args!("{max}")).unwrap();
                                        }
                                    },
                                    RangeTokenStream::Unlimited => (),
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

                                            #[inline]
                                            fn visit_string<E>(self, v: validators_prelude::String) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateString>::parse_string(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateSignedInteger>::parse_i8(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateSignedInteger>::parse_i16(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateSignedInteger>::parse_i32(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateSignedInteger>::parse_i64(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateSignedInteger>::parse_i128(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }
                                        }

                                        deserializer.deserialize_any(MyVisitor)
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
