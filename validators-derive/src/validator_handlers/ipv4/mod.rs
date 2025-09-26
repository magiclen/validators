use educe::Educe;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{
        attributes::ip_xx_attribute::IpXXAttribute, tri_allow::TriAllow, type_enum::TypeEnum,
    },
    panic,
};

pub(crate) struct Ipv4Handler;

#[derive(Debug)]
#[allow(dead_code)] // Used for parsing
pub struct Struct(TypeEnum);

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
#[allow(dead_code)] // Used for parsing
pub struct StructAllowPort {
    ipv4: TypeEnum,
    port: TypeEnum,
}

const ITEM: Struct = Struct(TypeEnum::Ipv4Addr);

const ITEM_ALLOW_PORT: StructAllowPort = StructAllowPort {
    ipv4: TypeEnum::Ipv4Addr,
    port: TypeEnum::OptionU16,
};

const ITEM_WITH_PORT: StructAllowPort = StructAllowPort {
    ipv4: TypeEnum::Ipv4Addr,
    port: TypeEnum::U16,
};

impl ValidatorHandler for Ipv4Handler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = IpXXAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            match type_attribute.port {
                TriAllow::Allow => {
                    if let Fields::Named(_) = &data.fields {
                        if data.fields.len() != 2 {
                            return Err(panic::validator_for_specific_item(
                                meta.path().get_ident().unwrap(),
                                ITEM_ALLOW_PORT,
                            ));
                        }

                        for field in data.fields.iter() {
                            let ident_string = field.ident.as_ref().unwrap().to_string();

                            match ident_string.as_str() {
                                "ipv4" | "port" => (),
                                _ => {
                                    return Err(panic::validator_for_specific_item(
                                        meta.path().get_ident().unwrap(),
                                        ITEM_ALLOW_PORT,
                                    ));
                                },
                            }
                        }
                    } else {
                        return Err(panic::validator_for_specific_item(
                            meta.path().get_ident().unwrap(),
                            ITEM_ALLOW_PORT,
                        ));
                    }
                },
                TriAllow::Must => {
                    if let Fields::Named(_) = &data.fields {
                        if data.fields.len() != 2 {
                            return Err(panic::validator_for_specific_item(
                                meta.path().get_ident().unwrap(),
                                ITEM_WITH_PORT,
                            ));
                        }

                        for field in data.fields.iter() {
                            let ident_string = field.ident.as_ref().unwrap().to_string();

                            match ident_string.as_str() {
                                "ipv4" | "port" => (),
                                _ => {
                                    return Err(panic::validator_for_specific_item(
                                        meta.path().get_ident().unwrap(),
                                        ITEM_WITH_PORT,
                                    ));
                                },
                            }
                        }
                    } else {
                        return Err(panic::validator_for_specific_item(
                            meta.path().get_ident().unwrap(),
                            ITEM_WITH_PORT,
                        ));
                    }
                },
                TriAllow::Disallow => {
                    if let Fields::Unnamed(_) = &data.fields {
                        if data.fields.len() != 1 {
                            return Err(panic::validator_for_specific_item(
                                meta.path().get_ident().unwrap(),
                                ITEM,
                            ));
                        }
                    } else {
                        return Err(panic::validator_for_specific_item(
                            meta.path().get_ident().unwrap(),
                            ITEM,
                        ));
                    }
                },
            }

            let mut token_stream = proc_macro2::TokenStream::new();

            let name = ast.ident;

            let error_path: Path = syn::parse2(quote! { validators_prelude::Ipv4Error }).unwrap();

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

            let handle_local_ipv4 = if type_attribute.local == TriAllow::Allow {
                quote! {
                    false
                }
            } else {
                quote! {
                    validators_prelude::is_local_ipv4(ip)
                }
            };

            let handle_ipv4_with_port = if type_attribute.port.disallow() {
                quote! {
                    Some(_) => {
                        return Err(#error_path::PortDisallow);
                    }
                }
            } else {
                quote! {
                    Some(colon_index) => {
                        let ip_str = unsafe { ::core::str::from_utf8_unchecked(&bytes[..colon_index]) };

                        match ::std::net::Ipv4Addr::from_str(ip_str) {
                            Ok(ip) => {
                                let port_str =
                                    unsafe { ::core::str::from_utf8_unchecked(&bytes[(colon_index + 1)..]) };

                                match port_str.parse::<u16>() {
                                    Ok(port) => {
                                        let is_local = #handle_local_ipv4;

                                        #check_local

                                        (ip, Some(port), is_local)
                                    }
                                    Err(_) => return Err(#error_path::Invalid),
                                }
                            }
                            Err(_) => return Err(#error_path::Invalid),
                        }
                    }
                }
            };

            let handle_ipv4_without_port = if type_attribute.port.must() {
                quote! {
                    return Err(#error_path::PortMust);
                }
            } else {
                quote! {
                    match ::std::net::Ipv4Addr::from_str(s) {
                        Ok(ip) => {
                            let is_local = #handle_local_ipv4;

                            #check_local

                            (ip, None, is_local)
                        }
                        Err(_) => return Err(#error_path::Invalid),
                    }
                }
            };

            token_stream.extend(quote! {
                impl #name {
                    fn v_parse_str(s: &str) -> Result<(::std::net::Ipv4Addr, Option<u16>, bool), #error_path> {
                        use ::core::str::FromStr;

                        let bytes = s.as_bytes();

                        if bytes.is_empty() {
                            return Err(#error_path::Invalid);
                        }

                        Ok(match bytes.iter().copied().rposition(|e| e == b':') {
                            #handle_ipv4_with_port
                            None => {
                                #handle_ipv4_without_port
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
                                ipv4,
                                port: _port,
                            }
                        }
                    },
                    TriAllow::Must => {
                        quote! {
                            Self {
                                ipv4,
                                port: _port.unwrap(),
                            }
                        }
                    },
                    TriAllow::Disallow => {
                        quote! {
                            Self(ipv4)
                        }
                    },
                }
            };

            token_stream.extend(quote! {
                impl ValidateString for #name {
                    type Error = #error_path;

                    #[inline]
                    fn parse_string<S: Into<validators_prelude::String>>(s: S) -> Result<Self, Self::Error> {
                        let (ipv4, _port, _is_local) = Self::v_parse_str(s.into().as_str())?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> Result<Self, Self::Error> {
                        let (ipv4, _port, _is_local) = Self::v_parse_str(s.as_ref())?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error> {
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
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                                let ip = self.ipv4;

                                match self.port {
                                    Some(port) => validators_prelude::Cow::Owned(validators_prelude::format!("{}:{}", ip, port)),
                                    None => validators_prelude::Cow::Owned(validators_prelude::format!("{}", ip)),
                                }
                            }
                        }
                    }
                },
                TriAllow::Must => {
                    quote! {
                        impl ToUriAuthorityString for #name {
                            #[inline]
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                                let ip = self.ipv4;
                                let port = self.port;

                                validators_prelude::Cow::Owned(validators_prelude::format!("{}:{}", ip, port))
                            }
                        }
                    }
                },
                TriAllow::Disallow => {
                    quote! {
                        impl ToUriAuthorityString for #name {
                            #[inline]
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                                let ip = self.0;

                                validators_prelude::Cow::Owned(validators_prelude::format!("{}", ip))
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
                            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                where
                                    S: validators_prelude::serde::Serializer, {
                                serializer.serialize_str(&ToUriAuthorityString::to_uri_authority_string(self))
                            }
                        }
                    });
                }

                if type_attribute.serde_options.deserialize {
                    let expect = {
                        let mut s = String::from("an IPv4 string");

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
                                }

                                deserializer.deserialize_str(MyVisitor)
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

        Err(panic::validator_for_specific_item(meta.path().get_ident().unwrap(), ITEM))
    }
}
