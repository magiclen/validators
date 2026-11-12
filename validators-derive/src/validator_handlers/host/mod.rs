mod host_attribute;

use std::fmt::Debug;

use educe::Educe;
use host_attribute::HostAttribute;
use quote::quote;
use syn::{Data, DeriveInput, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{fields_match, tri_allow::TriAllow, type_enum::TypeEnum},
    panic,
};

pub(crate) struct HostHandler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowLocal {
    host:     TypeEnum,
    is_local: TypeEnum,
}

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowPort {
    host: TypeEnum,
    port: TypeEnum,
}

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
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
        #[cfg(feature = "serde")]
        let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
        #[cfg(feature = "serde")]
        let deserializer = ident_generator.next("__ValidatorsDeserializer");
        #[cfg(feature = "serde")]
        let deserializer_name = ident_generator.next("__validators_deserializer");
        #[cfg(feature = "serde")]
        let visitor = ident_generator.next("__ValidatorsVisitor");

        let type_attribute = HostAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            // `is_local` is kept only when `local` is allowed and `at_least_two_labels` is checked.
            let has_local_field = type_attribute.local == TriAllow::Allow
                && type_attribute.at_least_two_labels != TriAllow::Allow;

            let (names, item): (Option<&[&str]>, &dyn Debug) =
                match (has_local_field, type_attribute.port) {
                    (true, TriAllow::Allow) => {
                        (Some(&["host", "is_local", "port"]), &ITEM_ALLOW_LOCAL_ALLOW_PORT)
                    },
                    (true, TriAllow::Must) => {
                        (Some(&["host", "is_local", "port"]), &ITEM_ALLOW_LOCAL_WITH_PORT)
                    },
                    (true, TriAllow::Disallow) => (Some(&["host", "is_local"]), &ITEM_ALLOW_LOCAL),
                    (false, TriAllow::Allow) => (Some(&["host", "port"]), &ITEM_ALLOW_PORT),
                    (false, TriAllow::Must) => (Some(&["host", "port"]), &ITEM_WITH_PORT),
                    (false, TriAllow::Disallow) => (None, &ITEM),
                };

            if !fields_match(&data.fields, names) {
                return Err(panic::validator_for_specific_item(meta.path(), item));
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
                    let ip_str = &s[1..last_index];

                    match ::core::net::Ipv6Addr::from_str(ip_str) {
                        Ok(ip) => {
                            let is_local = #handle_local_ipv6;

                            #check_local

                            (Ok(::core::net::IpAddr::V6(ip)), None, is_local)
                        }
                        Err(_) => return Err(#error_path::Invalid),
                    }
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

                                                (Ok(::core::net::IpAddr::V6(ip)), Some(port), is_local)
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
                    // An IP literal has more than one label, but only a valid one is reported as such.
                    match bytes.iter().copied().position(|e| e == b']') {
                        Some(closing_bracket_index) if ::core::net::Ipv6Addr::from_str(&s[1..closing_bracket_index]).is_ok() => {
                            return Err(#error_path::AtLeastTwoLabelsDisallow);
                        }
                        _ => return Err(#error_path::Invalid),
                    }
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

                        (Ok(::core::net::IpAddr::V6(ip)), None, is_local)
                    }
                }
            };

            let handle_domain_and_port = if type_attribute.port.disallow() {
                quote! {
                    Some(colon_index) => {
                        // `host:port` has only one colon, so an input with more colons is just invalid.
                        if bytes[..colon_index].contains(&b':') {
                            return Err(#error_path::Invalid);
                        }

                        return Err(#error_path::PortDisallow);
                    }
                }
            } else {
                quote! {
                    Some(colon_index) => {
                        (
                            &s[..colon_index],
                            Some(&s[(colon_index + 1)..]),
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
                    (s, None::<&str>)
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
                            // A port has only digits, but `parse` also accepts a leading `+`.
                            if port_str.starts_with('+') {
                                return Err(#error_path::Invalid);
                            }

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

                        (Ok(::core::net::IpAddr::V4(ip)), port, is_local)
                    }
                }
            };

            let handle_domain = {
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
                            // Local names are not exempt here, because subdomains such as `foo.localhost` have two labels.
                            quote! {
                                if validators_prelude::is_at_least_two_labels_domain(&ascii_domain) {
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
                    Err(ascii_domain) => {
                        let port = #handle_port;

                        let is_local = #handle_local_domain;

                        #check_at_least_two_labels

                        #check_local

                        (Err(ascii_domain), port, is_local)
                    }
                }
            };

            token_stream.extend(quote! {
                impl #name {
                    fn v_parse_str(s: &str) -> ::core::result::Result<
                        (::core::result::Result<::core::net::IpAddr, validators_prelude::Cow<'_, str>>, Option<u16>, bool),
                        #error_path,
                    > {
                        use ::core::str::FromStr;

                        let bytes = s.as_bytes();

                        if bytes.is_empty() {
                            return Err(#error_path::Invalid);
                        }

                        Ok(if bytes[0] == b'[' {
                            #handle_ipv6_bracket
                        } else {
                            match ::core::net::Ipv6Addr::from_str(s) {
                                #handle_ipv6_non_bracket
                                Err(_) => {
                                    let (domain_str, port_str) = match bytes.iter().copied().rposition(|e| e == b':') {
                                        #handle_domain_and_port
                                        None => {
                                            #handle_domain_none_port
                                        }
                                    };

                                    let host = match ::core::net::Ipv4Addr::from_str(domain_str) {
                                        Ok(ip) => Ok(ip),
                                        Err(_) => {
                                            let ascii_domain = validators_prelude::idna::uts46::Uts46::new()
                                                .to_ascii(
                                                    domain_str.as_bytes(),
                                                    validators_prelude::idna::uts46::AsciiDenyList::STD3,
                                                    validators_prelude::idna::uts46::Hyphens::CheckFirstLast,
                                                    validators_prelude::idna::uts46::DnsLength::VerifyAllowRootDot,
                                                )
                                                .map_err(|_| #error_path::Invalid)?;

                                            if ascii_domain.ends_with('.') {
                                                return Err(#error_path::Invalid);
                                            }

                                            ::core::net::Ipv4Addr::from_str(&ascii_domain).map_err(|_| ascii_domain)
                                        }
                                    };

                                    match host {
                                        #handle_ipv4
                                        #handle_domain
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
                    fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                        let mut s = s.into();
                        let (host, _port, _is_local) = Self::v_parse_str(s.as_str())?;
                        let host = match host {
                            Ok(::core::net::IpAddr::V4(ip)) => validators_prelude::Host::IPv4(ip),
                            Ok(::core::net::IpAddr::V6(ip)) => validators_prelude::Host::IPv6(ip),
                            Err(validators_prelude::Cow::Borrowed(domain)) => {
                                // The borrowed domain is always a prefix of the input.
                                s.truncate(domain.len());
                                validators_prelude::Host::Domain(s)
                            }
                            Err(validators_prelude::Cow::Owned(domain)) => validators_prelude::Host::Domain(domain),
                        };

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                        let (host, _port, _is_local) = Self::v_parse_str(s.as_ref())?;
                        let host = match host {
                            Ok(::core::net::IpAddr::V4(ip)) => validators_prelude::Host::IPv4(ip),
                            Ok(::core::net::IpAddr::V6(ip)) => validators_prelude::Host::IPv6(ip),
                            Err(domain) => validators_prelude::Host::Domain(domain.into_owned()),
                        };

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
                        Self::v_parse_str(s.as_ref()).map(|_| ())
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
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<'_, str> {
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
                                    fn to_uri_authority_string(&self) -> validators_prelude::Cow<'_, str> {
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
                                    fn to_uri_authority_string(&self) -> validators_prelude::Cow<'_, str> {
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
                                    fn to_uri_authority_string(&self) -> validators_prelude::Cow<'_, str> {
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

                        // `at_least_two_labels(Disallow)` rejects every IP address.
                        if type_attribute.at_least_two_labels.disallow() {
                            s.push_str("domain name");
                        } else {
                            s.push_str("domain name or an IP string");
                        }

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

                                    #[inline]
                                    fn visit_string<E>(self, v: validators_prelude::String) -> ::core::result::Result<Self::Value, E>
                                    where
                                        E: validators_prelude::serde::de::Error, {
                                        <Self::Value as ValidateString>::parse_string(v).map_err(validators_prelude::serde::de::Error::custom)
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
