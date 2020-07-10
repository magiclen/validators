use crate::syn::{Ident, Path};

use crate::validators_options::*;

pub trait SynOption {
    fn from_ident(ident: &Ident) -> Self;
    fn to_path(&self) -> Path;
}

impl SynOption for ValidatorOption {
    #[inline]
    fn from_ident(ident: &Ident) -> ValidatorOption {
        if ident == "Must" {
            ValidatorOption::Must
        } else if ident == "Allow" {
            ValidatorOption::Allow
        } else if ident == "NotAllow" {
            ValidatorOption::NotAllow
        } else {
            panic!(
                "A validator option should be `Must`, `Allow` or `NotAllow` instead of `{}`.",
                ident
            )
        }
    }

    #[inline]
    fn to_path(&self) -> Path {
        match self {
            ValidatorOption::Must => {
                syn::parse2(quote! { validators_prelude::ValidatorOption::Must }).unwrap()
            }
            ValidatorOption::Allow => {
                syn::parse2(quote! { validators_prelude::ValidatorOption::Allow }).unwrap()
            }
            ValidatorOption::NotAllow => {
                syn::parse2(quote! { validators_prelude::ValidatorOption::NotAllow }).unwrap()
            }
        }
    }
}
