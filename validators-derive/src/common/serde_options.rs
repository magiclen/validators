use syn::{punctuated::Punctuated, spanned::Spanned, Expr, Ident, Lit, Meta, Token};

use crate::panic;

#[derive(Debug)]
pub(crate) struct SerdeOptions {
    pub(crate) serialize:   bool,
    pub(crate) deserialize: bool,
}

impl Default for SerdeOptions {
    #[inline]
    fn default() -> Self {
        Self {
            serialize: true, deserialize: true
        }
    }
}

impl SerdeOptions {
    #[inline]
    pub(crate) fn from_meta(meta: &Meta) -> syn::Result<Self> {
        debug_assert!(meta.path().is_ident("serde"));

        match meta {
            Meta::Path(_) => {
                #[cfg(feature = "serde")]
                {
                    Ok(Self::default())
                }
                #[cfg(not(feature = "serde"))]
                {
                    Err(syn::Error::new(meta.path().span(), "the `serde` feature is not enabled"))
                }
            },
            Meta::NameValue(name_value) => {
                if let Expr::Lit(lit) = &name_value.value {
                    if let Lit::Bool(lit) = &lit.lit {
                        let b = lit.value;

                        #[cfg(not(feature = "serde"))]
                        if b {
                            return Err(syn::Error::new(
                                lit.span(),
                                "the `serde` feature is not enabled, so the value cannot be `true`",
                            ));
                        }

                        return Ok(Self {
                            serialize: b, deserialize: b
                        });
                    }
                }

                Err(syn::Error::new(name_value.value.span(), "expected a bool"))
            },
            Meta::List(list) => {
                let result =
                    list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_separated_nonempty)?;

                let mut serialize = false;
                let mut deserialize = false;

                for p in result {
                    match p.to_string().as_str() {
                        "Serialize" => {
                            if serialize {
                                return Err(panic::parameter_reset(&p));
                            }

                            serialize = true;
                        },
                        "Deserialize" => {
                            if deserialize {
                                return Err(panic::parameter_reset(&p));
                            }

                            deserialize = true;
                        },
                        _ => {
                            return Err(syn::Error::new(
                                p.span(),
                                "expected Serialize/Deserialize",
                            ));
                        },
                    }

                    #[cfg(not(feature = "serde"))]
                    {
                        let _ = serialize;
                        let _ = deserialize;

                        return Err(syn::Error::new(
                            p.span(),
                            format!(
                                "cannot implement `{p}` because the `serde` feature is not enabled"
                            ),
                        ));
                    }
                }

                Ok(Self {
                    serialize,
                    deserialize,
                })
            },
        }
    }
}
