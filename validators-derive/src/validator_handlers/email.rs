use alloc::boxed::Box;
use alloc::string::{String, ToString};

use crate::proc_macro::TokenStream;
use crate::quote::ToTokens;
use crate::syn::{Data, DeriveInput, Fields, Meta, NestedMeta, Path};

use crate::{panic, SynOption, TypeEnum, Validator, ValidatorOption};

#[derive(Debug)]
pub struct Struct {
    local_part: TypeEnum,
    need_quoted: TypeEnum,
    domain_part: TypeEnum,
}

#[derive(Debug)]
pub struct StructAllowComment {
    local_part: TypeEnum,
    need_quoted: TypeEnum,
    domain_part: TypeEnum,
    comment_before_local_part: TypeEnum,
    comment_after_local_part: TypeEnum,
    comment_before_domain_part: TypeEnum,
    comment_after_domain_part: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowLocal {
    local_part: TypeEnum,
    need_quoted: TypeEnum,
    domain_part: TypeEnum,
    is_local: TypeEnum,
}

#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowCommentAllowLocal {
    local_part: TypeEnum,
    need_quoted: TypeEnum,
    domain_part: TypeEnum,
    comment_before_local_part: TypeEnum,
    comment_after_local_part: TypeEnum,
    comment_before_domain_part: TypeEnum,
    comment_after_domain_part: TypeEnum,
    is_local: TypeEnum,
}

const ITEM: Struct = Struct {
    local_part: TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::String,
};

const ITEM_ALLOW_IP: Struct = Struct {
    local_part: TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::Host,
};

const ITEM_MUST_IP: Struct = Struct {
    local_part: TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::IpAddr,
};

const ITEM_ALLOW_COMMENT: StructAllowComment = StructAllowComment {
    local_part: TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::String,
    comment_before_local_part: TypeEnum::OptionString,
    comment_after_local_part: TypeEnum::OptionString,
    comment_before_domain_part: TypeEnum::OptionString,
    comment_after_domain_part: TypeEnum::OptionString,
};

const ITEM_ALLOW_COMMENT_ALLOW_IP: StructAllowComment = StructAllowComment {
    local_part: TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::Host,
    comment_before_local_part: TypeEnum::OptionString,
    comment_after_local_part: TypeEnum::OptionString,
    comment_before_domain_part: TypeEnum::OptionString,
    comment_after_domain_part: TypeEnum::OptionString,
};

const ITEM_ALLOW_COMMENT_MUST_IP: StructAllowComment = StructAllowComment {
    local_part: TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::IpAddr,
    comment_before_local_part: TypeEnum::OptionString,
    comment_after_local_part: TypeEnum::OptionString,
    comment_before_domain_part: TypeEnum::OptionString,
    comment_after_domain_part: TypeEnum::OptionString,
};

const ITEM_ALLOW_LOCAL: StructAllowLocal = StructAllowLocal {
    local_part: TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::String,
    is_local: TypeEnum::Boolean,
};

const ITEM_ALLOW_LOCAL_ALLOW_IP: StructAllowLocal = StructAllowLocal {
    local_part: TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::Host,
    is_local: TypeEnum::Boolean,
};

const ITEM_ALLOW_LOCAL_MUST_IP: StructAllowLocal = StructAllowLocal {
    local_part: TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::IpAddr,
    is_local: TypeEnum::Boolean,
};

const ITEM_ALLOW_COMMENT_ALLOW_LOCAL: StructAllowCommentAllowLocal = StructAllowCommentAllowLocal {
    local_part: TypeEnum::String,
    need_quoted: TypeEnum::Boolean,
    domain_part: TypeEnum::String,
    comment_before_local_part: TypeEnum::OptionString,
    comment_after_local_part: TypeEnum::OptionString,
    comment_before_domain_part: TypeEnum::OptionString,
    comment_after_domain_part: TypeEnum::OptionString,
    is_local: TypeEnum::Boolean,
};

const ITEM_ALLOW_COMMENT_ALLOW_LOCAL_ALLOW_IP: StructAllowCommentAllowLocal =
    StructAllowCommentAllowLocal {
        local_part: TypeEnum::String,
        need_quoted: TypeEnum::Boolean,
        domain_part: TypeEnum::Host,
        comment_before_local_part: TypeEnum::OptionString,
        comment_after_local_part: TypeEnum::OptionString,
        comment_before_domain_part: TypeEnum::OptionString,
        comment_after_domain_part: TypeEnum::OptionString,
        is_local: TypeEnum::Boolean,
    };

const ITEM_ALLOW_COMMENT_ALLOW_LOCAL_MUST_IP: StructAllowCommentAllowLocal =
    StructAllowCommentAllowLocal {
        local_part: TypeEnum::String,
        need_quoted: TypeEnum::Boolean,
        domain_part: TypeEnum::IpAddr,
        comment_before_local_part: TypeEnum::OptionString,
        comment_after_local_part: TypeEnum::OptionString,
        comment_before_domain_part: TypeEnum::OptionString,
        comment_after_domain_part: TypeEnum::OptionString,
        is_local: TypeEnum::Boolean,
    };

const VALIDATOR: Validator = Validator::email;

pub fn email_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            let mut comment = ValidatorOption::new();
            let mut ip = ValidatorOption::new();
            let mut local = ValidatorOption::new();
            let mut at_least_two_labels = ValidatorOption::new();
            let mut non_ascii = ValidatorOption::new();
            let mut conflict = ValidatorOption::NotAllow;

            let correct_usage_for_attribute = [stringify!(#[validator(email)])];

            let correct_usage_for_comment = [
                stringify!(#[validator(email(comment(Allow)))]),
                stringify!(#[validator(email(comment(NotAllow)))]),
            ];

            let correct_usage_for_ip = [
                stringify!(#[validator(email(ip(Must)))]),
                stringify!(#[validator(email(ip(Allow)))]),
                stringify!(#[validator(email(ip(NotAllow)))]),
            ];

            let correct_usage_for_local = [
                stringify!(#[validator(email(local(Must)))]),
                stringify!(#[validator(email(local(Allow)))]),
                stringify!(#[validator(email(local(NotAllow)))]),
            ];

            let correct_usage_for_at_least_two_labels = [
                stringify!(#[validator(email(at_least_two_labels(Must)))]),
                stringify!(#[validator(email(at_least_two_labels(Allow)))]),
                stringify!(#[validator(email(at_least_two_labels(NotAllow)))]),
            ];

            let correct_usage_for_non_ascii = [
                stringify!(#[validator(email(non_ascii(Allow)))]),
                stringify!(#[validator(email(non_ascii(NotAllow)))]),
            ];

            let correct_usage_for_conflict = [
                stringify!(#[validator(domain(conflict(Allow)))]),
                stringify!(#[validator(domain(conflict(NotAllow)))]),
            ];

            match meta {
                Meta::Path(_) => (),
                Meta::List(list) => {
                    let mut comment_is_set = false;
                    let mut ip_is_set = false;
                    let mut local_is_set = false;
                    let mut at_least_two_labels_is_set = false;
                    let mut non_ascii_is_set = false;
                    let mut conflict_is_set = false;

                    for p in list.nested.iter() {
                        match p {
                            NestedMeta::Meta(meta) => {
                                let meta_name = meta.path().into_token_stream().to_string();

                                match meta_name.as_str() {
                                    "comment" => {
                                        comment = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut comment_is_set,
                                            &correct_usage_for_comment,
                                        );
                                    }
                                    "ip" => {
                                        ip = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut ip_is_set,
                                            &correct_usage_for_ip,
                                        );
                                    }
                                    "local" => {
                                        local = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut local_is_set,
                                            &correct_usage_for_local,
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
                                    "non_ascii" => {
                                        non_ascii = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut non_ascii_is_set,
                                            &correct_usage_for_non_ascii,
                                        );

                                        if non_ascii == ValidatorOption::Must {
                                            panic::parameter_incorrect_format(
                                                meta_name.as_str(),
                                                &correct_usage_for_non_ascii,
                                            );
                                        }
                                    }
                                    "conflict" => {
                                        conflict = ValidatorOption::from_meta(
                                            meta_name.as_str(),
                                            meta,
                                            &mut conflict_is_set,
                                            &correct_usage_for_conflict,
                                        );

                                        if conflict == ValidatorOption::Must {
                                            panic::parameter_incorrect_format(
                                                meta_name.as_str(),
                                                &correct_usage_for_conflict,
                                            );
                                        }
                                    }
                                    _ => panic::unknown_parameter("email", meta_name.as_str()),
                                }
                            }
                            NestedMeta::Lit(_) => {
                                panic::attribute_incorrect_format(
                                    "email",
                                    &correct_usage_for_attribute,
                                )
                            }
                        }
                    }
                }
                Meta::NameValue(_) => {
                    panic::attribute_incorrect_format("email", &correct_usage_for_attribute)
                }
            }

            let mut meta_is_conflict = false;

            if ip.must() && at_least_two_labels.not_allow() {
                if conflict.not_allow() {
                    panic!(
                        "`ip(Must)` and `at_least_two_labels(NotAllow)` cannot be used together."
                    );
                }

                meta_is_conflict = true;
            }

            if comment.allow() {
                match ip {
                    ValidatorOption::Allow => {
                        if local == ValidatorOption::Allow
                            && at_least_two_labels != ValidatorOption::Allow
                        {
                            if let Fields::Named(_) = &data.fields {
                                if data.fields.len() != 8 {
                                    panic::validator_only_support_for_item(
                                        VALIDATOR,
                                        Box::new(ITEM_ALLOW_COMMENT_ALLOW_LOCAL_ALLOW_IP),
                                    );
                                }

                                for field in data.fields.iter() {
                                    let ident = field.ident.as_ref().unwrap();

                                    if ident != "local_part"
                                        && ident != "need_quoted"
                                        && ident != "domain_part"
                                        && ident != "comment_before_local_part"
                                        && ident != "comment_after_local_part"
                                        && ident != "comment_before_domain_part"
                                        && ident != "comment_after_domain_part"
                                        && ident != "is_local"
                                    {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_COMMENT_ALLOW_LOCAL_ALLOW_IP),
                                        );
                                    }
                                }
                            } else {
                                panic::validator_only_support_for_item(
                                    VALIDATOR,
                                    Box::new(ITEM_ALLOW_COMMENT_ALLOW_LOCAL_ALLOW_IP),
                                );
                            }
                        } else if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 7 {
                                panic::validator_only_support_for_item(
                                    VALIDATOR,
                                    Box::new(ITEM_ALLOW_COMMENT_ALLOW_IP),
                                );
                            }

                            for field in data.fields.iter() {
                                let ident = field.ident.as_ref().unwrap();

                                if ident != "local_part"
                                    && ident != "need_quoted"
                                    && ident != "domain_part"
                                    && ident != "comment_before_local_part"
                                    && ident != "comment_after_local_part"
                                    && ident != "comment_before_domain_part"
                                    && ident != "comment_after_domain_part"
                                {
                                    panic::validator_only_support_for_item(
                                        VALIDATOR,
                                        Box::new(ITEM_ALLOW_COMMENT_ALLOW_IP),
                                    );
                                }
                            }
                        } else {
                            panic::validator_only_support_for_item(
                                VALIDATOR,
                                Box::new(ITEM_ALLOW_COMMENT_ALLOW_IP),
                            );
                        }
                    }
                    ValidatorOption::Must => {
                        if local == ValidatorOption::Allow
                            && at_least_two_labels != ValidatorOption::Allow
                        {
                            if let Fields::Named(_) = &data.fields {
                                if data.fields.len() != 8 {
                                    panic::validator_only_support_for_item(
                                        VALIDATOR,
                                        Box::new(ITEM_ALLOW_COMMENT_ALLOW_LOCAL_MUST_IP),
                                    );
                                }

                                for field in data.fields.iter() {
                                    let ident = field.ident.as_ref().unwrap();

                                    if ident != "local_part"
                                        && ident != "need_quoted"
                                        && ident != "domain_part"
                                        && ident != "comment_before_local_part"
                                        && ident != "comment_after_local_part"
                                        && ident != "comment_before_domain_part"
                                        && ident != "comment_after_domain_part"
                                        && ident != "is_local"
                                    {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_COMMENT_ALLOW_LOCAL_MUST_IP),
                                        );
                                    }
                                }
                            } else {
                                panic::validator_only_support_for_item(
                                    VALIDATOR,
                                    Box::new(ITEM_ALLOW_COMMENT_ALLOW_LOCAL_MUST_IP),
                                );
                            }
                        } else if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 7 {
                                panic::validator_only_support_for_item(
                                    VALIDATOR,
                                    Box::new(ITEM_ALLOW_COMMENT_MUST_IP),
                                );
                            }

                            for field in data.fields.iter() {
                                let ident = field.ident.as_ref().unwrap();

                                if ident != "local_part"
                                    && ident != "need_quoted"
                                    && ident != "domain_part"
                                    && ident != "comment_before_local_part"
                                    && ident != "comment_after_local_part"
                                    && ident != "comment_before_domain_part"
                                    && ident != "comment_after_domain_part"
                                {
                                    panic::validator_only_support_for_item(
                                        VALIDATOR,
                                        Box::new(ITEM_ALLOW_COMMENT_MUST_IP),
                                    );
                                }
                            }
                        } else {
                            panic::validator_only_support_for_item(
                                VALIDATOR,
                                Box::new(ITEM_ALLOW_COMMENT_MUST_IP),
                            );
                        }
                    }
                    ValidatorOption::NotAllow => {
                        if local == ValidatorOption::Allow
                            && at_least_two_labels != ValidatorOption::Allow
                        {
                            if let Fields::Named(_) = &data.fields {
                                if data.fields.len() != 8 {
                                    panic::validator_only_support_for_item(
                                        VALIDATOR,
                                        Box::new(ITEM_ALLOW_COMMENT_ALLOW_LOCAL),
                                    );
                                }

                                for field in data.fields.iter() {
                                    let ident = field.ident.as_ref().unwrap();

                                    if ident != "local_part"
                                        && ident != "need_quoted"
                                        && ident != "domain_part"
                                        && ident != "comment_before_local_part"
                                        && ident != "comment_after_local_part"
                                        && ident != "comment_before_domain_part"
                                        && ident != "comment_after_domain_part"
                                        && ident != "is_local"
                                    {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_COMMENT_ALLOW_LOCAL),
                                        );
                                    }
                                }
                            } else {
                                panic::validator_only_support_for_item(
                                    VALIDATOR,
                                    Box::new(ITEM_ALLOW_COMMENT_ALLOW_LOCAL),
                                );
                            }
                        } else if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 7 {
                                panic::validator_only_support_for_item(
                                    VALIDATOR,
                                    Box::new(ITEM_ALLOW_COMMENT),
                                );
                            }

                            for field in data.fields.iter() {
                                let ident = field.ident.as_ref().unwrap();

                                if ident != "local_part"
                                    && ident != "need_quoted"
                                    && ident != "domain_part"
                                    && ident != "comment_before_local_part"
                                    && ident != "comment_after_local_part"
                                    && ident != "comment_before_domain_part"
                                    && ident != "comment_after_domain_part"
                                {
                                    panic::validator_only_support_for_item(
                                        VALIDATOR,
                                        Box::new(ITEM_ALLOW_COMMENT),
                                    );
                                }
                            }
                        } else {
                            panic::validator_only_support_for_item(
                                VALIDATOR,
                                Box::new(ITEM_ALLOW_COMMENT),
                            );
                        }
                    }
                }
            } else {
                match ip {
                    ValidatorOption::Allow => {
                        if local == ValidatorOption::Allow
                            && at_least_two_labels != ValidatorOption::Allow
                        {
                            if let Fields::Named(_) = &data.fields {
                                if data.fields.len() != 4 {
                                    panic::validator_only_support_for_item(
                                        VALIDATOR,
                                        Box::new(ITEM_ALLOW_LOCAL_ALLOW_IP),
                                    );
                                }

                                for field in data.fields.iter() {
                                    let ident = field.ident.as_ref().unwrap();

                                    if ident != "local_part"
                                        && ident != "need_quoted"
                                        && ident != "domain_part"
                                        && ident != "is_local"
                                    {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_LOCAL_ALLOW_IP),
                                        );
                                    }
                                }
                            } else {
                                panic::validator_only_support_for_item(
                                    VALIDATOR,
                                    Box::new(ITEM_ALLOW_LOCAL_ALLOW_IP),
                                );
                            }
                        } else if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 3 {
                                panic::validator_only_support_for_item(
                                    VALIDATOR,
                                    Box::new(ITEM_ALLOW_IP),
                                );
                            }

                            for field in data.fields.iter() {
                                let ident = field.ident.as_ref().unwrap();

                                if ident != "local_part"
                                    && ident != "need_quoted"
                                    && ident != "domain_part"
                                {
                                    panic::validator_only_support_for_item(
                                        VALIDATOR,
                                        Box::new(ITEM_ALLOW_IP),
                                    );
                                }
                            }
                        } else {
                            panic::validator_only_support_for_item(
                                VALIDATOR,
                                Box::new(ITEM_ALLOW_IP),
                            );
                        }
                    }
                    ValidatorOption::Must => {
                        if local == ValidatorOption::Allow
                            && at_least_two_labels != ValidatorOption::Allow
                        {
                            if let Fields::Named(_) = &data.fields {
                                if data.fields.len() != 4 {
                                    panic::validator_only_support_for_item(
                                        VALIDATOR,
                                        Box::new(ITEM_ALLOW_LOCAL_MUST_IP),
                                    );
                                }

                                for field in data.fields.iter() {
                                    let ident = field.ident.as_ref().unwrap();

                                    if ident != "local_part"
                                        && ident != "need_quoted"
                                        && ident != "domain_part"
                                        && ident != "is_local"
                                    {
                                        panic::validator_only_support_for_item(
                                            VALIDATOR,
                                            Box::new(ITEM_ALLOW_LOCAL_MUST_IP),
                                        );
                                    }
                                }
                            } else {
                                panic::validator_only_support_for_item(
                                    VALIDATOR,
                                    Box::new(ITEM_ALLOW_LOCAL_MUST_IP),
                                );
                            }
                        } else if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 3 {
                                panic::validator_only_support_for_item(
                                    VALIDATOR,
                                    Box::new(ITEM_MUST_IP),
                                );
                            }

                            for field in data.fields.iter() {
                                let ident = field.ident.as_ref().unwrap();

                                if ident != "local_part"
                                    && ident != "need_quoted"
                                    && ident != "domain_part"
                                {
                                    panic::validator_only_support_for_item(
                                        VALIDATOR,
                                        Box::new(ITEM_MUST_IP),
                                    );
                                }
                            }
                        } else {
                            panic::validator_only_support_for_item(
                                VALIDATOR,
                                Box::new(ITEM_MUST_IP),
                            );
                        }
                    }
                    ValidatorOption::NotAllow => {
                        if local == ValidatorOption::Allow
                            && at_least_two_labels != ValidatorOption::Allow
                        {
                            if let Fields::Named(_) = &data.fields {
                                if data.fields.len() != 4 {
                                    panic::validator_only_support_for_item(
                                        VALIDATOR,
                                        Box::new(ITEM_ALLOW_LOCAL),
                                    );
                                }

                                for field in data.fields.iter() {
                                    let ident = field.ident.as_ref().unwrap();

                                    if ident != "local_part"
                                        && ident != "need_quoted"
                                        && ident != "domain_part"
                                        && ident != "is_local"
                                    {
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
                        } else if let Fields::Named(_) = &data.fields {
                            if data.fields.len() != 3 {
                                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                            }

                            for field in data.fields.iter() {
                                let ident = field.ident.as_ref().unwrap();

                                if ident != "local_part"
                                    && ident != "need_quoted"
                                    && ident != "domain_part"
                                {
                                    panic::validator_only_support_for_item(
                                        VALIDATOR,
                                        Box::new(ITEM),
                                    );
                                }
                            }
                        } else {
                            panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                        }
                    }
                }
            }

            let name = ast.ident;

            // TODO impl

            let error_path: Path =
                syn::parse2(quote! { validators_prelude::EmailError }).unwrap();

            let comment_path = comment.to_expr();
            let ip_path = ip.to_expr();
            let local_path = local.to_expr();
            let at_least_two_labels_path = at_least_two_labels.to_expr();
            let non_ascii_path = non_ascii.to_expr();

            let parameters_impl = quote! {
                impl #name {
                    pub(crate) const V_COMMENT: validators_prelude::ValidatorOption = #comment_path;
                    pub(crate) const V_IP: validators_prelude::ValidatorOption = #ip_path;
                    pub(crate) const V_LOCAL: validators_prelude::ValidatorOption = #local_path;
                    pub(crate) const V_AT_LEAST_TWO_LABELS: validators_prelude::ValidatorOption = #at_least_two_labels_path;
                    pub(crate) const V_NON_ASCII: validators_prelude::ValidatorOption = #non_ascii_path;
                }
            };

            let handle_unrecognized_first_byte_quoted = if non_ascii.not_allow() {
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

            let handle_unrecognized_first_byte_unquoted = if non_ascii.not_allow() {
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

            let handle_unrecognized_byte_quoted = if non_ascii.not_allow() {
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

            let handle_unrecognized_byte_unquoted = if non_ascii.not_allow() {
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

            let handle_ip = if ip.not_allow() {
                quote! {
                    return Err(#error_path::IPNotAllow);
                }
            } else if at_least_two_labels.not_allow() {
                quote! {
                    return Err(#error_path::AtLeastTwoLabelsNotAllow);
                }
            } else {
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

                quote! {
                    use core::str::FromStr;

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
                                            validators_prelude::from_utf8_unchecked(
                                                &bytes[domain_part_length..closing_bracket_index],
                                            )
                                        };

                                        match validators_prelude::Ipv6Addr::from_str(ip_str) {
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
                                        validators_prelude::from_utf8_unchecked(
                                            &bytes[domain_part_length..closing_bracket_index],
                                        )
                                    };

                                    match validators_prelude::Ipv4Addr::from_str(ip_str) {
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

            let handle_domain = if ip.must() {
                quote! {
                    return Err(#error_path::IPMust);
                }
            } else {
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
                    let domain_str = {
                        match bytes[domain_part_length..].iter().copied().position(|e| e == b'(') {
                            Some(mut open_parenthesis_index) => {
                                open_parenthesis_index += domain_part_length;
                                unsafe {
                                    validators_prelude::from_utf8_unchecked(
                                        &bytes[domain_part_length..open_parenthesis_index],
                                    )
                                }
                            }
                            None => unsafe { validators_prelude::from_utf8_unchecked(&bytes[domain_part_length..]) },
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

            let handle_comment_1 = if comment.not_allow() {
                quote! {
                    return Err(#error_path::CommentNotAllow);
                }
            } else {
                quote! {
                    if length > 1 {
                        // only take 63 - 1, 1 is reserved for the meaningful local part.
                        match bytes[1..].iter().take(62).copied().position(|e| e == b')') {
                            Some(mut index) => {
                                index += 1;
                                let comment = unsafe { validators_prelude::from_utf8_unchecked(&bytes[1..index]) };

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

            let handle_comment_2 = if comment.not_allow() {
                quote! {
                    return Err(#error_path::CommentNotAllow);
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
                                    unsafe { validators_prelude::from_utf8_unchecked(&bytes[local_part_length..index]) };

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

            let handle_comment_3 = if comment.not_allow() {
                quote! {
                    return Err(#error_path::CommentNotAllow);
                }
            } else {
                quote! {
                    if length > 1 {
                        // only take 254 - 1, 1 is reserved for the meaningful domain part.
                        match bytes[1..].iter().take(253).copied().position(|e| e == b')') {
                            Some(mut index) => {
                                index += 1;
                                let comment = unsafe { validators_prelude::from_utf8_unchecked(&bytes[1..index]) };

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

            let handle_comment_4 = if comment.not_allow() {
                quote! {
                    return Err(#error_path::CommentNotAllow);
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
                                        unsafe { validators_prelude::from_utf8_unchecked(&bytes[domain_part_length..index]) };

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

            // TODO reuse the input `s`
            let v_parse_str = quote! {
                #conflict_meta
                fn v_parse_str(s: validators_prelude::Cow<str>) -> Result<(validators_prelude::String, bool, validators_prelude::Host, Option<validators_prelude::String>, Option<validators_prelude::String>, Option<validators_prelude::String>, Option<validators_prelude::String>, bool), #error_path> {
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
                                            unsafe { validators_prelude::from_utf8_unchecked(&bytes[(local_part_length + 1)..p]) };

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
                                        unsafe { validators_prelude::from_utf8_unchecked(&bytes[local_part_length..p]) };

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
            };

            let parse_impl = quote! {
                impl #name {
                    #v_parse_str
                }
            };

            let to_email_string = {
                if comment.not_allow() {
                    match ip {
                        ValidatorOption::Allow => {
                            quote! {
                                #[inline]
                                pub fn to_email_string(&self) -> validators_prelude::String {
                                    match &self.domain_part {
                                        validators_prelude::Host::IPv4(ip) => validators_prelude::format!("{}@[{}]", self.local_part, ip),
                                        validators_prelude::Host::IPv6(ip) => validators_prelude::format!("{}@[IPv6:{}]", self.local_part, ip),
                                        validators_prelude::Host::Domain(domain) => validators_prelude::format!("{}@{}", self.local_part, domain),
                                    }
                                }
                            }
                        }
                        ValidatorOption::Must => {
                            quote! {
                                #[inline]
                                pub fn to_email_string(&self) -> validators_prelude::String {
                                    match &self.domain_part {
                                        validators_prelude::IpAddr::V4(ip) => validators_prelude::format!("{}@[{}]", self.local_part, ip),
                                        validators_prelude::IpAddr::V6(ip) => validators_prelude::format!("{}@[IPv6:{}]", self.local_part, ip),
                                    }
                                }
                            }
                        }
                        ValidatorOption::NotAllow => {
                            quote! {
                                #[inline]
                                pub fn to_email_string(&self) -> validators_prelude::String {
                                    validators_prelude::format!("{}@{}", self.local_part, self.domain_part)
                                }
                            }
                        }
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

                    match ip {
                        ValidatorOption::Allow => {
                            quote! {
                                #[inline]
                                pub fn to_email_string(&self) -> validators_prelude::String {
                                    #generate_s

                                    use validators_prelude::fmt::Write;

                                    match &self.domain_part {
                                        validators_prelude::Host::IPv4(ip) => s.write_fmt(format_args!("[{}]", ip)).unwrap(),
                                        validators_prelude::Host::IPv6(ip) => s.write_fmt(format_args!("[IPv6:{}]", ip)).unwrap(),
                                        validators_prelude::Host::Domain(domain) => s.push_str(domain),
                                    }

                                    #finish_s

                                    s
                                }
                            }
                        }
                        ValidatorOption::Must => {
                            quote! {
                                #[inline]
                                pub fn to_email_string(&self) -> validators_prelude::String {
                                    #generate_s

                                    use validators_prelude::fmt::Write;

                                    match &self.domain_part {
                                        validators_prelude::IpAddr::V4(ip) => s.write_fmt(format_args!("[{}]", ip)).unwrap(),
                                        validators_prelude::IpAddr::V6(ip) => s.write_fmt(format_args!("[IPv6:{}]", ip)).unwrap(),
                                    }

                                    #finish_s

                                    s
                                }
                            }
                        }
                        ValidatorOption::NotAllow => {
                            quote! {
                                #[inline]
                                pub fn to_email_string(&self) -> validators_prelude::String {
                                    #generate_s

                                    s.push_str(self.domain_part.as_str());

                                    #finish_s

                                    s
                                }
                            }
                        }
                    }
                }
            };

            let other_functions = quote! {
                impl #name {
                    #to_email_string
                }
            };

            let create_instance = {
                if comment.allow() {
                    match ip {
                        ValidatorOption::Allow => {
                            if local == ValidatorOption::Allow
                                && at_least_two_labels != ValidatorOption::Allow
                            {
                                quote! {
                                    #name {
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
                                    #name {
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
                        }
                        ValidatorOption::Must => {
                            if local == ValidatorOption::Allow
                                && at_least_two_labels != ValidatorOption::Allow
                            {
                                quote! {
                                    #name {
                                        local_part,
                                        need_quoted,
                                        domain_part: {
                                            match domain_part {
                                                validators_prelude::Host::IPv4(ip) => validators_prelude::IpAddr::V4(ip),
                                                validators_prelude::Host::IPv6(ip) => validators_prelude::IpAddr::V6(ip),
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
                                    #name {
                                        local_part,
                                        need_quoted,
                                        domain_part: {
                                            match domain_part {
                                                validators_prelude::Host::IPv4(ip) => validators_prelude::IpAddr::V4(ip),
                                                validators_prelude::Host::IPv6(ip) => validators_prelude::IpAddr::V6(ip),
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
                        }
                        ValidatorOption::NotAllow => {
                            if local == ValidatorOption::Allow
                                && at_least_two_labels != ValidatorOption::Allow
                            {
                                quote! {
                                    #name {
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
                                    #name {
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
                        }
                    }
                } else {
                    match ip {
                        ValidatorOption::Allow => {
                            if local == ValidatorOption::Allow
                                && at_least_two_labels != ValidatorOption::Allow
                            {
                                quote! {
                                    #name {
                                        local_part,
                                        need_quoted,
                                        domain_part,
                                        is_local: _is_local,
                                    }
                                }
                            } else {
                                quote! {
                                    #name {
                                        local_part,
                                        need_quoted,
                                        domain_part,
                                    }
                                }
                            }
                        }
                        ValidatorOption::Must => {
                            if local == ValidatorOption::Allow
                                && at_least_two_labels != ValidatorOption::Allow
                            {
                                quote! {
                                    #name {
                                        local_part,
                                        need_quoted,
                                        domain_part: {
                                            match domain_part {
                                                validators_prelude::Host::IPv4(ip) => validators_prelude::IpAddr::V4(ip),
                                                validators_prelude::Host::IPv6(ip) => validators_prelude::IpAddr::V6(ip),
                                                validators_prelude::Host::Domain(_) => unreachable!(),
                                            }
                                        },
                                        is_local: _is_local,
                                    }
                                }
                            } else {
                                quote! {
                                    #name {
                                        local_part,
                                        need_quoted,
                                        domain_part: {
                                            match domain_part {
                                                validators_prelude::Host::IPv4(ip) => validators_prelude::IpAddr::V4(ip),
                                                validators_prelude::Host::IPv6(ip) => validators_prelude::IpAddr::V6(ip),
                                                _ => unreachable!(),
                                            }
                                        },
                                    }
                                }
                            }
                        }
                        ValidatorOption::NotAllow => {
                            if local == ValidatorOption::Allow
                                && at_least_two_labels != ValidatorOption::Allow
                            {
                                quote! {
                                    #name {
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
                                    #name {
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
                        let (local_part, need_quoted, domain_part, _comment_before_local_part, _comment_after_local_part, _comment_before_domain_part, _comment_after_domain_part, _is_local) = Self::v_parse_str(validators_prelude::Cow::from(s.into()))?;

                        Ok(#create_instance)
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> Result<Self::Output, Self::Error> {
                        let (local_part, need_quoted, domain_part, _comment_before_local_part, _comment_after_local_part, _comment_before_domain_part, _comment_after_domain_part, _is_local) = Self::v_parse_str(validators_prelude::Cow::from(s.as_ref()))?;

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
                    let mut s = if non_ascii.not_allow() {
                        String::from("an ASCII email")
                    } else {
                        String::from("an email")
                    };

                    match ip {
                        ValidatorOption::Allow => {
                            match at_least_two_labels {
                                ValidatorOption::Allow => (),
                                ValidatorOption::Must => {
                                    s.push_str(" whose domain part must be at-least-two-labels");
                                }
                                ValidatorOption::NotAllow => {
                                    s.push_str(" whose domain part must be one-label");
                                }
                            }
                        }
                        ValidatorOption::Must => {
                            s.push_str(" whose domain part must be an IP");
                        }
                        ValidatorOption::NotAllow => {
                            s.push_str(" whose domain part must be non-IP");

                            match at_least_two_labels {
                                ValidatorOption::Allow => (),
                                ValidatorOption::Must => {
                                    s.push_str(" and at-least-two-labels");
                                }
                                ValidatorOption::NotAllow => {
                                    s.push_str(" and one-label");
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
                            serializer.serialize_str(&self.to_email_string())
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

            let email_impl = quote! {
                #parameters_impl

                #parse_impl

                #validate_string_impl

                #other_functions

                #serde_impl

                #rocket_impl
            };

            email_impl.into()
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM_ALLOW_COMMENT)),
    }
}
