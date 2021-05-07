use core::fmt::Write;

use alloc::boxed::Box;
use alloc::string::{String, ToString};

use crate::proc_macro::TokenStream;
use crate::quote::ToTokens;
use crate::syn::{Data, DeriveInput, Fields, Lit, Meta, NestedMeta, Path};

use crate::{panic, TypeEnum, Validator};

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::String);
const VALIDATOR: Validator = Validator::text;

pub fn text_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() != 1 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                let mut min: Option<usize> = None;
                let mut trimmed_min: Option<usize> = None;
                let mut max: Option<usize> = None;

                let correct_usage_for_attribute = [stringify!(#[validator(text)])];

                let correct_usage_for_char_length = [
                    stringify!(#[validator(text(char_length(max = 100)))]),
                    stringify!(#[validator(text(char_length(trimmed_min = 1)))]),
                    stringify!(#[validator(text(char_length(min = 5, max = 12)))]),
                    stringify!(#[validator(text(char_length(trimmed_min = 1, min = 5, max = 12)))]),
                ];

                let correct_usage_for_byte_length = [
                    stringify!(#[validator(text(byte_length(max = 100)))]),
                    stringify!(#[validator(text(byte_length(trimmed_min = 1)))]),
                    stringify!(#[validator(text(byte_length(min = 5, max = 12)))]),
                    stringify!(#[validator(text(byte_length(trimmed_min = 1, min = 5, max = 12)))]),
                ];

                let mut is_byte_length = false;
                let mut length_name = "char_length";
                let mut length_usage = &correct_usage_for_char_length;

                match meta {
                    Meta::Path(_) => (),
                    Meta::List(list) => {
                        let mut length_is_set = false;

                        for p in list.nested.iter() {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    let meta_name = meta.path().into_token_stream().to_string();

                                    match meta_name.as_str() {
                                        "char_length" | "byte_length" => {
                                            if length_is_set {
                                                panic::reset_parameter(length_name);
                                            }

                                            length_is_set = true;

                                            if meta_name.eq("byte_length") {
                                                is_byte_length = true;
                                                length_name = "byte_length";
                                                length_usage = &correct_usage_for_byte_length;
                                            }

                                            if let Meta::List(list) = meta {
                                                let char_length = list.nested.len();

                                                if (1..=3).contains(&char_length) {
                                                    let mut min_is_set = false;
                                                    let mut trimmed_min_is_set = false;
                                                    let mut max_is_set = false;

                                                    for p in list.nested.iter() {
                                                        match p {
                                                            NestedMeta::Meta(meta) => {
                                                                if let Meta::NameValue(name_value) =
                                                                    meta
                                                                {
                                                                    match &name_value.lit {
                                                                        Lit::Int(i) => {
                                                                            if let Some(ident) =
                                                                                meta.path()
                                                                                    .get_ident()
                                                                            {
                                                                                if ident == "min" {
                                                                                    if min_is_set {
                                                                                        panic::reset_parameter("min");
                                                                                    }

                                                                                    min_is_set =
                                                                                        true;

                                                                                    min = Some(
                                                                                        i.base10_digits().parse().unwrap(),
                                                                                    );
                                                                                } else if ident
                                                                                    == "trimmed_min"
                                                                                {
                                                                                    if trimmed_min_is_set {
                                                                                        panic::reset_parameter("trimmed_min");
                                                                                    }

                                                                                    trimmed_min_is_set = true;

                                                                                    trimmed_min = Some(
                                                                                        i.base10_digits().parse().unwrap(),
                                                                                    );
                                                                                } else if ident
                                                                                    == "max"
                                                                                {
                                                                                    if max_is_set {
                                                                                        panic::reset_parameter("max");
                                                                                    }

                                                                                    max_is_set =
                                                                                        true;

                                                                                    max = Some(
                                                                                        i.base10_digits().parse().unwrap(),
                                                                                    );
                                                                                } else {
                                                                                    panic::parameter_incorrect_format(length_name, length_usage);
                                                                                }
                                                                            } else {
                                                                                panic::parameter_incorrect_format(length_name, length_usage);
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            panic::parameter_incorrect_format(length_name, length_usage);
                                                                        }
                                                                    }
                                                                } else {
                                                                    panic::parameter_incorrect_format(length_name, length_usage);
                                                                }
                                                            }
                                                            NestedMeta::Lit(_) => {
                                                                panic::parameter_incorrect_format(
                                                                    length_name,
                                                                    length_usage,
                                                                );
                                                            }
                                                        }
                                                    }

                                                    if let Some(min) = min {
                                                        if let Some(trimmed_min) = trimmed_min {
                                                            if trimmed_min > min {
                                                                panic!(
                                                                    "{} > {} (trimmed_min > min)",
                                                                    trimmed_min, min
                                                                );
                                                            }
                                                        }

                                                        if let Some(max) = max {
                                                            if min > max {
                                                                panic!(
                                                                    "{} > {} (min > max)",
                                                                    min, max
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else if let Some(trimmed_min) = trimmed_min {
                                                    if let Some(max) = max {
                                                        if trimmed_min > max {
                                                            panic!(
                                                                "{} > {} (trimmed_min > max)",
                                                                trimmed_min, max
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    panic::parameter_incorrect_format(
                                                        "char_length",
                                                        &correct_usage_for_char_length,
                                                    );
                                                }
                                            } else {
                                                panic::parameter_incorrect_format(
                                                    "char_length",
                                                    &correct_usage_for_char_length,
                                                );
                                            }
                                        }
                                        _ => panic::unknown_parameter("text", meta_name.as_str()),
                                    }
                                }
                                NestedMeta::Lit(_) => {
                                    panic::attribute_incorrect_format(
                                        "text",
                                        &correct_usage_for_attribute,
                                    )
                                }
                            }
                        }
                    }
                    Meta::NameValue(_) => {
                        panic::attribute_incorrect_format("text", &correct_usage_for_attribute)
                    }
                }

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::TextError }).unwrap();

                let min_expr = {
                    match min {
                        Some(min) => {
                            quote! {
                                Some(#min)
                            }
                        }
                        None => {
                            quote! {
                                None
                            }
                        }
                    }
                };

                let trimmed_min_expr = {
                    match trimmed_min {
                        Some(trimmed_min) => {
                            quote! {
                                Some(#trimmed_min)
                            }
                        }
                        None => {
                            quote! {
                                None
                            }
                        }
                    }
                };

                let max_expr = {
                    match max {
                        Some(max) => {
                            quote! {
                                Some(#max)
                            }
                        }
                        None => {
                            quote! {
                                None
                            }
                        }
                    }
                };

                let parameters_impl = if is_byte_length {
                    quote! {
                        impl #name {
                            pub(crate) const V_BYTE_LENGTH_MIN: Option<usize> = #min_expr;
                            pub(crate) const V_BYTE_LENGTH_TRIMMED_MIN: Option<usize> = #trimmed_min_expr;
                            pub(crate) const V_BYTE_LENGTH_MAX: Option<usize> = #max_expr;
                        }
                    }
                } else {
                    quote! {
                        impl #name {
                            pub(crate) const V_CHAR_LENGTH_MIN: Option<usize> = #min_expr;
                            pub(crate) const V_CHAR_LENGTH_TRIMMED_MIN: Option<usize> = #trimmed_min_expr;
                            pub(crate) const V_CHAR_LENGTH_MAX: Option<usize> = #max_expr;
                        }
                    }
                };

                let handle_str = {
                    match max {
                        Some(max) => {
                            match min {
                                Some(min) => {
                                    match trimmed_min {
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
                                        }
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
                                        }
                                    }
                                }
                                None => {
                                    match trimmed_min {
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
                                        }
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
                                        }
                                    }
                                }
                            }
                        }
                        None => {
                            match min {
                                Some(min) => {
                                    match trimmed_min {
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
                                        }
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
                                        }
                                    }
                                }
                                None => {
                                    match trimmed_min {
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
                                        }
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
                                        }
                                    }
                                }
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
                    let expect = {
                        let mut s = String::from("a text");

                        if min.is_some() || max.is_some() || trimmed_min.is_some() {
                            s.push_str(" in ");

                            if let Some(min) = min {
                                s.write_fmt(format_args!("{}", min)).unwrap();
                            }

                            if let Some(trimmed_min) = trimmed_min {
                                s.write_fmt(format_args!("({})", trimmed_min)).unwrap();
                            }

                            s.push_str("..");

                            if let Some(max) = max {
                                s.write_fmt(format_args!("={}", max)).unwrap();
                            }
                        }

                        s
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

                let text_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #serde_impl

                    #rocket_impl
                };

                text_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
