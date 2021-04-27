use alloc::boxed::Box;

use crate::proc_macro::TokenStream;
use crate::syn::{Data, DeriveInput, Fields, Meta, Path};

use crate::{panic, TypeEnum, Validator};

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::Boolean);
const VALIDATOR: Validator = Validator::boolean;

pub fn boolean_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() != 1 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                let correct_usage_for_attribute = [stringify!(#[validator(boolean)])];

                match meta {
                    Meta::Path(_) => (),
                    _ => panic::attribute_incorrect_format("boolean", &correct_usage_for_attribute),
                }

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::BooleanError }).unwrap();

                let parameters_impl = quote! {};

                let v_parse_str = quote! {
                    #[inline]
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
                };

                let v_parse_char = quote! {
                    #[inline]
                    fn v_parse_char(c: char) -> Result<bool, #error_path> {
                        match c {
                            't' | 'T' | '1' | 'y' | 'Y' => Ok(true),
                            'f' | 'F' | '0' | 'n' | 'N' => Ok(false),
                            _ => Err(#error_path),
                        }
                    }
                };

                let v_parse_i128 = quote! {
                    #[inline]
                    fn v_parse_i128(i: i128) -> Result<bool, #error_path> {
                        match i {
                            1 => Ok(true),
                            0 => Ok(false),
                            _ => Err(#error_path),
                        }
                    }
                };

                let v_parse_u128 = quote! {
                    #[inline]
                    fn v_parse_u128(u: u128) -> Result<bool, #error_path> {
                        match u {
                            1 => Ok(true),
                            0 => Ok(false),
                            _ => Err(#error_path),
                        }
                    }
                };

                let parse_impl = quote! {
                    impl #name {
                        #v_parse_str

                        #v_parse_char

                        #v_parse_i128

                        #v_parse_u128
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

                let validate_char_impl = quote! {
                    impl ValidateChar for #name {
                        type Error = #error_path;
                        type Output = Self;

                        #[inline]
                        fn parse_char(c: char) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_char(c)?))
                        }

                        #[inline]
                        fn validate_char(c: char) -> Result<(), Self::Error> {
                            Self::v_parse_char(c)?;

                            Ok(())
                        }
                    }
                };

                let validate_signed_integer_impl = quote! {
                    impl ValidateSignedInteger for #name {
                        type Error = #error_path;
                        type Output = Self;

                        #[inline]
                        fn parse_i128(i: i128) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_i128(i)?))
                        }

                        #[inline]
                        fn validate_i128(i: i128) -> Result<(), Self::Error> {
                            Self::v_parse_i128(i)?;

                            Ok(())
                        }
                    }
                };

                let validate_unsigned_integer_impl = quote! {
                    impl ValidateUnsignedInteger for #name {
                        type Error = #error_path;
                        type Output = Self;

                        #[inline]
                        fn parse_u128(u: u128) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_u128(u)?))
                        }

                        #[inline]
                        fn validate_u128(u: u128) -> Result<(), Self::Error> {
                            Self::v_parse_u128(u)?;

                            Ok(())
                        }
                    }
                };

                let serde_impl = if cfg!(feature = "serde") {
                    let expect = "a boolean, a boolean string like \"true\", \"false\", \"0\", \"1\", \"on\", \"off\", \"yes\", \"no\", or a number `0` or `1`";

                    quote! {
                        impl validators_prelude::Serialize for #name {
                            #[inline]
                            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                where
                                    S: validators_prelude::Serializer, {
                                serializer.serialize_bool(self.0)
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

                                    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        Ok(#name(v))
                                    }

                                    #[inline]
                                    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateChar>::parse_char(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    #[inline]
                                    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                         <#name as ValidateSignedInteger>::parse_i64(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    #[inline]
                                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateUnsignedInteger>::parse_u64(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    validators_prelude::serde_if_integer128! {
                                        #[inline]
                                        fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
                                        where
                                            E: validators_prelude::DeError,
                                        {
                                            <#name as ValidateSignedInteger>::parse_i128(v).map_err(validators_prelude::DeError::custom)
                                        }

                                        #[inline]
                                        fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                                        where
                                            E: validators_prelude::DeError,
                                        {
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

                let boolean_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #validate_char_impl

                    #validate_signed_integer_impl

                    #validate_unsigned_integer_impl

                    #serde_impl

                    #rocket_impl
                };

                boolean_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
