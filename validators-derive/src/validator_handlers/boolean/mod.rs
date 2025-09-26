use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{attributes::basic_attribute::BasicAttribute, type_enum::TypeEnum},
    panic,
};

pub(crate) struct BooleanHandler;

#[derive(Debug)]
#[allow(dead_code)] // Used for parsing
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::Boolean);

impl ValidatorHandler for BooleanHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        #[allow(unused_variables)]
        let type_attribute = BasicAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() == 1 {
                    let mut token_stream = proc_macro2::TokenStream::new();

                    let name = ast.ident;

                    let error_path: Path =
                        syn::parse2(quote! { validators_prelude::BooleanError }).unwrap();

                    token_stream.extend(quote! {
                        impl #name {
                            fn v_parse_str(s: &str) -> Result<bool, #error_path> {
                                let bytes = s.as_bytes();

                                let mut iter = bytes.iter().copied();

                                if let Some(e) = iter.next() {
                                    match e {
                                        b'y' | b'Y' => {
                                            if let Some(e) = iter.next() {
                                                match e {
                                                    b'e' | b'E' => {
                                                        if let Some(e) = iter.next() {
                                                            match e {
                                                                b's' | b'S' => {
                                                                    if iter.next().is_none() {
                                                                        return Ok(true);
                                                                    }
                                                                }
                                                                _ => (),
                                                            }
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            } else {
                                                return Ok(true);
                                            }
                                        }
                                        b't' | b'T' => {
                                            if let Some(e) = iter.next() {
                                                match e {
                                                    b'r' | b'R' => {
                                                        if let Some(e) = iter.next() {
                                                            match e {
                                                                b'u' | b'U' => {
                                                                    if let Some(e) = iter.next() {
                                                                        match e {
                                                                            b'e' | b'E' => {
                                                                                if iter.next().is_none() {
                                                                                    return Ok(true);
                                                                                }
                                                                            }
                                                                            _ => (),
                                                                        }
                                                                    }
                                                                }
                                                                _ => (),
                                                            }
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            } else {
                                                return Ok(true);
                                            }
                                        }
                                        b'1' => {
                                            if iter.next().is_none() {
                                                return Ok(true);
                                            }
                                        }
                                        b'o' | b'O' => {
                                            if let Some(e) = iter.next() {
                                                match e {
                                                    b'n' | b'N' => {
                                                        if iter.next().is_none() {
                                                            return Ok(true);
                                                        }
                                                    }
                                                    b'f' | b'F' => {
                                                        if let Some(e) = iter.next() {
                                                            match e {
                                                                b'f' | b'F' => {
                                                                    if iter.next().is_none() {
                                                                        return Ok(false);
                                                                    }
                                                                }
                                                                _ => (),
                                                            }
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                        }
                                        b'n' | b'N' => {
                                            if let Some(e) = iter.next() {
                                                match e {
                                                    b'o' | b'O' => {
                                                        if iter.next().is_none() {
                                                            return Ok(false);
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            } else {
                                                return Ok(false);
                                            }
                                        }
                                        b'f' | b'F' => {
                                            if let Some(e) = iter.next() {
                                                match e {
                                                    b'a' | b'A' => {
                                                        if let Some(e) = iter.next() {
                                                            match e {
                                                                b'l' | b'L' => {
                                                                    if let Some(e) = iter.next() {
                                                                        match e {
                                                                            b's' | b'S' => {
                                                                                if let Some(e) = iter.next() {
                                                                                    match e {
                                                                                        b'e' | b'E' => {
                                                                                            if iter.next().is_none() {
                                                                                                return Ok(false);
                                                                                            }
                                                                                        }
                                                                                        _ => (),
                                                                                    }
                                                                                }
                                                                            }
                                                                            _ => (),
                                                                        }
                                                                    }
                                                                }
                                                                _ => (),
                                                            }
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            } else {
                                                return Ok(false);
                                            }
                                        }
                                        b'0' => {
                                            if iter.next().is_none() {
                                                return Ok(false);
                                            }
                                        }
                                        _ => (),
                                    }
                                }

                                Err(#error_path)
                            }

                            #[inline]
                            fn v_parse_char(c: char) -> Result<bool, #error_path> {
                                match c {
                                    't' | 'T' | '1' | 'y' | 'Y' => Ok(true),
                                    'f' | 'F' | '0' | 'n' | 'N' => Ok(false),
                                    _ => Err(#error_path),
                                }
                            }

                            #[inline]
                            fn v_parse_i128(i: i128) -> Result<bool, #error_path> {
                                match i {
                                    1 => Ok(true),
                                    0 => Ok(false),
                                    _ => Err(#error_path),
                                }
                            }

                            #[inline]
                            fn v_parse_u128(u: u128) -> Result<bool, #error_path> {
                                match u {
                                    1 => Ok(true),
                                    0 => Ok(false),
                                    _ => Err(#error_path),
                                }
                            }

                            #[inline]
                            fn v_parse_bool(b: bool) -> Result<bool, #error_path> {
                                Ok(b)
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

                        impl ValidateChar for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_char(c: char) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_char(c)?))
                            }

                            #[inline]
                            fn validate_char(c: char) -> Result<(), Self::Error> {
                                Self::v_parse_char(c)?;

                                Ok(())
                            }
                        }

                        impl ValidateSignedInteger for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_i128(i: i128) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_i128(i)?))
                            }

                            #[inline]
                            fn validate_i128(i: i128) -> Result<(), Self::Error> {
                                Self::v_parse_i128(i)?;

                                Ok(())
                            }
                        }

                        impl ValidateUnsignedInteger for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_u128(u: u128) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_u128(u)?))
                            }

                            #[inline]
                            fn validate_u128(u: u128) -> Result<(), Self::Error> {
                                Self::v_parse_u128(u)?;

                                Ok(())
                            }
                        }

                        impl ValidateBoolean for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_bool(b: bool) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_bool(b)?))
                            }

                            #[inline]
                            fn validate_bool(b: bool) -> Result<(), Self::Error> {
                                Self::v_parse_bool(b)?;

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
                                        serializer.serialize_bool(self.0)
                                    }
                                }
                            });
                        }

                        if type_attribute.serde_options.deserialize {
                            let expect = "a boolean, a boolean string like \"true\", \"false\", \
                                          \"0\", \"1\", \"on\", \"off\", \"yes\", \"no\", or a \
                                          number `0` or `1`";

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

                                            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                Ok(#name(v))
                                            }

                                            #[inline]
                                            fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateChar>::parse_char(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                 <#name as ValidateSignedInteger>::parse_i64(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateUnsignedInteger>::parse_u64(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error,
                                            {
                                                <#name as ValidateSignedInteger>::parse_i128(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error,
                                            {
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
