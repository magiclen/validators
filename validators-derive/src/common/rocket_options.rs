use syn::{punctuated::Punctuated, spanned::Spanned, Expr, Ident, Lit, Meta, Token};

use crate::panic;

#[derive(Debug)]
pub(crate) struct RocketOptions {
    pub(crate) from_form_field: bool,
    pub(crate) from_param:      bool,
}

impl Default for RocketOptions {
    #[inline]
    fn default() -> Self {
        Self {
            from_form_field: true, from_param: true
        }
    }
}

impl RocketOptions {
    #[inline]
    pub(crate) fn from_meta(meta: &Meta) -> syn::Result<Self> {
        debug_assert!(meta.path().is_ident("rocket"));

        match meta {
            Meta::Path(_) => {
                #[cfg(feature = "rocket")]
                {
                    Ok(Self::default())
                }
                #[cfg(not(feature = "rocket"))]
                {
                    Err(syn::Error::new(meta.path().span(), "the `rocket` feature is not enabled"))
                }
            },
            Meta::NameValue(name_value) => {
                if let Expr::Lit(lit) = &name_value.value {
                    if let Lit::Bool(lit) = &lit.lit {
                        let b = lit.value;

                        #[cfg(not(feature = "rocket"))]
                        if b {
                            return Err(syn::Error::new(
                                lit.span(),
                                "the `rocket` feature is not enabled, so the value cannot be \
                                 `true`",
                            ));
                        }

                        return Ok(Self {
                            from_form_field: b, from_param: b
                        });
                    }
                }

                Err(syn::Error::new(name_value.value.span(), "expected a bool"))
            },
            Meta::List(list) => {
                let result =
                    list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_separated_nonempty)?;

                let mut from_form_field = false;
                let mut from_param = false;

                for p in result {
                    match p.to_string().as_str() {
                        "FromFormField" => {
                            if from_form_field {
                                return Err(panic::parameter_reset(&p));
                            }

                            from_form_field = true;
                        },
                        "FromParam" => {
                            if from_param {
                                return Err(panic::parameter_reset(&p));
                            }

                            from_param = true;
                        },
                        _ => {
                            return Err(syn::Error::new(
                                p.span(),
                                "expected FromFormField/FromParam",
                            ));
                        },
                    }

                    #[cfg(not(feature = "rocket"))]
                    {
                        let _ = from_form_field;
                        let _ = from_param;

                        return Err(syn::Error::new(
                            p.span(),
                            format!(
                                "cannot implement `{p}` because the `rocket` feature is not \
                                 enabled"
                            ),
                        ));
                    }
                }

                Ok(Self {
                    from_form_field,
                    from_param,
                })
            },
        }
    }
}
