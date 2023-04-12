use alloc::{boxed::Box, string::ToString};

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, Meta, NestedMeta, Path};

use crate::{panic, SynOption, TypeEnum, Validator, ValidatorOption};

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::VecU8);
const VALIDATOR: Validator = Validator::base64_decoded;

pub fn base64_decoded_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() != 1 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                let mut padding = ValidatorOption::new();

                let correct_usage_for_attribute = [stringify!(#[validator(base64_decoded)])];

                let correct_usage_for_padding = [
                    stringify!(#[validator(base64_decoded(padding(Must)))]),
                    stringify!(#[validator(base64_decoded(padding(Allow)))]),
                    stringify!(#[validator(base64_decoded(padding(NotAllow)))]),
                ];

                match meta {
                    Meta::Path(_) => (),
                    Meta::List(list) => {
                        let mut padding_is_set = false;

                        for p in list.nested.iter() {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    let meta_name = meta.path().into_token_stream().to_string();

                                    match meta_name.as_str() {
                                        "padding" => {
                                            padding = ValidatorOption::from_meta(
                                                meta_name.as_str(),
                                                meta,
                                                &mut padding_is_set,
                                                &correct_usage_for_padding,
                                            );
                                        },
                                        _ => panic::unknown_parameter(
                                            "base64_decoded",
                                            meta_name.as_str(),
                                        ),
                                    }
                                },
                                NestedMeta::Lit(_) => panic::attribute_incorrect_format(
                                    "base64_decoded",
                                    &correct_usage_for_attribute,
                                ),
                            }
                        }
                    },
                    Meta::NameValue(_) => panic::attribute_incorrect_format(
                        "base64_decoded",
                        &correct_usage_for_attribute,
                    ),
                }

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::Base64DecodedError }).unwrap();

                let padding_path = padding.to_expr();

                let parameters_impl = quote! {
                    impl #name {
                        pub(crate) const V_PADDING: validators_prelude::ValidatorOption = #padding_path;
                    }
                };

                let check_last_length = if padding.must() {
                    quote! {
                        if last_length != 4 {
                            return Err(#error_path::PaddingMust);
                        }
                    }
                } else {
                    quote! {}
                };

                let handle_padding = if padding.not_allow() {
                    quote! {
                        return Err(#error_path::PaddingNotAllow);
                    }
                } else {
                    quote! {
                        match p {
                            2 | 3 => {
                                if last_length != 4 {
                                    // has padding
                                    return Err(#error_path::Invalid);
                                }

                                for e in last_bytes[p + 1..].iter().copied() {
                                    if e != b'=' {
                                        return Err(#error_path::Invalid);
                                    }
                                }

                                return Ok(());
                            }
                            _ => return Err(#error_path::Invalid),
                        }
                    }
                };

                let decode = match padding {
                    ValidatorOption::Allow => {
                        quote! {
                            if v[v.len() - 1] == b'=' {
                                validators_prelude::data_encoding::BASE64.decode(v.as_ref()).map_err(|_| #error_path::Decode)
                            } else {
                                validators_prelude::data_encoding::BASE64_NOPAD.decode(v.as_ref()).map_err(|_| #error_path::Decode)
                            }
                        }
                    },
                    ValidatorOption::Must => {
                        quote! {
                            validators_prelude::data_encoding::BASE64.decode(v.as_ref()).map_err(|_| #error_path::Decode)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        quote! {
                            validators_prelude::data_encoding::BASE64_NOPAD.decode(v.as_ref()).map_err(|_| #error_path::Decode)
                        }
                    },
                };

                let v_parse_str = quote! {
                    #[inline]
                    fn v_parse_str(s: &str) -> Result<validators_prelude::Vec<u8>, #error_path> {
                        Self::v_parse_u8_slice(s.as_bytes())
                    }
                };

                let v_parse_u8_slice = quote! {
                    fn v_parse_u8_slice(v: &[u8]) -> Result<validators_prelude::Vec<u8>, #error_path> {
                        if v.is_empty() {
                            return Err(#error_path::Invalid);
                        }

                        #decode
                    }
                };

                let v_validate_str = quote! {
                    #[inline]
                    fn v_validate_str(s: &str) -> Result<(), #error_path> {
                        Self::v_validate_u8_slice(s.as_bytes())
                    }
                };

                let v_validate_u8_slice = quote! {
                    fn v_validate_u8_slice(v: &[u8]) -> Result<(), #error_path> {
                        let length = v.len();

                        if length == 0 {
                            return Err(#error_path::Invalid);
                        }

                        let last_length = {
                            let l = length & 0b11;

                            if l == 0 {
                                4
                            } else {
                                l
                            }
                        };

                        #check_last_length

                        let last_bytes = if length > 4 {
                            for e in v.iter().copied().take(length - last_length) {
                                match e {
                                    b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'/' => (),
                                    _ => return Err(#error_path::Invalid),
                                }
                            }

                            &v[(length - last_length)..]
                        } else {
                            v.as_ref()
                        };

                        let mut p = 0;

                        loop {
                            if p == last_length {
                                return Ok(());
                            }

                            let e = last_bytes[p];

                            match e {
                                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'/' => (),
                                b'=' => {
                                    #handle_padding
                                }
                                _ => return Err(#error_path::Invalid),
                            }

                            p += 1;
                        }
                    }
                };

                let parse_impl = quote! {
                    impl #name {
                        #v_parse_str

                        #v_parse_u8_slice

                        #v_validate_str

                        #v_validate_u8_slice
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
                            Self::v_validate_str(s.as_ref())?;

                            Ok(())
                        }
                    }
                };

                let validate_bytes_impl = quote! {
                    impl ValidateBytes for #name {
                        type Error = #error_path;
                        type Output = Self;

                        #[inline]
                        fn parse_vec_u8<V: Into<validators_prelude::Vec<u8>>>(v: V) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_u8_slice(v.into().as_slice())?))
                        }

                        #[inline]
                        fn parse_u8_slice<V: AsRef<[u8]>>(v: V) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_u8_slice(v.as_ref())?))
                        }

                        #[inline]
                        fn validate_u8_slice<V: AsRef<[u8]>>(v: V) -> Result<(), Self::Error> {
                            Self::v_validate_u8_slice(v.as_ref())?;

                            Ok(())
                        }
                    }
                };

                let collection_length_impl = quote! {
                    impl CollectionLength for #name {
                         #[inline]
                        fn len(&self) -> usize {
                            self.0.len()
                        }
                    }
                };

                let serde_impl = if cfg!(feature = "serde") {
                    let expect = match padding {
                        ValidatorOption::Allow => "a Base64 string or data",
                        ValidatorOption::Must => "a Base64 string or data with padding",
                        ValidatorOption::NotAllow => "a Base64 string or data without padding",
                    };

                    quote! {
                        impl validators_prelude::Serialize for #name {
                            #[inline]
                            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                where
                                    S: validators_prelude::Serializer, {
                                serializer.serialize_bytes(self.0.as_slice())
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
                                    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateBytes>::parse_u8_slice(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    #[inline]
                                    fn visit_byte_buf<E>(self, v: validators_prelude::Vec<u8>) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateBytes>::parse_vec_u8(v).map_err(validators_prelude::DeError::custom)
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

                let base64_decoded_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #validate_bytes_impl

                    #collection_length_impl

                    #serde_impl

                    #rocket_impl
                };

                base64_decoded_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        },
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
