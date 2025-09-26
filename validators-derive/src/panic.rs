use core::{
    fmt,
    fmt::{Display, Formatter},
};
use std::fmt::Debug;

use proc_macro2::Span;
use syn::{spanned::Spanned, Ident, Path};

use crate::{common::path_to_string, Validator};

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
pub(crate) fn derive_attribute_not_set_up_yet() -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        "you are using `Validator` in the `derive` attribute, but it has not been set up yet",
    )
}

#[inline]
pub(crate) fn validator_format_incorrect(span: Span) -> syn::Error {
    syn::Error::new(span, "you are using an incorrect format of the `Validator` attribute")
}

#[inline]
pub(crate) fn unsupported_validator(name: &Path) -> syn::Error {
    let span = name.span();

    match name.get_ident() {
        Some(name) => syn::Error::new(
            span,
            format!("unsupported validator `{name}`, available validators:{DisplayValidators}"),
        ),
        None => {
            let name = path_to_string(name);

            syn::Error::new(
                span,
                format!("unsupported validator `{name}`, available validators:{DisplayValidators}"),
            )
        },
    }
}

#[inline]
pub(crate) fn validator_only_one_at_a_time(span: Span) -> syn::Error {
    syn::Error::new(span, "`validator` can be used only one at a time")
}

#[inline]
pub(crate) fn validator_for_specific_item(name: &Ident, item: impl Debug) -> syn::Error {
    syn::Error::new(
        name.span(),
        format!("the `{name}` validator should be implemented for\n{item:#?}"),
    )
}

#[inline]
pub(crate) fn attribute_incorrect_format(name: &Ident) -> syn::Error {
    syn::Error::new(
        name.span(),
        format!("you are using an incorrect format of the `{name}` attribute",),
    )
}

#[inline]
pub(crate) fn parameter_reset(name: &Ident) -> syn::Error {
    syn::Error::new(name.span(), format!("you are trying to reset the `{name}` parameter"))
}

#[inline]
pub(crate) fn parameter_incorrect_format(
    path: &Path,
    correct_parameter: &[&'static str],
) -> syn::Error {
    let name = path_to_string(path);

    syn::Error::new(
        path.span(),
        format!(
            "unsupported parameter `{name}`, available parameters:{}",
            DisplayStringSlice(correct_parameter)
        ),
    )
}
