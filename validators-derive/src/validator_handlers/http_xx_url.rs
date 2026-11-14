use std::fmt::{self, Debug, Formatter};

use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Meta, Path};

use crate::{
    common::{
        attributes::http_xx_url_attribute::HttpXXUrlAttribute, tri_allow::TriAllow,
        type_enum::TypeEnum,
    },
    panic,
};

#[derive(Clone, Copy)]
pub(super) enum UrlKind {
    #[cfg(feature = "http_url")]
    Http,
    #[cfg(feature = "http_ftp_url")]
    HttpFtp,
}

impl UrlKind {
    fn protocol_field(self) -> (&'static str, TypeEnum) {
        match self {
            #[cfg(feature = "http_url")]
            Self::Http => ("is_https", TypeEnum::Boolean),
            #[cfg(feature = "http_ftp_url")]
            Self::HttpFtp => ("protocol", TypeEnum::Protocol),
        }
    }

    fn error_path(self) -> Path {
        syn::parse2(match self {
            #[cfg(feature = "http_url")]
            Self::Http => quote! { validators_prelude::HttpURLError },
            #[cfg(feature = "http_ftp_url")]
            Self::HttpFtp => quote! { validators_prelude::HttpFtpURLError },
        })
        .unwrap()
    }

    fn protocol(self) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        match self {
            #[cfg(feature = "http_url")]
            Self::Http => (quote! { bool }, quote! {
                b"http:" => false,
                b"https:" => true,
            }),
            #[cfg(feature = "http_ftp_url")]
            Self::HttpFtp => (quote! { validators_prelude::Protocol }, quote! {
                b"http:" => validators_prelude::Protocol::HTTP,
                b"https:" => validators_prelude::Protocol::HTTPS,
                b"ftp:" => validators_prelude::Protocol::FTP,
            }),
        }
    }

    #[cfg(feature = "serde")]
    fn expectation(self, local: TriAllow) -> String {
        let label = match self {
            #[cfg(feature = "http_url")]
            Self::Http => "a http/https url",
            #[cfg(feature = "http_ftp_url")]
            Self::HttpFtp => "a http/https/ftp url",
        };
        let suffix = match local {
            TriAllow::Allow => "",
            TriAllow::Must => " which must be local",
            TriAllow::Disallow => " which must not be local",
        };

        format!("{label}{suffix}")
    }
}

impl Debug for UrlKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (name, ty) = self.protocol_field();

        f.debug_struct("Struct").field("url", &TypeEnum::Url).field(name, &ty).finish()
    }
}

pub(super) fn url_handler(
    ast: DeriveInput,
    meta: Meta,
    kind: UrlKind,
) -> syn::Result<proc_macro2::TokenStream> {
    #[cfg(feature = "serde")]
    let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
    #[cfg(feature = "serde")]
    let deserializer = ident_generator.next("__ValidatorsDeserializer");
    #[cfg(feature = "serde")]
    let deserializer_name = ident_generator.next("__validators_deserializer");
    #[cfg(feature = "serde")]
    let visitor = ident_generator.next("__ValidatorsVisitor");

    let type_attribute = HttpXXUrlAttribute::build_from_meta(&meta)?;
    let (field_name, _) = kind.protocol_field();

    if let Data::Struct(data) = ast.data
        && let Fields::Named(_) = &data.fields
        && data.fields.len() == 2
        && data.fields.iter().all(|field| {
            field.ident.as_ref().is_some_and(|ident| ident == "url" || ident == field_name)
        })
    {
        let mut token_stream = proc_macro2::TokenStream::new();
        let name = ast.ident;
        let error_path = kind.error_path();
        let protocol_field = format_ident!("{}", field_name);
        let (protocol_type, protocol_arms) = kind.protocol();

        #[cfg(feature = "test")]
        {
            let v_local = type_attribute.local;

            token_stream.extend(quote! {
                impl #name {
                    pub(crate) const V_LOCAL: validators_prelude::TriAllow = #v_local;
                }
            });
        }

        let handle_local = match type_attribute.local {
            TriAllow::Allow => quote! {},
            _ => {
                let check_local = if type_attribute.local.disallow() {
                    quote! {
                        if is_local {
                            return ::core::result::Result::Err(#error_path::LocalDisallow);
                        }
                    }
                } else {
                    quote! {
                        if !is_local {
                            return ::core::result::Result::Err(#error_path::LocalMust);
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
        };

        token_stream.extend(quote! {
            impl #name {
                fn v_parse_str(s: &str) -> ::core::result::Result<(validators_prelude::url::Url, #protocol_type), #error_path> {
                    // Read the scheme with the same whitespace rules as the URL parser.
                    let mut scheme = [0u8; 6];
                    let mut scheme_length = 0;

                    for e in s.bytes().skip_while(|&e| e <= b' ').filter(|&e| !::core::matches!(e, b'\t' | b'\n' | b'\r')).take(scheme.len()) {
                        scheme[scheme_length] = e.to_ascii_lowercase();
                        scheme_length += 1;

                        if e == b':' {
                            break;
                        }
                    }

                    let protocol = match &scheme[..scheme_length] {
                        #protocol_arms
                        _ => return ::core::result::Result::Err(#error_path::ProtocolError),
                    };

                    let url = validators_prelude::url::Url::parse(s)?;

                    #handle_local

                    ::core::result::Result::Ok((url, protocol))
                }
            }

            impl ValidateString for #name {
                type Error = #error_path;

                #[inline]
                fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                    let (url, protocol) = Self::v_parse_str(s.into().as_str())?;

                    ::core::result::Result::Ok(Self {
                        url,
                        #protocol_field: protocol,
                    })
                }

                #[inline]
                fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                    let (url, protocol) = Self::v_parse_str(s.as_ref())?;

                    ::core::result::Result::Ok(Self {
                        url,
                        #protocol_field: protocol,
                    })
                }

                #[inline]
                fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
                    Self::v_parse_str(s.as_ref())?;

                    ::core::result::Result::Ok(())
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
                let expect = kind.expectation(type_attribute.local);

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

    Err(panic::validator_for_specific_item(meta.path(), kind))
}
