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
    host: TypeEnum,
    is_local: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowPort {
    host: TypeEnum,
    port: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowPortAllowLocal {
    host: TypeEnum,
    is_local: TypeEnum,
    port: TypeEnum,
}

const ITEM: Struct = Struct(TypeEnum::Host);
const ITEM_ALLOW_LOCAL: StructAllowLocal = StructAllowLocal {
    host: TypeEnum::Host,
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
    host: TypeEnum::Host,
    is_local: TypeEnum::Boolean,
    port: TypeEnum::OptionU16,
};
const ITEM_ALLOW_LOCAL_WITH_PORT: StructAllowPortAllowLocal = StructAllowPortAllowLocal {
    host: TypeEnum::Host,
    is_local: TypeEnum::Boolean,
    port: TypeEnum::U16,
};

const VALIDATOR: Validator = Validator::host;

pub fn host_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            let mut local = ValidatorOption::new();
            let mut port = ValidatorOption::new();
            let mut at_least_two_labels = ValidatorOption::new();

            let correct_usage_for_attribute = [stringify!(#[validator(host)])];

            let correct_usage_for_local = [
                stringify!(#[validator(host(local(Must)))]),
                stringify!(#[validator(host(local(Allow)))]),
                stringify!(#[validator(host(local(NotAllow)))]),
            ];

            let correct_usage_for_port = [
                stringify!(#[validator(host(port(Must)))]),
                stringify!(#[validator(host(port(Allow)))]),
                stringify!(#[validator(host(port(NotAllow)))]),
            ];

            let correct_usage_for_at_least_two_labels = [
                stringify!(#[validator(host(at_least_two_labels(Must)))]),
                stringify!(#[validator(host(at_least_two_labels(Allow)))]),
                stringify!(#[validator(host(at_least_two_labels(NotAllow)))]),
            ];

            match meta {
                Meta::Path(_) => (),
                Meta::List(list) => {
                    let mut local_is_set = false;
                    let mut port_is_set = false;
                    let mut at_least_two_labels_is_set = false;

                    for p in list.nested.iter() {
                        match p {
                            NestedMeta::Meta(meta) => {
                                let meta_name = meta.path().into_token_stream().to_string();

                                match meta_name.as_str() {
                                    "local" => {
                                        local = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut local_is_set,
                                            &correct_usage_for_local,
                                        );
                                    }
                                    "port" => {
                                        port = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut port_is_set,
                                            &correct_usage_for_port,
                                        );
                                    }
                                    "at_least_two_labels" => {
                                        at_least_two_labels = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut at_least_two_labels_is_set,
                                            &correct_usage_for_at_least_two_labels,
                                        );
                                    }
                                    _ => panic::unknown_parameter("host", meta_name.as_str()),
                                }
                            }
                            NestedMeta::Lit(_) => {
                                panic::attribute_incorrect_format(
                                    "host",
                                    &correct_usage_for_attribute,
                                )
                            }
                        }
                    }
                }
                Meta::NameValue(_) => {
                    panic::attribute_incorrect_format("host", &correct_usage_for_attribute)
                }
            }

            if local == ValidatorOption::Allow && at_least_two_labels != ValidatorOption::Allow {
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

                                if ident != "host" && ident != "is_local" && ident != "port" {
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

                                if ident != "host" && ident != "is_local" && ident != "port" {
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

                                if ident != "host" && ident != "is_local" {
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
            } else {
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

                                if ident != "host" && ident != "port" {
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

                                if ident != "host" && ident != "port" {
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
                                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                            }
                        } else {
                            panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                        }
                    }
                }
            }

            let name = ast.ident;

            // TODO impl

            let error_path: Path = syn::parse2(quote! { validators_prelude::HostError }).unwrap();

            let local_path = local.to_expr();
            let port_path = port.to_expr();
            let at_least_two_labels_path = at_least_two_labels.to_expr();

            let parameters_impl = quote! {
                impl #name {
                    pub(crate) const V_LOCAL: validators_prelude::ValidatorOption = #local_path;
                    pub(crate) const V_PORT: validators_prelude::ValidatorOption = #port_path;
                    pub(crate) const V_AT_LEAST_TWO_LABELS: validators_prelude::ValidatorOption = #at_least_two_labels_path;
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

            let handle_local_ipv6 = if at_least_two_labels == ValidatorOption::Allow
                && local == ValidatorOption::Allow
            {
                quote! {
                    false
                }
            } else {
                quote! {
                    validators_prelude::is_local_ipv6(ip)
                }
            };

            let handle_ipv6_without_port = if port.must() {
                quote! {
                    return Err(#error_path::PortMust);
                }
            } else {
                quote! {
                    let ip_str = unsafe { validators_prelude::from_utf8_unchecked(&bytes[1..last_index]) };

                    match validators_prelude::Ipv6Addr::from_str(ip_str) {
                        Ok(ip) => {
                            let is_local = #handle_local_ipv6;

                            #check_local

                            (validators_prelude::Host::IPv6(ip), None, is_local)
                        }
                        Err(_) => return Err(#error_path::Invalid),
                    }
                }
            };

            let handle_ipv6_with_port = if port.not_allow() {
                quote! {
                    return Err(#error_path::PortNotAllow);
                }
            } else {
                quote! {
                    match bytes.iter().copied().rposition(|e| e == b':') {
                        Some(colon_index) => {
                            if colon_index > 2 && bytes[colon_index - 1] == b']' {
                                let ip_str = unsafe { validators_prelude::from_utf8_unchecked(&bytes[1..(colon_index - 1)]) };

                                match validators_prelude::Ipv6Addr::from_str(ip_str) {
                                    Ok(ip) => {
                                        let port_str = unsafe { validators_prelude::from_utf8_unchecked(&bytes[(colon_index + 1)..]) };

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

            let handle_ipv6_bracket = if at_least_two_labels.not_allow() {
                quote! {
                    return Err(#error_path::AtLeastTwoLabelsNotAllow);
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

            let handle_ipv6_non_bracket = if at_least_two_labels.not_allow() {
                quote! {
                    Ok(_) => {
                        return Err(#error_path::AtLeastTwoLabelsNotAllow);
                    }
                }
            } else if port.must() {
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

            let handle_domain_none_port = if port.must() {
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

            let handle_local_ipv4 = if at_least_two_labels == ValidatorOption::Allow
                && local == ValidatorOption::Allow
            {
                quote! {
                    false
                }
            } else {
                quote! {
                    validators_prelude::is_local_ipv4(ip)
                }
            };

            let handle_ipv4 = if at_least_two_labels.not_allow() {
                quote! {
                    Ok(_) => {
                        return Err(#error_path::AtLeastTwoLabelsNotAllow);
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

                let handle_local_domain = if at_least_two_labels == ValidatorOption::Allow
                    && local == ValidatorOption::Allow
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

            let v_parse_str = quote! {
                fn v_parse_str(s: &str) -> Result<(validators_prelude::Host, Option<u16>, bool), #error_path> {
                    use core::str::FromStr;

                    let bytes = s.as_bytes();

                    if bytes.is_empty() {
                        return Err(#error_path::Invalid);
                    }

                    Ok(if bytes[0] == b'[' {
                        #handle_ipv6_bracket
                    } else {
                        match validators_prelude::Ipv6Addr::from_str(s) {
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

                                match validators_prelude::Ipv4Addr::from_str(domain_str) {
                                    #handle_ipv4
                                    Err(_) => {
                                        #handle_none_ipv4
                                    }
                                }
                            }
                        }
                    })
                }
            };

            let parse_impl = quote! {
                impl #name {
                    #v_parse_str
                }
            };

            let to_uri_authority_string = if local != ValidatorOption::Allow && port.not_allow() {
                quote! {
                    #[inline]
                    pub fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                        match &self.0 {
                            validators_prelude::Host::IPv4(ip) => validators_prelude::Cow::from(validators_prelude::format!("{}", ip)),
                            validators_prelude::Host::IPv6(ip) => validators_prelude::Cow::from(validators_prelude::format!("[{}]", ip)),
                            validators_prelude::Host::Domain(domain) => validators_prelude::Cow::from(domain),
                        }
                    }
                }
            } else {
                match port {
                    ValidatorOption::Allow => {
                        quote! {
                            #[inline]
                            pub fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                                match &self.host {
                                    validators_prelude::Host::IPv4(ip) => {
                                        match self.port {
                                            Some(port) => validators_prelude::Cow::from(validators_prelude::format!("{}:{}", ip, port)),
                                            None => validators_prelude::Cow::from(validators_prelude::format!("{}", ip)),
                                        }
                                    },
                                    validators_prelude::Host::IPv6(ip) => {
                                        match self.port {
                                            Some(port) => validators_prelude::Cow::from(validators_prelude::format!("[{}]:{}", ip, port)),
                                            None => validators_prelude::Cow::from(validators_prelude::format!("[{}]", ip)),
                                        }
                                    },
                                    validators_prelude::Host::Domain(domain) => {
                                        match self.port {
                                            Some(port) => validators_prelude::Cow::from(validators_prelude::format!("{}:{}", domain, port)),
                                            None => validators_prelude::Cow::from(domain),
                                        }
                                    },
                                }
                            }
                        }
                    }
                    ValidatorOption::Must => {
                        quote! {
                            #[inline]
                            pub fn to_uri_authority_string(&self) -> validators_prelude::String {
                                match &self.host {
                                    validators_prelude::Host::IPv4(ip) => validators_prelude::format!("{}:{}", ip, self.port),
                                    validators_prelude::Host::IPv6(ip) => validators_prelude::format!("[{}]:{}", ip, self.port),
                                    validators_prelude::Host::Domain(domain) => validators_prelude::format!("{}:{}", domain, self.port),
                                }
                            }
                        }
                    }
                    ValidatorOption::NotAllow => {
                        quote! {
                            #[inline]
                            pub fn to_uri_authority_string(&self) -> validators_prelude::Cow<str> {
                                match &self.host {
                                    validators_prelude::Host::IPv4(ip) => validators_prelude::Cow::from(validators_prelude::format!("{}", ip)),
                                    validators_prelude::Host::IPv6(ip) => validators_prelude::Cow::from(validators_prelude::format!("[{}]", ip)),
                                    validators_prelude::Host::Domain(domain) => validators_prelude::Cow::from(domain),
                                }
                            }
                        }
                    }
                }
            };

            let other_functions = quote! {
                impl #name {
                    #to_uri_authority_string
                }
            };

            let create_instance = {
                if local == ValidatorOption::Allow && at_least_two_labels != ValidatorOption::Allow
                {
                    match port {
                        ValidatorOption::Allow => {
                            quote! {
                                #name {
                                    host,
                                    is_local: _is_local,
                                    port: _port,
                                }
                            }
                        }
                        ValidatorOption::Must => {
                            quote! {
                                #name {
                                    host,
                                    is_local: _is_local,
                                    port: _port.unwrap(),
                                }
                            }
                        }
                        ValidatorOption::NotAllow => {
                            quote! {
                                #name {
                                    host,
                                    is_local: _is_local,
                                }
                            }
                        }
                    }
                } else {
                    match port {
                        ValidatorOption::Allow => {
                            quote! {
                                #name {
                                    host,
                                    port: _port,
                                }
                            }
                        }
                        ValidatorOption::Must => {
                            quote! {
                                #name {
                                    host,
                                    port: _port.unwrap(),
                                }
                            }
                        }
                        ValidatorOption::NotAllow => {
                            quote! {
                                #name(host)
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
                        let (host, _port, _is_local) = Self::v_parse_str(s.into().as_str())?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> Result<Self::Output, Self::Error> {
                        let (host, _port, _is_local) = Self::v_parse_str(s.as_ref())?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error> {
                        Self::v_parse_str(s.as_ref())?;

                        Ok(())
                    }
                }
            };

            let serde_impl = if cfg!(feature = "serde") {
                let expect = {
                    let mut s = String::from("a non-fully-qualified ");

                    match at_least_two_labels {
                        ValidatorOption::Allow => (),
                        ValidatorOption::Must => {
                            s.push_str("and at-least-two-labels ");
                        }
                        ValidatorOption::NotAllow => {
                            s.push_str("and one-label ");
                        }
                    }

                    s.push_str("domain name or an IP string");

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
                            }

                            deserializer.deserialize_str(ValidatingVisitor)
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

            let host_impl = quote! {
                #parameters_impl

                #parse_impl

                #validate_string_impl

                #other_functions

                #serde_impl

                #rocket_impl
            };

            host_impl.into()
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
