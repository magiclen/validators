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
        #[cfg(feature = "serde")]
        let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
        #[cfg(feature = "serde")]
        let deserializer = ident_generator.next("__ValidatorsDeserializer");
        #[cfg(feature = "serde")]
        let deserializer_name = ident_generator.next("__validators_deserializer");

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

            let padding_must = type_attribute.padding.must();
            let padding_disallow = type_attribute.padding.disallow();
            let encoding = quote! { validators_prelude::base32 };
            let padded = quote! { validators_prelude::data_encoding::BASE32 };
            let unpadded = quote! { validators_prelude::data_encoding::BASE32_NOPAD };

            let decode_padded = quote! {
                // The decoder accepts concatenated values, so check for padding before the last block.
                let last_block_start = (v.len() - 1) / 8 * 8;

                if v[..last_block_start].contains(&b'=') {
                    return Err(#error_path::Decode);
                }

                #padded.decode(v).map_err(|_| #error_path::Decode)
            };
            let decode_unpadded = quote! {
                #unpadded.decode(v).map_err(|_| #error_path::Decode)
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

                                // The decoder only reports `Decode`, so ask the validator for a more specific error.
                                Self::v_decode_u8_slice(v).map_err(|error| Self::v_validate_u8_slice(v).err().unwrap_or(error))
                            }

                            fn v_decode_u8_slice(v: &[u8]) -> ::core::result::Result<validators_prelude::Vec<u8>, #error_path> {
                                #decode
                            }

                            #[inline]
                            fn v_validate_str(s: &str) -> ::core::result::Result<(), #error_path> {
                                Self::v_validate_u8_slice(s.as_bytes())
                            }

                            #[inline]
                            fn v_validate_u8_slice(v: &[u8]) -> ::core::result::Result<(), #error_path> {
                                #encoding::validate_decoded::<#padding_must, #padding_disallow>(v)
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
                    let output_encoding =
                        if type_attribute.padding.disallow() { &unpadded } else { &padded };

                    token_stream.extend(quote! {
                        impl validators_prelude::serde::Serialize for #name {
                            #[inline]
                            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                                where
                                    S: validators_prelude::serde::Serializer, {
                                // Human-readable formats use the encoded string, and other formats use the bytes.
                                if serializer.is_human_readable() {
                                    serializer.serialize_str(&#output_encoding.encode(&self.0))
                                } else {
                                    serializer.serialize_bytes(self.0.as_slice())
                                }
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
                                    fn deserialize<#deserializer>(#deserializer_name: #deserializer) -> ::core::result::Result<Self, #deserializer::Error>
                                    where
                                        #deserializer: validators_prelude::serde::Deserializer<'de>, {
                                        validators_prelude::deserialize_decoded_bytes(#deserializer_name, #expect)?
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
