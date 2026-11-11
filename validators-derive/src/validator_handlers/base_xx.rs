use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Meta, Path};

#[cfg(any(
    feature = "serde",
    feature = "base32_decoded",
    feature = "base64_decoded",
    feature = "base64_url_decoded"
))]
use crate::common::tri_allow::TriAllow;
use crate::{
    common::{attributes::base_xx_attribute::BaseXXAttribute, type_enum::TypeEnum},
    panic,
};

#[derive(Clone, Copy)]
pub(super) enum Encoding {
    #[cfg(any(feature = "base32", feature = "base32_decoded"))]
    Base32,
    #[cfg(any(feature = "base64", feature = "base64_decoded"))]
    Base64,
    #[cfg(any(feature = "base64_url", feature = "base64_url_decoded"))]
    Base64Url,
}

impl Encoding {
    fn error_path(self, decoded: bool) -> Path {
        let prefix = match self {
            #[cfg(any(feature = "base32", feature = "base32_decoded"))]
            Self::Base32 => "Base32",
            #[cfg(any(feature = "base64", feature = "base64_decoded"))]
            Self::Base64 => "Base64",
            #[cfg(any(feature = "base64_url", feature = "base64_url_decoded"))]
            Self::Base64Url => "Base64Url",
        };
        let error = format_ident!("{}{}Error", prefix, if decoded { "Decoded" } else { "" });

        syn::parse2(quote! { validators_prelude::#error }).unwrap()
    }

    fn module_path(self) -> proc_macro2::TokenStream {
        let module = match self {
            #[cfg(any(feature = "base32", feature = "base32_decoded"))]
            Self::Base32 => "base32",
            #[cfg(any(feature = "base64", feature = "base64_decoded"))]
            Self::Base64 => "base64",
            #[cfg(any(feature = "base64_url", feature = "base64_url_decoded"))]
            Self::Base64Url => "base64_url",
        };
        let module = format_ident!("{}", module);

        quote! { validators_prelude::#module }
    }

    #[cfg(feature = "serde")]
    fn expectation(self, padding: TriAllow) -> String {
        let label = match self {
            #[cfg(any(feature = "base32", feature = "base32_decoded"))]
            Self::Base32 => "Base32",
            #[cfg(any(feature = "base64", feature = "base64_decoded"))]
            Self::Base64 => "Base64",
            #[cfg(any(feature = "base64_url", feature = "base64_url_decoded"))]
            Self::Base64Url => "Base64-url",
        };
        let suffix = match padding {
            TriAllow::Allow => "",
            TriAllow::Must => " with padding",
            TriAllow::Disallow => " without padding",
        };

        format!("a {label} string or data{suffix}")
    }

    #[cfg(any(
        feature = "base32_decoded",
        feature = "base64_decoded",
        feature = "base64_url_decoded"
    ))]
    fn decoder(self) -> (proc_macro2::TokenStream, proc_macro2::TokenStream, usize) {
        let (codec, block_len) = match self {
            #[cfg(any(feature = "base32", feature = "base32_decoded"))]
            Self::Base32 => ("BASE32", 8),
            #[cfg(any(feature = "base64", feature = "base64_decoded"))]
            Self::Base64 => ("BASE64", 4),
            #[cfg(any(feature = "base64_url", feature = "base64_url_decoded"))]
            Self::Base64Url => ("BASE64URL", 4),
        };
        let padded = format_ident!("{}", codec);
        let unpadded = format_ident!("{}_NOPAD", codec);

        (
            quote! { validators_prelude::data_encoding::#padded },
            quote! { validators_prelude::data_encoding::#unpadded },
            block_len,
        )
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

#[cfg(any(feature = "base32", feature = "base64", feature = "base64_url"))]
const ITEM_RAW: Struct = Struct(TypeEnum::String);
#[cfg(any(feature = "base32_decoded", feature = "base64_decoded", feature = "base64_url_decoded"))]
const ITEM_DECODED: Struct = Struct(TypeEnum::VecU8);

#[cfg(any(feature = "base32", feature = "base64", feature = "base64_url"))]
pub(super) fn encoded_handler(
    ast: DeriveInput,
    meta: Meta,
    encoding: Encoding,
) -> syn::Result<proc_macro2::TokenStream> {
    #[cfg(feature = "serde")]
    let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
    #[cfg(feature = "serde")]
    let deserializer = ident_generator.next("__ValidatorsDeserializer");
    #[cfg(feature = "serde")]
    let deserializer_name = ident_generator.next("__validators_deserializer");
    #[cfg(feature = "serde")]
    let visitor = ident_generator.next("__ValidatorsVisitor");

    let type_attribute = BaseXXAttribute::build_from_meta(&meta)?;

    if let Data::Struct(data) = ast.data
        && let Fields::Unnamed(_) = &data.fields
        && data.fields.len() == 1
    {
        let mut token_stream = proc_macro2::TokenStream::new();

        let name = ast.ident;

        let error_path = encoding.error_path(false);

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
        let encoding_path = encoding.module_path();

        token_stream.extend(quote! {
            impl #name {
                #[inline]
                fn v_parse_str(s: &str) -> ::core::result::Result<(), #error_path> {
                    Self::v_parse_u8_slice(s.as_bytes())
                }

                #[inline]
                fn v_parse_u8_slice(v: &[u8]) -> ::core::result::Result<(), #error_path> {
                    #encoding_path::validate::<#padding_must, #padding_disallow>(v)
                }
            }
        });

        token_stream.extend(quote! {
                    impl ValidateString for #name {
                        type Error = #error_path;

                        #[inline]
                        fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                            let s = s.into();

                            Self::v_parse_str(s.as_str())?;

                            Ok(Self(s))
                        }

                        #[inline]
                        fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                            let s = s.as_ref();

                            Self::v_parse_str(s)?;

                            Ok(Self(validators_prelude::String::from(s)))
                        }

                        #[inline]
                        fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
                            Self::v_parse_str(s.as_ref())
                        }
                    }

                    impl ValidateBytes for #name {
                        type Error = #error_path;

                        #[inline]
                        fn parse_vec_u8<V: Into<validators_prelude::Vec<u8>>>(v: V) -> ::core::result::Result<Self, Self::Error> {
                            #encoding_path::parse::<#padding_must, #padding_disallow>(validators_prelude::Cow::Owned(v.into())).map(Self)
                        }

                        #[inline]
                        fn parse_u8_slice<V: AsRef<[u8]>>(v: V) -> ::core::result::Result<Self, Self::Error> {
                            #encoding_path::parse::<#padding_must, #padding_disallow>(validators_prelude::Cow::Borrowed(v.as_ref())).map(Self)
                        }

                        #[inline]
                        fn validate_u8_slice<V: AsRef<[u8]>>(v: V) -> ::core::result::Result<(), Self::Error> {
                            Self::v_parse_u8_slice(v.as_ref())
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
                            serializer.serialize_str(self.0.as_str())
                        }
                    }
                });
            }

            if type_attribute.serde_options.deserialize {
                let expect = encoding.expectation(type_attribute.padding);

                token_stream.extend(quote! {
                            impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                                #[inline]
                                fn deserialize<#deserializer>(#deserializer_name: #deserializer) -> ::core::result::Result<Self, #deserializer::Error>
                                where
                                    #deserializer: validators_prelude::serde::Deserializer<'de>, {
                                    struct #visitor;

                                    impl<'de> validators_prelude::serde::de::Visitor<'de> for #visitor {
                                        type Value = #name;

                                        #[inline]
                                        fn expecting(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                            f.write_str(#expect)
                                        }

                                        #[inline]
                                        fn visit_str<E>(self, v: &str) -> ::core::result::Result<Self::Value, E>
                                        where
                                            E: validators_prelude::serde::de::Error, {
                                            <Self::Value as ValidateString>::parse_str(v).map_err(validators_prelude::serde::de::Error::custom)
                                        }

                                        #[inline]
                                        fn visit_string<E>(self, v: validators_prelude::String) -> ::core::result::Result<Self::Value, E>
                                        where
                                            E: validators_prelude::serde::de::Error, {
                                            <Self::Value as ValidateString>::parse_string(v).map_err(validators_prelude::serde::de::Error::custom)
                                        }

                                        #[inline]
                                        fn visit_bytes<E>(self, v: &[u8]) -> ::core::result::Result<Self::Value, E>
                                        where
                                            E: validators_prelude::serde::de::Error, {
                                            <Self::Value as ValidateBytes>::parse_u8_slice(v).map_err(validators_prelude::serde::de::Error::custom)
                                        }

                                        #[inline]
                                        fn visit_byte_buf<E>(self, v: validators_prelude::Vec<u8>) -> ::core::result::Result<Self::Value, E>
                                        where
                                            E: validators_prelude::serde::de::Error, {
                                            <Self::Value as ValidateBytes>::parse_vec_u8(v).map_err(validators_prelude::serde::de::Error::custom)
                                        }
                                    }

                                    if #deserializer_name.is_human_readable() {
                                        #deserializer_name.deserialize_any(#visitor)
                                    } else {
                                        #deserializer_name.deserialize_str(#visitor)
                                    }
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

    Err(panic::validator_for_specific_item(meta.path(), ITEM_RAW))
}

#[cfg(any(feature = "base32_decoded", feature = "base64_decoded", feature = "base64_url_decoded"))]
pub(super) fn decoded_handler(
    ast: DeriveInput,
    meta: Meta,
    encoding: Encoding,
) -> syn::Result<proc_macro2::TokenStream> {
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

        let error_path = encoding.error_path(true);

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
        let encoding_path = encoding.module_path();
        let (padded, unpadded, block_len) = encoding.decoder();

        let decode_padded = quote! {
            // The decoder accepts concatenated values, so check for padding before the last block.
            let last_block_start = (v.len() - 1) / #block_len * #block_len;

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
                            #encoding_path::validate_decoded::<#padding_must, #padding_disallow>(v)
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
                let expect = encoding.expectation(type_attribute.padding);

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

    Err(panic::validator_for_specific_item(meta.path(), ITEM_DECODED))
}
