use std::fmt::Debug;

use educe::Educe;
use quote::quote;
use syn::{Data, DeriveInput, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{
        attributes::ip_xx_attribute::IpXXAttribute, fields_match, tri_allow::TriAllow,
        type_enum::TypeEnum,
    },
    panic,
};

pub(crate) struct Ipv6Handler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowPort {
    ipv6: TypeEnum,
    port: TypeEnum,
}

const ITEM: Struct = Struct(TypeEnum::Ipv6Addr);

const ITEM_ALLOW_PORT: StructAllowPort = StructAllowPort {
    ipv6: TypeEnum::Ipv6Addr,
    port: TypeEnum::OptionU16,
};

const ITEM_WITH_PORT: StructAllowPort = StructAllowPort {
    ipv6: TypeEnum::Ipv6Addr,
    port: TypeEnum::U16,
};

impl ValidatorHandler for Ipv6Handler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        #[cfg(feature = "serde")]
        let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
        #[cfg(feature = "serde")]
        let deserializer = ident_generator.next("__ValidatorsDeserializer");
        #[cfg(feature = "serde")]
        let deserializer_name = ident_generator.next("__validators_deserializer");
        #[cfg(feature = "serde")]
        let visitor = ident_generator.next("__ValidatorsVisitor");

        let type_attribute = IpXXAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            let (names, item): (Option<&[&str]>, &dyn Debug) = match type_attribute.port {
                TriAllow::Allow => (Some(&["ipv6", "port"]), &ITEM_ALLOW_PORT),
                TriAllow::Must => (Some(&["ipv6", "port"]), &ITEM_WITH_PORT),
                TriAllow::Disallow => (None, &ITEM),
            };

            if !fields_match(&data.fields, names) {
                return Err(panic::validator_for_specific_item(meta.path(), item));
            }

            let mut token_stream = proc_macro2::TokenStream::new();

            let name = ast.ident;

            let error_path: Path = syn::parse2(quote! { validators_prelude::Ipv6Error }).unwrap();

            #[cfg(feature = "test")]
            {
                let v_local = type_attribute.local;
                let v_port = type_attribute.port;

                token_stream.extend(quote! {
                    impl #name {
                        pub(crate) const V_LOCAL: validators_prelude::TriAllow = #v_local;
                        pub(crate) const V_PORT: validators_prelude::TriAllow = #v_port;
                    }
                });
            }

            let check_local = {
                match type_attribute.local {
                    TriAllow::Allow => quote! {},
                    TriAllow::Must => {
                        quote! {
                            if !is_local {
                                return Err(#error_path::LocalMust);
                            }
                        }
                    },
                    TriAllow::Disallow => {
                        quote! {
                            if is_local {
                                return Err(#error_path::LocalDisallow);
                            }
                        }
                    },
                }
            };

            let handle_local_ipv6 = if type_attribute.local == TriAllow::Allow {
                quote! {
                    false
                }
            } else {
                quote! {
                    validators_prelude::is_local_ipv6(ip)
                }
            };

            let handle_ipv6_with_port = if type_attribute.port.disallow() {
                quote! {
                    // Only an input in the `[...]:...` form seems to have a port.
                    match bytes.iter().copied().rposition(|e| e == b':') {
                        Some(colon_index) if colon_index > 2 && bytes[colon_index - 1] == b']' => {
                            return Err(#error_path::PortDisallow);
                        }
                        _ => return Err(#error_path::Invalid),
                    }
                }
            } else {
                quote! {
                    match bytes.iter().copied().rposition(|e| e == b':') {
                        Some(colon_index) => {
                            if colon_index > 2 && bytes[colon_index - 1] == b']' {
                                let ip_str = &s[1..(colon_index - 1)];

                                match ::core::net::Ipv6Addr::from_str(ip_str) {
                                    Ok(ip) => {
                                        let port_str = &s[(colon_index + 1)..];

                                        // A port has only digits, but `parse` also accepts a leading `+`.
                                        if port_str.starts_with('+') {
                                            return Err(#error_path::Invalid);
                                        }

                                        match port_str.parse::<u16>() {
                                            Ok(port) => {
                                                let is_local = #handle_local_ipv6;

                                                #check_local

                                                (ip, Some(port), is_local)
                                            }
                                            Err(_) => return Err(#error_path::Invalid),
                                        }
                                    }
                                    Err(_) => return Err(#error_path::Invalid),
                                }
                            } else {
                                return Err(#error_path::Invalid);
                            }
                        }
                        None => return Err(#error_path::Invalid),
                    }
                }
            };

            let handle_ipv6_without_port = if type_attribute.port.must() {
                quote! {
                    return Err(#error_path::PortMust);
                }
            } else {
                quote! {
                    let ip_str = &s[1..last_index];

                    match ::core::net::Ipv6Addr::from_str(ip_str) {
                        Ok(ip) => {
                            let is_local = #handle_local_ipv6;

                            #check_local

                            (ip, None, is_local)
                        }
                        Err(_) => return Err(#error_path::Invalid),
                    }
                }
            };

            let handle_ipv6_non_bracket = if type_attribute.port.must() {
                quote! {
                    Ok(_) => {
                        return Err(#error_path::PortMust);
                    }
                }
            } else {
                quote! {
                    Ok(ip) => {
                        let is_local = #handle_local_ipv6;

                        #check_local

                        (ip, None, is_local)
                    }
                }
            };

            token_stream.extend(quote! {
                impl #name {
                    fn v_parse_str(s: &str) -> ::core::result::Result<(::core::net::Ipv6Addr, Option<u16>, bool), #error_path> {
                        use ::core::str::FromStr;

                        let bytes = s.as_bytes();

                        if bytes.is_empty() {
                            return Err(#error_path::Invalid);
                        }

                        Ok(if bytes[0] == b'[' {
                            let last_index = bytes.len() - 1;

                            if bytes[last_index] == b']' {
                                #handle_ipv6_without_port
                            } else {
                                #handle_ipv6_with_port
                            }
                        } else {
                            match ::core::net::Ipv6Addr::from_str(s) {
                                #handle_ipv6_non_bracket
                                Err(_) => return Err(#error_path::Invalid)
                            }
                        })
                    }
                }
            });

            let create_instance = {
                match type_attribute.port {
                    TriAllow::Allow => {
                        quote! {
                            Self {
                                ipv6,
                                port: _port,
                            }
                        }
                    },
                    TriAllow::Must => {
                        quote! {
                            Self {
                                ipv6,
                                port: _port.unwrap(),
                            }
                        }
                    },
                    TriAllow::Disallow => {
                        quote! {
                            Self(ipv6)
                        }
                    },
                }
            };

            token_stream.extend(quote! {
                impl ValidateString for #name {
                    type Error = #error_path;

                    #[inline]
                    fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                        let (ipv6, _port, _is_local) = Self::v_parse_str(s.into().as_str())?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                        let (ipv6, _port, _is_local) = Self::v_parse_str(s.as_ref())?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
                        Self::v_parse_str(s.as_ref())?;

                        Ok(())
                    }
                }
            });

            token_stream.extend(match type_attribute.port {
                TriAllow::Allow => {
                    quote! {
                        impl ToUriAuthorityString for #name {
                            #[inline]
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<'_, str> {
                                let ip = self.ipv6;

                                match self.port {
                                    Some(port) => validators_prelude::Cow::Owned(validators_prelude::format!("[{}]:{}", ip, port)),
                                    None => validators_prelude::Cow::Owned(validators_prelude::format!("[{}]", ip)),
                                }
                            }
                        }
                    }
                },
                TriAllow::Must => {
                    quote! {
                        impl ToUriAuthorityString for #name {
                            #[inline]
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<'_, str> {
                                let ip = self.ipv6;
                                let port = self.port;

                                validators_prelude::Cow::Owned(validators_prelude::format!("[{}]:{}", ip, port))
                            }
                        }
                    }
                },
                TriAllow::Disallow => {
                    quote! {
                        impl ToUriAuthorityString for #name {
                            #[inline]
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<'_, str> {
                                let ip = self.0;

                                validators_prelude::Cow::Owned(validators_prelude::format!("[{}]", ip))
                            }
                        }
                    }
                },
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
                                serializer.serialize_str(&ToUriAuthorityString::to_uri_authority_string(self))
                            }
                        }
                    });
                }

                if type_attribute.serde_options.deserialize {
                    let expect = {
                        let mut s = String::from("an IPv6 string");

                        match type_attribute.local {
                            TriAllow::Allow => match type_attribute.port {
                                TriAllow::Allow => {
                                    s.push_str(" with an optional port");
                                },
                                TriAllow::Must => {
                                    s.push_str(" with a port");
                                },
                                TriAllow::Disallow => {
                                    s.push_str(" without ports");
                                },
                            },
                            TriAllow::Must => {
                                s.push_str(" which must be local");

                                match type_attribute.port {
                                    TriAllow::Allow => (),
                                    TriAllow::Must => {
                                        s.push_str(" and with a port");
                                    },
                                    TriAllow::Disallow => {
                                        s.push_str(" and without ports");
                                    },
                                }
                            },
                            TriAllow::Disallow => {
                                s.push_str(" which must not be local");

                                match type_attribute.port {
                                    TriAllow::Allow => (),
                                    TriAllow::Must => {
                                        s.push_str(" and must be with a port");
                                    },
                                    TriAllow::Disallow => {
                                        s.push_str(" and must be without ports");
                                    },
                                }
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
