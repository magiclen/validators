mod email_attribute;

use educe::Educe;
use email_attribute::EmailAttribute;
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{tri_allow::TriAllow, type_enum::TypeEnum},
    panic,
};

pub(crate) struct EmailHandler;

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct Struct {
    local_part:  TypeEnum,
    need_quoted: TypeEnum,
    domain_part: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowComment {
    local_part:                 TypeEnum,
    need_quoted:                TypeEnum,
    domain_part:                TypeEnum,
    comment_before_local_part:  TypeEnum,
    comment_after_local_part:   TypeEnum,
    comment_before_domain_part: TypeEnum,
    comment_after_domain_part:  TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowLocal {
    local_part:  TypeEnum,
    need_quoted: TypeEnum,
    domain_part: TypeEnum,
    is_local:    TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowCommentAllowLocal {
    local_part:                 TypeEnum,
    need_quoted:                TypeEnum,
    domain_part:                TypeEnum,
    comment_before_local_part:  TypeEnum,
    comment_after_local_part:   TypeEnum,
    comment_before_domain_part: TypeEnum,
    comment_after_domain_part:  TypeEnum,
    is_local:                   TypeEnum,
}

const ITEM: Struct = Struct {
    local_part:  TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::String,
};

const ITEM_ALLOW_IP: Struct = Struct {
    local_part:  TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::Host,
};

const ITEM_MUST_IP: Struct = Struct {
    local_part:  TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::IpAddr,
};

const ITEM_ALLOW_COMMENT: StructAllowComment = StructAllowComment {
    local_part:                 TypeEnum::String,
    need_quoted:                TypeEnum::Boolean,
    domain_part:                TypeEnum::String,
    comment_before_local_part:  TypeEnum::OptionString,
    comment_after_local_part:   TypeEnum::OptionString,
    comment_before_domain_part: TypeEnum::OptionString,
    comment_after_domain_part:  TypeEnum::OptionString,
};

const ITEM_ALLOW_COMMENT_ALLOW_IP: StructAllowComment = StructAllowComment {
    local_part:                 TypeEnum::String,
    need_quoted:                TypeEnum::Boolean,
    domain_part:                TypeEnum::Host,
    comment_before_local_part:  TypeEnum::OptionString,
    comment_after_local_part:   TypeEnum::OptionString,
    comment_before_domain_part: TypeEnum::OptionString,
    comment_after_domain_part:  TypeEnum::OptionString,
};

const ITEM_ALLOW_COMMENT_MUST_IP: StructAllowComment = StructAllowComment {
    local_part:                 TypeEnum::String,
    need_quoted:                TypeEnum::Boolean,
    domain_part:                TypeEnum::IpAddr,
    comment_before_local_part:  TypeEnum::OptionString,
    comment_after_local_part:   TypeEnum::OptionString,
    comment_before_domain_part: TypeEnum::OptionString,
    comment_after_domain_part:  TypeEnum::OptionString,
};

const ITEM_ALLOW_LOCAL: StructAllowLocal = StructAllowLocal {
    local_part:  TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::String,
    is_local:    TypeEnum::Boolean,
};

const ITEM_ALLOW_LOCAL_ALLOW_IP: StructAllowLocal = StructAllowLocal {
    local_part:  TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::Host,
    is_local:    TypeEnum::Boolean,
};

const ITEM_ALLOW_LOCAL_MUST_IP: StructAllowLocal = StructAllowLocal {
    local_part:  TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::IpAddr,
    is_local:    TypeEnum::Boolean,
};

const ITEM_ALLOW_COMMENT_ALLOW_LOCAL: StructAllowCommentAllowLocal = StructAllowCommentAllowLocal {
    local_part:                 TypeEnum::String,
    need_quoted:                TypeEnum::Boolean,
    domain_part:                TypeEnum::String,
    comment_before_local_part:  TypeEnum::OptionString,
    comment_after_local_part:   TypeEnum::OptionString,
    comment_before_domain_part: TypeEnum::OptionString,
    comment_after_domain_part:  TypeEnum::OptionString,
    is_local:                   TypeEnum::Boolean,
};

const ITEM_ALLOW_COMMENT_ALLOW_LOCAL_ALLOW_IP: StructAllowCommentAllowLocal =
    StructAllowCommentAllowLocal {
        local_part:                 TypeEnum::String,
        need_quoted:                TypeEnum::Boolean,
        domain_part:                TypeEnum::Host,
        comment_before_local_part:  TypeEnum::OptionString,
        comment_after_local_part:   TypeEnum::OptionString,
        comment_before_domain_part: TypeEnum::OptionString,
        comment_after_domain_part:  TypeEnum::OptionString,
        is_local:                   TypeEnum::Boolean,
    };

const ITEM_ALLOW_COMMENT_ALLOW_LOCAL_MUST_IP: StructAllowCommentAllowLocal =
    StructAllowCommentAllowLocal {
        local_part:                 TypeEnum::String,
        need_quoted:                TypeEnum::Boolean,
        domain_part:                TypeEnum::IpAddr,
        comment_before_local_part:  TypeEnum::OptionString,
        comment_after_local_part:   TypeEnum::OptionString,
        comment_before_domain_part: TypeEnum::OptionString,
        comment_after_domain_part:  TypeEnum::OptionString,
        is_local:                   TypeEnum::Boolean,
    };

impl ValidatorHandler for EmailHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = EmailAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            let mut meta_is_conflict = false;

            if type_attribute.ip.must() && type_attribute.at_least_two_labels.disallow() {
                if type_attribute.conflict.disallow() {
                    return Err(syn::Error::new(
                        meta.span(),
                        "`ip(Must)` and `at_least_two_labels(Disallow)` cannot be used together.",
                    ));
                }

                meta_is_conflict = true;
            }

            if type_attribute.comment.allow() {
                match type_attribute.ip {
                    TriAllow::Allow => {
                        if type_attribute.local == TriAllow::Allow
                            && type_attribute.at_least_two_labels != TriAllow::Allow
                        {
                            if let Fields::Named(_) = &data.fields {
                                if data.fields.len() != 8 {
                                    return Err(panic::validator_for_specific_item(
                                        meta.path().get_ident().unwrap(),
                                        ITEM_ALLOW_COMMENT_ALLOW_LOCAL_ALLOW_IP,
                                    ));
                                }

                                for field in data.fields.iter() {
                                    let ident_string = field.ident.as_ref().unwrap().to_string();

                                    match ident_string.as_str() {
                                        "local_part"
                                        | "need_quoted"
                                        | "domain_part"
                                        | "comment_before_local_part"
                                        | "comment_after_local_part"
                                        | "comment_before_domain_part"
                                        | "comment_after_domain_part"
                                        | "is_local" => (),
                                        _ => {
                                            return Err(panic::validator_for_specific_item(
                                                meta.path().get_ident().unwrap(),
                                                ITEM_ALLOW_COMMENT_ALLOW_LOCAL_ALLOW_IP,
                                            ));
                                        },
                                    }
                                }
                            } else {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_ALLOW_COMMENT_ALLOW_LOCAL_ALLOW_IP,
                                ));
                            }
                        } else if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 7 {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_ALLOW_COMMENT_ALLOW_IP,
                                ));
                            }

                            for field in data.fields.iter() {
                                let ident_string = field.ident.as_ref().unwrap().to_string();

                                match ident_string.as_str() {
                                    "local_part"
                                    | "need_quoted"
                                    | "domain_part"
                                    | "comment_before_local_part"
                                    | "comment_after_local_part"
                                    | "comment_before_domain_part"
                                    | "comment_after_domain_part" => (),
                                    _ => {
                                        return Err(panic::validator_for_specific_item(
                                            meta.path().get_ident().unwrap(),
                                            ITEM_ALLOW_COMMENT_ALLOW_IP,
                                        ));
                                    },
                                }
                            }
                        } else {
                            return Err(panic::validator_for_specific_item(
                                meta.path().get_ident().unwrap(),
                                ITEM_ALLOW_COMMENT_ALLOW_IP,
                            ));
                        }
                    },
                    TriAllow::Must => {
                        if type_attribute.local == TriAllow::Allow
                            && type_attribute.at_least_two_labels != TriAllow::Allow
                        {
                            if let Fields::Named(_) = &data.fields {
                                if data.fields.len() != 8 {
                                    return Err(panic::validator_for_specific_item(
                                        meta.path().get_ident().unwrap(),
                                        ITEM_ALLOW_COMMENT_ALLOW_LOCAL_MUST_IP,
                                    ));
                                }

                                for field in data.fields.iter() {
                                    let ident_string = field.ident.as_ref().unwrap().to_string();

                                    match ident_string.as_str() {
                                        "local_part"
                                        | "need_quoted"
                                        | "domain_part"
                                        | "comment_before_local_part"
                                        | "comment_after_local_part"
                                        | "comment_before_domain_part"
                                        | "comment_after_domain_part"
                                        | "is_local" => (),
                                        _ => {
                                            return Err(panic::validator_for_specific_item(
                                                meta.path().get_ident().unwrap(),
                                                ITEM_ALLOW_COMMENT_ALLOW_LOCAL_MUST_IP,
                                            ));
                                        },
                                    }
                                }
                            } else {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_ALLOW_COMMENT_ALLOW_LOCAL_MUST_IP,
                                ));
                            }
                        } else if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 7 {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_ALLOW_COMMENT_MUST_IP,
                                ));
                            }

                            for field in data.fields.iter() {
                                let ident_string = field.ident.as_ref().unwrap().to_string();

                                match ident_string.as_str() {
                                    "local_part"
                                    | "need_quoted"
                                    | "domain_part"
                                    | "comment_before_local_part"
                                    | "comment_after_local_part"
                                    | "comment_before_domain_part"
                                    | "comment_after_domain_part" => (),
                                    _ => {
                                        return Err(panic::validator_for_specific_item(
                                            meta.path().get_ident().unwrap(),
                                            ITEM_ALLOW_COMMENT_MUST_IP,
                                        ));
                                    },
                                }
                            }
                        } else {
                            return Err(panic::validator_for_specific_item(
                                meta.path().get_ident().unwrap(),
                                ITEM_ALLOW_COMMENT_MUST_IP,
                            ));
                        }
                    },
                    TriAllow::Disallow => {
                        if type_attribute.local == TriAllow::Allow
                            && type_attribute.at_least_two_labels != TriAllow::Allow
                        {
                            if let Fields::Named(_) = &data.fields {
                                if data.fields.len() != 8 {
                                    return Err(panic::validator_for_specific_item(
                                        meta.path().get_ident().unwrap(),
                                        ITEM_ALLOW_COMMENT_ALLOW_LOCAL,
                                    ));
                                }

                                for field in data.fields.iter() {
                                    let ident_string = field.ident.as_ref().unwrap().to_string();

                                    match ident_string.as_str() {
                                        "local_part"
                                        | "need_quoted"
                                        | "domain_part"
                                        | "comment_before_local_part"
                                        | "comment_after_local_part"
                                        | "comment_before_domain_part"
                                        | "comment_after_domain_part"
                                        | "is_local" => (),
                                        _ => {
                                            return Err(panic::validator_for_specific_item(
                                                meta.path().get_ident().unwrap(),
                                                ITEM_ALLOW_COMMENT_ALLOW_LOCAL,
                                            ));
                                        },
                                    }
                                }
                            } else {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_ALLOW_COMMENT_ALLOW_LOCAL,
                                ));
                            }
                        } else if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 7 {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_ALLOW_COMMENT,
                                ));
                            }

                            for field in data.fields.iter() {
                                let ident_string = field.ident.as_ref().unwrap().to_string();

                                match ident_string.as_str() {
                                    "local_part"
                                    | "need_quoted"
                                    | "domain_part"
                                    | "comment_before_local_part"
                                    | "comment_after_local_part"
                                    | "comment_before_domain_part"
                                    | "comment_after_domain_part" => (),
                                    _ => {
                                        return Err(panic::validator_for_specific_item(
                                            meta.path().get_ident().unwrap(),
                                            ITEM_ALLOW_COMMENT,
                                        ));
                                    },
                                }
                            }
                        } else {
                            return Err(panic::validator_for_specific_item(
                                meta.path().get_ident().unwrap(),
                                ITEM_ALLOW_COMMENT,
                            ));
                        }
                    },
                }
            } else {
                match type_attribute.ip {
                    TriAllow::Allow => {
                        if type_attribute.local == TriAllow::Allow
                            && type_attribute.at_least_two_labels != TriAllow::Allow
                        {
                            if let Fields::Named(_) = &data.fields {
                                if data.fields.len() != 4 {
                                    return Err(panic::validator_for_specific_item(
                                        meta.path().get_ident().unwrap(),
                                        ITEM_ALLOW_LOCAL_ALLOW_IP,
                                    ));
                                }

                                for field in data.fields.iter() {
                                    let ident_string = field.ident.as_ref().unwrap().to_string();

                                    match ident_string.as_str() {
                                        "local_part" | "need_quoted" | "domain_part"
                                        | "is_local" => (),
                                        _ => {
                                            return Err(panic::validator_for_specific_item(
                                                meta.path().get_ident().unwrap(),
                                                ITEM_ALLOW_LOCAL_ALLOW_IP,
                                            ));
                                        },
                                    }
                                }
                            } else {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_ALLOW_LOCAL_ALLOW_IP,
                                ));
                            }
                        } else if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 3 {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_ALLOW_IP,
                                ));
                            }

                            for field in data.fields.iter() {
                                let ident_string = field.ident.as_ref().unwrap().to_string();

                                match ident_string.as_str() {
                                    "local_part" | "need_quoted" | "domain_part" => (),
                                    _ => {
                                        return Err(panic::validator_for_specific_item(
                                            meta.path().get_ident().unwrap(),
                                            ITEM_ALLOW_IP,
                                        ));
                                    },
                                }
                            }
                        } else {
                            return Err(panic::validator_for_specific_item(
                                meta.path().get_ident().unwrap(),
                                ITEM_ALLOW_IP,
                            ));
                        }
                    },
                    TriAllow::Must => {
                        if type_attribute.local == TriAllow::Allow
                            && type_attribute.at_least_two_labels != TriAllow::Allow
                        {
                            if let Fields::Named(_) = &data.fields {
                                if data.fields.len() != 4 {
                                    return Err(panic::validator_for_specific_item(
                                        meta.path().get_ident().unwrap(),
                                        ITEM_ALLOW_LOCAL_MUST_IP,
                                    ));
                                }

                                for field in data.fields.iter() {
                                    let ident_string = field.ident.as_ref().unwrap().to_string();

                                    match ident_string.as_str() {
                                        "local_part" | "need_quoted" | "domain_part"
                                        | "is_local" => (),
                                        _ => {
                                            return Err(panic::validator_for_specific_item(
                                                meta.path().get_ident().unwrap(),
                                                ITEM_ALLOW_LOCAL_MUST_IP,
                                            ));
                                        },
                                    }
                                }
                            } else {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_ALLOW_LOCAL_MUST_IP,
                                ));
                            }
                        } else if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 3 {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM_MUST_IP,
                                ));
                            }

                            for field in data.fields.iter() {
                                let ident_string = field.ident.as_ref().unwrap().to_string();

                                match ident_string.as_str() {
                                    "local_part" | "need_quoted" | "domain_part" => (),
                                    _ => {
                                        return Err(panic::validator_for_specific_item(
                                            meta.path().get_ident().unwrap(),
                                            ITEM_MUST_IP,
                                        ));
                                    },
                                }
                            }
                        } else {
                            return Err(panic::validator_for_specific_item(
                                meta.path().get_ident().unwrap(),
                                ITEM_MUST_IP,
                            ));
                        }
                    },
                    TriAllow::Disallow => {
                        if type_attribute.local == TriAllow::Allow
                            && type_attribute.at_least_two_labels != TriAllow::Allow
                        {
                            if let Fields::Named(_) = &data.fields {
                                if data.fields.len() != 4 {
                                    return Err(panic::validator_for_specific_item(
                                        meta.path().get_ident().unwrap(),
                                        ITEM_ALLOW_LOCAL,
                                    ));
                                }

                                for field in data.fields.iter() {
                                    let ident_string = field.ident.as_ref().unwrap().to_string();

                                    match ident_string.as_str() {
                                        "local_part" | "need_quoted" | "domain_part"
                                        | "is_local" => (),
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
                        } else if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 3 {
                                return Err(panic::validator_for_specific_item(
                                    meta.path().get_ident().unwrap(),
                                    ITEM,
                                ));
                            }

                            for field in data.fields.iter() {
                                let ident_string = field.ident.as_ref().unwrap().to_string();

                                match ident_string.as_str() {
                                    "local_part" | "need_quoted" | "domain_part" => (),
                                    _ => {
                                        return Err(panic::validator_for_specific_item(
                                            meta.path().get_ident().unwrap(),
                                            ITEM,
                                        ));
                                    },
                                }
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

            let error_path: Path = syn::parse2(quote! { validators_prelude::EmailError }).unwrap();

            #[cfg(feature = "test")]
            {
                let v_comment = type_attribute.comment;
                let v_ip = type_attribute.ip;
                let v_local = type_attribute.local;
                let v_at_least_two_labels = type_attribute.at_least_two_labels;
                let v_non_ascii = type_attribute.non_ascii;

                token_stream.extend(quote! {
                    impl #name {
                        pub(crate) const V_COMMENT: validators_prelude::TriAllow = #v_comment;
                        pub(crate) const V_IP: validators_prelude::TriAllow = #v_ip;
                        pub(crate) const V_LOCAL: validators_prelude::TriAllow = #v_local;
                        pub(crate) const V_AT_LEAST_TWO_LABELS: validators_prelude::TriAllow = #v_at_least_two_labels;
                        pub(crate) const V_NON_ASCII: validators_prelude::TriAllow = #v_non_ascii;
                    }
                });
            }

            let handle_unrecognized_first_byte_quoted = if type_attribute.non_ascii.disallow() {
                quote! {
                    return Err(#error_path::Invalid);
                }
            } else {
                quote! {
                    if e < 128 {
                        return Err(#error_path::Invalid);
                    } else {
                        (false ,false, false)
                    }
                }
            };

            let handle_unrecognized_first_byte_unquoted = if type_attribute.non_ascii.disallow() {
                quote! {
                    return Err(#error_path::Invalid);
                }
            } else {
                quote! {
                    if e < 128 {
                        return Err(#error_path::Invalid);
                    }
                }
            };

            let handle_unrecognized_byte_quoted = if type_attribute.non_ascii.disallow() {
                quote! {
                    return Err(#error_path::Invalid);
                }
            } else {
                quote! {
                    if e < 128 {
                        return Err(#error_path::Invalid);
                    } else {
                        last_dot = false;
                        escaping = false;
                    }
                }
            };

            let handle_unrecognized_byte_unquoted = if type_attribute.non_ascii.disallow() {
                quote! {
                    return Err(#error_path::Invalid);
                }
            } else {
                quote! {
                    if e < 128 {
                        return Err(#error_path::Invalid);
                    } else {
                        last_dot = false;
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

            let handle_ip = if type_attribute.ip.disallow() {
                quote! {
                    return Err(#error_path::IPDisallow);
                }
            } else if type_attribute.at_least_two_labels.disallow() {
                quote! {
                    return Err(#error_path::AtLeastTwoLabelsDisallow);
                }
            } else {
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

                quote! {
                    use ::core::str::FromStr;

                    domain_part_length += 1;

                    if length > domain_part_length {
                        if bytes[domain_part_length..].starts_with(b"IPv6:") {
                            domain_part_length += 5;

                            if length > domain_part_length {
                                match bytes[domain_part_length..]
                                    .iter()
                                    .copied()
                                    .position(|e| e == b']')
                                {
                                    Some(mut closing_bracket_index) => {
                                        closing_bracket_index += domain_part_length;

                                        let ip_str = unsafe {
                                            ::core::str::from_utf8_unchecked(
                                                &bytes[domain_part_length..closing_bracket_index],
                                            )
                                        };

                                        match ::std::net::Ipv6Addr::from_str(ip_str) {
                                            Ok(ip) => {
                                                domain_part_length = closing_bracket_index + 1;

                                                // adjust length
                                                domain_part_length_offset += format!("{}", ip).len()
                                                    as isize
                                                    - ip_str.len() as isize;

                                                (validators_prelude::Host::IPv6(ip), #handle_local_ipv6)
                                            }
                                            Err(_) => return Err(#error_path::Invalid),
                                        }
                                    }
                                    None => return Err(#error_path::Invalid),
                                }
                            } else {
                                return Err(#error_path::Invalid);
                            }
                        } else {
                            match bytes[domain_part_length..].iter().copied().position(|e| e == b']') {
                                Some(mut closing_bracket_index) => {
                                    closing_bracket_index += domain_part_length;

                                    let ip_str = unsafe {
                                        ::core::str::from_utf8_unchecked(
                                            &bytes[domain_part_length..closing_bracket_index],
                                        )
                                    };

                                    match ::std::net::Ipv4Addr::from_str(ip_str) {
                                        Ok(ip) => {
                                            domain_part_length = closing_bracket_index + 1;

                                            (validators_prelude::Host::IPv4(ip), #handle_local_ipv4)
                                        }
                                        Err(_) => return Err(#error_path::Invalid),
                                    }
                                }
                                None => return Err(#error_path::Invalid),
                            }
                        }
                    } else {
                        return Err(#error_path::Invalid);
                    }
                }
            };

            let handle_domain = if type_attribute.ip.must() {
                quote! {
                    return Err(#error_path::IPMust);
                }
            } else {
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
                    let domain_str = {
                        match bytes[domain_part_length..].iter().copied().position(|e| e == b'(') {
                            Some(mut open_parenthesis_index) => {
                                open_parenthesis_index += domain_part_length;
                                unsafe {
                                    ::core::str::from_utf8_unchecked(
                                        &bytes[domain_part_length..open_parenthesis_index],
                                    )
                                }
                            }
                            None => unsafe { ::core::str::from_utf8_unchecked(&bytes[domain_part_length..]) },
                        }
                    };

                    if domain_str.ends_with(".") {
                        return Err(#error_path::Invalid);
                    }

                    match validators_prelude::idna::Config::default()
                        .use_std3_ascii_rules(true)
                        .verify_dns_length(true)
                        .check_hyphens(true)
                        .to_ascii(domain_str)
                    {
                        Ok(ascii_domain) => {
                            domain_part_length += domain_str.len();

                            let is_local = #handle_local_domain;

                            #check_at_least_two_labels

                            // adjust length
                            domain_part_length_offset +=
                                ascii_domain.len() as isize - domain_str.len() as isize;

                            (validators_prelude::Host::Domain(ascii_domain), is_local)
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

            let handle_comment_1 = if type_attribute.comment.disallow() {
                quote! {
                    return Err(#error_path::CommentDisallow);
                }
            } else {
                quote! {
                    if length > 1 {
                        // only take 63 - 1, 1 is reserved for the meaningful local part.
                        match bytes[1..].iter().take(62).copied().position(|e| e == b')') {
                            Some(mut index) => {
                                index += 1;
                                let comment = unsafe { ::core::str::from_utf8_unchecked(&bytes[1..index]) };

                                let local_part_length = index + 1;

                                if local_part_length == length {
                                    // end
                                    return Err(#error_path::Invalid);
                                }

                                (local_part_length, Some(comment))
                            }
                            None => return Err(#error_path::Invalid),
                        }
                    } else {
                        return Err(#error_path::Invalid);
                    }
                }
            };

            let handle_comment_2 = if type_attribute.comment.disallow() {
                quote! {
                    return Err(#error_path::CommentDisallow);
                }
            } else {
                quote! {
                    local_part_length += 1;

                    if length > local_part_length {
                        match bytes[local_part_length..]
                            .iter()
                            .take(64 - local_part_length)
                            .copied()
                            .position(|e| e == b')')
                        {
                            Some(mut index) => {
                                index += local_part_length;
                                let comment =
                                    unsafe { ::core::str::from_utf8_unchecked(&bytes[local_part_length..index]) };

                                local_part_length = index + 1;

                                if local_part_length == length {
                                    // end
                                    return Err(#error_path::Invalid);
                                }

                                Some(comment)
                            }
                            None => return Err(#error_path::Invalid),
                        }
                    } else {
                        return Err(#error_path::Invalid);
                    }
                }
            };

            let handle_comment_3 = if type_attribute.comment.disallow() {
                quote! {
                    return Err(#error_path::CommentDisallow);
                }
            } else {
                quote! {
                    if length > 1 {
                        // only take 254 - 1, 1 is reserved for the meaningful domain part.
                        match bytes[1..].iter().take(253).copied().position(|e| e == b')') {
                            Some(mut index) => {
                                index += 1;
                                let comment = unsafe { ::core::str::from_utf8_unchecked(&bytes[1..index]) };

                                let domain_part_length = index + 1;

                                if domain_part_length == length {
                                    // end
                                    return Err(#error_path::Invalid);
                                }

                                (domain_part_length, Some(comment))
                            }
                            None => return Err(#error_path::Invalid),
                        }
                    } else {
                        return Err(#error_path::Invalid);
                    }
                }
            };

            let handle_comment_4 = if type_attribute.comment.disallow() {
                quote! {
                    return Err(#error_path::CommentDisallow);
                }
            } else {
                quote! {
                    if bytes[domain_part_length] == b'(' {
                        domain_part_length += 1;

                        if length > domain_part_length {
                            match bytes[domain_part_length..].iter().copied().position(|e| e == b')') {
                                Some(mut index) => {
                                    index += domain_part_length;
                                    let comment =
                                        unsafe { ::core::str::from_utf8_unchecked(&bytes[domain_part_length..index]) };

                                    Some(comment)
                                }
                                None => return Err(#error_path::Invalid),
                            }
                        } else {
                            return Err(#error_path::Invalid);
                        }
                    } else {
                        return Err(#error_path::Invalid);
                    }
                }
            };

            token_stream.extend(quote! {
                impl #name {
                    #conflict_meta
                    fn v_parse_str(s: &str) -> Result<(validators_prelude::String, bool, validators_prelude::Host, Option<validators_prelude::String>, Option<validators_prelude::String>, Option<validators_prelude::String>, Option<validators_prelude::String>, bool), #error_path> {
                        let bytes = s.as_bytes();
                        let length = bytes.len();

                        if length == 0 || length > 320 {
                            return Err(#error_path::Invalid);
                        }

                        // comment 1
                        let (mut local_part_length, comment_before_local_part) = if bytes[0] == b'(' {
                            #handle_comment_1
                        } else {
                            (0, None::<&str>)
                        };

                        let (local_part, need_quoted, quoted) = if bytes[local_part_length] == b'"' {
                            // quoted
                            let mut p = local_part_length + 1;

                            if p == 63 || p == length {
                                // too long
                                return Err(#error_path::Invalid);
                            }

                            let e = bytes[p];

                            let (mut escaping, mut need_quoted, mut last_dot) = match e {
                                b'A'..=b'Z'
                                | b'a'..=b'z'
                                | b'0'..=b'9'
                                | b'!'
                                | b'#'..=b'\''
                                | b'*'
                                | b'+'
                                | b'-'
                                | b'/'
                                | b'='
                                | b'?'
                                | b'^'..=b'`'
                                | b'{'..=b'~' => (false, false, false),
                                b'.' => (false, true, true),
                                b'\\' => (true, true, false),
                                b' ' | b'\t' => (false, true, false),
                                _ => {
                                    #handle_unrecognized_first_byte_quoted
                                },
                            };

                            p += 1;

                            loop {
                                if p == length {
                                    // not found '"', so returns Err.
                                    return Err(#error_path::Invalid);
                                }

                                let e = bytes[p];

                                match e {
                                    b'A'..=b'Z'
                                    | b'a'..=b'z'
                                    | b'0'..=b'9'
                                    | b'!'
                                    | b'#'..=b'\''
                                    | b'*'
                                    | b'+'
                                    | b'-'
                                    | b'/'
                                    | b'='
                                    | b'?'
                                    | b'^'..=b'`'
                                    | b'{'..=b'~' => {
                                        if p == 63 {
                                            // too long
                                            return Err(#error_path::Invalid);
                                        }

                                        last_dot = false;
                                        escaping = false;
                                    }
                                    b'.' => {
                                        if p == 63 {
                                            // too long
                                            return Err(#error_path::Invalid);
                                        }

                                        if last_dot {
                                            need_quoted = true;
                                        }

                                        last_dot = true;
                                        escaping = false;
                                    }
                                    b'\\' => {
                                        if p == 63 {
                                            // too long
                                            return Err(#error_path::Invalid);
                                        }

                                        need_quoted = true;
                                        escaping = !escaping;
                                    }
                                    b' ' | b'\t' => {
                                        if p == 63 {
                                            // too long
                                            return Err(#error_path::Invalid);
                                        }

                                        need_quoted = true;
                                    }
                                    b'(' | b')' | b',' | b':'..=b'<' | b'>' | b'@' | b'[' | b']' => {
                                        if p == 63 || !escaping {
                                            // too long or not in escaping
                                            return Err(#error_path::Invalid);
                                        }

                                        escaping = false;
                                    }
                                    b'"' => {
                                        if escaping {
                                            if p == 63 {
                                                // too long or not in escaping
                                                return Err(#error_path::Invalid);
                                            }

                                            escaping = false;
                                        } else {
                                            let local_part =
                                                unsafe { ::core::str::from_utf8_unchecked(&bytes[(local_part_length + 1)..p]) };

                                            local_part_length = p + 1;

                                            if local_part_length == length {
                                                // end
                                                return Err(#error_path::Invalid);
                                            }

                                            break (local_part, need_quoted, true);
                                        }
                                    }
                                    _ => {
                                        #handle_unrecognized_byte_quoted
                                    },
                                }

                                p += 1;
                            }
                        } else {
                            let e = bytes[local_part_length];

                            // unquoted
                            match e {
                                b'A'..=b'Z'
                                | b'a'..=b'z'
                                | b'0'..=b'9'
                                | b'!'
                                | b'#'..=b'\''
                                | b'*'
                                | b'+'
                                | b'-'
                                | b'/'
                                | b'='
                                | b'?'
                                | b'^'..=b'`'
                                | b'{'..=b'~' => {
                                    // '.' is not allowed as the first character
                                }
                                _ => {
                                    #handle_unrecognized_first_byte_unquoted
                                },
                            }

                            let mut p = local_part_length + 1;
                            let mut last_dot = false;

                            loop {
                                if p == length {
                                    // not found '@' or '(', so returns Err.
                                    return Err(#error_path::Invalid);
                                }

                                let e = bytes[p];

                                match e {
                                    b'A'..=b'Z'
                                    | b'a'..=b'z'
                                    | b'0'..=b'9'
                                    | b'!'
                                    | b'#'..=b'\''
                                    | b'*'
                                    | b'+'
                                    | b'-'
                                    | b'/'
                                    | b'='
                                    | b'?'
                                    | b'^'..=b'`'
                                    | b'{'..=b'~' => {
                                        if p == 64 {
                                            // too long
                                            return Err(#error_path::Invalid);
                                        }

                                        last_dot = false;
                                    }
                                    b'.' => {
                                        if p == 64 || last_dot {
                                            // too long or '.' appears consecutively
                                            return Err(#error_path::Invalid);
                                        }

                                        last_dot = true;
                                    }
                                    b'@' | b'(' => {
                                        let local_part =
                                            unsafe { ::core::str::from_utf8_unchecked(&bytes[local_part_length..p]) };

                                        local_part_length = p;

                                        break (local_part, false, false);
                                    }
                                    _ => {
                                        #handle_unrecognized_byte_unquoted
                                    },
                                }

                                p += 1;
                            }
                        };

                        // comment 2
                        let comment_after_local_part = if bytes[local_part_length] == b'(' {
                            #handle_comment_2
                        } else {
                            None::<&str>
                        };

                        if bytes[local_part_length] != b'@' || local_part_length + 1 == length {
                            return Err(#error_path::Invalid);
                        }

                        let bytes = &bytes[(local_part_length + 1)..];
                        let length = bytes.len();
                        if length > 255 {
                            return Err(#error_path::Invalid);
                        }

                        // comment 3
                        let (mut domain_part_length, comment_before_domain_part) = if bytes[0] == b'(' {
                            #handle_comment_3
                        } else {
                            (0, None::<&str>)
                        };

                        let mut domain_part_length_offset: isize = if quoted && need_quoted {
                            -2
                        } else {
                            0
                        };

                        let (host, is_local): (validators_prelude::Host, bool) = match bytes[domain_part_length] {
                            b'(' => return Err(#error_path::Invalid),
                            b'[' => {
                                #handle_ip
                            }
                            _ => {
                                #handle_domain
                            }
                        };

                        #check_local

                        if domain_part_length_offset > 0 && length + domain_part_length_offset as usize > 255 {
                            return Err(#error_path::Invalid);
                        }

                        // comment 4
                        let comment_after_domain_part = if length > domain_part_length {
                            #handle_comment_4
                        } else {
                            None::<&str>
                        };

                        return Ok((
                            validators_prelude::String::from(local_part),
                            need_quoted,
                            host,
                            comment_before_local_part.map(validators_prelude::String::from),
                            comment_after_local_part.map(validators_prelude::String::from),
                            comment_before_domain_part.map(validators_prelude::String::from),
                            comment_after_domain_part.map(validators_prelude::String::from),
                            is_local,
                        ));
                    }
                }
            });

            let create_instance = {
                if type_attribute.comment.allow() {
                    match type_attribute.ip {
                        TriAllow::Allow => {
                            if type_attribute.local == TriAllow::Allow
                                && type_attribute.at_least_two_labels != TriAllow::Allow
                            {
                                quote! {
                                    Self {
                                        local_part,
                                        need_quoted,
                                        domain_part,
                                        comment_before_local_part: _comment_before_local_part,
                                        comment_after_local_part: _comment_after_local_part,
                                        comment_before_domain_part: _comment_before_domain_part,
                                        comment_after_domain_part: _comment_after_domain_part,
                                        is_local: _is_local,
                                    }
                                }
                            } else {
                                quote! {
                                    Self {
                                        local_part,
                                        need_quoted,
                                        domain_part,
                                        comment_before_local_part: _comment_before_local_part,
                                        comment_after_local_part: _comment_after_local_part,
                                        comment_before_domain_part: _comment_before_domain_part,
                                        comment_after_domain_part: _comment_after_domain_part,
                                    }
                                }
                            }
                        },
                        TriAllow::Must => {
                            if type_attribute.local == TriAllow::Allow
                                && type_attribute.at_least_two_labels != TriAllow::Allow
                            {
                                quote! {
                                    Self {
                                        local_part,
                                        need_quoted,
                                        domain_part: {
                                            match domain_part {
                                                validators_prelude::Host::IPv4(ip) => ::std::net::IpAddr::V4(ip),
                                                validators_prelude::Host::IPv6(ip) => ::std::net::IpAddr::V6(ip),
                                                validators_prelude::Host::Domain(_) => unreachable!(),
                                            }
                                        },
                                        comment_before_local_part: _comment_before_local_part,
                                        comment_after_local_part: _comment_after_local_part,
                                        comment_before_domain_part: _comment_before_domain_part,
                                        comment_after_domain_part: _comment_after_domain_part,
                                        is_local: _is_local,
                                    }
                                }
                            } else {
                                quote! {
                                    Self {
                                        local_part,
                                        need_quoted,
                                        domain_part: {
                                            match domain_part {
                                                validators_prelude::Host::IPv4(ip) => ::std::net::IpAddr::V4(ip),
                                                validators_prelude::Host::IPv6(ip) => ::std::net::IpAddr::V6(ip),
                                                _ => unreachable!(),
                                            }
                                        },
                                        comment_before_local_part: _comment_before_local_part,
                                        comment_after_local_part: _comment_after_local_part,
                                        comment_before_domain_part: _comment_before_domain_part,
                                        comment_after_domain_part: _comment_after_domain_part,
                                    }
                                }
                            }
                        },
                        TriAllow::Disallow => {
                            if type_attribute.local == TriAllow::Allow
                                && type_attribute.at_least_two_labels != TriAllow::Allow
                            {
                                quote! {
                                    Self {
                                        local_part,
                                        need_quoted,
                                        domain_part: {
                                            match domain_part {
                                                validators_prelude::Host::Domain(domain) => domain,
                                                _ => unreachable!(),
                                            }
                                        },
                                        comment_before_local_part: _comment_before_local_part,
                                        comment_after_local_part: _comment_after_local_part,
                                        comment_before_domain_part: _comment_before_domain_part,
                                        comment_after_domain_part: _comment_after_domain_part,
                                        is_local: _is_local,
                                    }
                                }
                            } else {
                                quote! {
                                    Self {
                                        local_part,
                                        need_quoted,
                                        domain_part: {
                                            match domain_part {
                                                validators_prelude::Host::Domain(domain) => domain,
                                                _ => unreachable!(),
                                            }
                                        },
                                        comment_before_local_part: _comment_before_local_part,
                                        comment_after_local_part: _comment_after_local_part,
                                        comment_before_domain_part: _comment_before_domain_part,
                                        comment_after_domain_part: _comment_after_domain_part,
                                    }
                                }
                            }
                        },
                    }
                } else {
                    match type_attribute.ip {
                        TriAllow::Allow => {
                            if type_attribute.local == TriAllow::Allow
                                && type_attribute.at_least_two_labels != TriAllow::Allow
                            {
                                quote! {
                                    Self {
                                        local_part,
                                        need_quoted,
                                        domain_part,
                                        is_local: _is_local,
                                    }
                                }
                            } else {
                                quote! {
                                    Self {
                                        local_part,
                                        need_quoted,
                                        domain_part,
                                    }
                                }
                            }
                        },
                        TriAllow::Must => {
                            if type_attribute.local == TriAllow::Allow
                                && type_attribute.at_least_two_labels != TriAllow::Allow
                            {
                                quote! {
                                    Self {
                                        local_part,
                                        need_quoted,
                                        domain_part: {
                                            match domain_part {
                                                validators_prelude::Host::IPv4(ip) => ::std::net::IpAddr::V4(ip),
                                                validators_prelude::Host::IPv6(ip) => ::std::net::IpAddr::V6(ip),
                                                validators_prelude::Host::Domain(_) => unreachable!(),
                                            }
                                        },
                                        is_local: _is_local,
                                    }
                                }
                            } else {
                                quote! {
                                    Self {
                                        local_part,
                                        need_quoted,
                                        domain_part: {
                                            match domain_part {
                                                validators_prelude::Host::IPv4(ip) => ::std::net::IpAddr::V4(ip),
                                                validators_prelude::Host::IPv6(ip) => ::std::net::IpAddr::V6(ip),
                                                _ => unreachable!(),
                                            }
                                        },
                                    }
                                }
                            }
                        },
                        TriAllow::Disallow => {
                            if type_attribute.local == TriAllow::Allow
                                && type_attribute.at_least_two_labels != TriAllow::Allow
                            {
                                quote! {
                                    Self {
                                        local_part,
                                        need_quoted,
                                        domain_part: {
                                            match domain_part {
                                                validators_prelude::Host::Domain(domain) => domain,
                                                _ => unreachable!(),
                                            }
                                        },
                                        is_local: _is_local,
                                    }
                                }
                            } else {
                                quote! {
                                    Self {
                                        local_part,
                                        need_quoted,
                                        domain_part: {
                                            match domain_part {
                                                validators_prelude::Host::Domain(domain) => domain,
                                                _ => unreachable!(),
                                            }
                                        },
                                    }
                                }
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
                        let (local_part, need_quoted, domain_part, _comment_before_local_part, _comment_after_local_part, _comment_before_domain_part, _comment_after_domain_part, _is_local) = Self::v_parse_str(s.into().as_str())?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> Result<Self, Self::Error> {
                        let (local_part, need_quoted, domain_part, _comment_before_local_part, _comment_after_local_part, _comment_before_domain_part, _comment_after_domain_part, _is_local) = Self::v_parse_str(s.as_ref())?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error> {
                        Self::v_parse_str(s.as_ref())?;

                        Ok(())
                    }
                }
            });

            token_stream.extend(if type_attribute.comment.disallow() {
                match type_attribute.ip {
                    TriAllow::Allow => {
                        quote! {
                            impl ToEmailString for #name {
                                #[inline]
                                fn to_email_string(&self) -> validators_prelude::String {
                                    let local_part = &self.local_part;

                                    match &self.domain_part {
                                        validators_prelude::Host::IPv4(ip) => validators_prelude::format!("{}@[{}]", local_part, ip),
                                        validators_prelude::Host::IPv6(ip) => validators_prelude::format!("{}@[IPv6:{}]", local_part, ip),
                                        validators_prelude::Host::Domain(domain) => validators_prelude::format!("{}@{}", local_part, domain),
                                    }
                                }
                            }
                        }
                    },
                    TriAllow::Must => {
                        quote! {
                            impl ToEmailString for #name {
                                #[inline]
                                fn to_email_string(&self) -> validators_prelude::String {
                                    let local_part = &self.local_part;

                                    match &self.domain_part {
                                        ::std::net::IpAddr::V4(ip) => validators_prelude::format!("{}@[{}]", local_part, ip),
                                        ::std::net::IpAddr::V6(ip) => validators_prelude::format!("{}@[IPv6:{}]", local_part, ip),
                                    }
                                }
                            }
                        }
                    },
                    TriAllow::Disallow => {
                        quote! {
                            impl ToEmailString for #name {
                                #[inline]
                                fn to_email_string(&self) -> validators_prelude::String {
                                    let local_part = &self.local_part;
                                    let domain_part = &self.domain_part;

                                    validators_prelude::format!("{}@{}", local_part, domain_part)
                                }
                            }
                        }
                    },
                }
            } else {
                let generate_s = quote! {
                    let mut s = validators_prelude::String::with_capacity(64);

                    if let Some(comment) = &self.comment_before_local_part {
                        s.push('(');
                        s.push_str(comment);
                        s.push(')');
                    }

                    if self.need_quoted {
                        s.push('"');
                    }

                    s.push_str(self.local_part.as_str());

                    if self.need_quoted {
                        s.push('"');
                    }

                    if let Some(comment) = &self.comment_after_local_part {
                        s.push('(');
                        s.push_str(comment);
                        s.push(')');
                    }

                    s.push('@');

                    if let Some(comment) = &self.comment_before_domain_part {
                        s.push('(');
                        s.push_str(comment);
                        s.push(')');
                    }
                };

                let finish_s = quote! {
                    if let Some(comment) = &self.comment_after_domain_part {
                        s.push('(');
                        s.push_str(comment);
                        s.push(')');
                    }
                };

                match type_attribute.ip {
                    TriAllow::Allow => {
                        quote! {
                            impl ToEmailString for #name {
                                #[inline]
                                fn to_email_string(&self) -> validators_prelude::String {
                                    #generate_s

                                    use ::core::fmt::Write;

                                    match &self.domain_part {
                                        validators_prelude::Host::IPv4(ip) => s.write_fmt(format_args!("[{ip}]")).unwrap(),
                                        validators_prelude::Host::IPv6(ip) => s.write_fmt(format_args!("[IPv6:{ip}]")).unwrap(),
                                        validators_prelude::Host::Domain(domain) => s.push_str(domain),
                                    }

                                    #finish_s

                                    s
                                }
                            }
                        }
                    },
                    TriAllow::Must => {
                        quote! {
                            impl ToEmailString for #name {
                                #[inline]
                                fn to_email_string(&self) -> validators_prelude::String {
                                    #generate_s

                                    use ::core::fmt::Write;

                                    match &self.domain_part {
                                        ::std::net::IpAddr::V4(ip) => s.write_fmt(format_args!("[{ip}]")).unwrap(),
                                        ::std::net::IpAddr::V6(ip) => s.write_fmt(format_args!("[IPv6:{ip}]")).unwrap(),
                                    }

                                    #finish_s

                                    s
                                }
                            }
                        }
                    },
                    TriAllow::Disallow => {
                        quote! {
                            impl ToEmailString for #name {
                                #[inline]
                                fn to_email_string(&self) -> validators_prelude::String {
                                    #generate_s

                                    s.push_str(self.domain_part.as_str());

                                    #finish_s

                                    s
                                }
                            }
                        }
                    },
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
                                serializer.serialize_str(&ToEmailString::to_email_string(self))
                            }
                        }
                    });
                }

                if type_attribute.serde_options.deserialize {
                    let expect = {
                        let mut s = if type_attribute.non_ascii.disallow() {
                            String::from("an ASCII email")
                        } else {
                            String::from("an email")
                        };

                        match type_attribute.ip {
                            TriAllow::Allow => match type_attribute.at_least_two_labels {
                                TriAllow::Allow => (),
                                TriAllow::Must => {
                                    s.push_str(" whose domain part must be at-least-two-labels");
                                },
                                TriAllow::Disallow => {
                                    s.push_str(" whose domain part must be one-label");
                                },
                            },
                            TriAllow::Must => {
                                s.push_str(" whose domain part must be an IP");
                            },
                            TriAllow::Disallow => {
                                s.push_str(" whose domain part must be non-IP");

                                match type_attribute.at_least_two_labels {
                                    TriAllow::Allow => (),
                                    TriAllow::Must => {
                                        s.push_str(" and at-least-two-labels");
                                    },
                                    TriAllow::Disallow => {
                                        s.push_str(" and one-label");
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

        Err(panic::validator_for_specific_item(
            meta.path().get_ident().unwrap(),
            ITEM_ALLOW_COMMENT,
        ))
    }
}
