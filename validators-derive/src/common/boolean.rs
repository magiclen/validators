use syn::{Expr, Lit, LitBool, Meta};

use crate::common::path_to_string;

#[inline]
pub(crate) fn meta_2_bool(meta: &Meta) -> syn::Result<bool> {
    match &meta {
        Meta::NameValue(name_value) => match &name_value.value {
            Expr::Lit(lit) => {
                if let Lit::Bool(lit) = &lit.lit {
                    return Ok(lit.value);
                }
            },
            Expr::Group(group) => {
                // should not use this, but macro rules will end up here...
                if let Expr::Lit(lit) = group.expr.as_ref()
                    && let Lit::Bool(lit) = &lit.lit
                {
                    return Ok(lit.value);
                }
            },
            _ => (),
        },
        Meta::List(list) => {
            let lit = list.parse_args::<LitBool>()?;

            return Ok(lit.value);
        },
        Meta::Path(_) => (),
    }

    let path = meta.path();

    Err(syn::Error::new_spanned(
        path,
        format!("expected `{path} = false` or `{path}(false)`", path = path_to_string(path)),
    ))
}
