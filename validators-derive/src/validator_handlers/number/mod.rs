mod number_attribute;

use number_attribute::NumberAttribute;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{range_option::RangeTokenStream, tri_allow::TriAllow, type_enum::TypeEnum},
    panic,
};

pub(crate) struct NumberHandler;

#[derive(Debug)]
#[allow(dead_code)] // Used for parsing
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::Number);

enum NumberType {
    F32,
    F64,
}

impl ValidatorHandler for NumberHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        if let Data::Struct(data) = ast.data {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() == 1 {
                    let data_type = data.fields.into_iter().next().unwrap().ty;

                    let number_type = {
                        let data_type_name = data_type.to_token_stream().to_string();

                        match data_type_name.as_str() {
                            "f32" => NumberType::F32,
                            "f64" => NumberType::F64,
                            _ => {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM,
                                ))
                            },
                        }
                    };

                    let type_attribute = match number_type {
                        NumberType::F64 => NumberAttribute::build_from_meta::<f64>(&meta)?,
                        NumberType::F32 => NumberAttribute::build_from_meta::<f32>(&meta)?,
                    };

                    if type_attribute.nan.must()
                        && type_attribute.range.inside()
                        && type_attribute.conflict.disallow()
                    {
                        return Err(syn::Error::new(
                            meta.span(),
                            "`nan(Must)` and `range(Inside)` cannot be used together",
                        ));
                    }

                    let mut token_stream = proc_macro2::TokenStream::new();

                    let name = ast.ident;

                    let error_path: Path =
                        syn::parse2(quote! { validators_prelude::NumberError }).unwrap();

                    #[cfg(feature = "test")]
                    {
                        let v_nan = type_attribute.nan;
                        let v_range = &type_attribute.range;

                        token_stream.extend(quote! {
                            impl #name {
                                pub(crate) const V_NAN: validators_prelude::TriAllow = #v_nan;
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
                                        match ::core::cmp::PartialOrd::partial_cmp(&f, &#min) {
                                            Some(::core::cmp::Ordering::Equal) | None => (),
                                            Some(::core::cmp::Ordering::Less) => return Err(#error_path::TooSmall),
                                            Some(::core::cmp::Ordering::Greater) => return Err(#error_path::TooLarge),
                                        }
                                    }
                                } else {
                                    let mut token_stream = proc_macro2::TokenStream::new();

                                    if let Some(min) = min {
                                        token_stream.extend(quote! {
                                            if f < #min {
                                                return Err(#error_path::TooSmall);
                                            }
                                        });
                                    }

                                    if let Some(max) = max {
                                        token_stream.extend(if *inclusive {
                                            quote! {
                                                if f > #max {
                                                    return Err(#error_path::TooLarge);
                                                }
                                            }
                                        } else {
                                            quote! {
                                                if f >= #max {
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
                                        if f == #min {
                                            return Err(#error_path::Forbidden);
                                        }
                                    }
                                } else {
                                    match min {
                                        Some(min) => match max {
                                            Some(max) => {
                                                if *inclusive {
                                                    quote! {
                                                        if (#min..=#max).contains(&f){
                                                            return Err(#error_path::Forbidden);
                                                        }
                                                    }
                                                } else {
                                                    quote! {
                                                        if (#min..#max).contains(&f){
                                                            return Err(#error_path::Forbidden);
                                                        }
                                                    }
                                                }
                                            },
                                            None => {
                                                quote! {
                                                    if f >= #min {
                                                        return Err(#error_path::Forbidden);
                                                    }
                                                }
                                            },
                                        },
                                        None => match max {
                                            Some(max) => {
                                                if *inclusive {
                                                    quote! {
                                                        if f <= #max {
                                                            return Err(#error_path::Forbidden);
                                                        }
                                                    }
                                                } else {
                                                    quote! {
                                                        if f < #max {
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

                    let handle_nan = {
                        match type_attribute.nan {
                            TriAllow::Allow => quote! {},
                            TriAllow::Must => {
                                quote! {
                                    if !f.is_nan() {
                                        return Err(#error_path::NaNMust);
                                    }
                                }
                            },
                            TriAllow::Disallow => {
                                quote! {
                                    if f.is_nan() {
                                        return Err(#error_path::NaNDisallow);
                                    }
                                }
                            },
                        }
                    };

                    token_stream.extend(quote! {
                        impl #name {
                            fn v_parse_str(s: &str) -> Result<#data_type, #error_path> {
                                use ::core::str::FromStr;

                                let f = FromStr::from_str(s)?;

                                Self::v_parse_f(f)?;

                                Ok(f)
                            }

                            fn v_parse_f(f: #data_type) -> Result<(), #error_path> {
                                #handle_range

                                #handle_nan

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

                    token_stream.extend(match number_type {
                        NumberType::F64 => {
                            quote! {
                                impl ValidateNumber for #name {
                                    type Error = #error_path;

                                    #[inline]
                                    fn parse_f64(f: f64) -> Result<Self, Self::Error> {
                                        Self::v_parse_f(f)?;

                                        Ok(Self(f))
                                    }

                                    #[inline]
                                    fn validate_f64(f: f64) -> Result<(), Self::Error> {
                                        Self::v_parse_f(f)?;

                                        Ok(())
                                    }
                                }
                            }
                        },
                        NumberType::F32 => {
                            quote! {
                                impl ValidateNumber for #name {
                                    type Error = #error_path;

                                    #[inline]
                                    fn parse_f64(f: f64) -> Result<Self, Self::Error> {
                                        Self::parse_f32(f as f32)
                                    }

                                    #[inline]
                                    fn validate_f64(f: f64) -> Result<(), Self::Error> {
                                        Self::validate_f32(f as f32)
                                    }

                                    #[inline]
                                    fn parse_f32(f: f32) -> Result<Self, Self::Error> {
                                        Self::v_parse_f(f)?;

                                        Ok(Self(f))
                                    }

                                    #[inline]
                                    fn validate_f32(f: f32) -> Result<(), Self::Error> {
                                        Self::v_parse_f(f)?;

                                        Ok(())
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
                                let mut s = String::from("a number");

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

                                match type_attribute.nan {
                                    TriAllow::Allow => (),
                                    TriAllow::Must => {
                                        s.push_str(" which must be NaN");
                                    },
                                    TriAllow::Disallow => {
                                        s.push_str(" which must not be NaN");
                                    },
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
                                            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateNumber>::parse_f64(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateNumber>::parse_f32(v).map_err(validators_prelude::serde::de::Error::custom)
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
