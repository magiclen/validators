use proc_macro2::Ident;
use syn::{spanned::Spanned, Expr, Meta};

use crate::common::path_to_string;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum TriAllow {
    Must,
    Allow,
    Disallow,
}

impl TriAllow {
    #[inline]
    pub(crate) const fn allow(self) -> bool {
        match self {
            Self::Must => true,
            Self::Allow => true,
            Self::Disallow => false,
        }
    }

    #[inline]
    pub(crate) const fn disallow(self) -> bool {
        match self {
            Self::Must => false,
            Self::Allow => false,
            Self::Disallow => true,
        }
    }

    #[inline]
    pub(crate) const fn must(self) -> bool {
        match self {
            Self::Must => true,
            Self::Allow => false,
            Self::Disallow => false,
        }
    }
}

impl TriAllow {
    #[inline]
    pub(crate) fn from_ident(ident: &Ident) -> syn::Result<Self> {
        let ident_string = ident.to_string();

        match ident_string.as_str() {
            "Must" => Ok(Self::Must),
            "Allow" => Ok(Self::Allow),
            "Disallow" => Ok(Self::Disallow),
            _ => Err(syn::Error::new(ident.span(), "expected Must/Allow/Disallow")),
        }
    }

    #[inline]
    pub(crate) fn from_meta(meta: &Meta) -> syn::Result<Self> {
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
                "expected `{path} = Must/Allow/Disallow` or `{path}(Must/Allow/Disallow)`",
                path = path_to_string(path)
            ),
        ))
    }
}
