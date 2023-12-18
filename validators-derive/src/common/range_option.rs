use std::{fmt::Display, str::FromStr};

use syn::{spanned::Spanned, Meta};

use crate::common::{
    path_to_string,
    range::{range_equal, Range, RangedNumber},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum RangeOption<T: FromStr + Copy + PartialOrd + Display>
where
    T::Err: Display, {
    Inside { min: Option<T>, max: Option<T>, inclusive: bool },
    Outside { min: Option<T>, max: Option<T>, inclusive: bool },
    Unlimited,
}

impl<T: RangedNumber> RangeOption<T>
where
    T::Err: Display,
{
    #[inline]
    pub(crate) fn from_meta(meta: &Meta) -> syn::Result<Self> {
        debug_assert!(meta.path().is_ident("range"));

        if let Meta::List(list) = meta {
            let meta: Meta = list.parse_args()?;

            if let Some(ident) = meta.path().get_ident() {
                match ident.to_string().as_str() {
                    "Inside" => {
                        let Range {
                            min,
                            max,
                            inclusive,
                        } = Range::from_meta(&meta)?;

                        return Ok(Self::Inside {
                            min,
                            max,
                            inclusive,
                        });
                    },
                    "Outside" => {
                        let Range {
                            min,
                            max,
                            inclusive,
                        } = Range::from_meta(&meta)?;

                        return Ok(Self::Outside {
                            min,
                            max,
                            inclusive,
                        });
                    },
                    "Unlimited" => return Ok(Self::Unlimited),
                    _ => (),
                }
            }
        }

        let path = meta.path();

        Err(syn::Error::new(
            path.span(),
            format!(
                "expected `{path}(Inside(..)/Outside(..)/Unlimited)`",
                path = path_to_string(path)
            ),
        ))
    }
}

pub(crate) enum RangeTokenStream {
    Inside {
        min:       Option<proc_macro2::TokenStream>,
        max:       Option<proc_macro2::TokenStream>,
        inclusive: bool,
        equal:     bool,
    },
    Outside {
        min:       Option<proc_macro2::TokenStream>,
        max:       Option<proc_macro2::TokenStream>,
        inclusive: bool,
        equal:     bool,
    },
    Unlimited,
}

impl RangeTokenStream {
    #[inline]
    pub(crate) fn inside(&self) -> bool {
        matches!(self, Self::Inside { .. })
    }
}

impl<T: RangedNumber> From<RangeOption<T>> for RangeTokenStream
where
    T::Err: Display,
{
    fn from(value: RangeOption<T>) -> Self {
        match value {
            RangeOption::Inside {
                min,
                max,
                inclusive,
            } => Self::Inside {
                min: min.map(|a| a.into_token_stream()),
                max: max.map(|a| a.into_token_stream()),
                inclusive,
                equal: range_equal(min, max, inclusive),
            },
            RangeOption::Outside {
                min,
                max,
                inclusive,
            } => Self::Outside {
                min: min.map(|a| a.into_token_stream()),
                max: max.map(|a| a.into_token_stream()),
                inclusive,
                equal: range_equal(min, max, inclusive),
            },
            RangeOption::Unlimited => Self::Unlimited,
        }
    }
}
