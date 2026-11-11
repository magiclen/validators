use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{
        attributes::http_xx_url_attribute::HttpXXUrlAttribute, tri_allow::TriAllow,
        type_enum::TypeEnum,
    },
    panic,
};

pub(crate) struct HttpFtpUrlHandler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct {
    url:      TypeEnum,
    protocol: TypeEnum,
}

const ITEM: Struct = Struct {
    url: TypeEnum::Url, protocol: TypeEnum::Protocol
};

impl ValidatorHandler for HttpFtpUrlHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        #[cfg(feature = "serde")]
        let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
        #[cfg(feature = "serde")]
        let deserializer = ident_generator.next("__ValidatorsDeserializer");
        #[cfg(feature = "serde")]
        let deserializer_name = ident_generator.next("__validators_deserializer");
        #[cfg(feature = "serde")]
        let visitor = ident_generator.next("__ValidatorsVisitor");

        let type_attribute = HttpXXUrlAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data
            && let Fields::Named(_) = &data.fields
            && data.fields.len() == 2
        {
            let mut token_stream = proc_macro2::TokenStream::new();

            let name = ast.ident;

            let error_path: Path =
                syn::parse2(quote! { validators_prelude::HttpFtpURLError }).unwrap();

            #[cfg(feature = "test")]
            {
                let v_local = type_attribute.local;

                token_stream.extend(quote! {
                    impl #name {
                        pub(crate) const V_LOCAL: validators_prelude::TriAllow = #v_local;
                    }
                });
            }

            let handle_local = {
                match type_attribute.local {
                    TriAllow::Allow => {
                        quote! {}
                    },
                    _ => {
                        let check_local = if type_attribute.local.disallow() {
                            quote! {
                                if is_local {
                                    return Err(#error_path::LocalDisallow);
                                }
                            }
                        } else {
                            quote! {
                                if !is_local {
                                    return Err(#error_path::LocalMust);
                                }
                            }
                        };

                        quote! {
                            let is_local = {
                                match url.host().unwrap() {
                                    validators_prelude::url::Host::Domain(domain) => validators_prelude::is_local_domain(domain),
                                    validators_prelude::url::Host::Ipv4(ip) => validators_prelude::is_local_ipv4(ip),
                                    validators_prelude::url::Host::Ipv6(ip) => validators_prelude::is_local_ipv6(ip),
                                }
                            };

                            #check_local
                        }
                    },
                }
            };

            token_stream.extend(quote! {
                        impl #name {
                            fn v_parse_str(s: &str) -> ::core::result::Result<(validators_prelude::url::Url, validators_prelude::Protocol), #error_path> {
                                let (url, protocol) = {
                                    use validators_prelude::str_utils::StartsWithIgnoreAsciiCaseMultiple;

                                    // The URL parser ignores leading C0 control characters and spaces, so do the same here.
                                    let scheme_str = s.trim_start_matches(|c: char| c <= ' ');

                                    if let Some(index) = scheme_str.starts_with_ignore_ascii_case_with_lowercase_multiple(&["http:", "https:", "ftp:"]) {
                                        let protocol = match index {
                                            0 => validators_prelude::Protocol::HTTP,
                                            1 => validators_prelude::Protocol::HTTPS,
                                            2 => validators_prelude::Protocol::FTP,
                                            _ => unreachable!()
                                        };

                                        (validators_prelude::url::Url::parse(s)?, protocol)
                                    } else {
                                        // The URL parser also removes tabs and newlines inside the scheme.
                                        let url = validators_prelude::url::Url::parse(s).map_err(|_| #error_path::ProtocolError)?;
                                        let protocol = match url.scheme() {
                                            "http" => validators_prelude::Protocol::HTTP,
                                            "https" => validators_prelude::Protocol::HTTPS,
                                            "ftp" => validators_prelude::Protocol::FTP,
                                            _ => return Err(#error_path::ProtocolError),
                                        };

                                        (url, protocol)
                                    }
                                };

                                #handle_local

                                Ok((url, protocol))
                            }
                        }
                    });

            token_stream.extend(quote! {
                        impl ValidateString for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                                let (url, protocol) = Self::v_parse_str(s.into().as_str())?;

                                Ok(Self {
                                    url,
                                    protocol,
                                })
                            }

                            #[inline]
                            fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                                let (url, protocol) = Self::v_parse_str(s.as_ref())?;

                                Ok(Self {
                                    url,
                                    protocol,
                                })
                            }

                            #[inline]
                            fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
                                Self::v_parse_str(s.as_ref())?;

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
                                    fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                                        where
                                            S: validators_prelude::serde::Serializer, {
                                        validators_prelude::serde::Serialize::serialize(&self.url, serializer)
                                    }
                                }
                            });
                }

                if type_attribute.serde_options.deserialize {
                    let expect = {
                        let mut s = String::from("a http/https/ftp url");

                        match type_attribute.local {
                            TriAllow::Allow => (),
                            TriAllow::Must => {
                                s.push_str(" which must be local");
                            },
                            TriAllow::Disallow => {
                                s.push_str(" which must not be local");
                            },
                        }

                        s
                    };

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
                                        }

                                        #deserializer_name.deserialize_str(#visitor)
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
