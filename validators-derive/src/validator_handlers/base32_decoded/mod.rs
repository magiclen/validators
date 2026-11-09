use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{
        attributes::base_xx_attribute::BaseXXAttribute, tri_allow::TriAllow, type_enum::TypeEnum,
    },
    panic,
};

pub(crate) struct Base32DecodedHandler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::VecU8);

impl ValidatorHandler for Base32DecodedHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = BaseXXAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data
            && let Fields::Unnamed(_) = &data.fields
            && data.fields.len() == 1
        {
            let mut token_stream = proc_macro2::TokenStream::new();

            let name = ast.ident;

            let error_path: Path =
                syn::parse2(quote! { validators_prelude::Base32DecodedError }).unwrap();

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
                    if last_length != 8 {
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
                        2 | 4 | 5 | 7 => {
                            if last_length != 8 {
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
                                &validators_prelude::data_encoding::BASE32,
                            );
                        }
                        _ => return Err(#error_path::Invalid),
                    }
                }
            };

            let decode_padded = quote! {
                // The decoder accepts concatenated values, so check for padding before the last block.
                let last_block_start = (v.len() - 1) / 8 * 8;

                if v[..last_block_start].contains(&b'=') {
                    return Err(#error_path::Decode);
                }

                validators_prelude::data_encoding::BASE32.decode(v).map_err(|_| #error_path::Decode)
            };
            let decode_unpadded = quote! {
                validators_prelude::data_encoding::BASE32_NOPAD.decode(v).map_err(|_| #error_path::Decode)
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
                            fn v_parse_str(s: &str) -> ::core::result::Result<validators_prelude::Vec<u8>, #error_path> {
                                Self::v_parse_u8_slice(s.as_bytes())
                            }

                            fn v_parse_u8_slice(v: &[u8]) -> ::core::result::Result<validators_prelude::Vec<u8>, #error_path> {
                                if v.is_empty() {
                                    return Err(#error_path::Invalid);
                                }

                                #decode
                            }

                            #[inline]
                            fn v_validate_str(s: &str) -> ::core::result::Result<(), #error_path> {
                                Self::v_validate_u8_slice(s.as_bytes())
                            }

                            fn v_validate_u8_slice(v: &[u8]) -> ::core::result::Result<(), #error_path> {
                                let length = v.len();

                                if length == 0 {
                                    return Err(#error_path::Invalid);
                                }

                                let last_length = {
                                    let l = length & 0b111;

                                    if l == 0 {
                                        8
                                    } else {
                                        l
                                    }
                                };

                                #check_last_length

                                let last_bytes = if length > 8 {
                                    for e in v.iter().copied().take(length - last_length) {
                                        match e {
                                            b'A'..=b'Z' | b'2'..=b'7' => (),
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
                                        return if last_length == 8 {
                                            Ok(())
                                        } else {
                                            Self::v_validate_last_block(
                                                last_bytes,
                                                &validators_prelude::data_encoding::BASE32_NOPAD,
                                            )
                                        };
                                    }

                                    let e = last_bytes[p];

                                    match e {
                                        b'A'..=b'Z' | b'2'..=b'7' => (),
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
                            ) -> ::core::result::Result<(), #error_path> {
                                let output_len = encoding.decode_len(last_bytes.len()).map_err(|_| #error_path::Decode)?;
                                let mut output = [0; 5];

                                encoding.decode_mut(last_bytes, &mut output[..output_len]).map_err(|_| #error_path::Decode)?;

                                Ok(())
                            }
                        }
                    });

            token_stream.extend(quote! {
                        impl ValidateString for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_str(s.into().as_str())?))
                            }

                            #[inline]
                            fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_str(s.as_ref())?))
                            }

                            #[inline]
                            fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
                                Self::v_validate_str(s.as_ref())?;

                                Ok(())
                            }
                        }

                        impl ValidateBytes for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_vec_u8<V: Into<validators_prelude::Vec<u8>>>(v: V) -> ::core::result::Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_u8_slice(v.into().as_slice())?))
                            }

                            #[inline]
                            fn parse_u8_slice<V: AsRef<[u8]>>(v: V) -> ::core::result::Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_u8_slice(v.as_ref())?))
                            }

                            #[inline]
                            fn validate_u8_slice<V: AsRef<[u8]>>(v: V) -> ::core::result::Result<(), Self::Error> {
                                Self::v_validate_u8_slice(v.as_ref())?;

                                Ok(())
                            }
                            #[inline]
                            fn parse_decoded_vec_u8(v: validators_prelude::Vec<u8>) -> ::core::result::Result<Self, Self::Error> {
                                if v.is_empty() {
                                    return Err(#error_path::Invalid);
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
                    token_stream.extend(quote! {
                        impl validators_prelude::serde::Serialize for #name {
                            #[inline]
                            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                                where
                                    S: validators_prelude::serde::Serializer, {
                                serializer.serialize_bytes(self.0.as_slice())
                            }
                        }
                    });
                }

                if type_attribute.serde_options.deserialize {
                    let expect = match type_attribute.padding {
                        TriAllow::Allow => "a Base32 string or data",
                        TriAllow::Must => "a Base32 string or data with padding",
                        TriAllow::Disallow => "a Base32 string or data without padding",
                    };

                    token_stream.extend(quote! {
                                impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                                    #[inline]
                                    fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
                                    where
                                        D: validators_prelude::serde::Deserializer<'de>, {
                                        validators_prelude::deserialize_decoded_bytes(deserializer, #expect)?
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
