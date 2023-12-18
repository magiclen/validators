use syn::{spanned::Spanned, Expr, Ident, Meta};

use crate::common::path_to_string;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum CaseOption {
    Any,
    Upper,
    Lower,
}

impl CaseOption {
    #[inline]
    pub(crate) const fn any(self) -> bool {
        match self {
            Self::Any => true,
            Self::Upper => false,
            Self::Lower => false,
        }
    }

    #[inline]
    pub(crate) const fn upper(self) -> bool {
        match self {
            Self::Any => true,
            Self::Upper => true,
            Self::Lower => false,
        }
    }

    #[inline]
    pub(crate) const fn lower(self) -> bool {
        match self {
            Self::Any => true,
            Self::Upper => false,
            Self::Lower => true,
        }
    }
}

impl CaseOption {
    #[inline]
    pub(crate) fn from_ident(ident: &Ident) -> syn::Result<Self> {
        let ident_string = ident.to_string();

        match ident_string.as_str() {
            "Any" => Ok(Self::Any),
            "Upper" => Ok(Self::Upper),
            "Lower" => Ok(Self::Lower),
            _ => Err(syn::Error::new(ident.span(), "expected Any/Upper/Lower")),
        }
    }

    #[inline]
    pub(crate) fn from_meta(meta: &Meta) -> syn::Result<Self> {
        debug_assert!(meta.path().is_ident("case"));

        match meta {
            Meta::NameValue(name_value) => {
                if let Expr::Path(path) = &name_value.value {
                    if let Some(ident) = path.path.get_ident() {
                        return Self::from_ident(ident);
                    }
                }
            },
            Meta::List(list) => {
                if let Ok(ident) = list.parse_args::<Ident>() {
                    return Self::from_ident(&ident);
                }
            },
            _ => (),
        }

        let path = meta.path();

        Err(syn::Error::new(
            path.span(),
            format!(
                "expected `{path} = Any/Upper/Lower` or `{path}(Any/Upper/Lower)`",
                path = path_to_string(path)
            ),
        ))
    }
}
