use syn::{spanned::Spanned, Expr, Lit, LitByte, Meta};

use crate::common::path_to_string;

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
                                if let Expr::Lit(lit) = &name_value.value {
                                    if let Lit::Byte(lit) = &lit.lit {
                                        return Ok(Self::Must(lit.value()));
                                    }
                                }
                            },
                            "Allow" => {
                                if let Expr::Lit(lit) = &name_value.value {
                                    if let Lit::Byte(lit) = &lit.lit {
                                        return Ok(Self::Allow(lit.value()));
                                    }
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

                                return Ok(Self::Must(c.value()));
                            },
                            "Allow" => {
                                let c: LitByte = list.parse_args()?;

                                return Ok(Self::Allow(c.value()));
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

        Err(syn::Error::new(
            path.span(),
            format!(
                "expected `{path}(Must(b'-')/Allow(b'-')/Disallow)`",
                path = path_to_string(path)
            ),
        ))
    }
}
