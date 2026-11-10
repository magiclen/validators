mod hex_decoded_attribute;

use hex_decoded_attribute::HexDecodedAttribute;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{common::type_enum::TypeEnum, panic};

pub(crate) struct HexDecodedHandler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::VecU8);

impl ValidatorHandler for HexDecodedHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = HexDecodedAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data
            && let Fields::Unnamed(_) = &data.fields
            && data.fields.len() == 1
        {
            let name = ast.ident;
            let error_path: Path =
                syn::parse2(quote! { validators_prelude::HexDecodedError }).unwrap();
            let lower = type_attribute.case.lower().then(|| {
                quote! {
                    b'a'..=b'f' => Ok(digit - (b'a' - 10)),
                }
            });
            let upper = type_attribute.case.upper().then(|| {
                quote! {
                    b'A'..=b'F' => Ok(digit - (b'A' - 10)),
                }
            });

            let mut token_stream = proc_macro2::TokenStream::new();

            token_stream.extend(quote! {
                impl #name {
                    #[inline]
                    fn v_check_hex_length(v: &[u8]) -> ::core::result::Result<(), #error_path> {
                        if v.is_empty() || !v.len().is_multiple_of(2) {
                            return Err(#error_path);
                        }

                        Ok(())
                    }

                    #[inline]
                    fn v_decode_hex_digit(digit: u8) -> ::core::result::Result<u8, #error_path> {
                        match digit {
                            b'0'..=b'9' => Ok(digit - b'0'),
                            #lower
                            #upper
                            _ => Err(#error_path),
                        }
                    }

                    fn v_parse_u8_slice(v: &[u8]) -> ::core::result::Result<validators_prelude::Vec<u8>, #error_path> {
                        Self::v_check_hex_length(v)?;

                        let mut decoded = validators_prelude::Vec::with_capacity(v.len() / 2);

                        for pair in v.chunks_exact(2) {
                            let high = Self::v_decode_hex_digit(pair[0])?;
                            let low = Self::v_decode_hex_digit(pair[1])?;

                            decoded.push((high << 4) | low);
                        }

                        Ok(decoded)
                    }

                    fn v_parse_vec_u8(mut v: validators_prelude::Vec<u8>) -> ::core::result::Result<validators_prelude::Vec<u8>, #error_path> {
                        Self::v_check_hex_length(&v)?;

                        let decoded_len = v.len() / 2;

                        for index in 0..decoded_len {
                            let high = Self::v_decode_hex_digit(v[2 * index])?;
                            let low = Self::v_decode_hex_digit(v[2 * index + 1])?;

                            v[index] = (high << 4) | low;
                        }

                        v.truncate(decoded_len);

                        Ok(v)
                    }
                }

                impl ValidateString for #name {
                    type Error = #error_path;

                    #[inline]
                    fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                        <Self as ValidateBytes>::parse_vec_u8(s.into().into_bytes())
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                        Ok(Self(Self::v_parse_u8_slice(s.as_ref().as_bytes())?))
                    }

                    #[inline]
                    fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
                        <Self as ValidateBytes>::validate_u8_slice(s.as_ref().as_bytes())
                    }
                }

                impl ValidateBytes for #name {
                    type Error = #error_path;

                    #[inline]
                    fn parse_vec_u8<V: Into<validators_prelude::Vec<u8>>>(v: V) -> ::core::result::Result<Self, Self::Error> {
                        Self::v_parse_vec_u8(v.into()).map(Self)
                    }

                    #[inline]
                    fn parse_u8_slice<V: AsRef<[u8]>>(v: V) -> ::core::result::Result<Self, Self::Error> {
                        Ok(Self(Self::v_parse_u8_slice(v.as_ref())?))
                    }

                    #[inline]
                    fn validate_u8_slice<V: AsRef<[u8]>>(v: V) -> ::core::result::Result<(), Self::Error> {
                        let v = v.as_ref();

                        Self::v_check_hex_length(v)?;

                        for &digit in v {
                            Self::v_decode_hex_digit(digit)?;
                        }

                        Ok(())
                    }

                    #[inline]
                    fn parse_decoded_vec_u8(v: validators_prelude::Vec<u8>) -> ::core::result::Result<Self, Self::Error> {
                        if v.is_empty() {
                            return Err(#error_path);
                        }

                        Ok(Self(v))
                    }
                }

                impl CollectionLength for #name {
                    #[inline]
                    fn len(&self) -> usize {
                        self.0.len()
                    }
                }
            });

            #[cfg(feature = "serde")]
            {
                if type_attribute.serde_options.serialize {
                    let digits = if type_attribute.case.upper() {
                        quote! { b"0123456789ABCDEF" }
                    } else {
                        quote! { b"0123456789abcdef" }
                    };

                    token_stream.extend(quote! {
                        impl validators_prelude::serde::Serialize for #name {
                            #[inline]
                            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                            where
                                S: validators_prelude::serde::Serializer,
                            {
                                // Human-readable formats use a hex string, and other formats use the bytes.
                                if serializer.is_human_readable() {
                                    const DIGITS: &[u8; 16] = #digits;

                                    let mut s = validators_prelude::String::with_capacity(self.0.len() * 2);

                                    for e in self.0.iter().copied() {
                                        s.push(char::from(DIGITS[usize::from(e >> 4)]));
                                        s.push(char::from(DIGITS[usize::from(e & 0xF)]));
                                    }

                                    serializer.serialize_str(&s)
                                } else {
                                    serializer.serialize_bytes(self.0.as_slice())
                                }
                            }
                        }
                    });
                }

                if type_attribute.serde_options.deserialize {
                    use crate::common::case_option::CaseOption;

                    let expected = match type_attribute.case {
                        CaseOption::Any => "a hexadecimal string or decoded bytes",
                        CaseOption::Upper => "an uppercase hexadecimal string or decoded bytes",
                        CaseOption::Lower => "a lowercase hexadecimal string or decoded bytes",
                    };

                    token_stream.extend(quote! {
                        impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                            #[inline]
                            fn deserialize<__D>(deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                            where
                                __D: validators_prelude::serde::Deserializer<'de>,
                            {
                                validators_prelude::deserialize_decoded_bytes(deserializer, #expected)?
                                    .map_err(validators_prelude::serde::de::Error::custom)
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
                    crate::common::rocket::impl_from_param(&mut token_stream, &name, &error_path);
                }
            }

            return Ok(token_stream);
        }

        Err(panic::validator_for_specific_item(meta.path(), ITEM))
    }
}
