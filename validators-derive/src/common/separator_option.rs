use syn::{Expr, Lit, LitByte, Meta};

use crate::common::{path_to_string, peel_expr_groups};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum SeparatorOption {
    Must(u8),
    Allow(u8),
    Disallow,
}

impl SeparatorOption {
    #[inline]
    pub(crate) const fn allow(self) -> Option<u8> {
        match self {
            Self::Must(c) | Self::Allow(c) => Some(c),
            Self::Disallow => None,
        }
    }

    #[inline]
    pub(crate) const fn disallow(self) -> bool {
        match self {
            Self::Must(_) => false,
            Self::Allow(_) => false,
            Self::Disallow => true,
        }
    }

    #[inline]
    pub(crate) const fn must(self) -> Option<u8> {
        match self {
            Self::Must(c) => Some(c),
            Self::Allow(_) => None,
            Self::Disallow => None,
        }
    }
}

impl SeparatorOption {
    fn parse_separator(lit: &LitByte) -> syn::Result<u8> {
        let separator = lit.value();

        if separator.is_ascii() {
            Ok(separator)
        } else {
            Err(syn::Error::new_spanned(lit, "expected an ASCII separator"))
        }
    }

    #[inline]
    pub(crate) fn from_meta(meta: &Meta) -> syn::Result<Self> {
        debug_assert!(meta.path().is_ident("separator"));

        if let Meta::List(list) = meta {
            let meta: Meta = list.parse_args()?;

            match &meta {
                Meta::NameValue(name_value) => {
                    if let Some(ident) = name_value.path.get_ident() {
                        match ident.to_string().as_str() {
                            "Must" => {
                                if let Expr::Lit(lit) = peel_expr_groups(&name_value.value)
                                    && let Lit::Byte(lit) = &lit.lit
                                {
                                    return Ok(Self::Must(Self::parse_separator(lit)?));
                                }
                            },
                            "Allow" => {
                                if let Expr::Lit(lit) = peel_expr_groups(&name_value.value)
                                    && let Lit::Byte(lit) = &lit.lit
                                {
                                    return Ok(Self::Allow(Self::parse_separator(lit)?));
                                }
                            },
                            _ => (),
                        }
                    }
                },
                Meta::List(list) => {
                    if let Some(ident) = list.path.get_ident() {
                        match ident.to_string().as_str() {
                            "Must" => {
                                let c: LitByte = list.parse_args()?;

                                return Ok(Self::Must(Self::parse_separator(&c)?));
                            },
                            "Allow" => {
                                let c: LitByte = list.parse_args()?;

                                return Ok(Self::Allow(Self::parse_separator(&c)?));
                            },
                            _ => (),
                        }
                    }
                },
                Meta::Path(path) => {
                    if path.is_ident("Disallow") {
                        return Ok(Self::Disallow);
                    }
                },
            }
        }

        let path = meta.path();

        Err(syn::Error::new_spanned(
            path,
            format!(
                "expected `{path}(Must(b'-')/Allow(b'-')/Disallow)`",
                path = path_to_string(path)
            ),
        ))
    }
}
