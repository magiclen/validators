#![cfg(feature = "email")]

#[macro_use]
extern crate validators_derive;

extern crate validators;

use validators::prelude::*;

#[test]
fn basic() {
    macro_rules! test_case {
        ($test:ident, $validator:ident) => {
            type Validator = $validator;
            let test = $test;

            test("", false);
            test("Abc.example.com", false);
            test("A@b@c@example.com", false);
            test("a\"b(c)d,e:f;g<h>i[j\\k]l@example.com", false);
            test("just\"not\"right@example.com", false);
            test("this is\"not\\allowed@example.com", false);
            test("this\\ still\\\"not\\\\allowed@example.com", false);
            test(
                "1234567890123456789012345678901234567890123456789012345678901234+x@example.com",
                false,
            );
            test("i_like_underscore@but_its_not_allow_in _this_part.example.com", false);
            test("\"joh\"n\"\"@example.com", false);
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
                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(NotAllow)))]
                    pub struct EmailAllowIPAllowLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: validators::models::Host,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(NotAllow), local(Must)))]
                    pub struct EmailAllowIPLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: validators::models::Host,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(NotAllow), local(NotAllow)))]
                    pub struct EmailAllowIPNonLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: validators::models::Host,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(NotAllow), ip(Must), conflict(Allow)))]
                    pub struct EmailIPAllowLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: std::net::IpAddr,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(NotAllow), ip(Must), local(Must), conflict(Allow)))]
                    pub struct EmailIPLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: std::net::IpAddr,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(NotAllow), ip(Must), local(NotAllow), conflict(Allow)))]
                    pub struct EmailIPNonLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: std::net::IpAddr,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(NotAllow), ip(NotAllow)))]
                    pub struct EmailNonIPAllowLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: String,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(NotAllow), ip(NotAllow), local(Must)))]
                    pub struct EmailNonIPLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: String,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*comment(NotAllow), ip(NotAllow), local(NotAllow)))]
                    pub struct EmailNonIPNonLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: String,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*))]
                    pub struct EmailAllowCommentAllowIPAllowLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: validators::models::Host,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*local(Must)))]
                    pub struct EmailAllowCommentAllowIPLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: validators::models::Host,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*local(NotAllow)))]
                    pub struct EmailAllowCommentAllowIPNonLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: validators::models::Host,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*ip(Must), conflict(Allow)))]
                    pub struct EmailAllowCommentIPAllowLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: std::net::IpAddr,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*ip(Must), local(Must), conflict(Allow)))]
                    pub struct EmailAllowCommentIPLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: std::net::IpAddr,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*ip(Must), local(NotAllow), conflict(Allow)))]
                    pub struct EmailAllowCommentIPNonLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: std::net::IpAddr,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*ip(NotAllow)))]
                    pub struct EmailAllowCommentNonIPAllowLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: String,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                        pub is_local: bool,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*ip(NotAllow), local(Must)))]
                    pub struct EmailAllowCommentNonIPLocal {
                        pub local_part: String,
                        pub need_quoted: bool,
                        pub domain_part: String,
                        pub comment_before_local_part: Option<String>,
                        pub comment_after_local_part: Option<String>,
                        pub comment_before_domain_part: Option<String>,
                        pub comment_after_domain_part: Option<String>,
                    }

                    #[derive(Validator)]
                    #[validator(email($($p($v),)*ip(NotAllow), local(NotAllow)))]
                    pub struct EmailAllowCommentNonIPNonLocal {
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
                        EmailAllowIPAllowLocal,
                        EmailAllowIPLocal,
                        EmailAllowIPNonLocal,
                        EmailIPAllowLocal,
                        EmailIPLocal,
                        EmailIPNonLocal,
                        EmailNonIPAllowLocal,
                        EmailNonIPLocal,
                        EmailNonIPNonLocal,
                        EmailAllowCommentAllowIPAllowLocal,
                        EmailAllowCommentAllowIPLocal,
                        EmailAllowCommentAllowIPNonLocal,
                        EmailAllowCommentIPAllowLocal,
                        EmailAllowCommentIPLocal,
                        EmailAllowCommentIPNonLocal,
                        EmailAllowCommentNonIPAllowLocal,
                        EmailAllowCommentNonIPLocal,
                        EmailAllowCommentNonIPNonLocal,
                    );
                }
            )*
        }
    }

    test! {
        {
            at_least_two_labels => Allow,
            non_ascii => Allow,
        },
        {
            at_least_two_labels => Must,
            non_ascii => Allow,
        },
        {
            at_least_two_labels => NotAllow,
            non_ascii => Allow,
        },
        {
            at_least_two_labels => Allow,
            non_ascii => NotAllow,
        },
        {
            at_least_two_labels => Must,
            non_ascii => NotAllow,
        },
        {
            at_least_two_labels => NotAllow,
            non_ascii => NotAllow,
        },
    }
}
