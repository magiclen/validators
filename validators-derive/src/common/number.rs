use std::{fmt::Display, str::FromStr};

use syn::{spanned::Spanned, Expr, ExprLit, Lit, LitFloat, LitInt, Meta, UnOp};

use crate::common::path_to_string;

pub(crate) fn expr_lit_2_number<T: FromStr>(lit: &ExprLit) -> syn::Result<T>
where
    T::Err: Display, {
    match &lit.lit {
        Lit::Int(lit) => {
            return lit.base10_parse();
        },
        Lit::Float(lit) => {
            return lit.base10_parse();
        },
        _ => (),
    }

    Err(syn::Error::new(lit.span(), "expected a number"))
}

pub(crate) fn expr_2_number<T: FromStr>(expr: &Expr) -> syn::Result<T>
where
    T::Err: Display, {
    match expr {
        Expr::Lit(lit) => return expr_lit_2_number(lit),
        Expr::Unary(unary) => {
            if let UnOp::Neg(_) = unary.op {
                if let Expr::Lit(lit) = unary.expr.as_ref() {
                    match &lit.lit {
                        Lit::Int(lit) => {
                            let s = format!("-{}", lit.base10_digits());

                            let n = <T as FromStr>::from_str(&s)
                                .map_err(|err| syn::Error::new(lit.span(), err))?;

                            return Ok(n);
                        },
                        Lit::Float(lit) => {
                            let s = format!("-{}", lit.base10_digits());

                            let n = <T as FromStr>::from_str(&s)
                                .map_err(|err| syn::Error::new(lit.span(), err))?;

                            return Ok(n);
                        },
                        _ => (),
                    }
                }
            }
        },
        Expr::Group(group) => {
            // should not use this, but macro rules will end up here...
            if let Expr::Lit(lit) = group.expr.as_ref() {
                return expr_lit_2_number(lit);
            }
        },
        _ => (),
    }

    Err(syn::Error::new(expr.span(), "expected a number"))
}

#[inline]
pub(crate) fn meta_2_number<T: FromStr>(meta: &Meta) -> syn::Result<T>
where
    T::Err: Display, {
    match &meta {
        Meta::NameValue(name_value) => {
            return expr_2_number(&name_value.value);
        },
        Meta::List(list) => match list.parse_args::<LitInt>() {
            Ok(lit) => {
                return lit.base10_parse();
            },
            Err(_) => {
                let lit = list.parse_args::<LitFloat>()?;

                return lit.base10_parse();
            },
        },
        Meta::Path(_) => (),
    }

    let path = meta.path();

    Err(syn::Error::new(
        path.span(),
        format!("expected `{path} = 0` or `{path}(0)`", path = path_to_string(path)),
    ))
}
