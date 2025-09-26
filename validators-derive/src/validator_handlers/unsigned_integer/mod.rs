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

pub(crate) struct UnsignedIntegerHandler;

#[derive(Debug)]
#[allow(dead_code)] // Used for parsing
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::SignedInteger);

enum UnsignedIntegerType {
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

impl ValidatorHandler for UnsignedIntegerHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        if let Data::Struct(data) = ast.data {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() == 1 {
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
                            _ => {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM,
                                ))
                            },
                        }
                    };

                    let type_attribute = match unsigned_integer_type {
                        UnsignedIntegerType::U128 => {
                            RangeAttribute::build_from_meta::<u128>(&meta)?
                        },
                        UnsignedIntegerType::U64 => RangeAttribute::build_from_meta::<u64>(&meta)?,
                        UnsignedIntegerType::U32 => RangeAttribute::build_from_meta::<u32>(&meta)?,
                        UnsignedIntegerType::U16 => RangeAttribute::build_from_meta::<u16>(&meta)?,
                        UnsignedIntegerType::U8 => RangeAttribute::build_from_meta::<u8>(&meta)?,
                        UnsignedIntegerType::Usize => {
                            RangeAttribute::build_from_meta::<usize>(&meta)?
                        },
                    };

                    let mut token_stream = proc_macro2::TokenStream::new();

                    let name = ast.ident;

                    let error_path: Path =
                        syn::parse2(quote! { validators_prelude::UnsignedIntegerError }).unwrap();

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
                                        match ::core::cmp::Ord::cmp(&u, &#min) {
                                            ::core::cmp::Ordering::Equal => (),
                                            ::core::cmp::Ordering::Less => return Err(#error_path::TooSmall),
                                            ::core::cmp::Ordering::Greater => return Err(#error_path::TooLarge),
                                        }
                                    }
                                } else {
                                    let mut token_stream = proc_macro2::TokenStream::new();

                                    if let Some(min) = min {
                                        token_stream.extend(quote! {
                                            if u < #min {
                                                return Err(#error_path::TooSmall);
                                            }
                                        });
                                    }

                                    if let Some(max) = max {
                                        token_stream.extend(if *inclusive {
                                            quote! {
                                                if u > #max {
                                                    return Err(#error_path::TooLarge);
                                                }
                                            }
                                        } else {
                                            quote! {
                                                if u >= #max {
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
                                        if u == #min {
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
                                                    if u >= #min {
                                                        return Err(#error_path::Forbidden);
                                                    }
                                                }
                                            },
                                        },
                                        None => match max {
                                            Some(max) => {
                                                if *inclusive {
                                                    quote! {
                                                        if u <= #max {
                                                            return Err(#error_path::Forbidden);
                                                        }
                                                    }
                                                } else {
                                                    quote! {
                                                        if u < #max {
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

                                let u = FromStr::from_str(s)?;

                                Self::v_parse_u(u)?;

                                Ok(u)
                            }

                            fn v_parse_u(u: #data_type) -> Result<(), #error_path> {
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

                    token_stream.extend(match unsigned_integer_type {
                        UnsignedIntegerType::U128 => {
                            quote! {
                                impl ValidateUnsignedInteger for #name {
                                    type Error = #error_path;

                                    #[inline]
                                    fn parse_u128(u: u128) -> Result<Self, Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(Self(u))
                                    }

                                    #[inline]
                                    fn validate_u128(u: u128) -> Result<(), Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(())
                                    }
                                }
                            }
                        },
                        UnsignedIntegerType::U64 => {
                            quote! {
                                impl ValidateUnsignedInteger for #name {
                                    type Error = #error_path;

                                    #[inline]
                                    fn parse_u128(u: u128) -> Result<Self, Self::Error> {
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
                                    fn parse_u64(u: u64) -> Result<Self, Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(Self(u))
                                    }

                                    #[inline]
                                    fn validate_u64(u: u64) -> Result<(), Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(())
                                    }
                                }
                            }
                        },
                        UnsignedIntegerType::U32 => {
                            quote! {
                                impl ValidateUnsignedInteger for #name {
                                    type Error = #error_path;

                                    #[inline]
                                    fn parse_u128(u: u128) -> Result<Self, Self::Error> {
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
                                    fn parse_u32(u: u32) -> Result<Self, Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(Self(u))
                                    }

                                    #[inline]
                                    fn validate_u32(u: u32) -> Result<(), Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(())
                                    }
                                }
                            }
                        },
                        UnsignedIntegerType::U16 => {
                            quote! {
                                impl ValidateUnsignedInteger for #name {
                                    type Error = #error_path;

                                    #[inline]
                                    fn parse_u128(u: u128) -> Result<Self, Self::Error> {
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
                                    fn parse_u16(u: u16) -> Result<Self, Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(Self(u))
                                    }

                                    #[inline]
                                    fn validate_u16(u: u16) -> Result<(), Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(())
                                    }
                                }
                            }
                        },
                        UnsignedIntegerType::U8 => {
                            quote! {
                                impl ValidateUnsignedInteger for #name {
                                    type Error = #error_path;

                                    #[inline]
                                    fn parse_u128(u: u128) -> Result<Self, Self::Error> {
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
                                    fn parse_u8(u: u8) -> Result<Self, Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(Self(u))
                                    }

                                    #[inline]
                                    fn validate_u8(u: u8) -> Result<(), Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(())
                                    }
                                }
                            }
                        },
                        UnsignedIntegerType::Usize => {
                            quote! {
                                impl ValidateUnsignedInteger for #name {
                                    type Error = #error_path;

                                    #[cfg(not(target_pointer_width = "128"))]
                                    #[inline]
                                    fn parse_u128(u: u128) -> Result<Self, Self::Error> {
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
                                    fn parse_usize(u: usize) -> Result<Self, Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(Self(u))
                                    }

                                    #[inline]
                                    fn validate_usize(u: usize) -> Result<(), Self::Error> {
                                        Self::v_parse_u(u)?;

                                        Ok(())
                                    }

                                    #[cfg(target_pointer_width = "8")]
                                    fn parse_u8(u: u8) -> Result<Self, Self::Error> {
                                        Self::parse_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "16")]
                                    fn parse_u16(u: u16) -> Result<Self, Self::Error> {
                                        Self::parse_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "32")]
                                    fn parse_u32(u: u32) -> Result<Self, Self::Error> {
                                        Self::parse_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "64")]
                                    fn parse_u64(u: u64) -> Result<Self, Self::Error> {
                                        Self::parse_usize(u as usize)
                                    }

                                    #[cfg(target_pointer_width = "128")]
                                    fn parse_u128(u: u128) -> Result<Self, Self::Error> {
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
                                let mut s = String::from("a unsigned integer");

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
                                            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateUnsignedInteger>::parse_u8(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateUnsignedInteger>::parse_u16(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateUnsignedInteger>::parse_u32(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateUnsignedInteger>::parse_u64(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateUnsignedInteger>::parse_u128(v).map_err(validators_prelude::serde::de::Error::custom)
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
