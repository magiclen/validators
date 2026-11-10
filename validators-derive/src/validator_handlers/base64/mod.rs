use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{attributes::base_xx_attribute::BaseXXAttribute, type_enum::TypeEnum},
    panic,
};

pub(crate) struct Base64Handler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::String);

impl ValidatorHandler for Base64Handler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = BaseXXAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data
            && let Fields::Unnamed(_) = &data.fields
            && data.fields.len() == 1
        {
            let mut token_stream = proc_macro2::TokenStream::new();

            let name = ast.ident;

            let error_path: Path = syn::parse2(quote! { validators_prelude::Base64Error }).unwrap();

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
            let encoding = quote! { validators_prelude::base64 };

            token_stream.extend(quote! {
                impl #name {
                    #[inline]
                    fn v_parse_str(s: &str) -> ::core::result::Result<(), #error_path> {
                        Self::v_parse_u8_slice(s.as_bytes())
                    }

                    #[inline]
                    fn v_parse_u8_slice(v: &[u8]) -> ::core::result::Result<(), #error_path> {
                        #encoding::validate::<#padding_must, #padding_disallow>(v)
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
                                #encoding::parse::<#padding_must, #padding_disallow>(validators_prelude::Cow::Owned(v.into())).map(Self)
                            }

                            #[inline]
                            fn parse_u8_slice<V: AsRef<[u8]>>(v: V) -> ::core::result::Result<Self, Self::Error> {
                                #encoding::parse::<#padding_must, #padding_disallow>(validators_prelude::Cow::Borrowed(v.as_ref())).map(Self)
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
                    use crate::common::tri_allow::TriAllow;

                    let expect = match type_attribute.padding {
                        TriAllow::Allow => "a Base64 string or data",
                        TriAllow::Must => "a Base64 string or data with padding",
                        TriAllow::Disallow => "a Base64 string or data without padding",
                    };

                    token_stream.extend(quote! {
                                impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                                    #[inline]
                                    fn deserialize<__D>(deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                                    where
                                        __D: validators_prelude::serde::Deserializer<'de>, {
                                        struct MyVisitor;

                                        impl<'de> validators_prelude::serde::de::Visitor<'de> for MyVisitor {
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

                                        if deserializer.is_human_readable() {
                                            deserializer.deserialize_any(MyVisitor)
                                        } else {
                                            deserializer.deserialize_str(MyVisitor)
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

        Err(panic::validator_for_specific_item(meta.path(), ITEM))
    }
}
