mod domain_attribute;

use std::fmt::Debug;

use domain_attribute::DomainAttribute;
use educe::Educe;
use quote::quote;
use syn::{Data, DeriveInput, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{fields_match, tri_allow::TriAllow, type_enum::TypeEnum},
    panic,
};

pub(crate) struct DomainHandler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowLocal {
    domain:   TypeEnum,
    is_local: TypeEnum,
}

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowPort {
    domain: TypeEnum,
    port:   TypeEnum,
}

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowPortAllowLocal {
    domain:   TypeEnum,
    is_local: TypeEnum,
    port:     TypeEnum,
}

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowIPv4 {
    domain:  TypeEnum,
    is_ipv4: TypeEnum,
}

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowIPv4AllowLocal {
    domain:   TypeEnum,
    is_ipv4:  TypeEnum,
    is_local: TypeEnum,
}

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowIPv4AllowPort {
    domain:  TypeEnum,
    is_ipv4: TypeEnum,
    port:    TypeEnum,
}

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
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
        #[cfg(feature = "serde")]
        let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
        #[cfg(feature = "serde")]
        let deserializer = ident_generator.next("__ValidatorsDeserializer");
        #[cfg(feature = "serde")]
        let deserializer_name = ident_generator.next("__validators_deserializer");
        #[cfg(feature = "serde")]
        let visitor = ident_generator.next("__ValidatorsVisitor");

        let type_attribute = DomainAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            let mut meta_is_conflict = false;

            if type_attribute.ipv4.must() && type_attribute.at_least_two_labels.disallow() {
                if type_attribute.conflict.disallow() {
                    return Err(syn::Error::new_spanned(
                        &meta,
                        "`ipv4(Must)` and `at_least_two_labels(Disallow)` cannot be used together",
                    ));
                }

                meta_is_conflict = true;
            }

            // `is_local` (and `is_ipv4` when `ipv4` is allowed) is kept only when `local` is allowed and `at_least_two_labels` is checked.
            let has_local_field = type_attribute.local == TriAllow::Allow
                && type_attribute.at_least_two_labels != TriAllow::Allow;
            let has_ipv4_field = has_local_field && type_attribute.ipv4 == TriAllow::Allow;

            let (names, item): (Option<&[&str]>, &dyn Debug) =
                match (has_ipv4_field, has_local_field, type_attribute.port) {
                    (true, _, TriAllow::Allow) => (
                        Some(&["domain", "is_ipv4", "is_local", "port"]),
                        &ITEM_ALLOW_IPV4_ALLOW_LOCAL_ALLOW_PORT,
                    ),
                    (true, _, TriAllow::Must) => (
                        Some(&["domain", "is_ipv4", "is_local", "port"]),
                        &ITEM_ALLOW_IPV4_ALLOW_LOCAL_WITH_PORT,
                    ),
                    (true, _, TriAllow::Disallow) => {
                        (Some(&["domain", "is_ipv4", "is_local"]), &ITEM_ALLOW_IPV4_ALLOW_LOCAL)
                    },
                    (false, true, TriAllow::Allow) => {
                        (Some(&["domain", "is_local", "port"]), &ITEM_ALLOW_LOCAL_ALLOW_PORT)
                    },
                    (false, true, TriAllow::Must) => {
                        (Some(&["domain", "is_local", "port"]), &ITEM_ALLOW_LOCAL_WITH_PORT)
                    },
                    (false, true, TriAllow::Disallow) => {
                        (Some(&["domain", "is_local"]), &ITEM_ALLOW_LOCAL)
                    },
                    (false, false, TriAllow::Allow) => {
                        (Some(&["domain", "port"]), &ITEM_ALLOW_PORT)
                    },
                    (false, false, TriAllow::Must) => (Some(&["domain", "port"]), &ITEM_WITH_PORT),
                    (false, false, TriAllow::Disallow) => (None, &ITEM),
                };

            if !fields_match(&data.fields, names) {
                return Err(panic::validator_for_specific_item(meta.path(), item));
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
                    ::core::option::Option::Some(colon_index) => {
                        // `host:port` has only one colon, so an input with more colons is just invalid.
                        if bytes[..colon_index].contains(&b':') {
                            return ::core::result::Result::Err(#error_path::Invalid);
                        }

                        return ::core::result::Result::Err(#error_path::PortDisallow);
                    }
                }
            } else {
                quote! {
                    ::core::option::Option::Some(colon_index) => {
                        (
                            &s[..colon_index],
                            ::core::option::Option::Some(&s[(colon_index + 1)..]),
                        )
                    }
                }
            };

            let handle_none_port = if type_attribute.port.must() {
                quote! {
                    return ::core::result::Result::Err(#error_path::PortMust);
                }
            } else {
                quote! {
                    (s, ::core::option::Option::None::<&str>)
                }
            };

            let handle_port = if type_attribute.port.disallow() {
                quote! {
                    ::core::option::Option::None::<u16>
                }
            } else {
                quote! {
                    match port_str {
                        ::core::option::Option::Some(port_str) => {
                            // A port has only digits, but `parse` also accepts a leading `+`.
                            if port_str.starts_with('+') {
                                return ::core::result::Result::Err(#error_path::Invalid);
                            }

                            match port_str.parse::<u16>() {
                                ::core::result::Result::Ok(port) => ::core::option::Option::Some(port),
                                ::core::result::Result::Err(_) => return ::core::result::Result::Err(#error_path::Invalid),
                            }
                        }
                        ::core::option::Option::None => ::core::option::Option::None,
                    }
                }
            };

            let check_local = {
                match type_attribute.local {
                    TriAllow::Allow => quote! {},
                    TriAllow::Must => {
                        quote! {
                            if !is_local {
                                return ::core::result::Result::Err(#error_path::LocalMust);
                            }
                        }
                    },
                    TriAllow::Disallow => {
                        quote! {
                            if is_local {
                                return ::core::result::Result::Err(#error_path::LocalDisallow);
                            }
                        }
                    },
                }
            };

            let invalid_domain = if type_attribute.ipv4.must() {
                quote! { #error_path::IPv4Must }
            } else {
                quote! { #error_path::Invalid }
            };
            let normalize_domain = quote! {
                validators_prelude::idna::uts46::Uts46::new()
                    .to_ascii(
                        domain_str.as_bytes(),
                        validators_prelude::idna::uts46::AsciiDenyList::STD3,
                        validators_prelude::idna::uts46::Hyphens::CheckFirstLast,
                        validators_prelude::idna::uts46::DnsLength::VerifyAllowRootDot,
                    )
                    .map_err(|_| #invalid_domain)?
            };

            let handle_domain_str_and_port_str = {
                if type_attribute.ipv4 == TriAllow::Allow
                    && type_attribute.local == TriAllow::Allow
                    && type_attribute.at_least_two_labels == TriAllow::Allow
                {
                    quote! {{
                        let ascii_domain = #normalize_domain;
                        let port = #handle_port;

                        (ascii_domain, port, false, false)
                    }}
                } else {
                    let handle_ipv4 = if type_attribute.ipv4.disallow() {
                        quote! {
                            ::core::result::Result::Ok(_) => {
                                return ::core::result::Result::Err(#error_path::IPv4Disallow);
                            }
                        }
                    } else if type_attribute.at_least_two_labels.disallow() {
                        quote! {
                            ::core::result::Result::Ok(_) => {
                                return ::core::result::Result::Err(#error_path::AtLeastTwoLabelsDisallow);
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
                            ::core::result::Result::Ok((ip, ascii_domain)) => {
                                let port = #handle_port;

                                let is_local = #handle_local_ipv4;

                                #check_local

                                (ascii_domain, port, true, is_local)
                            }
                        }
                    };

                    let handle_none_ipv4 = if type_attribute.ipv4.must() {
                        quote! {
                            ::core::result::Result::Err(_) => return ::core::result::Result::Err(#error_path::IPv4Must),
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
                                            return ::core::result::Result::Err(#error_path::AtLeastTwoLabelsMust);
                                        }
                                    }
                                },
                                TriAllow::Disallow => {
                                    // Local names are not exempt here, because subdomains such as `foo.localhost` have two labels.
                                    quote! {
                                        if validators_prelude::is_at_least_two_labels_domain(&ascii_domain) {
                                            return ::core::result::Result::Err(#error_path::AtLeastTwoLabelsDisallow);
                                        }
                                    }
                                },
                            }
                        };

                        quote! {
                            ::core::result::Result::Err(ascii_domain) => {
                                let port = #handle_port;

                                let is_local = #handle_local_domain;

                                #check_at_least_two_labels

                                #check_local

                                (ascii_domain, port, false, is_local)
                            }
                        }
                    };

                    quote! {{
                        let domain = match validators_prelude::parse_ipv4_allow_an_ended_dot(domain_str) {
                            ::core::result::Result::Ok(ip) => ::core::result::Result::Ok((ip, validators_prelude::Cow::Borrowed(domain_str))),
                            ::core::result::Result::Err(_) => {
                                let ascii_domain = #normalize_domain;

                                match validators_prelude::parse_ipv4_allow_an_ended_dot(&ascii_domain) {
                                    ::core::result::Result::Ok(ip) => ::core::result::Result::Ok((ip, ascii_domain)),
                                    ::core::result::Result::Err(_) => ::core::result::Result::Err(ascii_domain),
                                }
                            }
                        };

                        match domain {
                            #handle_ipv4
                            #handle_none_ipv4
                        }
                    }}
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
                    fn v_parse_str(s: &str) -> ::core::result::Result<(validators_prelude::Cow<'_, str>, ::core::option::Option<u16>, bool, bool), #error_path> {
                        let bytes = s.as_bytes();

                        if bytes.is_empty() {
                            return ::core::result::Result::Err(#error_path::Invalid);
                        }

                        let (domain_str, port_str) = match bytes.iter().copied().rposition(|e| e == b':') {
                            #handle_domain_and_port
                            ::core::option::Option::None => {
                                #handle_none_port
                            }
                        };

                        ::core::result::Result::Ok(#handle_domain_str_and_port_str)
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
                    fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                        let mut s = s.into();
                        let (domain, _port, _is_ipv4, _is_local) = Self::v_parse_str(s.as_str())?;
                        let domain = match domain {
                            validators_prelude::Cow::Borrowed(domain) => {
                                // The borrowed domain is always a prefix of the input.
                                s.truncate(domain.len());
                                s
                            }
                            validators_prelude::Cow::Owned(domain) => domain,
                        };

                        ::core::result::Result::Ok(#create_instance)
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                        let (domain, _port, _is_ipv4, _is_local) = Self::v_parse_str(s.as_ref())?;
                        let domain = domain.into_owned();

                        ::core::result::Result::Ok(#create_instance)
                    }

                    #[inline]
                    fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
                        Self::v_parse_str(s.as_ref())?;

                        ::core::result::Result::Ok(())
                    }
                }
            });

            let is_tuple = type_attribute.port == TriAllow::Disallow
                && (type_attribute.local != TriAllow::Allow
                    || type_attribute.at_least_two_labels == TriAllow::Allow);

            token_stream.extend(if is_tuple {
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
            });

            token_stream.extend(match type_attribute.port {
                TriAllow::Allow => {
                    quote! {
                        impl ToUriAuthorityString for #name {
                            #[inline]
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<'_, str> {
                                match self.port {
                                    ::core::option::Option::Some(port) => {
                                        let non_fully_qualified_domain = QualifyDomain::get_domain_non_fully_qualified(self);

                                        validators_prelude::Cow::Owned(validators_prelude::format!("{}:{}", non_fully_qualified_domain, port))
                                    },
                                    ::core::option::Option::None => validators_prelude::Cow::Borrowed(self.get_domain_non_fully_qualified()),
                                }
                            }
                        }
                    }
                }
                TriAllow::Must => {
                    quote! {
                        impl ToUriAuthorityString for #name {
                            #[inline]
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<'_, str> {
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
                            fn to_uri_authority_string(&self) -> validators_prelude::Cow<'_, str> {
                                validators_prelude::Cow::Borrowed(self.get_domain_non_fully_qualified())
                            }
                        }
                    }
                }
            });

            #[cfg(feature = "serde")]
            {
                if type_attribute.serde_options.serialize {
                    let serialize_domain = match type_attribute.port {
                        TriAllow::Allow => quote! {
                            if let ::core::option::Option::Some(port) = self.port {
                                serializer.collect_str(&::core::format_args!("{}:{}", self.domain, port))
                            } else {
                                serializer.serialize_str(self.domain.as_str())
                            }
                        },
                        TriAllow::Must => quote! {
                            serializer.collect_str(&::core::format_args!("{}:{}", self.domain, self.port))
                        },
                        TriAllow::Disallow if is_tuple => quote! {
                            serializer.serialize_str(self.0.as_str())
                        },
                        TriAllow::Disallow => quote! {
                            serializer.serialize_str(self.domain.as_str())
                        },
                    };

                    token_stream.extend(quote! {
                        impl validators_prelude::serde::Serialize for #name {
                            #[inline]
                            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                                where
                                    S: validators_prelude::serde::Serializer, {
                                #serialize_domain
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
                                match type_attribute.at_least_two_labels {
                                    TriAllow::Allow => (),
                                    TriAllow::Must => {
                                        s.push_str("at-least-two-labels and ");
                                    },
                                    TriAllow::Disallow => {
                                        s.push_str("one-label and ");
                                    },
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
