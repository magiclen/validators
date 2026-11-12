mod email_attribute;

use std::fmt::Debug;

use educe::Educe;
use email_attribute::EmailAttribute;
use quote::quote;
use syn::{Data, DeriveInput, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{fields_match, tri_allow::TriAllow, type_enum::TypeEnum},
    panic,
};

pub(crate) struct EmailHandler;

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct Struct {
    local_part:  TypeEnum,
    need_quoted: TypeEnum,
    domain_part: TypeEnum,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Debug(name = "Struct"))]
pub struct StructAllowLocal {
    local_part:  TypeEnum,
    need_quoted: TypeEnum,
    domain_part: TypeEnum,
    is_local:    TypeEnum,
}

#[allow(dead_code)]
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
        #[cfg(feature = "serde")]
        let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
        #[cfg(feature = "serde")]
        let deserializer = ident_generator.next("__ValidatorsDeserializer");
        #[cfg(feature = "serde")]
        let deserializer_name = ident_generator.next("__validators_deserializer");
        #[cfg(feature = "serde")]
        let visitor = ident_generator.next("__ValidatorsVisitor");

        let type_attribute = EmailAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            let mut meta_is_conflict = false;

            if type_attribute.ip.must() && type_attribute.at_least_two_labels.disallow() {
                if type_attribute.conflict.disallow() {
                    return Err(syn::Error::new_spanned(
                        &meta,
                        "`ip(Must)` and `at_least_two_labels(Disallow)` cannot be used together",
                    ));
                }

                meta_is_conflict = true;
            }

            // `is_local` is kept only when `local` is allowed and `at_least_two_labels` is checked.
            let has_local_field = type_attribute.local == TriAllow::Allow
                && type_attribute.at_least_two_labels != TriAllow::Allow;

            let item: &dyn Debug =
                match (type_attribute.comment.allow(), type_attribute.ip, has_local_field) {
                    (true, TriAllow::Allow, true) => &ITEM_ALLOW_COMMENT_ALLOW_LOCAL_ALLOW_IP,
                    (true, TriAllow::Allow, false) => &ITEM_ALLOW_COMMENT_ALLOW_IP,
                    (true, TriAllow::Must, true) => &ITEM_ALLOW_COMMENT_ALLOW_LOCAL_MUST_IP,
                    (true, TriAllow::Must, false) => &ITEM_ALLOW_COMMENT_MUST_IP,
                    (true, TriAllow::Disallow, true) => &ITEM_ALLOW_COMMENT_ALLOW_LOCAL,
                    (true, TriAllow::Disallow, false) => &ITEM_ALLOW_COMMENT,
                    (false, TriAllow::Allow, true) => &ITEM_ALLOW_LOCAL_ALLOW_IP,
                    (false, TriAllow::Allow, false) => &ITEM_ALLOW_IP,
                    (false, TriAllow::Must, true) => &ITEM_ALLOW_LOCAL_MUST_IP,
                    (false, TriAllow::Must, false) => &ITEM_MUST_IP,
                    (false, TriAllow::Disallow, true) => &ITEM_ALLOW_LOCAL,
                    (false, TriAllow::Disallow, false) => &ITEM,
                };

            let mut names = vec!["local_part", "need_quoted", "domain_part"];

            if type_attribute.comment.allow() {
                names.extend([
                    "comment_before_local_part",
                    "comment_after_local_part",
                    "comment_before_domain_part",
                    "comment_after_domain_part",
                ]);
            }

            if has_local_field {
                names.push("is_local");
            }

            if !fields_match(&data.fields, Some(&names)) {
                return Err(panic::validator_for_specific_item(meta.path(), item));
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

            let quoted_only = quote! {
                b' ' | b'\t' | b'(' | b')' | b',' | b':'..=b'<' | b'>' | b'@' | b'[' | b']'
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
                        let ipv6_tag = b"IPv6:";

                        if bytes[domain_part_length..].get(..ipv6_tag.len()).is_some_and(|prefix| prefix.eq_ignore_ascii_case(ipv6_tag)) {
                            domain_part_length += ipv6_tag.len();

                            if length > domain_part_length {
                                match bytes[domain_part_length..]
                                    .iter()
                                    .copied()
                                    .position(|e| e == b']')
                                {
                                    Some(mut closing_bracket_index) => {
                                        closing_bracket_index += domain_part_length;

                                        let ip_str = &s[domain_part_length..closing_bracket_index];

                                        match ::core::net::Ipv6Addr::from_str(ip_str) {
                                            Ok(ip) => {
                                                domain_part_length = closing_bracket_index + 1;

                                                struct LengthCounter(usize);

                                                impl ::core::fmt::Write for LengthCounter {
                                                    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
                                                        self.0 += s.len();

                                                        Ok(())
                                                    }
                                                }

                                                let mut ip_length = LengthCounter(0);

                                                ::core::fmt::write(&mut ip_length, ::core::format_args!("{}", ip)).unwrap();

                                                // adjust length
                                                domain_part_length_offset += ip_length.0
                                                    as isize
                                                    - ip_str.len() as isize;

                                                (::core::result::Result::Ok(::core::net::IpAddr::V6(ip)), #handle_local_ipv6)
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

                                    let ip_str = &s[domain_part_length..closing_bracket_index];

                                    match ::core::net::Ipv4Addr::from_str(ip_str) {
                                        Ok(ip) => {
                                            domain_part_length = closing_bracket_index + 1;

                                            (::core::result::Result::Ok(::core::net::IpAddr::V4(ip)), #handle_local_ipv4)
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
                            // Local names are not exempt here, because subdomains such as `foo.localhost` have two labels.
                            quote! {
                                if validators_prelude::is_at_least_two_labels_domain(&ascii_domain) {
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
                                &s[domain_part_length..open_parenthesis_index]
                            }
                            None => &s[domain_part_length..],
                        }
                    };

                    match validators_prelude::idna::uts46::Uts46::new().to_ascii(
                        domain_str.as_bytes(),
                        validators_prelude::idna::uts46::AsciiDenyList::STD3,
                        validators_prelude::idna::uts46::Hyphens::CheckFirstLast,
                        validators_prelude::idna::uts46::DnsLength::VerifyAllowRootDot,
                    ) {
                        Ok(ascii_domain) => {
                            if ascii_domain.ends_with('.') {
                                return Err(#error_path::Invalid);
                            }

                            // An IPv4 address must be written in brackets, such as `[127.0.0.1]`.
                            if ascii_domain.parse::<::core::net::Ipv4Addr>().is_ok() {
                                return Err(#error_path::Invalid);
                            }

                            domain_part_length += domain_str.len();

                            let is_local = #handle_local_domain;

                            #check_at_least_two_labels

                            // adjust length
                            domain_part_length_offset +=
                                ascii_domain.len() as isize - domain_str.len() as isize;

                            (::core::result::Result::Err(ascii_domain), is_local)
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

            let parse_comment = if type_attribute.comment.disallow() {
                quote! {}
            } else {
                let invalid_byte = if type_attribute.non_ascii.disallow() {
                    quote! { b'\x00'..=b'\x08' | b'\x0A'..=b'\x1F' | b'\x7F'..=b'\xFF' }
                } else {
                    quote! { b'\x00'..=b'\x08' | b'\x0A'..=b'\x1F' | b'\x7F' }
                };

                let check_c1_control = if type_attribute.non_ascii.disallow() {
                    quote! {}
                } else {
                    quote! {
                        // 0xC2 followed by 0x80..=0x9F encodes a C1 control character (U+0080..=U+009F).
                        if byte == 0xC2 && ::core::matches!(bytes.get(index + 1), Some(0x80..=0x9F)) {
                            return Err(#error_path::Invalid);
                        }
                    }
                };

                quote! {
                    fn parse_comment(s: &str, max_len: usize) -> ::core::result::Result<(&str, usize), #error_path> {
                        let bytes = s.as_bytes();
                        let mut escaped = false;

                        // The limit includes the opening and closing parentheses.
                        for (index, byte) in bytes.iter().copied().take(max_len).enumerate().skip(1) {
                            if ::core::matches!(byte, #invalid_byte) {
                                return Err(#error_path::Invalid);
                            }

                            #check_c1_control

                            if escaped {
                                escaped = false;
                            } else {
                                match byte {
                                    b'\\' => escaped = true,
                                    b'(' => return Err(#error_path::Invalid),
                                    b')' => return Ok((&s[1..index], index + 1)),
                                    _ => (),
                                }
                            }
                        }

                        Err(#error_path::Invalid)
                    }
                }
            };

            let handle_comment_1 = if type_attribute.comment.disallow() {
                quote! {
                    return Err(#error_path::CommentDisallow);
                }
            } else {
                quote! {
                    let (comment, local_part_length) = parse_comment(s, length)?;

                    if local_part_length == length {
                        return Err(#error_path::Invalid);
                    }

                    (local_part_length, Some(comment))
                }
            };

            let handle_comment_2 = if type_attribute.comment.disallow() {
                quote! {
                    return Err(#error_path::CommentDisallow);
                }
            } else {
                quote! {
                    let (comment, comment_len) = parse_comment(&s[local_part_length..], length - local_part_length)?;

                    local_part_length += comment_len;

                    if local_part_length == length {
                        return Err(#error_path::Invalid);
                    }

                    Some(comment)
                }
            };

            let handle_comment_3 = if type_attribute.comment.disallow() {
                quote! {
                    return Err(#error_path::CommentDisallow);
                }
            } else {
                quote! {
                    let (comment, domain_part_length) = parse_comment(s, length)?;

                    if domain_part_length == length {
                        return Err(#error_path::Invalid);
                    }

                    (domain_part_length, Some(comment))
                }
            };

            let handle_comment_4 = if type_attribute.comment.disallow() {
                quote! {
                    // Other bytes after an IP literal are not a comment.
                    if bytes[domain_part_length] != b'(' {
                        return Err(#error_path::Invalid);
                    }

                    return Err(#error_path::CommentDisallow);
                }
            } else {
                quote! {
                    if bytes[domain_part_length] != b'(' {
                        return Err(#error_path::Invalid);
                    }

                    let tail = &s[domain_part_length..];
                    let (comment, comment_len) = parse_comment(tail, tail.len())?;

                    if comment_len != tail.len() {
                        return Err(#error_path::Invalid);
                    }

                    Some(comment)
                }
            };

            token_stream.extend(quote! {
                impl #name {
                    #conflict_meta
                    fn v_parse_str(s: &str) -> ::core::result::Result<(&str, bool, ::core::result::Result<::core::net::IpAddr, validators_prelude::Cow<'_, str>>, Option<&str>, Option<&str>, Option<&str>, Option<&str>, bool), #error_path> {
                        #parse_comment

                        let bytes = s.as_bytes();
                        let length = bytes.len();

                        if length == 0 || length > 320 {
                            return Err(#error_path::Invalid);
                        }

                        let input_length = length;

                        // comment 1
                        let (mut local_part_length, comment_before_local_part) = if bytes[0] == b'(' {
                            #handle_comment_1
                        } else {
                            (0, None::<&str>)
                        };

                        // RFC 5321 limits the local part to 64 octets, and comments are not part of it.
                        let local_part_start = local_part_length;
                        let quoted = bytes[local_part_length] == b'"';

                        let (local_part, need_quoted) = if quoted {
                            // quoted
                            let mut p = local_part_length + 1;

                            if p - local_part_start >= 64 || p == length {
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
                                b'"' => (false, true, false),
                                b'.' => (false, true, true),
                                b'\\' => (true, true, false),
                                #quoted_only => (false, true, false),
                                _ => {
                                    #handle_unrecognized_first_byte_quoted
                                },
                            };

                            if e != b'"' {
                                p += 1;
                            }

                            loop {
                                if p == length || p - local_part_start >= 64 {
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
                                        last_dot = false;
                                        escaping = false;
                                    }
                                    b'.' => {
                                        if last_dot {
                                            need_quoted = true;
                                        }

                                        last_dot = true;
                                        escaping = false;
                                    }
                                    b'\\' => {
                                        need_quoted = true;
                                        escaping = !escaping;
                                    }
                                    #quoted_only => {
                                        need_quoted = true;
                                        last_dot = false;
                                        escaping = false;
                                    }
                                    b'"' => {
                                        if escaping {
                                            escaping = false;
                                        } else {
                                            let local_part =
                                                &s[(local_part_length + 1)..p];

                                            local_part_length = p + 1;

                                            if local_part_length == length {
                                                // end
                                                return Err(#error_path::Invalid);
                                            }

                                            // A trailing dot is only valid inside quotes.
                                            break (local_part, need_quoted || last_dot);
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
                                if p == length || p - local_part_start > 64 {
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
                                        last_dot = false;
                                    }
                                    b'.' => {
                                        if last_dot {
                                            return Err(#error_path::Invalid);
                                        }

                                        last_dot = true;
                                    }
                                    b'@' | b'(' => {
                                        if last_dot {
                                            return Err(#error_path::Invalid);
                                        }

                                        let local_part =
                                            &s[local_part_length..p];

                                        local_part_length = p;

                                        break (local_part, false);
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

                        let s = &s[(local_part_length + 1)..];
                        let bytes = s.as_bytes();
                        let length = bytes.len();

                        // comment 3
                        let (mut domain_part_length, comment_before_domain_part) = if bytes[0] == b'(' {
                            #handle_comment_3
                        } else {
                            (0, None::<&str>)
                        };

                        let domain_part_start = domain_part_length;
                        let mut domain_part_length_offset = 0isize;

                        let (host, is_local): (::core::result::Result<::core::net::IpAddr, validators_prelude::Cow<'_, str>>, bool) = match bytes[domain_part_length] {
                            b'(' => return Err(#error_path::Invalid),
                            b'[' => {
                                #handle_ip
                            }
                            _ => {
                                #handle_domain
                            }
                        };

                        #check_local

                        // RFC 5321 limits the domain to 255 octets, and comments are not part of it.
                        if (domain_part_length - domain_part_start) as isize + domain_part_length_offset > 255 {
                            return Err(#error_path::Invalid);
                        }

                        // RFC 5321 limits an address without comments to 254 octets.
                        // An IP literal is too short to reach this limit.
                        if let ::core::result::Result::Err(domain) = &host {
                            let quotes_length = if need_quoted { 2 } else { 0 };

                            if local_part.len() + quotes_length + 1 + domain.len() > 254 {
                                return Err(#error_path::Invalid);
                            }
                        }

                        // The normalized address written by `to_email_string` must also fit, so that it can be read back.
                        // Only the domain part can grow, and quotes that are not needed are removed.
                        let removed_quotes_length = if quoted && !need_quoted { 2 } else { 0 };

                        if input_length as isize + domain_part_length_offset - removed_quotes_length > 320 {
                            return Err(#error_path::Invalid);
                        }

                        // comment 4
                        let comment_after_domain_part = if length > domain_part_length {
                            #handle_comment_4
                        } else {
                            None::<&str>
                        };

                        return Ok((
                            local_part,
                            need_quoted,
                            host,
                            comment_before_local_part,
                            comment_after_local_part,
                            comment_before_domain_part,
                            comment_after_domain_part,
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
                                                validators_prelude::Host::IPv4(ip) => ::core::net::IpAddr::V4(ip),
                                                validators_prelude::Host::IPv6(ip) => ::core::net::IpAddr::V6(ip),
                                                validators_prelude::Host::Domain(_) => ::core::unreachable!(),
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
                                                validators_prelude::Host::IPv4(ip) => ::core::net::IpAddr::V4(ip),
                                                validators_prelude::Host::IPv6(ip) => ::core::net::IpAddr::V6(ip),
                                                _ => ::core::unreachable!(),
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
                                                _ => ::core::unreachable!(),
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
                                                _ => ::core::unreachable!(),
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
                                                validators_prelude::Host::IPv4(ip) => ::core::net::IpAddr::V4(ip),
                                                validators_prelude::Host::IPv6(ip) => ::core::net::IpAddr::V6(ip),
                                                validators_prelude::Host::Domain(_) => ::core::unreachable!(),
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
                                                validators_prelude::Host::IPv4(ip) => ::core::net::IpAddr::V4(ip),
                                                validators_prelude::Host::IPv6(ip) => ::core::net::IpAddr::V6(ip),
                                                _ => ::core::unreachable!(),
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
                                                _ => ::core::unreachable!(),
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
                                                _ => ::core::unreachable!(),
                                            }
                                        },
                                    }
                                }
                            }
                        },
                    }
                }
            };

            let own_comments = if type_attribute.comment.allow() {
                quote! {
                    let _comment_before_local_part = _comment_before_local_part.map(validators_prelude::String::from);
                    let _comment_after_local_part = _comment_after_local_part.map(validators_prelude::String::from);
                    let _comment_before_domain_part = _comment_before_domain_part.map(validators_prelude::String::from);
                    let _comment_after_domain_part = _comment_after_domain_part.map(validators_prelude::String::from);
                }
            } else {
                quote! {}
            };

            let parse = quote! {
                let (local_part, need_quoted, domain_part, _comment_before_local_part, _comment_after_local_part, _comment_before_domain_part, _comment_after_domain_part, _is_local) = Self::v_parse_str(s)?;
                let domain_part = match domain_part {
                    ::core::result::Result::Ok(::core::net::IpAddr::V4(ip)) => validators_prelude::Host::IPv4(ip),
                    ::core::result::Result::Ok(::core::net::IpAddr::V6(ip)) => validators_prelude::Host::IPv6(ip),
                    ::core::result::Result::Err(domain) => validators_prelude::Host::Domain(domain.into_owned()),
                };
                let local_part = validators_prelude::String::from(local_part);

                #own_comments

                Ok(#create_instance)
            };

            token_stream.extend(quote! {
                impl ValidateString for #name {
                    type Error = #error_path;

                    #[inline]
                    fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                        let s = s.into();
                        let s = s.as_str();

                        #parse
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                        let s = s.as_ref();

                        #parse
                    }

                    #[inline]
                    fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
                        Self::v_parse_str(s.as_ref()).map(|_| ())
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
                                    let quote = if self.need_quoted { "\"" } else { "" };

                                    match &self.domain_part {
                                        validators_prelude::Host::IPv4(ip) => validators_prelude::format!("{quote}{local_part}{quote}@[{ip}]"),
                                        validators_prelude::Host::IPv6(ip) => validators_prelude::format!("{quote}{local_part}{quote}@[IPv6:{ip}]"),
                                        validators_prelude::Host::Domain(domain) => validators_prelude::format!("{quote}{local_part}{quote}@{domain}"),
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
                                    let quote = if self.need_quoted { "\"" } else { "" };

                                    match &self.domain_part {
                                        ::core::net::IpAddr::V4(ip) => validators_prelude::format!("{quote}{local_part}{quote}@[{ip}]"),
                                        ::core::net::IpAddr::V6(ip) => validators_prelude::format!("{quote}{local_part}{quote}@[IPv6:{ip}]"),
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
                                    let quote = if self.need_quoted { "\"" } else { "" };

                                    validators_prelude::format!("{quote}{local_part}{quote}@{domain_part}")
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
                                        validators_prelude::Host::IPv4(ip) => s.write_fmt(::core::format_args!("[{ip}]")).unwrap(),
                                        validators_prelude::Host::IPv6(ip) => s.write_fmt(::core::format_args!("[IPv6:{ip}]")).unwrap(),
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
                                        ::core::net::IpAddr::V4(ip) => s.write_fmt(::core::format_args!("[{ip}]")).unwrap(),
                                        ::core::net::IpAddr::V6(ip) => s.write_fmt(::core::format_args!("[IPv6:{ip}]")).unwrap(),
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
                            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
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

        Err(panic::validator_for_specific_item(meta.path(), ITEM_ALLOW_COMMENT))
    }
}
