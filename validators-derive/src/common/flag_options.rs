use syn::{Expr, Ident, Lit, Meta, Token, punctuated::Punctuated};

use crate::panic;

/// Parse a framework option pair like `serde(Serialize, Deserialize)` or `rocket(FromFormField, FromParam)` into two boolean flags.
///
/// The `feature_enabled` argument should be the result of `cfg!(feature = ...)` for the corresponding framework feature.
pub(crate) fn meta_2_flags(
    meta: &Meta,
    feature_name: &str,
    feature_enabled: bool,
    first_name: &str,
    second_name: &str,
) -> syn::Result<(bool, bool)> {
    match meta {
        Meta::Path(path) => {
            if feature_enabled {
                Ok((true, true))
            } else {
                Err(syn::Error::new_spanned(
                    path,
                    format!("the `{feature_name}` feature is not enabled"),
                ))
            }
        },
        Meta::NameValue(name_value) => {
            if let Expr::Lit(lit) = &name_value.value
                && let Lit::Bool(lit) = &lit.lit
            {
                let b = lit.value;

                if b && !feature_enabled {
                    return Err(syn::Error::new_spanned(
                        lit,
                        format!(
                            "the `{feature_name}` feature is not enabled, so the value cannot be \
                             `true`"
                        ),
                    ));
                }

                return Ok((b, b));
            }

            Err(syn::Error::new_spanned(&name_value.value, "expected a bool"))
        },
        Meta::List(list) => {
            let result =
                list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_separated_nonempty)?;

            let mut first = false;
            let mut second = false;

            for p in result {
                let name = p.to_string();

                if name == first_name {
                    if first {
                        return Err(panic::parameter_reset(&p));
                    }

                    first = true;
                } else if name == second_name {
                    if second {
                        return Err(panic::parameter_reset(&p));
                    }

                    second = true;
                } else {
                    return Err(syn::Error::new_spanned(
                        &p,
                        format!("expected {first_name}/{second_name}"),
                    ));
                }

                if !feature_enabled {
                    return Err(syn::Error::new_spanned(
                        &p,
                        format!(
                            "cannot implement `{p}` because the `{feature_name}` feature is not \
                             enabled"
                        ),
                    ));
                }
            }

            Ok((first, second))
        },
    }
}
