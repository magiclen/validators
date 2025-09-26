mod host_attribute;

use educe::Educe;
use host_attribute::HostAttribute;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{tri_allow::TriAllow, type_enum::TypeEnum},
    panic,
};

pub(crate) struct HostHandler;

#[derive(Debug)]
#[allow(dead_code)] // Used for parsing
pub struct Struct(TypeEnum);

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
#[allow(dead_code)] // Used for parsing
pub struct StructAllowLocal {
    host:     TypeEnum,
    is_local: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
#[allow(dead_code)] // Used for parsing
pub struct StructAllowPort {
    host: TypeEnum,
    port: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
#[allow(dead_code)] // Used for parsing
pub struct StructAllowPortAllowLocal {
    host:     TypeEnum,
    is_local: TypeEnum,
    port:     TypeEnum,
}

const ITEM: Struct = Struct(TypeEnum::Host);
const ITEM_ALLOW_LOCAL: StructAllowLocal = StructAllowLocal {
    host:     TypeEnum::Host,
    is_local: TypeEnum::Boolean,
};
const ITEM_ALLOW_PORT: StructAllowPort = StructAllowPort {
    host: TypeEnum::Host,
    port: TypeEnum::OptionU16,
};
const ITEM_WITH_PORT: StructAllowPort = StructAllowPort {
    host: TypeEnum::Host,
    port: TypeEnum::U16,
};
const ITEM_ALLOW_LOCAL_ALLOW_PORT: StructAllowPortAllowLocal = StructAllowPortAllowLocal {
    host:     TypeEnum::Host,
    is_local: TypeEnum::Boolean,
    port:     TypeEnum::OptionU16,
};
const ITEM_ALLOW_LOCAL_WITH_PORT: StructAllowPortAllowLocal = StructAllowPortAllowLocal {
    host:     TypeEnum::Host,
    is_local: TypeEnum::Boolean,
    port:     TypeEnum::U16,
};

impl ValidatorHandler for HostHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = HostAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            if type_attribute.local == TriAllow::Allow
                && type_attribute.at_least_two_labels != TriAllow::Allow
            {
                match type_attribute.port {
                    TriAllow::Allow => {
                        if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 3 {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_ALLOW_LOCAL_ALLOW_PORT,
                                ));
                            }

                            for field in data.fields.iter() {
                                let ident_string = field.ident.as_ref().unwrap().to_string();

                                match ident_string.as_str() {
                                    "host" | "is_local" | "port" => (),
                                    _ => {
                                        return Err(panic::validator_for_specific_item(
                                            meta.path().get_ident().unwrap(),
                                            ITEM_ALLOW_LOCAL_ALLOW_PORT,
                                        ));
                                    },
                                }
                            }
                        } else {
                            return Err(panic::validator_for_specific_item(
                                meta.path().get_ident().unwrap(),
                                ITEM_ALLOW_LOCAL_ALLOW_PORT,
                            ));
                        }
                    },
                    TriAllow::Must => {
                        if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 3 {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_ALLOW_LOCAL_WITH_PORT,
                                ));
                            }

                            for field in data.fields.iter() {
                                let ident_string = field.ident.as_ref().unwrap().to_string();

                                match ident_string.as_str() {
                                    "host" | "is_local" | "port" => (),
                                    _ => {
                                        return Err(panic::validator_for_specific_item(
                                            meta.path().get_ident().unwrap(),
                                            ITEM_ALLOW_LOCAL_WITH_PORT,
                                        ));
                                    },
                                }
                            }
                        } else {
                            return Err(panic::validator_for_specific_item(
                                meta.path().get_ident().unwrap(),
                                ITEM_ALLOW_LOCAL_WITH_PORT,
                            ));
                        }
                    },
                    TriAllow::Disallow => {
                        if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 2 {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_ALLOW_LOCAL,
                                ));
                            }

                            for field in data.fields.iter() {
                                let ident_string = field.ident.as_ref().unwrap().to_string();

                                match ident_string.as_str() {
                                    "host" | "is_local" => (),
                                    _ => {
                                        return Err(panic::validator_for_specific_item(
                                            meta.path().get_ident().unwrap(),
                                            ITEM_ALLOW_LOCAL,
                                        ));
                                    },
                                }
                            }
                        } else {
                            return Err(panic::validator_for_specific_item(
                                meta.path().get_ident().unwrap(),
                                ITEM_ALLOW_LOCAL,
                            ));
                        }
                    },
                }
            } else {
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
                                    "host" | "port" => (),
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
                                    "host" | "port" => (),
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
            }

            let mut token_stream = proc_macro2::TokenStream::new();

            let name = ast.ident;

            let error_path: Path = syn::parse2(quote! { validators_prelude::HostError }).unwrap();

            #[cfg(feature = "test")]
            {
                let v_local = type_attribute.local;
                let v_port = type_attribute.port;
                let v_at_least_two_labels = type_attribute.at_least_two_labels;

                token_stream.extend(quote! {
                    impl #name {
                        pub(crate) const V_LOCAL: validators_prelude::TriAllow = #v_local;
                        pub(crate) const V_PORT: validators_prelude::TriAllow = #v_port;
                        pub(crate) const V_AT_LEAST_TWO_LABELS: validators_prelude::TriAllow = #v_at_least_two_labels;
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

            let handle_local_ipv6 = if type_attribute.at_least_two_labels == TriAllow::Allow
                && type_attribute.local == TriAllow::Allow
            {
                quote! {
                    false
                }
            } else {
                quote! {
                    validators_prelude::is_local_ipv6(ip)
                }
            };

            let handle_ipv6_without_port = if type_attribute.port.must() {
                quote! {
                    return Err(#error_path::PortMust);
                }
            } else {
                quote! {
                    let ip_str = unsafe { ::core::str::from_utf8_unchecked(&bytes[1..last_index]) };

                    match ::std::net::Ipv6Addr::from_str(ip_str) {
                        Ok(ip) => {
                            let is_local = #handle_local_ipv6;

                            #check_local

                            (validators_prelude::Host::IPv6(ip), None, is_local)
                        }
                        Err(_) => return Err(#error_path::Invalid),
                    }
                }
            };

            let handle_ipv6_with_port = if type_attribute.port.disallow() {
                quote! {
                    return Err(#error_path::PortDisallow);
                }
            } else {
                quote! {
                    match bytes.iter().copied().rposition(|e| e == b':') {
                        Some(colon_index) => {
                            if colon_index > 2 && bytes[colon_index - 1] == b']' {
                                let ip_str = unsafe { ::core::str::from_utf8_unchecked(&bytes[1..(colon_index - 1)]) };

                                match ::std::net::Ipv6Addr::from_str(ip_str) {
                                    Ok(ip) => {
                                        let port_str = unsafe { ::core::str::from_utf8_unchecked(&bytes[(colon_index + 1)..]) };

                                        match port_str.parse::<u16>() {
                                            Ok(port) => {
                                                let is_local = #handle_local_ipv6;

                                                #check_local

                                                (validators_prelude::Host::IPv6(ip), Some(port), is_local)
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

            let handle_ipv6_bracket = if type_attribute.at_least_two_labels.disallow() {
                quote! {
                    return Err(#error_path::AtLeastTwoLabelsDisallow);
                }
            } else {
                quote! {
                    let last_index = bytes.len() - 1;

                    if bytes[last_index] == b']' {
                        #handle_ipv6_without_port
                    } else {
                        #handle_ipv6_with_port
                    }
                }
            };

            let handle_ipv6_non_bracket = if type_attribute.at_least_two_labels.disallow() {
                quote! {
                    Ok(_) => {
                        return Err(#error_path::AtLeastTwoLabelsDisallow);
                    }
                }
            } else if type_attribute.port.must() {
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

                        (validators_prelude::Host::IPv6(ip), None, is_local)
                    }
                }
            };

            let handle_domain_and_port = if type_attribute.port.disallow() {
                quote! {
                    Some(_) => {
                        return Err(#error_path::PortDisallow);
                    }
                }
            } else {
                quote! {
                    Some(colon_index) => {
                        (
                            unsafe { ::core::str::from_utf8_unchecked(&bytes[..colon_index]) },
                            Some(unsafe { ::core::str::from_utf8_unchecked(&bytes[(colon_index + 1)..]) }),
                        )
                    }
                }
            };

            let handle_domain_none_port = if type_attribute.port.must() {
                quote! {
                    return Err(#error_path::PortMust);
                }
            } else {
                quote! {
                    (unsafe { ::core::str::from_utf8_unchecked(bytes) }, None::<&str>)
                }
            };

            let handle_port = if type_attribute.port.disallow() {
                quote! {
                    None::<u16>
                }
            } else {
                quote! {
                    match port_str {
                        Some(port_str) => {
                            match port_str.parse::<u16>() {
                                Ok(port) => Some(port),
                                Err(_) => return Err(#error_path::Invalid),
                            }
                        }
                        None => None,
                    }
                }
            };

            let handle_local_ipv4 = if type_attribute.at_least_two_labels == TriAllow::Allow
                && type_attribute.local == TriAllow::Allow
            {
                quote! {
                    false
                }
            } else {
                quote! {
                    validators_prelude::is_local_ipv4(ip)
                }
            };

            let handle_ipv4 = if type_attribute.at_least_two_labels.disallow() {
                quote! {
                    Ok(_) => {
                        return Err(#error_path::AtLeastTwoLabelsDisallow);
                    }
                }
            } else {
                quote! {
                    Ok(ip) => {
                        let port = #handle_port;

                        let is_local = #handle_local_ipv4;

                        #check_local

                        (validators_prelude::Host::IPv4(ip), port, is_local)
                    }
                }
            };

            let handle_none_ipv4 = {
                let check_at_least_two_labels = {
                    match type_attribute.at_least_two_labels {
                        TriAllow::Allow => quote! {},
                        TriAllow::Must => {
                            quote! {
                                if !is_local && !validators_prelude::is_at_least_two_labels_domain(&ascii_domain) {
                                    return Err(#error_path::AtLeastTwoLabelsMust);
                                }
                            }
                        },
                        TriAllow::Disallow => {
                            quote! {
                                if !is_local && validators_prelude::is_at_least_two_labels_domain(&ascii_domain) {
                                    return Err(#error_path::AtLeastTwoLabelsDisallow);
                                }
                            }
                        },
                    }
                };

                let handle_local_domain = if type_attribute.at_least_two_labels == TriAllow::Allow
                    && type_attribute.local == TriAllow::Allow
                {
                    quote! {
                        false
                    }
                } else {
                    quote! {
                        validators_prelude::is_local_domain(&ascii_domain)
                    }
                };

                quote! {
                    match validators_prelude::idna::Config::default()
                        .use_std3_ascii_rules(true)
                        .verify_dns_length(true)
                        .check_hyphens(true)
                        .to_ascii(domain_str)
                    {
                        Ok(ascii_domain) => {
                            let port = #handle_port;

                            let is_local = #handle_local_domain;

                            #check_at_least_two_labels

                            #check_local

                            (validators_prelude::Host::Domain(ascii_domain), port, is_local)
                        }
                        Err(_) => return Err(#error_path::Invalid),
                    }
                }
            };

            token_stream.extend(quote! {
                impl #name {
                    fn v_parse_str(s: &str) -> Result<(validators_prelude::Host, Option<u16>, bool), #error_path> {
                        use ::core::str::FromStr;

                        let bytes = s.as_bytes();

                        if bytes.is_empty() {
                            return Err(#error_path::Invalid);
                        }

                        Ok(if bytes[0] == b'[' {
                            #handle_ipv6_bracket
                        } else {
                            match ::std::net::Ipv6Addr::from_str(s) {
                                #handle_ipv6_non_bracket
                                Err(_) => {
                                    let (domain_str, port_str) = match bytes.iter().copied().rposition(|e| e == b':') {
                                        #handle_domain_and_port
                                        None => {
                                            #handle_domain_none_port
                                        }
                                    };

                                    if domain_str.ends_with('.') {
                                        return Err(#error_path::Invalid);
                                    }

                                    match ::std::net::Ipv4Addr::from_str(domain_str) {
                                        #handle_ipv4
                                        Err(_) => {
                                            #handle_none_ipv4
                                        }
                                    }
                                }
                            }
                        })
                    }
                }
            });

            let create_instance = {
                if type_attribute.local == TriAllow::Allow
                    && type_attribute.at_least_two_labels != TriAllow::Allow
                {
                    match type_attribute.port {
                        TriAllow::Allow => {
                            quote! {
                                Self {
                                    host,
                                    is_local: _is_local,
                                    port: _port,
                                }
                            }
                        },
                        TriAllow::Must => {
                            quote! {
                                Self {
                                    host,
                                    is_local: _is_local,
                                    port: _port.unwrap(),
                                }
                            }
                        },
                        TriAllow::Disallow => {
                            quote! {
                                Self {
                                    host,
                                    is_local: _is_local,
                                }
                            }
                        },
                    }
                } else {
                    match type_attribute.port {
                        TriAllow::Allow => {
                            quote! {
                                Self {
                                    host,
                                    port: _port,
                                }
                            }
                        },
                        TriAllow::Must => {
                            quote! {
                                Self {
                                    host,
                                    port: _port.unwrap(),
                                }
                            }
                        },
                        TriAllow::Disallow => {
                            quote! {
                                Self(host)
                            }
                        },
                    }
                }
            };

            token_stream.extend(quote! {
                impl ValidateString for #name {
                    type Error = #error_path;

                    #[inline]
                    fn parse_string<S: Into<validators_prelude::String>>(s: S) -> Result<Self, Self::Error> {
                        let (host, _port, _is_local) = Self::v_parse_str(s.into().as_str())?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> Result<Self, Self::Error> {
                        let (host, _port, _is_local) = Self::v_parse_str(s.as_ref())?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error> {
                        Self::v_parse_str(s.as_ref())?;

                        Ok(())
                    }
                }
            });

            token_stream.extend(
                if (type_attribute.local != TriAllow::Allow
                    || type_attribute.at_least_two_labels == TriAllow::Allow)
                    && type_attribute.port.disallow()
                {
                    quote! {
                        impl ToUriAuthorityString for #name {
                            #[inline]
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                                match &self.0 {
                                    validators_prelude::Host::IPv4(ip) => validators_prelude::Cow::Owned(validators_prelude::format!("{}", ip)),
                                    validators_prelude::Host::IPv6(ip) => validators_prelude::Cow::Owned(validators_prelude::format!("[{}]", ip)),
                                    validators_prelude::Host::Domain(domain) => validators_prelude::Cow::Borrowed(domain),
                                }
                            }
                        }
                    }
                } else {
                    match type_attribute.port {
                        TriAllow::Allow => {
                            quote! {
                                impl ToUriAuthorityString for #name {
                                    #[inline]
                                    fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                                        match &self.host {
                                            validators_prelude::Host::IPv4(ip) => {
                                                match self.port {
                                                    Some(port) => validators_prelude::Cow::Owned(validators_prelude::format!("{}:{}", ip, port)),
                                                    None => validators_prelude::Cow::Owned(validators_prelude::format!("{}", ip)),
                                                }
                                            },
                                            validators_prelude::Host::IPv6(ip) => {
                                                match self.port {
                                                    Some(port) => validators_prelude::Cow::Owned(validators_prelude::format!("[{}]:{}", ip, port)),
                                                    None => validators_prelude::Cow::Owned(validators_prelude::format!("[{}]", ip)),
                                                }
                                            },
                                            validators_prelude::Host::Domain(domain) => {
                                                match self.port {
                                                    Some(port) => validators_prelude::Cow::Owned(validators_prelude::format!("{}:{}", domain, port)),
                                                    None => validators_prelude::Cow::Borrowed(domain),
                                                }
                                            },
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
                                        let port = self.port;

                                        match &self.host {
                                            validators_prelude::Host::IPv4(ip) => validators_prelude::Cow::Owned(validators_prelude::format!("{}:{}", ip, port)),
                                            validators_prelude::Host::IPv6(ip) => validators_prelude::Cow::Owned(validators_prelude::format!("[{}]:{}", ip, port)),
                                            validators_prelude::Host::Domain(domain) => validators_prelude::Cow::Owned(validators_prelude::format!("{}:{}", domain, port)),
                                        }
                                    }
                                }
                            }
                        },
                        TriAllow::Disallow => {
                            quote! {
                                impl ToUriAuthorityString for #name {
                                    #[inline]
                                    fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                                        match &self.host {
                                            validators_prelude::Host::IPv4(ip) => validators_prelude::Cow::Owned(validators_prelude::format!("{}", ip)),
                                            validators_prelude::Host::IPv6(ip) => validators_prelude::Cow::Owned(validators_prelude::format!("[{}]", ip)),
                                            validators_prelude::Host::Domain(domain) => validators_prelude::Cow::Borrowed(domain),
                                        }
                                    }
                                }
                            }
                        },
                    }
                },
            );

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
                        let mut s = String::from("a non-fully-qualified ");

                        match type_attribute.at_least_two_labels {
                            TriAllow::Allow => (),
                            TriAllow::Must => {
                                s.push_str("and at-least-two-labels ");
                            },
                            TriAllow::Disallow => {
                                s.push_str("and one-label ");
                            },
                        }

                        s.push_str("domain name or an IP string");

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
