use syn::{Expr, Lit, LitBool, Meta};

use crate::common::{path_to_string, peel_expr_groups};

#[inline]
pub(crate) fn meta_2_bool(meta: &Meta) -> syn::Result<bool> {
    match &meta {
        Meta::NameValue(name_value) => {
            if let Expr::Lit(lit) = peel_expr_groups(&name_value.value)
                && let Lit::Bool(lit) = &lit.lit
            {
                return Ok(lit.value);
            }
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
