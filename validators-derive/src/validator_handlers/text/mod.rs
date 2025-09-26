use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{attributes::utf8_attribute::Utf8Attribute, length::Length, type_enum::TypeEnum},
    panic,
};

pub(crate) struct TextHandler;

#[derive(Debug)]
#[allow(dead_code)] // Used for parsing
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::String);

impl ValidatorHandler for TextHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = Utf8Attribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() == 1 {
                    let mut token_stream = proc_macro2::TokenStream::new();

                    let name = ast.ident;

                    let error_path: Path =
                        syn::parse2(quote! { validators_prelude::TextError }).unwrap();

                    let (is_byte_length, min, trimmed_min, max) = if let Some((
                        is_byte_length,
                        Length {
                            min,
                            trimmed_min,
                            max,
                        },
                    )) = type_attribute.length
                    {
                        (is_byte_length, min, trimmed_min, max)
                    } else {
                        (false, None, None, None)
                    };

                    #[cfg(feature = "test")]
                    {
                        let v_min = crate::common::test::OptionToken(min);

                        let v_trimmed_min = crate::common::test::OptionToken(trimmed_min);

                        let v_max = crate::common::test::OptionToken(max);

                        token_stream.extend(if is_byte_length {
                            quote! {
                                impl #name {
                                    pub(crate) const V_BYTE_LENGTH_MIN: Option<usize> = #v_min;
                                    pub(crate) const V_BYTE_LENGTH_TRIMMED_MIN: Option<usize> = #v_trimmed_min;
                                    pub(crate) const V_BYTE_LENGTH_MAX: Option<usize> = #v_max;
                                }
                            }
                        } else {
                            quote! {
                                impl #name {
                                    pub(crate) const V_CHAR_LENGTH_MIN: Option<usize> = #v_min;
                                    pub(crate) const V_CHAR_LENGTH_TRIMMED_MIN: Option<usize> = #v_trimmed_min;
                                    pub(crate) const V_CHAR_LENGTH_MAX: Option<usize> = #v_max;
                                }
                            }
                        });
                    }

                    let handle_str = {
                        match max {
                            Some(max) => match min {
                                Some(min) => match trimmed_min {
                                    Some(trimmed_min) => {
                                        if is_byte_length {
                                            let handle_trimmed_empty = if trimmed_min == 0 {
                                                quote! {
                                                    if counter < #min {
                                                        Err(#error_path::TooShort)
                                                    } else {
                                                        Ok(())
                                                    }
                                                }
                                            } else {
                                                quote! {
                                                    Err(#error_path::TooShort)
                                                }
                                            };

                                            quote! {
                                                let mut chars = s.chars();
                                                let mut counter = 0;

                                                while let Some(c) = chars.next() {
                                                    let len = c.len_utf8();

                                                    counter += len;

                                                    if counter > #max {
                                                        return Err(#error_path::TooLong);
                                                    }

                                                    if !c.is_whitespace() {
                                                        match c {
                                                            '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                return Err(#error_path::Invalid);
                                                            }
                                                            _ => (),
                                                        }

                                                        let mut trimmed_counter = len;
                                                        let mut temp_counter = 0;

                                                        while let Some(c) = chars.next() {
                                                            match c {
                                                                '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                    return Err(#error_path::Invalid);
                                                                }
                                                                _ => {
                                                                    let len = c.len_utf8();

                                                                    counter += len;

                                                                    if counter > #max {
                                                                        return Err(#error_path::TooLong);
                                                                    }

                                                                    if c.is_whitespace() {
                                                                        temp_counter += len;
                                                                    } else {
                                                                        trimmed_counter += temp_counter + len;
                                                                        temp_counter = 0;
                                                                    }
                                                                },
                                                            }
                                                        }

                                                        if trimmed_counter >= #trimmed_min && counter >= #min {
                                                            if counter <= #max {
                                                                return Ok(());
                                                            } else {
                                                                return Err(#error_path::TooLong);
                                                            }
                                                        } else {
                                                            return Err(#error_path::TooShort);
                                                        }
                                                    }
                                                }

                                                #handle_trimmed_empty
                                            }
                                        } else {
                                            let handle_trimmed_empty = if trimmed_min == 0 {
                                                quote! {
                                                    if counter < #min {
                                                        Err(#error_path::TooShort)
                                                    } else if counter > #max {
                                                        Err(#error_path::TooLong)
                                                    } else {
                                                        Ok(())
                                                    }
                                                }
                                            } else {
                                                quote! {
                                                    Err(#error_path::TooShort)
                                                }
                                            };

                                            quote! {
                                                let mut chars = s.chars().take(#max + 1);
                                                let mut counter = 0;

                                                while let Some(c) = chars.next() {
                                                    counter += 1;

                                                    if !c.is_whitespace() {
                                                        match c {
                                                            '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                return Err(#error_path::Invalid);
                                                            }
                                                            _ => (),
                                                        }

                                                        let mut trimmed_counter = 1;
                                                        let mut temp_counter = 0;

                                                        while let Some(c) = chars.next() {
                                                            match c {
                                                                '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                    return Err(#error_path::Invalid);
                                                                }
                                                                _ => {
                                                                    counter += 1;

                                                                    if c.is_whitespace() {
                                                                        temp_counter += 1;
                                                                    } else {
                                                                        trimmed_counter += temp_counter + 1;
                                                                        temp_counter = 0;
                                                                    }
                                                                },
                                                            }
                                                        }

                                                        if trimmed_counter >= #trimmed_min && counter >= #min {
                                                            if counter <= #max {
                                                                return Ok(());
                                                            } else {
                                                                return Err(#error_path::TooLong);
                                                            }
                                                        } else {
                                                            return Err(#error_path::TooShort);
                                                        }
                                                    }
                                                }

                                                #handle_trimmed_empty
                                            }
                                        }
                                    },
                                    None => {
                                        if is_byte_length {
                                            quote! {
                                                if s.len() > #max {
                                                    return Err(#error_path::TooLong);
                                                } else if s.len() < #min {
                                                    return Err(#error_path::TooShort);
                                                }

                                                for e in s.bytes() {
                                                    match e {
                                                        b'\x00'..=b'\x08' | b'\x0C' | b'\x0E'..=b'\x1F' | b'\x7F' => {
                                                            return Err(#error_path::Invalid);
                                                        }
                                                        _ => (),
                                                    }
                                                }

                                                Ok(())
                                            }
                                        } else {
                                            quote! {
                                                let mut chars = s.chars().take(#max + 1);
                                                let mut counter = 0;

                                                while let Some(c) = chars.next() {
                                                    match c {
                                                        '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                            return Err(#error_path::Invalid);
                                                        }
                                                        _ => counter += 1,
                                                    }
                                                }

                                                if counter < #min {
                                                    Err(#error_path::TooShort)
                                                } else if counter > #max {
                                                    Err(#error_path::TooLong)
                                                } else {
                                                    Ok(())
                                                }
                                            }
                                        }
                                    },
                                },
                                None => match trimmed_min {
                                    Some(trimmed_min) => {
                                        if is_byte_length {
                                            let handle_trimmed_empty = if trimmed_min == 0 {
                                                quote! {
                                                    Ok(())
                                                }
                                            } else {
                                                quote! {
                                                    Err(#error_path::TooShort)
                                                }
                                            };

                                            quote! {
                                                let mut chars = s.chars();
                                                let mut counter = 0;

                                                while let Some(c) = chars.next() {
                                                    let len = c.len_utf8();

                                                    counter += len;

                                                    if counter > #max {
                                                        return Err(#error_path::TooLong);
                                                    }

                                                    if !c.is_whitespace() {
                                                        match c {
                                                            '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                return Err(#error_path::Invalid);
                                                            }
                                                            _ => (),
                                                        }

                                                        let mut trimmed_counter = len;
                                                        let mut temp_counter = 0;

                                                        while let Some(c) = chars.next() {
                                                            match c {
                                                                '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                    return Err(#error_path::Invalid);
                                                                }
                                                                _ => {
                                                                    let len = c.len_utf8();

                                                                    counter += len;

                                                                    if c.is_whitespace() {
                                                                        temp_counter += len;
                                                                    } else {
                                                                        trimmed_counter += temp_counter + len;
                                                                        temp_counter = 0;
                                                                    }
                                                                },
                                                            }
                                                        }

                                                        if trimmed_counter >= #trimmed_min {
                                                            if counter <= #max {
                                                                return Ok(());
                                                            } else {
                                                                return Err(#error_path::TooLong);
                                                            }
                                                        } else {
                                                            return Err(#error_path::TooShort);
                                                        }
                                                    }
                                                }

                                                #handle_trimmed_empty
                                            }
                                        } else {
                                            let handle_trimmed_empty = if trimmed_min == 0 {
                                                quote! {
                                                    if counter <= #max {
                                                        Ok(())
                                                    } else {
                                                        Err(#error_path::TooLong)
                                                    }
                                                }
                                            } else {
                                                quote! {
                                                    Err(#error_path::TooShort)
                                                }
                                            };

                                            quote! {
                                                let mut chars = s.chars().take(#max + 1);
                                                let mut counter = 0;

                                                while let Some(c) = chars.next() {
                                                    counter += 1;

                                                    if !c.is_whitespace() {
                                                        match c {
                                                            '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                return Err(#error_path::Invalid);
                                                            }
                                                            _ => (),
                                                        }

                                                        let mut trimmed_counter = 1;
                                                        let mut temp_counter = 0;

                                                        while let Some(c) = chars.next() {
                                                            match c {
                                                                '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                    return Err(#error_path::Invalid);
                                                                }
                                                                _ => {
                                                                    counter += 1;

                                                                    if c.is_whitespace() {
                                                                        temp_counter += 1;
                                                                    } else {
                                                                        trimmed_counter += temp_counter + 1;
                                                                        temp_counter = 0;
                                                                    }
                                                                },
                                                            }
                                                        }

                                                        if trimmed_counter >= #trimmed_min {
                                                            if counter <= #max {
                                                                return Ok(());
                                                            } else {
                                                                return Err(#error_path::TooLong);
                                                            }
                                                        } else {
                                                            return Err(#error_path::TooShort);
                                                        }
                                                    }
                                                }

                                                #handle_trimmed_empty
                                            }
                                        }
                                    },
                                    None => {
                                        if is_byte_length {
                                            quote! {
                                                if s.len() > #max {
                                                    return Err(#error_path::TooLong);
                                                }

                                                for e in s.bytes() {
                                                    match e {
                                                        b'\x00'..=b'\x08' | b'\x0C' | b'\x0E'..=b'\x1F' | b'\x7F' => {
                                                            return Err(#error_path::Invalid);
                                                        }
                                                        _ => (),
                                                    }
                                                }

                                                Ok(())
                                            }
                                        } else {
                                            quote! {
                                                let mut chars = s.chars().take(#max + 1);
                                                let mut counter = 0;

                                                while let Some(c) = chars.next() {
                                                    match c {
                                                        '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                            return Err(#error_path::Invalid);
                                                        }
                                                        _ => counter += 1,
                                                    }
                                                }

                                                if counter <= #max {
                                                    Ok(())
                                                } else {
                                                    Err(#error_path::TooLong)
                                                }
                                            }
                                        }
                                    },
                                },
                            },
                            None => match min {
                                Some(min) => match trimmed_min {
                                    Some(trimmed_min) => {
                                        let handle_trimmed_empty = if trimmed_min == 0 {
                                            quote! {
                                                if counter >= #min {
                                                    Ok(())
                                                } else {
                                                    Err(#error_path::TooShort)
                                                }
                                            }
                                        } else {
                                            quote! {
                                                Err(#error_path::TooShort)
                                            }
                                        };

                                        if is_byte_length {
                                            quote! {
                                                let mut chars = s.chars();
                                                let mut counter = 0;

                                                while let Some(c) = chars.next() {
                                                    let len = c.len_utf8();

                                                    counter += len;

                                                    if !c.is_whitespace() {
                                                        match c {
                                                            '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                return Err(#error_path::Invalid);
                                                            }
                                                            _ => (),
                                                        }

                                                        let mut trimmed_counter = len;
                                                        let mut temp_counter = 0;

                                                        while let Some(c) = chars.next() {
                                                            match c {
                                                                '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                    return Err(#error_path::Invalid);
                                                                }
                                                                _ => {
                                                                    let len = c.len_utf8();

                                                                    counter += len;

                                                                    if c.is_whitespace() {
                                                                        temp_counter += len;
                                                                    } else {
                                                                        trimmed_counter += temp_counter + len;
                                                                        temp_counter = 0;
                                                                    }
                                                                },
                                                            }
                                                        }

                                                        if trimmed_counter >= #trimmed_min && counter >= #min {
                                                            return Ok(());
                                                        } else {
                                                            return Err(#error_path::TooShort);
                                                        }
                                                    }
                                                }

                                                #handle_trimmed_empty
                                            }
                                        } else {
                                            quote! {
                                                let mut chars = s.chars();
                                                let mut counter = 0;

                                                while let Some(c) = chars.next() {
                                                    counter += 1;

                                                    if !c.is_whitespace() {
                                                        match c {
                                                            '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                return Err(#error_path::Invalid);
                                                            }
                                                            _ => (),
                                                        }

                                                        let mut trimmed_counter = 1;
                                                        let mut temp_counter = 0;

                                                        while let Some(c) = chars.next() {
                                                            match c {
                                                                '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                    return Err(#error_path::Invalid);
                                                                }
                                                                _ => {
                                                                    counter += 1;

                                                                    if c.is_whitespace() {
                                                                        temp_counter += 1;
                                                                    } else {
                                                                        trimmed_counter += temp_counter + 1;
                                                                        temp_counter = 0;
                                                                    }
                                                                },
                                                            }
                                                        }

                                                        if trimmed_counter >= #trimmed_min && counter >= #min {
                                                            return Ok(());
                                                        } else {
                                                            return Err(#error_path::TooShort);
                                                        }
                                                    }
                                                }

                                                #handle_trimmed_empty
                                            }
                                        }
                                    },
                                    None => {
                                        if is_byte_length {
                                            quote! {
                                                if s.len() < #min {
                                                    return Err(#error_path::TooShort);
                                                }

                                                for e in s.bytes() {
                                                    match e {
                                                        b'\x00'..=b'\x08' | b'\x0C' | b'\x0E'..=b'\x1F' | b'\x7F' => {
                                                            return Err(#error_path::Invalid);
                                                        }
                                                        _ => (),
                                                    }
                                                }

                                                Ok(())
                                            }
                                        } else {
                                            quote! {
                                                let mut chars = s.chars();
                                                let mut counter = 0;

                                                while let Some(c) = chars.next() {
                                                    match c {
                                                        '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                            return Err(#error_path::Invalid);
                                                        }
                                                        _ => counter += 1,
                                                    }
                                                }

                                                if counter >= #min {
                                                    Ok(())
                                                } else {
                                                    Err(#error_path::TooShort)
                                                }
                                            }
                                        }
                                    },
                                },
                                None => match trimmed_min {
                                    Some(trimmed_min) => {
                                        let handle_trimmed_empty = if trimmed_min == 0 {
                                            quote! {
                                                Ok(())
                                            }
                                        } else {
                                            quote! {
                                                Err(#error_path::TooShort)
                                            }
                                        };

                                        if is_byte_length {
                                            quote! {
                                                let mut chars = s.chars();

                                                while let Some(c) = chars.next() {
                                                    if !c.is_whitespace() {
                                                        match c {
                                                            '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                return Err(#error_path::Invalid);
                                                            }
                                                            _ => (),
                                                        }

                                                        let mut trimmed_counter = c.len_utf8();
                                                        let mut temp_counter = 0;

                                                        while let Some(c) = chars.next() {
                                                            match c {
                                                                '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                    return Err(#error_path::Invalid);
                                                                }
                                                                _ => {
                                                                    if c.is_whitespace() {
                                                                        temp_counter += c.len_utf8();
                                                                    } else {
                                                                        trimmed_counter += temp_counter + c.len_utf8();
                                                                        temp_counter = 0;
                                                                    }
                                                                },
                                                            }
                                                        }

                                                        if trimmed_counter >= #trimmed_min {
                                                            return Ok(());
                                                        } else {
                                                            return Err(#error_path::TooShort);
                                                        }
                                                    }
                                                }

                                                #handle_trimmed_empty
                                            }
                                        } else {
                                            quote! {
                                                let mut chars = s.chars();

                                                while let Some(c) = chars.next() {
                                                    if !c.is_whitespace() {
                                                        match c {
                                                            '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                return Err(#error_path::Invalid);
                                                            }
                                                            _ => (),
                                                        }

                                                        let mut trimmed_counter = 1;
                                                        let mut temp_counter = 0;

                                                        while let Some(c) = chars.next() {
                                                            match c {
                                                                '\x00'..='\x08' | '\x0C' | '\x0E'..='\x1F' | '\x7F' => {
                                                                    return Err(#error_path::Invalid);
                                                                }
                                                                _ => {
                                                                    if c.is_whitespace() {
                                                                        temp_counter += 1;
                                                                    } else {
                                                                        trimmed_counter += temp_counter + 1;
                                                                        temp_counter = 0;
                                                                    }
                                                                },
                                                            }
                                                        }

                                                        if trimmed_counter >= #trimmed_min {
                                                            return Ok(());
                                                        } else {
                                                            return Err(#error_path::TooShort);
                                                        }
                                                    }
                                                }

                                                #handle_trimmed_empty
                                            }
                                        }
                                    },
                                    None => {
                                        quote! {
                                            for e in s.bytes() {
                                                match e {
                                                    b'\x00'..=b'\x08' | b'\x0C' | b'\x0E'..=b'\x1F' | b'\x7F' => {
                                                        return Err(#error_path::Invalid);
                                                    }
                                                    _ => (),
                                                }
                                            }

                                            Ok(())
                                        }
                                    },
                                },
                            },
                        }
                    };

                    token_stream.extend(quote! {
                        impl #name {
                            fn v_parse_str(s: &str) -> Result<(), #error_path> {
                                #handle_str
                            }
                        }
                    });

                    token_stream.extend(quote! {
                        impl ValidateString for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_string<S: Into<validators_prelude::String>>(s: S) -> Result<Self, Self::Error> {
                                let s = s.into();

                                Self::v_parse_str(s.as_str())?;

                                Ok(Self(s))
                            }

                            #[inline]
                            fn parse_str<S: AsRef<str>>(s: S) -> Result<Self, Self::Error> {
                                let s = s.as_ref();

                                Self::v_parse_str(s)?;

                                Ok(Self(validators_prelude::String::from(s)))
                            }

                            #[inline]
                            fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error> {
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
                                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                        where
                                            S: validators_prelude::serde::Serializer, {
                                        serializer.serialize_str(self.0.as_str())
                                    }
                                }
                            });
                        }

                        if type_attribute.serde_options.deserialize {
                            use std::fmt::Write;

                            let expect = {
                                let mut s = String::from("a line");

                                if min.is_some() || max.is_some() || trimmed_min.is_some() {
                                    s.push_str(" in ");

                                    if let Some(min) = min {
                                        s.write_fmt(format_args!("{min}")).unwrap();
                                    }

                                    if let Some(trimmed_min) = trimmed_min {
                                        s.write_fmt(format_args!("({trimmed_min})")).unwrap();
                                    }

                                    s.push_str("..");

                                    if let Some(max) = max {
                                        s.write_fmt(format_args!("={max}")).unwrap();
                                    }
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
                                        }

                                        deserializer.deserialize_string(MyVisitor)
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
