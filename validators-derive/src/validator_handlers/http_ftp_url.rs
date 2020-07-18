use alloc::boxed::Box;
use alloc::string::{String, ToString};

use crate::proc_macro::TokenStream;
use crate::quote::ToTokens;
use crate::syn::{Data, DeriveInput, Fields, Meta, NestedMeta, Path};

use crate::{panic, SynOption, TypeEnum, Validator, ValidatorOption};

#[derive(Debug)]
pub struct Struct {
    url: TypeEnum,
    protocol: TypeEnum,
}

const ITEM: Struct = Struct {
    url: TypeEnum::Url,
    protocol: TypeEnum::Protocol,
};
const VALIDATOR: Validator = Validator::http_ftp_url;

pub fn http_ftp_url_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Named(_) = &data.fields {
                if data.fields.len() != 2 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                for field in data.fields.iter() {
                    let ident = field.ident.as_ref().unwrap();

                    if ident != "url" && ident != "protocol" {
                        panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                    }
                }

                let mut local = ValidatorOption::new();

                let correct_usage_for_attribute = [stringify!(#[validator(http_ftp_url)])];

                let correct_usage_for_local = [
                    stringify!(#[validator(http_ftp_url(local(Must)))]),
                    stringify!(#[validator(http_ftp_url(local(Allow)))]),
                    stringify!(#[validator(http_ftp_url(local(NotAllow)))]),
                ];

                match meta {
                    Meta::Path(_) => (),
                    Meta::List(list) => {
                        let mut local_is_set = false;

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
                                        _ => {
                                            panic::unknown_parameter(
                                                "http_ftp_url",
                                                meta_name.as_str(),
                                            )
                                        }
                                    }
                                }
                                NestedMeta::Lit(_) => {
                                    panic::attribute_incorrect_format(
                                        "http_ftp_url",
                                        &correct_usage_for_attribute,
                                    )
                                }
                            }
                        }
                    }
                    Meta::NameValue(_) => {
                        panic::attribute_incorrect_format(
                            "http_ftp_url",
                            &correct_usage_for_attribute,
                        )
                    }
                }

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::HttpFtpURLError }).unwrap();

                let local_path = local.to_expr();

                let parameters_impl = quote! {
                    impl #name {
                        pub(crate) const V_LOCAL: validators_prelude::ValidatorOption = #local_path;
                    }
                };

                let handle_local = {
                    match local {
                        ValidatorOption::Allow => {
                            quote! {}
                        }
                        _ => {
                            let check_local = if local.not_allow() {
                                quote! {
                                    if is_local {
                                        return Err(#error_path::LocalNotAllow);
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
                        }
                    }
                };

                let v_parse_str = quote! {
                    #[inline]
                    fn v_parse_str(s: &str) -> Result<(validators_prelude::url::Url, validators_prelude::Protocol), #error_path> {
                        let protocol = {
                            use validators_prelude::str_utils::StartsWithIgnoreAsciiCaseMultiple;

                            if let Some(index) = s.starts_with_ignore_ascii_case_with_lowercase_multiple(&["http:", "https:", "ftp:"]) {
                                match index {
                                    0 => validators_prelude::Protocol::HTTP,
                                    1 => validators_prelude::Protocol::HTTPS,
                                    2 => validators_prelude::Protocol::FTP,
                                    _ => unreachable!()
                                }
                            } else {
                                return Err(#error_path::ProtocolError);
                            }
                        };

                        let url = validators_prelude::url::Url::parse(s)?;

                        #handle_local

                        Ok((url, protocol))
                    }
                };

                let parse_impl = quote! {
                    impl #name {
                        #v_parse_str
                    }
                };

                let validate_string_impl = quote! {
                    impl ValidateString for #name {
                        type Error = #error_path;
                        type Output = Self;

                        #[inline]
                        fn parse_string<S: Into<validators_prelude::String>>(s: S) -> Result<Self::Output, Self::Error> {
                            let (url, protocol) = Self::v_parse_str(s.into().as_str())?;

                            Ok(#name {
                                url,
                                protocol,
                            })
                        }

                        #[inline]
                        fn parse_str<S: AsRef<str>>(s: S) -> Result<Self::Output, Self::Error> {
                            let (url, protocol) = Self::v_parse_str(s.as_ref())?;

                            Ok(#name {
                                url,
                                protocol,
                            })
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
                        let mut s = String::from("a http/https/ftp url");

                        match local {
                            ValidatorOption::Allow => (),
                            ValidatorOption::Must => {
                                s.push_str(" which must be local");
                            }
                            ValidatorOption::NotAllow => {
                                s.push_str(" which must not be local");
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
                                validators_prelude::Serialize::serialize(&self.url, serializer)
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

                let http_ftp_url_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #serde_impl

                    #rocket_impl
                };

                http_ftp_url_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
