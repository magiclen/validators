use alloc::boxed::Box;
use alloc::string::{String, ToString};

use crate::proc_macro::TokenStream;
use crate::quote::ToTokens;
use crate::syn::{Data, DeriveInput, Fields, Meta, NestedMeta, Path};

use crate::{panic, SynOption, TypeEnum, Validator, ValidatorOption};

#[derive(Debug)]
pub struct Struct(TypeEnum);

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowLocal {
    domain: TypeEnum,
    is_local: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowPort {
    domain: TypeEnum,
    port: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowPortAllowLocal {
    domain: TypeEnum,
    is_local: TypeEnum,
    port: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowIPv4 {
    domain: TypeEnum,
    is_ipv4: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowIPv4AllowLocal {
    domain: TypeEnum,
    is_ipv4: TypeEnum,
    is_local: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowIPv4AllowPort {
    domain: TypeEnum,
    is_ipv4: TypeEnum,
    port: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowIPv4AllowPortAllowLocal {
    domain: TypeEnum,
    is_ipv4: TypeEnum,
    is_local: TypeEnum,
    port: TypeEnum,
}

const ITEM: Struct = Struct(TypeEnum::String);
const ITEM_ALLOW_LOCAL: StructAllowLocal = StructAllowLocal {
    domain: TypeEnum::String,
    is_local: TypeEnum::Boolean,
};
const ITEM_ALLOW_PORT: StructAllowPort = StructAllowPort {
    domain: TypeEnum::String,
    port: TypeEnum::OptionU16,
};
const ITEM_WITH_PORT: StructAllowPort = StructAllowPort {
    domain: TypeEnum::String,
    port: TypeEnum::U16,
};
const ITEM_ALLOW_LOCAL_ALLOW_PORT: StructAllowPortAllowLocal = StructAllowPortAllowLocal {
    domain: TypeEnum::String,
    is_local: TypeEnum::Boolean,
    port: TypeEnum::OptionU16,
};
const ITEM_ALLOW_LOCAL_WITH_PORT: StructAllowPortAllowLocal = StructAllowPortAllowLocal {
    domain: TypeEnum::String,
    is_local: TypeEnum::Boolean,
    port: TypeEnum::U16,
};
const ITEM_ALLOW_IPV4: StructAllowIPv4 = StructAllowIPv4 {
    domain: TypeEnum::String,
    is_ipv4: TypeEnum::Boolean,
};
const ITEM_ALLOW_IPV4_ALLOW_LOCAL: StructAllowIPv4AllowLocal = StructAllowIPv4AllowLocal {
    domain: TypeEnum::String,
    is_ipv4: TypeEnum::Boolean,
    is_local: TypeEnum::Boolean,
};
const ITEM_ALLOW_IPV4_ALLOW_PORT: StructAllowIPv4AllowPort = StructAllowIPv4AllowPort {
    domain: TypeEnum::String,
    is_ipv4: TypeEnum::Boolean,
    port: TypeEnum::OptionU16,
};
const ITEM_ALLOW_IPV4_WITH_PORT: StructAllowIPv4AllowPort = StructAllowIPv4AllowPort {
    domain: TypeEnum::String,
    is_ipv4: TypeEnum::Boolean,
    port: TypeEnum::U16,
};
const ITEM_ALLOW_IPV4_ALLOW_LOCAL_ALLOW_PORT: StructAllowIPv4AllowPortAllowLocal =
    StructAllowIPv4AllowPortAllowLocal {
        domain: TypeEnum::String,
        is_ipv4: TypeEnum::Boolean,
        is_local: TypeEnum::Boolean,
        port: TypeEnum::OptionU16,
    };
const ITEM_ALLOW_IPV4_ALLOW_LOCAL_WITH_PORT: StructAllowIPv4AllowPortAllowLocal =
    StructAllowIPv4AllowPortAllowLocal {
        domain: TypeEnum::String,
        is_ipv4: TypeEnum::Boolean,
        is_local: TypeEnum::Boolean,
        port: TypeEnum::U16,
    };

const VALIDATOR: Validator = Validator::domain;

pub fn domain_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            let mut ipv4 = ValidatorOption::new();
            let mut local = ValidatorOption::new();
            let mut port = ValidatorOption::new();
            let mut at_least_two_labels = ValidatorOption::new();
            let mut conflict = ValidatorOption::NotAllow;

            let correct_usage_for_attribute = [stringify!(#[validator(domain)])];

            let correct_usage_for_ipv4 = [
                stringify!(#[validator(domain(ipv4(Must)))]),
                stringify!(#[validator(domain(ipv4(Allow)))]),
                stringify!(#[validator(domain(ipv4(NotAllow)))]),
            ];

            let correct_usage_for_local = [
                stringify!(#[validator(domain(local(Must)))]),
                stringify!(#[validator(domain(local(Allow)))]),
                stringify!(#[validator(domain(local(NotAllow)))]),
            ];

            let correct_usage_for_port = [
                stringify!(#[validator(domain(port(Must)))]),
                stringify!(#[validator(domain(port(Allow)))]),
                stringify!(#[validator(domain(port(NotAllow)))]),
            ];

            let correct_usage_for_at_least_two_labels = [
                stringify!(#[validator(domain(at_least_two_labels(Must)))]),
                stringify!(#[validator(domain(at_least_two_labels(Allow)))]),
                stringify!(#[validator(domain(at_least_two_labels(NotAllow)))]),
            ];

            let correct_usage_for_conflict = [
                stringify!(#[validator(domain(conflict(Allow)))]),
                stringify!(#[validator(domain(conflict(NotAllow)))]),
            ];

            match meta {
                Meta::Path(_) => (),
                Meta::List(list) => {
                    let mut ipv4_is_set = false;
                    let mut local_is_set = false;
                    let mut port_is_set = false;
                    let mut at_least_two_labels_is_set = false;
                    let mut conflict_is_set = false;

                    for p in list.nested.iter() {
                        match p {
                            NestedMeta::Meta(meta) => {
                                let meta_name = meta.path().into_token_stream().to_string();

                                match meta_name.as_str() {
                                    "ipv4" => {
                                        if let Some(validator_option) = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut ipv4_is_set,
                                            &correct_usage_for_ipv4,
                                        ) {
                                            ipv4 = validator_option;
                                        }
                                    }
                                    "local" => {
                                        if let Some(validator_option) = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut local_is_set,
                                            &correct_usage_for_local,
                                        ) {
                                            local = validator_option;
                                        }
                                    }
                                    "port" => {
                                        if let Some(validator_option) = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut port_is_set,
                                            &correct_usage_for_port,
                                        ) {
                                            port = validator_option;
                                        }
                                    }
                                    "at_least_two_labels" => {
                                        if let Some(validator_option) = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut at_least_two_labels_is_set,
                                            &correct_usage_for_at_least_two_labels,
                                        ) {
                                            at_least_two_labels = validator_option;
                                        }
                                    }
                                    "conflict" => {
                                        if let Some(validator_option) = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut conflict_is_set,
                                            &correct_usage_for_conflict,
                                        ) {
                                            if validator_option == ValidatorOption::Must {
                                                panic::parameter_incorrect_format(
                                                    meta_name.as_str(),
                                                    &correct_usage_for_conflict,
                                                );
                                            }

                                            conflict = validator_option;
                                        }
                                    }
                                    _ => panic::unknown_parameter("domain", meta_name.as_str()),
                                }
                            }
                            NestedMeta::Lit(_) => {
                                panic::attribute_incorrect_format(
                                    "domain",
                                    &correct_usage_for_attribute,
                                )
                            }
                        }
                    }
                }
                Meta::NameValue(_) => {
                    panic::attribute_incorrect_format("domain", &correct_usage_for_attribute)
                }
            }

            let mut meta_is_conflict = false;

            if ipv4.must() && at_least_two_labels.not_allow() {
                if conflict.not_allow() {
                    panic!(
                        "`ipv4(Must)` and `at_least_two_labels(NotAllow)` cannot be used together."
                    );
                }

                meta_is_conflict = true;
            }

            match ipv4 {
                ValidatorOption::Allow => {
                    match local {
                        ValidatorOption::Allow => {
                            match port {
                                ValidatorOption::Allow => {
                                    if let Fields::Named(_) = &data.fields {
                                        if data.fields.len() != 4 {
                                            panic::validator_only_support_for_item(
                                                VALIDATOR,
                                                Box::new(ITEM_ALLOW_IPV4_ALLOW_LOCAL_ALLOW_PORT),
                                            );
                                        }

                                        for field in data.fields.iter() {
                                            let ident = field.ident.as_ref().unwrap();

                                            if ident != "domain"
                                                && ident != "is_ipv4"
                                                && ident != "is_local"
                                                && ident != "port"
                                            {
                                                panic::validator_only_support_for_item(
                                                    VALIDATOR,
                                                    Box::new(
                                                        ITEM_ALLOW_IPV4_ALLOW_LOCAL_ALLOW_PORT,
                                                    ),
                                                );
                                            }
                                        }
                                    } else {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_IPV4_ALLOW_LOCAL_ALLOW_PORT),
                                        );
                                    }
                                }
                                ValidatorOption::Must => {
                                    if let Fields::Named(_) = &data.fields {
                                        if data.fields.len() != 4 {
                                            panic::validator_only_support_for_item(
                                                VALIDATOR,
                                                Box::new(ITEM_ALLOW_IPV4_ALLOW_LOCAL_WITH_PORT),
                                            );
                                        }

                                        for field in data.fields.iter() {
                                            let ident = field.ident.as_ref().unwrap();

                                            if ident != "domain"
                                                && ident != "is_ipv4"
                                                && ident != "is_local"
                                                && ident != "port"
                                            {
                                                panic::validator_only_support_for_item(
                                                    VALIDATOR,
                                                    Box::new(ITEM_ALLOW_IPV4_ALLOW_LOCAL_WITH_PORT),
                                                );
                                            }
                                        }
                                    } else {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_IPV4_ALLOW_LOCAL_WITH_PORT),
                                        );
                                    }
                                }
                                ValidatorOption::NotAllow => {
                                    if let Fields::Named(_) = &data.fields {
                                        if data.fields.len() != 3 {
                                            panic::validator_only_support_for_item(
                                                VALIDATOR,
                                                Box::new(ITEM_ALLOW_IPV4_ALLOW_LOCAL),
                                            );
                                        }

                                        for field in data.fields.iter() {
                                            let ident = field.ident.as_ref().unwrap();

                                            if ident != "domain"
                                                && ident != "is_ipv4"
                                                && ident != "is_local"
                                            {
                                                panic::validator_only_support_for_item(
                                                    VALIDATOR,
                                                    Box::new(ITEM_ALLOW_IPV4_ALLOW_LOCAL),
                                                );
                                            }
                                        }
                                    } else {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_IPV4_ALLOW_LOCAL),
                                        );
                                    }
                                }
                            }
                        }
                        _ => {
                            match port {
                                ValidatorOption::Allow => {
                                    if let Fields::Named(_) = &data.fields {
                                        if data.fields.len() != 3 {
                                            panic::validator_only_support_for_item(
                                                VALIDATOR,
                                                Box::new(ITEM_ALLOW_IPV4_ALLOW_PORT),
                                            );
                                        }

                                        for field in data.fields.iter() {
                                            let ident = field.ident.as_ref().unwrap();

                                            if ident != "domain"
                                                && ident != "is_ipv4"
                                                && ident != "port"
                                            {
                                                panic::validator_only_support_for_item(
                                                    VALIDATOR,
                                                    Box::new(ITEM_ALLOW_IPV4_ALLOW_PORT),
                                                );
                                            }
                                        }
                                    } else {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_IPV4_ALLOW_PORT),
                                        );
                                    }
                                }
                                ValidatorOption::Must => {
                                    if let Fields::Named(_) = &data.fields {
                                        if data.fields.len() != 3 {
                                            panic::validator_only_support_for_item(
                                                VALIDATOR,
                                                Box::new(ITEM_ALLOW_IPV4_WITH_PORT),
                                            );
                                        }

                                        for field in data.fields.iter() {
                                            let ident = field.ident.as_ref().unwrap();

                                            if ident != "domain"
                                                && ident != "is_ipv4"
                                                && ident != "port"
                                            {
                                                panic::validator_only_support_for_item(
                                                    VALIDATOR,
                                                    Box::new(ITEM_ALLOW_IPV4_WITH_PORT),
                                                );
                                            }
                                        }
                                    } else {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_IPV4_WITH_PORT),
                                        );
                                    }
                                }
                                ValidatorOption::NotAllow => {
                                    if let Fields::Named(_) = &data.fields {
                                        if data.fields.len() != 2 {
                                            panic::validator_only_support_for_item(
                                                VALIDATOR,
                                                Box::new(ITEM_ALLOW_IPV4),
                                            );
                                        }

                                        for field in data.fields.iter() {
                                            let ident = field.ident.as_ref().unwrap();

                                            if ident != "domain" && ident != "is_ipv4" {
                                                panic::validator_only_support_for_item(
                                                    VALIDATOR,
                                                    Box::new(ITEM_ALLOW_IPV4),
                                                );
                                            }
                                        }
                                    } else {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_IPV4),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    match local {
                        ValidatorOption::Allow => {
                            match port {
                                ValidatorOption::Allow => {
                                    if let Fields::Named(_) = &data.fields {
                                        if data.fields.len() != 3 {
                                            panic::validator_only_support_for_item(
                                                VALIDATOR,
                                                Box::new(ITEM_ALLOW_LOCAL_ALLOW_PORT),
                                            );
                                        }

                                        for field in data.fields.iter() {
                                            let ident = field.ident.as_ref().unwrap();

                                            if ident != "domain"
                                                && ident != "is_local"
                                                && ident != "port"
                                            {
                                                panic::validator_only_support_for_item(
                                                    VALIDATOR,
                                                    Box::new(ITEM_ALLOW_LOCAL_ALLOW_PORT),
                                                );
                                            }
                                        }
                                    } else {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_LOCAL_ALLOW_PORT),
                                        );
                                    }
                                }
                                ValidatorOption::Must => {
                                    if let Fields::Named(_) = &data.fields {
                                        if data.fields.len() != 3 {
                                            panic::validator_only_support_for_item(
                                                VALIDATOR,
                                                Box::new(ITEM_ALLOW_LOCAL_WITH_PORT),
                                            );
                                        }

                                        for field in data.fields.iter() {
                                            let ident = field.ident.as_ref().unwrap();

                                            if ident != "domain"
                                                && ident != "is_local"
                                                && ident != "port"
                                            {
                                                panic::validator_only_support_for_item(
                                                    VALIDATOR,
                                                    Box::new(ITEM_ALLOW_LOCAL_WITH_PORT),
                                                );
                                            }
                                        }
                                    } else {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_LOCAL_WITH_PORT),
                                        );
                                    }
                                }
                                ValidatorOption::NotAllow => {
                                    if let Fields::Named(_) = &data.fields {
                                        if data.fields.len() != 2 {
                                            panic::validator_only_support_for_item(
                                                VALIDATOR,
                                                Box::new(ITEM_ALLOW_LOCAL),
                                            );
                                        }

                                        for field in data.fields.iter() {
                                            let ident = field.ident.as_ref().unwrap();

                                            if ident != "domain" && ident != "is_local" {
                                                panic::validator_only_support_for_item(
                                                    VALIDATOR,
                                                    Box::new(ITEM_ALLOW_LOCAL),
                                                );
                                            }
                                        }
                                    } else {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_LOCAL),
                                        );
                                    }
                                }
                            }
                        }
                        _ => {
                            match port {
                                ValidatorOption::Allow => {
                                    if let Fields::Named(_) = &data.fields {
                                        if data.fields.len() != 2 {
                                            panic::validator_only_support_for_item(
                                                VALIDATOR,
                                                Box::new(ITEM_ALLOW_PORT),
                                            );
                                        }

                                        for field in data.fields.iter() {
                                            let ident = field.ident.as_ref().unwrap();

                                            if ident != "domain" && ident != "port" {
                                                panic::validator_only_support_for_item(
                                                    VALIDATOR,
                                                    Box::new(ITEM_ALLOW_PORT),
                                                );
                                            }
                                        }
                                    } else {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_PORT),
                                        );
                                    }
                                }
                                ValidatorOption::Must => {
                                    if let Fields::Named(_) = &data.fields {
                                        if data.fields.len() != 2 {
                                            panic::validator_only_support_for_item(
                                                VALIDATOR,
                                                Box::new(ITEM_WITH_PORT),
                                            );
                                        }

                                        for field in data.fields.iter() {
                                            let ident = field.ident.as_ref().unwrap();

                                            if ident != "domain" && ident != "port" {
                                                panic::validator_only_support_for_item(
                                                    VALIDATOR,
                                                    Box::new(ITEM_WITH_PORT),
                                                );
                                            }
                                        }
                                    } else {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_WITH_PORT),
                                        );
                                    }
                                }
                                ValidatorOption::NotAllow => {
                                    if let Fields::Unnamed(_) = &data.fields {
                                        if data.fields.len() != 1 {
                                            panic::validator_only_support_for_item(
                                                VALIDATOR,
                                                Box::new(ITEM),
                                            );
                                        }
                                    } else {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let name = ast.ident;

            // TODO impl

            let error_path: Path =
                syn::parse2(quote! { validators_prelude::domain::DomainError }).unwrap();

            let ipv4_path = ipv4.to_path();
            let local_path = local.to_path();
            let port_path = port.to_path();
            let at_least_two_labels_path = at_least_two_labels.to_path();

            let parameters_impl = quote! {
                impl #name {
                    pub(crate) const V_IPV4: validators_prelude::ValidatorOption = #ipv4_path;
                    pub(crate) const V_LOCAL: validators_prelude::ValidatorOption = #local_path;
                    pub(crate) const V_PORT: validators_prelude::ValidatorOption = #port_path;
                    pub(crate) const V_AT_LEAST_TWO_LABELS: validators_prelude::ValidatorOption = #at_least_two_labels_path;
                }
            };

            let handle_domain_and_port = if port.not_allow() {
                quote! {
                    Some(_) => {
                        return Err(#error_path::PortNotAllow);
                    }
                }
            } else {
                quote! {
                    Some(colon_index) => {
                        (
                            unsafe { validators_prelude::from_utf8_unchecked(&bytes[..colon_index]) },
                            Some(unsafe { validators_prelude::from_utf8_unchecked(&bytes[(colon_index + 1)..]) }),
                        )
                    }
                }
            };

            let handle_none_port = if port.must() {
                quote! {
                    return Err(#error_path::PortMust);
                }
            } else {
                quote! {
                    (unsafe { validators_prelude::from_utf8_unchecked(bytes) }, None::<&str>)
                }
            };

            let handle_port = if port.not_allow() {
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
                match local {
                    ValidatorOption::Allow => quote! {},
                    ValidatorOption::Must => {
                        quote! {
                            if !is_local {
                                return Err(#error_path::LocalMust);
                            }
                        }
                    }
                    ValidatorOption::NotAllow => {
                        quote! {
                            if is_local {
                                return Err(#error_path::LocalNotAllow);
                            }
                        }
                    }
                }
            };

            let handle_ipv4 = if ipv4.not_allow() {
                quote! {
                    Ok(_) => {
                        return Err(#error_path::IPv4NotAllow);
                    }
                }
            } else if at_least_two_labels.not_allow() {
                quote! {
                    Ok(_) => {
                        return Err(#error_path::AtLeastTwoLabelsNotAllow);
                    }
                }
            } else {
                quote! {
                    Ok(ip) => {
                        let port = #handle_port;

                        let is_local = validators_prelude::is_local_ipv4(ip);

                        #check_local

                        (s.into_owned(), port, true, is_local)
                    }
                }
            };

            let handle_none_ipv4 = if ipv4.must() {
                quote! {
                    return Err(#error_path::IPv4Must);
                }
            } else {
                let check_at_least_two_labels = {
                    match at_least_two_labels {
                        ValidatorOption::Allow => quote! {},
                        ValidatorOption::Must => {
                            quote! {
                                if !is_local && !validators_prelude::is_at_least_two_labels_domain(&ascii_domain) {
                                    return Err(#error_path::AtLeastTwoLabelsMust);
                                }
                            }
                        }
                        ValidatorOption::NotAllow => {
                            quote! {
                                if !is_local && validators_prelude::is_at_least_two_labels_domain(&ascii_domain) {
                                    return Err(#error_path::AtLeastTwoLabelsNotAllow);
                                }
                            }
                        }
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

                            let is_local = validators_prelude::is_local_domain(&ascii_domain);

                            #check_at_least_two_labels

                            #check_local

                            (ascii_domain, port, false, is_local)
                        }
                        Err(_) => return Err(#error_path::Invalid),
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

            let v_parse_str = quote! {
                #conflict_meta
                pub(crate) fn v_parse_str(s: validators_prelude::Cow<str>) -> Result<(validators_prelude::String, Option<u16>, bool, bool), #error_path> {
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

                    Ok(match validators_prelude::parse_ipv4_allow_an_ended_dot(domain_str) {
                        #handle_ipv4
                        Err(_) => {
                            #handle_none_ipv4
                        }
                    })
                }
            };

            let parse_impl = quote! {
                impl #name {
                    #v_parse_str
                }
            };

            let fully_qualified = if ipv4 != ValidatorOption::Allow
                && local != ValidatorOption::Allow
                && port.not_allow()
            {
                quote! {
                    #[inline]
                    pub fn is_fully_qualified(&self) -> bool {
                        self.0.ends_with('.')
                    }

                    #[inline]
                    pub fn get_domain_non_fully_qualified(&self) -> &str {
                        if self.is_fully_qualified() {
                            &self.0[..(self.0.len() - 1)]
                        } else {
                            self.0.as_str()
                        }
                    }
                }
            } else {
                quote! {
                    #[inline]
                    pub fn is_fully_qualified(&self) -> bool {
                        self.domain.ends_with('.')
                    }

                    #[inline]
                    pub fn get_domain_non_fully_qualified(&self) -> &str {
                        if self.is_fully_qualified() {
                            &self.domain[..(self.domain.len() - 1)]
                        } else {
                            self.domain.as_str()
                        }
                    }
                }
            };

            let to_uri_authority_string = {
                match port {
                    ValidatorOption::Allow => {
                        quote! {
                            #[inline]
                            pub fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                                match self.port {
                                    Some(port) => validators_prelude::Cow::from(validators_prelude::format!("{}:{}", self.get_domain_non_fully_qualified(), port)),
                                    None => validators_prelude::Cow::from(self.get_domain_non_fully_qualified()),
                                }
                            }
                        }
                    }
                    ValidatorOption::Must => {
                        quote! {
                            #[inline]
                            pub fn to_uri_authority_string(&self) -> String {
                                validators_prelude::format!("{}:{}", self.get_domain_non_fully_qualified(), self.port)
                            }
                        }
                    }
                    ValidatorOption::NotAllow => {
                        quote! {
                            #[inline]
                            pub fn to_uri_authority_string(&self) -> &str {
                                self.get_domain_non_fully_qualified()
                            }
                        }
                    }
                }
            };

            let other_functions = quote! {
                impl #name {
                    #fully_qualified

                    #to_uri_authority_string
                }
            };

            let create_instance = {
                match ipv4 {
                    ValidatorOption::Allow => {
                        match local {
                            ValidatorOption::Allow => {
                                match port {
                                    ValidatorOption::Allow => {
                                        quote! {
                                            #name {
                                                domain,
                                                is_ipv4: _is_ipv4,
                                                is_local: _is_local,
                                                port: _port,
                                            }
                                        }
                                    }
                                    ValidatorOption::Must => {
                                        quote! {
                                            #name {
                                                domain,
                                                is_ipv4: _is_ipv4,
                                                is_local: _is_local,
                                                port: _port.unwrap(),
                                            }
                                        }
                                    }
                                    ValidatorOption::NotAllow => {
                                        quote! {
                                            #name {
                                                domain,
                                                is_ipv4: _is_ipv4,
                                                is_local: _is_local,
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                match port {
                                    ValidatorOption::Allow => {
                                        quote! {
                                            #name {
                                                domain,
                                                is_ipv4: _is_ipv4,
                                                port: _port,
                                            }
                                        }
                                    }
                                    ValidatorOption::Must => {
                                        quote! {
                                            #name {
                                                domain,
                                                is_ipv4: _is_ipv4,
                                                port: _port.unwrap(),
                                            }
                                        }
                                    }
                                    ValidatorOption::NotAllow => {
                                        quote! {
                                            #name {
                                                domain,
                                                is_ipv4: _is_ipv4,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        match local {
                            ValidatorOption::Allow => {
                                match port {
                                    ValidatorOption::Allow => {
                                        quote! {
                                            #name {
                                                domain,
                                                is_local: _is_local,
                                                port: _port,
                                            }
                                        }
                                    }
                                    ValidatorOption::Must => {
                                        quote! {
                                            #name {
                                                domain,
                                                is_local: _is_local,
                                                port: _port.unwrap(),
                                            }
                                        }
                                    }
                                    ValidatorOption::NotAllow => {
                                        quote! {
                                            #name {
                                                domain,
                                                is_local: _is_local,
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                match port {
                                    ValidatorOption::Allow => {
                                        quote! {
                                            #name {
                                                domain,
                                                port: _port,
                                            }
                                        }
                                    }
                                    ValidatorOption::Must => {
                                        quote! {
                                            #name {
                                                domain,
                                                port: _port.unwrap(),
                                            }
                                        }
                                    }
                                    ValidatorOption::NotAllow => {
                                        quote! {
                                            #name(domain)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };

            let validate_string_impl = quote! {
                impl ValidateString for #name {
                    type Error = #error_path;
                    type Output = Self;

                    #[inline]
                    fn parse_string<S: Into<validators_prelude::String>>(s: S) -> Result<Self::Output, Self::Error> {
                        let (domain, _port, _is_ipv4, _is_local) = Self::v_parse_str(validators_prelude::Cow::from(s.into()))?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> Result<Self::Output, Self::Error> {
                        let (domain, _port, _is_ipv4, _is_local) = Self::v_parse_str(validators_prelude::Cow::from(s.as_ref()))?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error> {
                        Self::v_parse_str(validators_prelude::Cow::from(s.as_ref()))?;

                        Ok(())
                    }
                }
            };

            let serde_impl = if cfg!(feature = "serde") {
                let expect = {
                    let mut s = String::from("a correct ");

                    match ipv4 {
                        ValidatorOption::Allow => {
                            match at_least_two_labels {
                                ValidatorOption::Allow => (),
                                ValidatorOption::Must => {
                                    s.push_str("at-least-two-labels ");
                                }
                                ValidatorOption::NotAllow => {
                                    s.push_str("one-label ");
                                }
                            }

                            s.push_str("domain name");
                        }
                        ValidatorOption::Must => {
                            s.push_str("IPv4 string");
                        }
                        ValidatorOption::NotAllow => {
                            if at_least_two_labels.must() {
                                s.push_str("at-least-two-labels and ");
                            }

                            s.push_str("non-IPv4 domain name");
                        }
                    }

                    match local {
                        ValidatorOption::Allow => {
                            match port {
                                ValidatorOption::Allow => {
                                    s.push_str(" with an optional port");
                                }
                                ValidatorOption::Must => {
                                    s.push_str(" with a port");
                                }
                                ValidatorOption::NotAllow => {
                                    s.push_str(" without ports");
                                }
                            }
                        }
                        ValidatorOption::Must => {
                            s.push_str(" which must be local");

                            match port {
                                ValidatorOption::Allow => (),
                                ValidatorOption::Must => {
                                    s.push_str(" and with a port");
                                }
                                ValidatorOption::NotAllow => {
                                    s.push_str(" and without ports");
                                }
                            }
                        }
                        ValidatorOption::NotAllow => {
                            s.push_str(" which must not be local");

                            match port {
                                ValidatorOption::Allow => (),
                                ValidatorOption::Must => {
                                    s.push_str(" and must be with a port");
                                }
                                ValidatorOption::NotAllow => {
                                    s.push_str(" and must be without ports");
                                }
                            }
                        }
                    }

                    s
                };

                quote! {
                    impl validators_prelude::Serialize for #name {
                        #[inline]
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                            where
                                S: validators_prelude::Serializer, {
                            serializer.serialize_str(&self.to_uri_authority_string())
                        }
                    }

                    impl<'de> validators_prelude::Deserialize<'de> for #name {
                        #[inline]
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: validators_prelude::Deserializer<'de>, {
                            struct ValidatingVisitor;

                            impl<'de> validators_prelude::Visitor<'de> for ValidatingVisitor {
                                type Value = #name;

                                #[inline]
                                fn expecting(&self, f: &mut validators_prelude::Formatter) -> Result<(), validators_prelude::fmt::Error> {
                                    f.write_str(#expect)
                                }

                                #[inline]
                                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                                where
                                    E: validators_prelude::DeError, {
                                    <#name as ValidateString>::parse_str(v).map_err(validators_prelude::DeError::custom)
                                }

                                #[inline]
                                fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                                where
                                    E: validators_prelude::DeError, {
                                    <#name as ValidateString>::parse_string(v).map_err(validators_prelude::DeError::custom)
                                }
                            }

                            deserializer.deserialize_string(ValidatingVisitor)
                        }
                    }
                }
            } else {
                quote! {}
            };

            let rocket_impl = if cfg!(feature = "rocket") {
                quote! {
                    impl<'a> validators_prelude::FromFormValue<'a> for #name {
                        type Error = #error_path;

                        #[inline]
                        fn from_form_value(v: &'a validators_prelude::RawStr) -> Result<Self, Self::Error> {
                            <#name as ValidateString>::parse_str(v)
                        }
                    }

                    impl<'a> validators_prelude::FromParam<'a> for #name {
                        type Error = #error_path;

                        #[inline]
                        fn from_param(v: &'a validators_prelude::RawStr) -> Result<Self, Self::Error> {
                            <#name as ValidateString>::parse_str(v)
                        }
                    }
                }
            } else {
                quote! {}
            };

            let domain_impl = quote! {
                #parameters_impl

                #parse_impl

                #validate_string_impl

                #other_functions

                #serde_impl

                #rocket_impl
            };

            domain_impl.into()
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
