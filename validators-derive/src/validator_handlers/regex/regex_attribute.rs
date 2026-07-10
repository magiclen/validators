use quote::quote;
use regex::Regex;
use syn::{punctuated::Punctuated, spanned::Spanned, Expr, ExprLit, Lit, Meta, Token};

use crate::{
    common::{rocket_options::RocketOptions, serde_options::SerdeOptions},
    panic,
};

pub(crate) struct RegexAttribute {
    pub(crate) regex:          Expr,
    #[cfg_attr(not(feature = "serde"), allow(dead_code))]
    pub(crate) serde_options:  SerdeOptions,
    #[cfg_attr(not(feature = "rocket"), allow(dead_code))]
    pub(crate) rocket_options: RocketOptions,
}

impl RegexAttribute {
    pub(crate) fn build_from_meta(meta: &Meta) -> syn::Result<Self> {
        let correct_parameters = ["regex", "serde", "rocket"];

        let mut regex = None;
        let mut serde_options = SerdeOptions::default();
        let mut rocket_options = RocketOptions::default();

        match meta {
            Meta::Path(_) => (),
            Meta::NameValue(_) => {
                return Err(panic::attribute_incorrect_format(meta.path().get_ident().unwrap()));
            },
            Meta::List(list) => {
                let result =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                let mut regex_is_set = false;
                let mut serde_options_is_set = false;
                let mut rocket_options_is_set = false;

                let mut handler = |meta: &Meta| -> syn::Result<bool> {
                    if let Some(ident) = meta.path().get_ident() {
                        match ident.to_string().as_str() {
                            "regex" => {
                                let v = meta_2_regex_expr(meta)?;

                                if regex_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                regex_is_set = true;

                                regex = Some(v);

                                return Ok(true);
                            },
                            "serde" => {
                                let v = SerdeOptions::from_meta(meta)?;

                                if serde_options_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                serde_options_is_set = true;

                                serde_options = v;

                                return Ok(true);
                            },
                            "rocket" => {
                                let v = RocketOptions::from_meta(meta)?;

                                if rocket_options_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                rocket_options_is_set = true;

                                rocket_options = v;

                                return Ok(true);
                            },
                            _ => (),
                        }
                    }

                    Ok(false)
                };

                for p in result {
                    if !handler(&p)? {
                        return Err(panic::parameter_incorrect_format(
                            p.path(),
                            &correct_parameters,
                        ));
                    }
                }
            },
        }

        if let Some(regex) = regex {
            Ok(Self {
                regex,
                serde_options,
                rocket_options,
            })
        } else {
            Err(syn::Error::new(meta.path().span(), "the `regex` parameter is not set"))
        }
    }
}

fn expr_lit_2_regex_expr(lit: &ExprLit) -> syn::Result<Expr> {
    if let Lit::Str(lit) = &lit.lit {
        let s = lit.value();

        if let Err(error) = Regex::new(s.as_str()) {
            return Err(syn::Error::new(lit.span(), error));
        }

        return Ok(
            syn::parse2(quote! ( validators_prelude::regex::Regex::new(#s).unwrap() )).unwrap()
        );
    }

    Err(syn::Error::new(lit.span(), "expected `\"regex\"`"))
}

fn meta_2_regex_expr(meta: &Meta) -> syn::Result<Expr> {
    match meta {
        Meta::NameValue(name_value) => {
            return if let Expr::Lit(lit) = &name_value.value {
                expr_lit_2_regex_expr(lit)
            } else {
                Ok(name_value.value.clone())
            }
        },
        Meta::List(list) => {
            let expr: Expr = list.parse_args()?;

            return if let Expr::Lit(lit) = &expr { expr_lit_2_regex_expr(lit) } else { Ok(expr) };
        },
        Meta::Path(_) => (),
    }

    let path = meta.path();

    Err(syn::Error::new(path.span(), "expected `\"regex\"` or an expr"))
}
