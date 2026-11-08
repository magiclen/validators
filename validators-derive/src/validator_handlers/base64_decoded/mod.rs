use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{
        attributes::base_xx_attribute::BaseXXAttribute, tri_allow::TriAllow, type_enum::TypeEnum,
    },
    panic,
};

pub(crate) struct Base64DecodedHandler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::VecU8);

impl ValidatorHandler for Base64DecodedHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = BaseXXAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data
            && let Fields::Unnamed(_) = &data.fields
            && data.fields.len() == 1
        {
            let mut token_stream = proc_macro2::TokenStream::new();

            let name = ast.ident;

            let error_path: Path =
                syn::parse2(quote! { validators_prelude::Base64DecodedError }).unwrap();

            #[cfg(feature = "test")]
            {
                let v_padding = type_attribute.padding;

                token_stream.extend(quote! {
                    impl #name {
                        pub(crate) const V_PADDING: validators_prelude::TriAllow = #v_padding;
                    }
                });
            }

            let check_last_length = if type_attribute.padding.must() {
                quote! {
                    if last_length != 4 {
                        return Err(#error_path::PaddingMust);
                    }
                }
            } else {
                quote! {}
            };

            let handle_padding = if type_attribute.padding.disallow() {
                quote! {
                    return Err(#error_path::PaddingDisallow);
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

                            return Self::v_validate_last_block(
                                last_bytes,
                                &validators_prelude::data_encoding::BASE64,
                            );
                        }
                        _ => return Err(#error_path::Invalid),
                    }
                }
            };

            let decode_padded = quote! {
                // The decoder accepts concatenated values, so check for padding before the last block.
                let last_block_start = (v.len() - 1) / 4 * 4;

                if v[..last_block_start].contains(&b'=') {
                    return Err(#error_path::Decode);
                }

                validators_prelude::data_encoding::BASE64.decode(v).map_err(|_| #error_path::Decode)
            };
            let decode_unpadded = quote! {
                validators_prelude::data_encoding::BASE64_NOPAD.decode(v).map_err(|_| #error_path::Decode)
            };

            let decode = match type_attribute.padding {
                TriAllow::Allow => quote! {
                    if v.last() == Some(&b'=') {
                        #decode_padded
                    } else {
                        #decode_unpadded
                    }
                },
                TriAllow::Must => decode_padded,
                TriAllow::Disallow => decode_unpadded,
            };

            token_stream.extend(quote! {
                        impl #name {
                            #[inline]
                            fn v_parse_str(s: &str) -> Result<validators_prelude::Vec<u8>, #error_path> {
                                Self::v_parse_u8_slice(s.as_bytes())
                            }

                            fn v_parse_u8_slice(v: &[u8]) -> Result<validators_prelude::Vec<u8>, #error_path> {
                                if v.is_empty() {
                                    return Err(#error_path::Invalid);
                                }

                                #decode
                            }

                            #[inline]
                            fn v_validate_str(s: &str) -> Result<(), #error_path> {
                                Self::v_validate_u8_slice(s.as_bytes())
                            }

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
                                        return if last_length == 4 {
                                            Ok(())
                                        } else {
                                            Self::v_validate_last_block(
                                                last_bytes,
                                                &validators_prelude::data_encoding::BASE64_NOPAD,
                                            )
                                        };
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

                            fn v_validate_last_block(
                                last_bytes: &[u8],
                                encoding: &validators_prelude::data_encoding::Encoding,
                            ) -> Result<(), #error_path> {
                                let output_len = encoding.decode_len(last_bytes.len()).map_err(|_| #error_path::Decode)?;
                                let mut output = [0; 3];

                                encoding.decode_mut(last_bytes, &mut output[..output_len]).map_err(|_| #error_path::Decode)?;

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
                                Self::v_validate_str(s.as_ref())?;

                                Ok(())
                            }
                        }

                        impl ValidateBytes for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_vec_u8<V: Into<validators_prelude::Vec<u8>>>(v: V) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_u8_slice(v.into().as_slice())?))
                            }

                            #[inline]
                            fn parse_u8_slice<V: AsRef<[u8]>>(v: V) -> Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_u8_slice(v.as_ref())?))
                            }

                            #[inline]
                            fn validate_u8_slice<V: AsRef<[u8]>>(v: V) -> Result<(), Self::Error> {
                                Self::v_validate_u8_slice(v.as_ref())?;

                                Ok(())
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
                    token_stream.extend(quote! {
                        impl validators_prelude::serde::Serialize for #name {
                            #[inline]
                            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                where
                                    S: validators_prelude::serde::Serializer, {
                                serializer.serialize_bytes(self.0.as_slice())
                            }
                        }
                    });
                }

                if type_attribute.serde_options.deserialize {
                    let expect = match type_attribute.padding {
                        TriAllow::Allow => "a Base64 string or data",
                        TriAllow::Must => "a Base64 string or data with padding",
                        TriAllow::Disallow => "a Base64 string or data without padding",
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
                                            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateBytes>::parse_u8_slice(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_byte_buf<E>(self, v: validators_prelude::Vec<u8>) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateBytes>::parse_vec_u8(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
                                            where
                                                A: validators_prelude::serde::de::SeqAccess<'de>, {
                                                let bytes: validators_prelude::Vec<u8> = validators_prelude::serde::Deserialize::deserialize(
                                                    validators_prelude::serde::de::value::SeqAccessDeserializer::new(seq),
                                                )?;

                                                if bytes.is_empty() {
                                                    return Err(validators_prelude::serde::de::Error::custom(#error_path::Invalid));
                                                }

                                                Ok(#name(bytes))
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
                    crate::common::rocket::impl_from_param(&mut token_stream, &name, &error_path);
                }
            }

            return Ok(token_stream);
        }

        Err(panic::validator_for_specific_item(meta.path(), ITEM))
    }
}
