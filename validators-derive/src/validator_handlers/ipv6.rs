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

const VALIDATOR: Validator = Validator::ipv6;

pub fn ipv6_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            let mut local = ValidatorOption::new();
            let mut port = ValidatorOption::new();

            let correct_usage_for_attribute = [stringify!(#[validator(ipv6)])];

            let correct_usage_for_local = [
                stringify!(#[validator(ipv6(local(Must)))]),
                stringify!(#[validator(ipv6(local(Allow)))]),
                stringify!(#[validator(ipv6(local(NotAllow)))]),
            ];

            let correct_usage_for_port = [
                stringify!(#[validator(ipv6(port(Must)))]),
                stringify!(#[validator(ipv6(port(Allow)))]),
                stringify!(#[validator(ipv6(port(NotAllow)))]),
            ];

            match meta {
                Meta::Path(_) => (),
                Meta::List(list) => {
                    let mut local_is_set = false;
                    let mut port_is_set = false;

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
                                    _ => panic::unknown_parameter("ipv6", meta_name.as_str()),
                                }
                            }
                            NestedMeta::Lit(_) => {
                                panic::attribute_incorrect_format(
                                    "ipv6",
                                    &correct_usage_for_attribute,
                                )
                            }
                        }
                    }
                }
                Meta::NameValue(_) => {
                    panic::attribute_incorrect_format("ipv6", &correct_usage_for_attribute)
                }
            }

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

                            if ident != "ipv6" && ident != "port" {
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

                            if ident != "ipv6" && ident != "port" {
                                panic::validator_only_support_for_item(
                                    VALIDATOR,
                                    Box::new(ITEM_WITH_PORT),
                                );
                            }
                        }
                    } else {
                        panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM_WITH_PORT));
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

            let name = ast.ident;

            // TODO impl

            let error_path: Path = syn::parse2(quote! { validators_prelude::IPv6Error }).unwrap();

            let local_path = local.to_expr();
            let port_path = port.to_expr();

            let parameters_impl = quote! {
                impl #name {
                    pub(crate) const V_LOCAL: validators_prelude::ValidatorOption = #local_path;
                    pub(crate) const V_PORT: validators_prelude::ValidatorOption = #port_path;
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

            let handle_local_ipv6 = if local == ValidatorOption::Allow {
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

                            (ip, None, is_local)
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

            let handle_ipv6_non_bracket = if port.must() {
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

            let v_parse_str = quote! {
                fn v_parse_str(s: &str) -> Result<(validators_prelude::Ipv6Addr, Option<u16>, bool), #error_path> {
                    use core::str::FromStr;

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
                        match validators_prelude::Ipv6Addr::from_str(s) {
                            #handle_ipv6_non_bracket
                            Err(_) => return Err(#error_path::Invalid)
                        }
                    })
                }
            };

            let parse_impl = quote! {
                impl #name {
                    #v_parse_str
                }
            };

            let to_uri_authority_string = {
                match port {
                    ValidatorOption::Allow => {
                        quote! {
                            #[inline]
                            pub fn to_uri_authority_string(&self) -> String {
                                match self.port {
                                    Some(port) => validators_prelude::format!("[{}]:{}", self.ipv6, port),
                                    None => validators_prelude::format!("[{}]", self.ipv6),
                                }
                            }
                        }
                    }
                    ValidatorOption::Must => {
                        quote! {
                            #[inline]
                            pub fn to_uri_authority_string(&self) -> String {
                                validators_prelude::format!("[{}]:{}", self.ipv6, self.port)
                            }
                        }
                    }
                    ValidatorOption::NotAllow => {
                        quote! {
                            #[inline]
                            pub fn to_uri_authority_string(&self) -> String {
                                validators_prelude::format!("[{}]", self.0)
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
                match port {
                    ValidatorOption::Allow => {
                        quote! {
                            #name {
                                ipv6,
                                port: _port,
                            }
                        }
                    }
                    ValidatorOption::Must => {
                        quote! {
                            #name {
                                ipv6,
                                port: _port.unwrap(),
                            }
                        }
                    }
                    ValidatorOption::NotAllow => {
                        quote! {
                            #name(ipv6)
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
                        let (ipv6, _port, _is_local) = Self::v_parse_str(s.into().as_str())?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> Result<Self::Output, Self::Error> {
                        let (ipv6, _port, _is_local) = Self::v_parse_str(s.as_ref())?;

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
                    let mut s = String::from("an IPv6 string");

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
                            <#name as ValidateString>::parse_string(v.url_decode().map_err(|_| #error_path::Invalid)?)
                        }
                    }

                    impl<'a> validators_prelude::FromParam<'a> for #name {
                        type Error = #error_path;

                        #[inline]
                        fn from_param(v: &'a validators_prelude::RawStr) -> Result<Self, Self::Error> {
                            <#name as ValidateString>::parse_string(v.url_decode().map_err(|_| #error_path::Invalid)?)
                        }
                    }
                }
            } else {
                quote! {}
            };

            let ipv6_impl = quote! {
                #parameters_impl

                #parse_impl

                #validate_string_impl

                #other_functions

                #serde_impl

                #rocket_impl
            };

            ipv6_impl.into()
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
