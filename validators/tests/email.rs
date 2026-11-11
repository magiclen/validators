#![cfg(all(feature = "test", feature = "derive", feature = "email"))]

use validators::{errors::EmailError, prelude::*};

#[test]
fn quoted_without_comments() {
    #[derive(Validator)]
    #[validator(email(comment(Disallow)))]
    struct Address {
        local_part:  String,
        need_quoted: bool,
        domain_part: validators::models::Host,
    }

    for input in [
        "\"alice smith\"@example.com",
        "\"alice.\"@example.com",
        "\"a@b\"@example.com",
        "\"@work\"@example.com",
        r#""a\ "@example.com"#,
    ] {
        let address = Address::parse_str(input).unwrap();

        assert_eq!(input, address.to_email_string());
    }

    let address = Address::parse_str("a@[ipv6:::1]").unwrap();

    assert_eq!("a@[IPv6:::1]", address.to_email_string());
}

#[derive(Validator)]
#[validator(email)]
struct AddressWithComments {
    local_part:                 String,
    need_quoted:                bool,
    domain_part:                validators::models::Host,
    comment_before_local_part:  Option<String>,
    comment_after_local_part:   Option<String>,
    comment_before_domain_part: Option<String>,
    comment_after_domain_part:  Option<String>,
}

#[test]
fn empty_quoted_local_part() {
    #[derive(Validator)]
    #[validator(email(comment(Disallow)))]
    struct Address {
        local_part:  String,
        need_quoted: bool,
        domain_part: validators::models::Host,
    }

    for domain in ["example.com", "[127.0.0.1]", "[IPv6:::1]"] {
        let input = format!(r#"""@{domain}"#);
        let address = Address::parse_str(&input).unwrap();

        assert_eq!("", address.local_part);
        assert!(address.need_quoted);
        assert_eq!(input, address.to_email_string());

        #[cfg(feature = "serde_json")]
        {
            use validators::serde_json;

            let json = serde_json::to_string(&address).unwrap();
            let decoded: Address = serde_json::from_str(&json).unwrap();

            assert_eq!(serde_json::to_string(&input).unwrap(), json);
            assert_eq!("", decoded.local_part);
            assert!(decoded.need_quoted);
            assert_eq!(input, decoded.to_email_string());
        }

        let input = format!(r#"(a)""(b)@(c){domain}(d)"#);
        let address = AddressWithComments::parse_str(&input).unwrap();

        assert_eq!("", address.local_part);
        assert!(address.need_quoted);
        assert_eq!(input, address.to_email_string());

        #[cfg(feature = "serde_json")]
        {
            use validators::serde_json;

            let json = serde_json::to_string(&address).unwrap();
            let decoded: AddressWithComments = serde_json::from_str(&json).unwrap();

            assert_eq!(serde_json::to_string(&input).unwrap(), json);
            assert_eq!("", decoded.local_part);
            assert!(decoded.need_quoted);
            assert_eq!(input, decoded.to_email_string());
        }
    }

    let comment = "x".repeat(60);

    assert!(AddressWithComments::validate_str(format!(r#"({comment})""@example.com"#)).is_ok());
    assert!(AddressWithComments::validate_str(format!(r#"({comment})"a"@example.com"#)).is_err());
}

#[test]
fn escaped_comments() {
    let input = r"(a\))alice(b\()@(c\\)example.com(d\))";
    let address = AddressWithComments::parse_str(input).unwrap();

    assert_eq!(input, address.to_email_string());
    assert!(AddressWithComments::validate_str(r"alice@example.com(note\)").is_err());
}

#[test]
fn normalized_domain_length() {
    #[derive(Validator)]
    #[validator(email(comment(Disallow)))]
    struct Address {
        local_part:  String,
        need_quoted: bool,
        domain_part: validators::models::Host,
    }

    let comment = "x".repeat(224);
    let input = format!(r#""a b"@[IPv6:::ffff:ffff:ffff]({comment})"#);
    let expected = format!(r#""a b"@[IPv6:::ffff:255.255.255.255]({comment})"#);
    let address = AddressWithComments::parse_str(&input).unwrap();
    let normalized = address.to_email_string();

    assert_eq!(expected, normalized);
    assert!(AddressWithComments::validate_str(normalized).is_ok());
    assert!(matches!(Address::validate_str(input), Err(EmailError::CommentDisallow)));

    let input = format!(r#""a b"@[IPv6:::ffff:ffff:ffff]({comment}x)"#);

    assert!(matches!(AddressWithComments::parse_str(&input), Err(EmailError::Invalid)));
    assert!(matches!(Address::validate_str(input), Err(EmailError::Invalid)));
}

#[test]
fn address_length() {
    #[derive(Validator)]
    #[validator(email(comment(Disallow)))]
    struct Address {
        local_part:  String,
        need_quoted: bool,
        domain_part: validators::models::Host,
    }

    let local_part = "a".repeat(64);
    let label = "b".repeat(63);
    let address = format!("{local_part}@{label}.{label}.{}.com", "c".repeat(57));

    assert_eq!(254, address.len());
    assert!(Address::validate_str(&address).is_ok());

    let address = format!("{local_part}@{label}.{label}.{}.com", "c".repeat(58));

    assert!(Address::validate_str(address).is_err());
}

#[test]
fn basic() {
    macro_rules! test_case {
        ($test:ident, $validator:ident) => {
            type Validator = $validator;
            let test = $test;

            test("", false);
            test("@example.com", false);
            test("Abc.example.com", false);
            test("A@b@c@example.com", false);
            test("a\"b(c)d,e:f;g<h>i[j\\k]l@example.com", false);
            test("just\"not\"right@example.com", false);
            test("a@example.com(note)extra", false);
            test("a@example.com。", false);
            test("simple@127.0.0.1", false);
            test("alice.@example.com", false);
            test("alice.(note)@example.com", false);
            test("(note\r\n)alice@example.com", false);
            test("alice(note\n)@example.com", false);
            test("alice@(note\0)example.com", false);
            test("alice@example.com(note\r)", false);
            test("(note\x1b)alice@example.com", false);
            test("alice(note\x7f)@example.com", false);
            test("(no(te)alice@example.com", false);
            test(&format!("{}(x)@example.com", "a".repeat(64)), false);
            test(&format!("{}@example.com", "中".repeat(22)), false);
            test("this is\"not\\allowed@example.com", false);
            test("this\\ still\\\"not\\\\allowed@example.com", false);
            test(
                "1234567890123456789012345678901234567890123456789012345678901234+x@example.com",
                false,
            );
            test("i_like_underscore@but_its_not_allow_in _this_part.example.com", false);
            test("\"joh\"n\"\"@example.com", false);
            test(
                "\"\"@example.com",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "simple@example.com",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "very.common@example.com",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "disposable.style.email.with+symbol@example.com",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "other.email-with-hyphen@example.com",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "fully-qualified-domain@example.com",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "user.name+tag+sorting@example.com",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "x@example.com",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "example-indeed@strange-example.com",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "admin@mailserver1",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && !Validator::V_AT_LEAST_TWO_LABELS.must(),
            );
            test(
                "example@s.example",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "\" \"@example.org",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "\"john..doe\"@example.org",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "mailhost!username@example.org",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "user%example.com@example.org",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "\"joh\\\"n\\\"\"@example.com",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test("simple@localhost", !Validator::V_IP.must() && Validator::V_LOCAL.allow());
            test(
                "simple@foo.localhost",
                !Validator::V_IP.must()
                    && Validator::V_LOCAL.allow()
                    && !Validator::V_AT_LEAST_TWO_LABELS.disallow(),
            );
            test(
                "simple@[168.17.212.1]",
                Validator::V_IP.allow()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "simple@[127.0.0.1]",
                Validator::V_IP.allow()
                    && Validator::V_LOCAL.allow()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "simple@[IPv6:0000:0000:0000:0000:0000:0000:370:7348]",
                Validator::V_IP.allow()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "simple@中文.com",
                !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "中文@example.com",
                Validator::V_NON_ASCII.allow()
                    && !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "(a)simple(b)@(c)example.com(d)",
                Validator::V_COMMENT.allow()
                    && !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "(a\tb)simple@example.com",
                Validator::V_COMMENT.allow()
                    && !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
            test(
                "(中文)simple@example.com",
                Validator::V_COMMENT.allow()
                    && Validator::V_NON_ASCII.allow()
                    && !Validator::V_IP.must()
                    && !Validator::V_LOCAL.must()
                    && Validator::V_AT_LEAST_TWO_LABELS.allow(),
            );
        };
    }

    macro_rules! test_inner {
        ( @unit $parameters:expr, $validator:ident) => {
            {
                fn test(s: &str, is_ok: bool) {
                    let panic = match $validator::validate_str(s) {
                        Ok(_) => !is_ok,
                        Err(_) if !is_ok => false,
                        Err(err) => {
                            eprintln!("{}", err);

                            true
                        }
                    };

                    if panic {
                        panic!("{:?}: {} {} expect {}", s, stringify!($validator), $parameters, is_ok);
                    }
                }

                test_case!(test, $validator);
            }
        };
        ( $parameters:expr ; $($validator:ident),* $(,)*) => {
            $(
                test_inner!(
                    @unit
                    $parameters,
                    $validator
                );
            )*
        };
    }

    macro_rules! test {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(Disallow)))]
                    pub struct EmailAllowIP {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: validators::models::Host,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(Disallow), ip(Must), conflict(Allow)))]
                    pub struct EmailIP {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: std::net::IpAddr,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(Disallow), ip(Disallow)))]
                    pub struct EmailNonIP {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: String,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*))]
                    pub struct EmailAllowCommentAllowIP {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: validators::models::Host,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*ip(Must), conflict(Allow)))]
                    pub struct EmailAllowCommentIP {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: std::net::IpAddr,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*ip(Disallow)))]
                    pub struct EmailAllowCommentNonIP {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: String,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                    }

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        EmailAllowIP,
                        EmailIP,
                        EmailNonIP,
                        EmailAllowCommentAllowIP,
                        EmailAllowCommentIP,
                        EmailAllowCommentNonIP,
                    );
                }
            )*
        }
    }

    macro_rules! test2 {
        ($( { $( $p:meta => $v:meta ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(Disallow)))]
                    pub struct EmailAllowIPIsLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: validators::models::Host,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(Disallow), ip(Must), conflict(Allow)))]
                    pub struct EmailIPIsLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: std::net::IpAddr,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(Disallow), ip(Disallow)))]
                    pub struct EmailNonIPIsLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: String,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*))]
                    pub struct EmailAllowCommentAllowIPIsLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: validators::models::Host,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*ip(Must), conflict(Allow)))]
                    pub struct EmailAllowCommentIPIsLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: std::net::IpAddr,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                        pub is_local: bool,
                    }

                    #[allow(dead_code)]
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*ip(Disallow)))]
                    pub struct EmailAllowCommentNonIPIsLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: String,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                        pub is_local: bool,
                    }

                    test_inner!(
                        stringify! {
                            $(
                                $p = $v,
                            )*
                        };
                        EmailAllowIPIsLocal,
                        EmailIPIsLocal,
                        EmailNonIPIsLocal,
                        EmailAllowCommentAllowIPIsLocal,
                        EmailAllowCommentIPIsLocal,
                        EmailAllowCommentNonIPIsLocal,
                    );
                }
            )*
        }
    }

    test! {
        {
            local => Allow,
            at_least_two_labels => Allow,
            non_ascii => Allow,
        },
        {
            local => Allow,
            at_least_two_labels => Allow,
            non_ascii => Disallow,
        },
        {
            local => Must,
            at_least_two_labels => Allow,
            non_ascii => Allow,
        },
        {
            local => Must,
            at_least_two_labels => Allow,
            non_ascii => Disallow,
        },
        {
            local => Disallow,
            at_least_two_labels => Allow,
            non_ascii => Allow,
        },
        {
            local => Disallow,
            at_least_two_labels => Allow,
            non_ascii => Disallow,
        },
        {
            local => Must,
            at_least_two_labels => Disallow,
            non_ascii => Allow,
        },
        {
            local => Must,
            at_least_two_labels => Disallow,
            non_ascii => Disallow,
        },
        {
            local => Disallow,
            at_least_two_labels => Disallow,
            non_ascii => Allow,
        },
        {
            local => Disallow,
            at_least_two_labels => Disallow,
            non_ascii => Disallow,
        },
        {
            local => Must,
            at_least_two_labels => Must,
            non_ascii => Allow,
        },
        {
            local => Must,
            at_least_two_labels => Must,
            non_ascii => Disallow,
        },
        {
            local => Disallow,
            at_least_two_labels => Must,
            non_ascii => Allow,
        },
        {
            local => Disallow,
            at_least_two_labels => Must,
            non_ascii => Disallow,
        },
    }

    test2! {
        {
            local => Allow,
            at_least_two_labels => Must,
            non_ascii => Allow,
        },
        {
            local => Allow,
            at_least_two_labels => Must,
            non_ascii => Disallow,
        },
        {
            local => Allow,
            at_least_two_labels => Disallow,
            non_ascii => Allow,
        },
        {
            local => Allow,
            at_least_two_labels => Disallow,
            non_ascii => Disallow,
        },
    }
}
