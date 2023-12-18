use std::{fmt::Display, ops::Add, str::FromStr};

use quote::ToTokens;
use syn::{punctuated::Punctuated, spanned::Spanned, Meta, Token};
#[cfg(feature = "full")]
use syn::{Expr, ExprRange, RangeLimits};

#[cfg(feature = "full")]
use crate::common::number::expr_lit_2_number;
use crate::{
    common::{boolean::meta_2_bool, number::meta_2_number},
    panic,
};

pub(crate) trait RangedNumber:
    FromStr + Copy + PartialOrd + Display + Add + ToTokens + 'static {
    const IS_FLOAT: bool;

    fn inc(self) -> Option<Self>;
}

macro_rules! impl_int {
    (@unit $ty:ty) => {
        impl RangedNumber for $ty {
            const IS_FLOAT: bool = false;

            fn inc(self) -> Option<Self> {
                self.checked_add(1)
            }
        }
    };
    ($($ty:ty),*) => {
        $( impl_int!(@unit $ty); )*
    };
}

macro_rules! impl_float {
    (@unit $ty:ty) => {
        impl RangedNumber for $ty {
            const IS_FLOAT: bool = true;

            fn inc(self) -> Option<Self> {
                Some(self + 1.0)
            }
        }
    };
    ($($ty:ty),*) => {
        $( impl_float!(@unit $ty); )*
    };
}

impl_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_float!(f32, f64);

#[derive(Debug, Clone)]
pub(crate) struct Range<T: RangedNumber>
where
    T::Err: Display, {
    pub(crate) min:       Option<T>,
    pub(crate) max:       Option<T>,
    pub(crate) inclusive: bool,
}

impl<T: RangedNumber> Range<T>
where
    T::Err: Display,
{
    #[inline]
    pub(crate) const fn new() -> Self {
        Self {
            min: None, max: None, inclusive: false
        }
    }
}

impl<T: RangedNumber> Range<T>
where
    T::Err: Display,
{
    #[cfg(feature = "full")]
    #[inline]
    pub(crate) fn from_expr_range(range: &ExprRange) -> syn::Result<Self> {
        let mut min = None;
        let mut max = None;
        let mut inclusive = true;

        if let Some(expr) = &range.start {
            if let Expr::Lit(lit) = expr.as_ref() {
                min = Some(expr_lit_2_number(lit)?);
            } else {
                return Err(syn::Error::new(expr.span(), "not a literal"));
            }
        }

        if let Some(expr) = &range.end {
            if let Expr::Lit(lit) = expr.as_ref() {
                max = Some(expr_lit_2_number(lit)?);
            } else {
                return Err(syn::Error::new(expr.span(), "not a literal"));
            }

            if let RangeLimits::HalfOpen(_) = range.limits {
                inclusive = false;
            }
        }

        Ok(Self {
            min,
            max,
            inclusive,
        })
    }

    #[inline]
    pub(crate) fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let correct_parameters = ["min", "max", "inclusive"];

        let mut min = None;
        let mut max = None;
        let mut inclusive = true;

        match meta {
            Meta::Path(_) => {
                return Err(panic::attribute_incorrect_format(meta.path().get_ident().unwrap()));
            },
            Meta::NameValue(name_value) => {
                #[cfg(feature = "full")]
                if let Expr::Range(range) = &name_value.value {
                    return Self::from_expr_range(range);
                }

                let _ = name_value;

                return Err(panic::attribute_incorrect_format(meta.path().get_ident().unwrap()));
            },
            Meta::List(list) => {
                #[cfg(feature = "full")]
                if let Ok(range) = list.parse_args::<ExprRange>() {
                    return Self::from_expr_range(&range);
                }

                let result =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                let mut min_is_set = false;
                let mut max_is_set = false;
                let mut inclusive_is_set = false;

                let mut handler = |meta: &Meta| -> syn::Result<bool> {
                    if let Some(ident) = meta.path().get_ident() {
                        match ident.to_string().as_str() {
                            "min" => {
                                let v = meta_2_number(meta)?;

                                if min_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                min_is_set = true;

                                min = Some(v);

                                return Ok(true);
                            },
                            "max" => {
                                let v = meta_2_number(meta)?;

                                if max_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                max_is_set = true;

                                max = Some(v);

                                return Ok(true);
                            },
                            "inclusive" => {
                                let v = meta_2_bool(meta)?;

                                if inclusive_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                inclusive_is_set = true;

                                inclusive = v;

                                return Ok(true);
                            },
                            _ => (),
                        }
                    }

                    Ok(false)
                };

                for p in result {
                    if !handler(&p)? {
                        return Err(panic::parameter_incorrect_format(
                            p.path(),
                            &correct_parameters,
                        ));
                    }
                }
            },
        }

        if let Some(min) = min {
            if let Some(max) = max {
                if inclusive {
                    if min > max {
                        return Err(syn::Error::new(
                            meta.path().span(),
                            format!("{min} > {max} (min > max)"),
                        ));
                    }
                } else if min >= max {
                    return Err(syn::Error::new(
                        meta.path().span(),
                        format!("{min} >= {max} (min >= max)"),
                    ));
                }
            }
        }

        Ok(Self {
            min,
            max,
            inclusive,
        })
    }
}
