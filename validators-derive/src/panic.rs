use core::{
    fmt,
    fmt::{Display, Formatter},
};
use std::fmt::Debug;

use quote::ToTokens;
use syn::{DeriveInput, Ident, Path};

use crate::{Validator, common::path_to_string};

struct DisplayStringSlice<'a>(&'a [&'static str]);

impl<'a> Display for DisplayStringSlice<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.0.is_empty() {
            for &s in self.0 {
                f.write_str("\n    ")?;
                f.write_str(s)?;
            }
        }

        Ok(())
    }
}

struct DisplayValidators;

impl Display for DisplayValidators {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for t in &Validator::VARIANTS[..Validator::VARIANTS.len() - 1] {
            f.write_str("\n    ")?;
            f.write_fmt(format_args!("{t:?}"))?;
        }

        Ok(())
    }
}

#[inline]
pub(crate) fn derive_attribute_not_set_up_yet(ast: &DeriveInput) -> syn::Error {
    syn::Error::new_spanned(
        &ast.ident,
        "you are using `Validator` in the `derive` attribute, but it has not been set up yet",
    )
}

#[inline]
pub(crate) fn validator_format_incorrect(node: impl ToTokens) -> syn::Error {
    syn::Error::new_spanned(node, "you are using an incorrect format of the `Validator` attribute")
}

#[inline]
pub(crate) fn unsupported_validator(name: &Path) -> syn::Error {
    syn::Error::new_spanned(
        name,
        format!(
            "unsupported validator `{}`, available validators:{}",
            path_to_string(name),
            DisplayValidators
        ),
    )
}

#[inline]
pub(crate) fn validator_only_one_at_a_time(node: impl ToTokens) -> syn::Error {
    syn::Error::new_spanned(node, "`validator` can be used only one at a time")
}

#[inline]
pub(crate) fn validator_for_specific_item(name: &Path, item: impl Debug) -> syn::Error {
    syn::Error::new_spanned(
        name,
        format!("the `{}` validator should be implemented for\n{item:#?}", path_to_string(name)),
    )
}

#[inline]
pub(crate) fn attribute_incorrect_format(name: &Path) -> syn::Error {
    syn::Error::new_spanned(
        name,
        format!("you are using an incorrect format of the `{}` attribute", path_to_string(name)),
    )
}

#[inline]
pub(crate) fn parameter_reset(name: &Ident) -> syn::Error {
    syn::Error::new_spanned(name, format!("you are trying to reset the `{name}` parameter"))
}

#[inline]
pub(crate) fn parameter_incorrect_format(
    path: &Path,
    correct_parameter: &[&'static str],
) -> syn::Error {
    let name = path_to_string(path);

    syn::Error::new_spanned(
        path,
        format!(
            "unsupported parameter `{name}`, available parameters:{}",
            DisplayStringSlice(correct_parameter)
        ),
    )
}
