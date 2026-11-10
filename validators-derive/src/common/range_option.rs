use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use quote::ToTokens;
use syn::Meta;

use crate::common::{
    path_to_string,
    range::{Range, RangedNumber, range_equal},
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
                    // `Unlimited` takes no arguments, so do not ignore them silently.
                    "Unlimited" if matches!(meta, Meta::Path(_)) => return Ok(Self::Unlimited),
                    _ => (),
                }
            }
        }

        let path = meta.path();

        Err(syn::Error::new_spanned(
            path,
            format!(
                "expected `{path}(Inside(..)/Outside(..)/Unlimited)`",
                path = path_to_string(path)
            ),
        ))
    }
}

/// A range bound.
/// It becomes a suffixed literal (such as `1u8`) in the generated code, and it is shown without the suffix (such as `1`) in messages.
pub(crate) struct RangeBound {
    tokens: proc_macro2::TokenStream,
    text:   String,
}

impl RangeBound {
    #[inline]
    fn new<T: RangedNumber>(value: T) -> Self {
        Self {
            tokens: value.into_token_stream(), text: value.to_string()
        }
    }
}

impl ToTokens for RangeBound {
    #[inline]
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        self.tokens.to_tokens(token_stream);
    }
}

impl Display for RangeBound {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}

pub(crate) enum RangeTokenStream {
    Inside {
        min:       Option<RangeBound>,
        max:       Option<RangeBound>,
        inclusive: bool,
        equal:     bool,
    },
    Outside {
        min:       Option<RangeBound>,
        max:       Option<RangeBound>,
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
                min: min.map(RangeBound::new),
                max: max.map(RangeBound::new),
                inclusive,
                equal: range_equal(min, max, inclusive),
            },
            RangeOption::Outside {
                min,
                max,
                inclusive,
            } => Self::Outside {
                min: min.map(RangeBound::new),
                max: max.map(RangeBound::new),
                inclusive,
                equal: range_equal(min, max, inclusive),
            },
            RangeOption::Unlimited => Self::Unlimited,
        }
    }
}
