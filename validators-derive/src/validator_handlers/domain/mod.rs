mod domain_attribute;

use domain_attribute::DomainAttribute;
use educe::Educe;
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{tri_allow::TriAllow, type_enum::TypeEnum},
    panic,
};

pub(crate) struct DomainHandler;

#[derive(Debug)]
#[allow(dead_code)] // Used for parsing
pub struct Struct(TypeEnum);

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
#[allow(dead_code)] // Used for parsing
pub struct StructAllowLocal {
    domain:   TypeEnum,
    is_local: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
#[allow(dead_code)] // Used for parsing
pub struct StructAllowPort {
    domain: TypeEnum,
    port:   TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
#[allow(dead_code)] // Used for parsing
pub struct StructAllowPortAllowLocal {
    domain:   TypeEnum,
    is_local: TypeEnum,
    port:     TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
#[allow(dead_code)] // Used for parsing
pub struct StructAllowIPv4 {
    domain:  TypeEnum,
    is_ipv4: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
#[allow(dead_code)] // Used for parsing
pub struct StructAllowIPv4AllowLocal {
    domain:   TypeEnum,
    is_ipv4:  TypeEnum,
    is_local: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
#[allow(dead_code)] // Used for parsing
pub struct StructAllowIPv4AllowPort {
    domain:  TypeEnum,
    is_ipv4: TypeEnum,
    port:    TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
#[allow(dead_code)] // Used for parsing
pub struct StructAllowIPv4AllowPortAllowLocal {
    domain:   TypeEnum,
    is_ipv4:  TypeEnum,
    is_local: TypeEnum,
    port:     TypeEnum,
}

const ITEM: Struct = Struct(TypeEnum::String);
const ITEM_ALLOW_LOCAL: StructAllowLocal = StructAllowLocal {
    domain:   TypeEnum::String,
    is_local: TypeEnum::Boolean,
};
const ITEM_ALLOW_PORT: StructAllowPort = StructAllowPort {
    domain: TypeEnum::String,
    port:   TypeEnum::OptionU16,
};
const ITEM_WITH_PORT: StructAllowPort = StructAllowPort {
    domain: TypeEnum::String,
    port:   TypeEnum::U16,
};
const ITEM_ALLOW_LOCAL_ALLOW_PORT: StructAllowPortAllowLocal = StructAllowPortAllowLocal {
    domain:   TypeEnum::String,
    is_local: TypeEnum::Boolean,
    port:     TypeEnum::OptionU16,
};
const ITEM_ALLOW_LOCAL_WITH_PORT: StructAllowPortAllowLocal = StructAllowPortAllowLocal {
    domain:   TypeEnum::String,
    is_local: TypeEnum::Boolean,
    port:     TypeEnum::U16,
};
const ITEM_ALLOW_IPV4_ALLOW_LOCAL: StructAllowIPv4AllowLocal = StructAllowIPv4AllowLocal {
    domain:   TypeEnum::String,
    is_ipv4:  TypeEnum::Boolean,
    is_local: TypeEnum::Boolean,
};
const ITEM_ALLOW_IPV4_ALLOW_LOCAL_ALLOW_PORT: StructAllowIPv4AllowPortAllowLocal =
    StructAllowIPv4AllowPortAllowLocal {
        domain:   TypeEnum::String,
        is_ipv4:  TypeEnum::Boolean,
        is_local: TypeEnum::Boolean,
        port:     TypeEnum::OptionU16,
    };
const ITEM_ALLOW_IPV4_ALLOW_LOCAL_WITH_PORT: StructAllowIPv4AllowPortAllowLocal =
    StructAllowIPv4AllowPortAllowLocal {
        domain:   TypeEnum::String,
        is_ipv4:  TypeEnum::Boolean,
        is_local: TypeEnum::Boolean,
        port:     TypeEnum::U16,
    };

impl ValidatorHandler for DomainHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = DomainAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            let mut meta_is_conflict = false;

            if type_attribute.ipv4.must() && type_attribute.at_least_two_labels.disallow() {
                if type_attribute.conflict.disallow() {
                    return Err(syn::Error::new(
                        meta.span(),
                        "`ipv4(Must)` and `at_least_two_labels(Disallow)` cannot be used together",
                    ));
                }

                meta_is_conflict = true;
            }

            match type_attribute.ipv4 {
                TriAllow::Allow => {
                    if type_attribute.local == TriAllow::Allow
                        && type_attribute.at_least_two_labels != TriAllow::Allow
                    {
                        match type_attribute.port {
                            TriAllow::Allow => {
                                if let Fields::Named(_) = &data.fields {
                                    if data.fields.len() != 4 {
                                        return Err(panic::validator_for_specific_item(
                                            meta.path().get_ident().unwrap(),
                                            ITEM_ALLOW_IPV4_ALLOW_LOCAL_ALLOW_PORT,
                                        ));
                                    }

                                    for field in data.fields.iter() {
                                        let ident_string =
                                            field.ident.as_ref().unwrap().to_string();

                                        match ident_string.as_str() {
                                            "domain" | "is_ipv4" | "is_local" | "port" => (),
                                            _ => {
                                                return Err(panic::validator_for_specific_item(
                                                    meta.path().get_ident().unwrap(),
                                                    ITEM_ALLOW_IPV4_ALLOW_LOCAL_ALLOW_PORT,
                                                ));
                                            },
                                        }
                                    }
                                } else {
                                    return Err(panic::validator_for_specific_item(
                                        meta.path().get_ident().unwrap(),
                                        ITEM_ALLOW_IPV4_ALLOW_LOCAL_ALLOW_PORT,
                                    ));
                                }
                            },
                            TriAllow::Must => {
                                if let Fields::Named(_) = &data.fields {
                                    if data.fields.len() != 4 {
                                        return Err(panic::validator_for_specific_item(
                                            meta.path().get_ident().unwrap(),
                                            ITEM_ALLOW_IPV4_ALLOW_LOCAL_WITH_PORT,
                                        ));
                                    }

                                    for field in data.fields.iter() {
                                        let ident_string =
                                            field.ident.as_ref().unwrap().to_string();

                                        match ident_string.as_str() {
                                            "domain" | "is_ipv4" | "is_local" | "port" => (),
                                            _ => {
                                                return Err(panic::validator_for_specific_item(
                                                    meta.path().get_ident().unwrap(),
                                                    ITEM_ALLOW_IPV4_ALLOW_LOCAL_WITH_PORT,
                                                ));
                                            },
                                        }
                                    }
                                } else {
                                    return Err(panic::validator_for_specific_item(
                                        meta.path().get_ident().unwrap(),
                                        ITEM_ALLOW_IPV4_ALLOW_LOCAL_WITH_PORT,
                                    ));
                                }
                            },
                            TriAllow::Disallow => {
                                if let Fields::Named(_) = &data.fields {
                                    if data.fields.len() != 3 {
                                        return Err(panic::validator_for_specific_item(
                                            meta.path().get_ident().unwrap(),
                                            ITEM_ALLOW_IPV4_ALLOW_LOCAL,
                                        ));
                                    }

                                    for field in data.fields.iter() {
                                        let ident_string =
                                            field.ident.as_ref().unwrap().to_string();

                                        match ident_string.as_str() {
                                            "domain" | "is_ipv4" | "is_local" => (),
                                            _ => {
                                                return Err(panic::validator_for_specific_item(
                                                    meta.path().get_ident().unwrap(),
                                                    ITEM_ALLOW_IPV4_ALLOW_LOCAL,
                                                ));
                                            },
                                        }
                                    }
                                } else {
                                    return Err(panic::validator_for_specific_item(
                                        meta.path().get_ident().unwrap(),
                                        ITEM_ALLOW_IPV4_ALLOW_LOCAL,
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
                                        let ident_string =
                                            field.ident.as_ref().unwrap().to_string();

                                        match ident_string.as_str() {
                                            "domain" | "port" => (),
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
                                        let ident_string =
                                            field.ident.as_ref().unwrap().to_string();

                                        match ident_string.as_str() {
                                            "domain" | "port" => (),
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
                },
                _ => {
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
                                        let ident_string =
                                            field.ident.as_ref().unwrap().to_string();

                                        match ident_string.as_str() {
                                            "domain" | "is_local" | "port" => (),
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
                                        let ident_string =
                                            field.ident.as_ref().unwrap().to_string();

                                        match ident_string.as_str() {
                                            "domain" | "is_local" | "port" => (),
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
                                        let ident_string =
                                            field.ident.as_ref().unwrap().to_string();

                                        match ident_string.as_str() {
                                            "domain" | "is_local" => (),
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
                                        let ident_string =
                                            field.ident.as_ref().unwrap().to_string();

                                        match ident_string.as_str() {
                                            "domain" | "port" => (),
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
                                        let ident_string =
                                            field.ident.as_ref().unwrap().to_string();

                                        match ident_string.as_str() {
                                            "domain" | "port" => (),
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
                },
            }

            let mut token_stream = proc_macro2::TokenStream::new();

            let name = ast.ident;

            let error_path: Path = syn::parse2(quote! { validators_prelude::DomainError }).unwrap();

            #[cfg(feature = "test")]
            {
                let v_ipv4 = type_attribute.ipv4;
                let v_local = type_attribute.local;
                let v_port = type_attribute.port;
                let v_at_least_two_labels = type_attribute.at_least_two_labels;

                token_stream.extend(quote! {
                    impl #name {
                        pub(crate) const V_IPV4: validators_prelude::TriAllow = #v_ipv4;
                        pub(crate) const V_LOCAL: validators_prelude::TriAllow = #v_local;
                        pub(crate) const V_PORT: validators_prelude::TriAllow = #v_port;
                        pub(crate) const V_AT_LEAST_TWO_LABELS: validators_prelude::TriAllow = #v_at_least_two_labels;
                    }
                });
            }

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

            let handle_none_port = if type_attribute.port.must() {
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

            let handle_domain_str_and_port_str = {
                if type_attribute.ipv4 == TriAllow::Allow
                    && type_attribute.local == TriAllow::Allow
                    && type_attribute.at_least_two_labels == TriAllow::Allow
                {
                    quote! {
                        match validators_prelude::idna::Config::default()
                            .use_std3_ascii_rules(true)
                            .verify_dns_length(true)
                            .check_hyphens(true)
                            .to_ascii(domain_str)
                        {
                            Ok(ascii_domain) => {
                                let port = #handle_port;

                                (ascii_domain, port, false, false)
                            }
                            Err(_) => return Err(#error_path::Invalid),
                        }
                    }
                } else {
                    let handle_ipv4 = if type_attribute.ipv4.disallow() {
                        quote! {
                            Ok(_) => {
                                return Err(#error_path::IPv4Disallow);
                            }
                        }
                    } else if type_attribute.at_least_two_labels.disallow() {
                        quote! {
                            Ok(_) => {
                                return Err(#error_path::AtLeastTwoLabelsDisallow);
                            }
                        }
                    } else {
                        let handle_local_ipv4 = if type_attribute.local == TriAllow::Allow
                            && type_attribute.at_least_two_labels == TriAllow::Allow
                        {
                            quote! {
                                false
                            }
                        } else {
                            quote! {
                                validators_prelude::is_local_ipv4(ip)
                            }
                        };

                        quote! {
                            Ok(ip) => {
                                let port = #handle_port;

                                let is_local = #handle_local_ipv4;

                                #check_local

                                (s.into_owned(), port, true, is_local)
                            }
                        }
                    };

                    let handle_none_ipv4 = if type_attribute.ipv4.must() {
                        quote! {
                            return Err(#error_path::IPv4Must);
                        }
                    } else {
                        let handle_local_domain = if type_attribute.local == TriAllow::Allow
                            && type_attribute.at_least_two_labels == TriAllow::Allow
                        {
                            quote! {
                                false
                            }
                        } else {
                            quote! {
                                validators_prelude::is_local_domain(&ascii_domain)
                            }
                        };

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

                                    (ascii_domain, port, false, is_local)
                                }
                                Err(_) => return Err(#error_path::Invalid),
                            }
                        }
                    };

                    quote! {
                        match validators_prelude::parse_ipv4_allow_an_ended_dot(domain_str) {
                            #handle_ipv4
                            Err(_) => {
                                #handle_none_ipv4
                            }
                        }
                    }
                }
            };

            let conflict_meta = if meta_is_conflict {
                quote! {
                    #[allow(unreachable_code)]
                }
            } else {
                quote! {}
            };

            token_stream.extend(quote! {
                impl #name {
                    #conflict_meta
                    fn v_parse_str(s: validators_prelude::Cow<str>) -> Result<(validators_prelude::String, Option<u16>, bool, bool), #error_path> {
                        let bytes = s.as_bytes();

                        if bytes.is_empty() {
                            return Err(#error_path::Invalid);
                        }

                        let (domain_str, port_str) = match bytes.iter().copied().rposition(|e| e == b':') {
                            #handle_domain_and_port
                            None => {
                                #handle_none_port
                            }
                        };

                        Ok(#handle_domain_str_and_port_str)
                    }
                }
            });

            let create_instance = {
                match type_attribute.ipv4 {
                    TriAllow::Allow => {
                        if type_attribute.local == TriAllow::Allow
                            && type_attribute.at_least_two_labels != TriAllow::Allow
                        {
                            match type_attribute.port {
                                TriAllow::Allow => {
                                    quote! {
                                        Self {
                                            domain,
                                            is_ipv4: _is_ipv4,
                                            is_local: _is_local,
                                            port: _port,
                                        }
                                    }
                                },
                                TriAllow::Must => {
                                    quote! {
                                        Self {
                                            domain,
                                            is_ipv4: _is_ipv4,
                                            is_local: _is_local,
                                            port: _port.unwrap(),
                                        }
                                    }
                                },
                                TriAllow::Disallow => {
                                    quote! {
                                        Self {
                                            domain,
                                            is_ipv4: _is_ipv4,
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
                                            domain,
                                            port: _port,
                                        }
                                    }
                                },
                                TriAllow::Must => {
                                    quote! {
                                        Self {
                                            domain,
                                            port: _port.unwrap(),
                                        }
                                    }
                                },
                                TriAllow::Disallow => {
                                    quote! {
                                        Self(domain)
                                    }
                                },
                            }
                        }
                    },
                    _ => {
                        if type_attribute.local == TriAllow::Allow
                            && type_attribute.at_least_two_labels != TriAllow::Allow
                        {
                            match type_attribute.port {
                                TriAllow::Allow => {
                                    quote! {
                                        Self {
                                            domain,
                                            is_local: _is_local,
                                            port: _port,
                                        }
                                    }
                                },
                                TriAllow::Must => {
                                    quote! {
                                        Self {
                                            domain,
                                            is_local: _is_local,
                                            port: _port.unwrap(),
                                        }
                                    }
                                },
                                TriAllow::Disallow => {
                                    quote! {
                                        Self {
                                            domain,
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
                                            domain,
                                            port: _port,
                                        }
                                    }
                                },
                                TriAllow::Must => {
                                    quote! {
                                        Self {
                                            domain,
                                            port: _port.unwrap(),
                                        }
                                    }
                                },
                                TriAllow::Disallow => {
                                    quote! {
                                        Self(domain)
                                    }
                                },
                            }
                        }
                    },
                }
            };

            token_stream.extend(quote! {
                impl ValidateString for #name {
                    type Error = #error_path;

                    #[inline]
                    fn parse_string<S: Into<validators_prelude::String>>(s: S) -> Result<Self, Self::Error> {
                        let (domain, _port, _is_ipv4, _is_local) = Self::v_parse_str(validators_prelude::Cow::Owned(s.into()))?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> Result<Self, Self::Error> {
                        let (domain, _port, _is_ipv4, _is_local) = Self::v_parse_str(validators_prelude::Cow::Borrowed(s.as_ref()))?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error> {
                        Self::v_parse_str(validators_prelude::Cow::Borrowed(s.as_ref()))?;

                        Ok(())
                    }
                }
            });

            token_stream.extend(
                if type_attribute.port == TriAllow::Disallow
                    && (type_attribute.local != TriAllow::Allow
                        || type_attribute.at_least_two_labels == TriAllow::Allow)
                {
                    quote! {
                        impl QualifyDomain for #name {
                            #[inline]
                            fn is_fully_qualified(&self) -> bool {
                                self.0.ends_with('.')
                            }

                            #[inline]
                            fn get_domain_non_fully_qualified(&self) -> &str {
                                if QualifyDomain::is_fully_qualified(self) {
                                    &self.0[..(self.0.len() - 1)]
                                } else {
                                    self.0.as_str()
                                }
                            }
                        }
                    }
                } else {
                    quote! {
                        impl QualifyDomain for #name {
                            #[inline]
                            fn is_fully_qualified(&self) -> bool {
                                self.domain.ends_with('.')
                            }

                            #[inline]
                            fn get_domain_non_fully_qualified(&self) -> &str {
                                if QualifyDomain::is_fully_qualified(self) {
                                    &self.domain[..(self.domain.len() - 1)]
                                } else {
                                    self.domain.as_str()
                                }
                            }
                        }
                    }
                },
            );

            token_stream.extend(match type_attribute.port {
                TriAllow::Allow => {
                    quote! {
                        impl ToUriAuthorityString for #name {
                            #[inline]
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                                match self.port {
                                    Some(port) => {
                                        let non_fully_qualified_domain = QualifyDomain::get_domain_non_fully_qualified(self);

                                        validators_prelude::Cow::Owned(validators_prelude::format!("{}:{}", non_fully_qualified_domain, port))
                                    },
                                    None => validators_prelude::Cow::Borrowed(self.get_domain_non_fully_qualified()),
                                }
                            }
                        }
                    }
                }
                TriAllow::Must => {
                    quote! {
                        impl ToUriAuthorityString for #name {
                            #[inline]
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                                let non_fully_qualified_domain = QualifyDomain::get_domain_non_fully_qualified(self);
                                let port = self.port;

                                validators_prelude::Cow::Owned(validators_prelude::format!("{}:{}", non_fully_qualified_domain, port))
                            }
                        }
                    }
                }
                TriAllow::Disallow => {
                    quote! {
                        impl ToUriAuthorityString for #name {
                            #[inline]
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                                validators_prelude::Cow::Borrowed(self.get_domain_non_fully_qualified())
                            }
                        }
                    }
                }
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
                        let mut s = String::from("a correct ");

                        match type_attribute.ipv4 {
                            TriAllow::Allow => {
                                match type_attribute.at_least_two_labels {
                                    TriAllow::Allow => (),
                                    TriAllow::Must => {
                                        s.push_str("at-least-two-labels ");
                                    },
                                    TriAllow::Disallow => {
                                        s.push_str("one-label ");
                                    },
                                }

                                s.push_str("domain name");
                            },
                            TriAllow::Must => {
                                s.push_str("IPv4 string");
                            },
                            TriAllow::Disallow => {
                                if type_attribute.at_least_two_labels.must() {
                                    s.push_str("at-least-two-labels and ");
                                }

                                s.push_str("non-IPv4 domain name");
                            },
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

                                    #[inline]
                                    fn visit_string<E>(self, v: validators_prelude::String) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::serde::de::Error, {
                                        <#name as ValidateString>::parse_string(v).map_err(validators_prelude::serde::de::Error::custom)
                                    }
                                }

                                deserializer.deserialize_any(MyVisitor)
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
