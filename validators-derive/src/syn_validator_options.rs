use crate::syn::{Expr, Lit, Meta, NestedMeta};

use crate::panic;
use crate::validators_options::*;

pub trait SynOption: Sized {
    fn from_meta(meta_name: &str, meta: &Meta, is_set: &mut bool, correct_usage: &[&str]) -> Self;
    fn to_expr(&self) -> Expr;
}

impl SynOption for ValidatorOption {
    fn from_meta(meta_name: &str, meta: &Meta, is_set: &mut bool, correct_usage: &[&str]) -> Self {
        if let Meta::List(list) = meta {
            if *is_set {
                panic::reset_parameter(meta_name);
            }

            *is_set = true;

            if list.nested.len() != 1 {
                panic::parameter_incorrect_format(meta_name, &correct_usage);
            }

            let p = list.nested.iter().next().unwrap();

            if let NestedMeta::Meta(meta) = p {
                if let Meta::Path(path) = meta {
                    if let Some(ident) = path.get_ident() {
                        if ident == "Allow" {
                            ValidatorOption::Allow
                        } else if ident == "Must" {
                            ValidatorOption::Must
                        } else if ident == "NotAllow" {
                            ValidatorOption::NotAllow
                        } else {
                            panic::parameter_incorrect_format(meta_name, &correct_usage);
                        }
                    } else {
                        panic::parameter_incorrect_format(meta_name, &correct_usage);
                    }
                } else {
                    panic::parameter_incorrect_format(meta_name, &correct_usage);
                }
            } else {
                panic::parameter_incorrect_format(meta_name, &correct_usage);
            }
        } else {
            panic::parameter_incorrect_format(meta_name, &correct_usage);
        }
    }

    #[inline]
    fn to_expr(&self) -> Expr {
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

impl SynOption for ValidatorCaseOption {
    fn from_meta(meta_name: &str, meta: &Meta, is_set: &mut bool, correct_usage: &[&str]) -> Self {
        if let Meta::List(list) = meta {
            if *is_set {
                panic::reset_parameter(meta_name);
            }

            *is_set = true;

            if list.nested.len() != 1 {
                panic::parameter_incorrect_format(meta_name, &correct_usage);
            }

            let p = list.nested.iter().next().unwrap();

            if let NestedMeta::Meta(meta) = p {
                if let Meta::Path(path) = meta {
                    if let Some(ident) = path.get_ident() {
                        if ident == "Any" {
                            ValidatorCaseOption::Any
                        } else if ident == "Upper" {
                            ValidatorCaseOption::Upper
                        } else if ident == "Lower" {
                            ValidatorCaseOption::Lower
                        } else {
                            panic::parameter_incorrect_format(meta_name, &correct_usage);
                        }
                    } else {
                        panic::parameter_incorrect_format(meta_name, &correct_usage);
                    }
                } else {
                    panic::parameter_incorrect_format(meta_name, &correct_usage);
                }
            } else {
                panic::parameter_incorrect_format(meta_name, &correct_usage);
            }
        } else {
            panic::parameter_incorrect_format(meta_name, &correct_usage);
        }
    }

    #[inline]
    fn to_expr(&self) -> Expr {
        match self {
            ValidatorCaseOption::Any => {
                syn::parse2(quote! { validators_prelude::ValidatorCaseOption::Any }).unwrap()
            }
            ValidatorCaseOption::Upper => {
                syn::parse2(quote! { validators_prelude::ValidatorCaseOption::Upper }).unwrap()
            }
            ValidatorCaseOption::Lower => {
                syn::parse2(quote! { validators_prelude::ValidatorCaseOption::Lower }).unwrap()
            }
        }
    }
}

impl SynOption for ValidatorSeparatorOption {
    fn from_meta(meta_name: &str, meta: &Meta, is_set: &mut bool, correct_usage: &[&str]) -> Self {
        if let Meta::List(list) = meta {
            if *is_set {
                panic::reset_parameter(meta_name);
            }

            *is_set = true;

            if list.nested.len() != 1 {
                panic::parameter_incorrect_format(meta_name, &correct_usage);
            }

            let p = list.nested.iter().next().unwrap();

            if let NestedMeta::Meta(meta) = p {
                if let Some(ident) = meta.path().get_ident() {
                    if ident == "Allow" {
                        ValidatorSeparatorOption::Allow(fetch_separator(
                            meta_name,
                            meta,
                            correct_usage,
                        ))
                    } else if ident == "Must" {
                        ValidatorSeparatorOption::Must(fetch_separator(
                            meta_name,
                            meta,
                            correct_usage,
                        ))
                    } else if ident == "NotAllow" {
                        ValidatorSeparatorOption::NotAllow
                    } else {
                        panic::parameter_incorrect_format(meta_name, &correct_usage);
                    }
                } else {
                    panic::parameter_incorrect_format(meta_name, &correct_usage);
                }
            } else {
                panic::parameter_incorrect_format(meta_name, &correct_usage);
            }
        } else {
            panic::parameter_incorrect_format(meta_name, &correct_usage);
        }
    }

    #[inline]
    fn to_expr(&self) -> Expr {
        match self {
            ValidatorSeparatorOption::Must(c) => {
                syn::parse2(quote! { validators_prelude::ValidatorSeparatorOption::Must(#c) })
                    .unwrap()
            }
            ValidatorSeparatorOption::Allow(c) => {
                syn::parse2(quote! { validators_prelude::ValidatorSeparatorOption::Allow(#c) })
                    .unwrap()
            }
            ValidatorSeparatorOption::NotAllow => {
                syn::parse2(quote! { validators_prelude::ValidatorSeparatorOption::NotAllow })
                    .unwrap()
            }
        }
    }
}

fn fetch_separator(meta_name: &str, meta: &Meta, correct_usage: &[&str]) -> u8 {
    if let Meta::List(list) = meta {
        if list.nested.len() == 1 {
            let p = list.nested.iter().next().unwrap();

            match p {
                NestedMeta::Meta(meta) => {
                    if let Meta::Path(path) = meta {
                        if let Some(ident) = path.get_ident() {
                            if ident == "colon" {
                                b':'
                            } else if ident == "hyphen" {
                                b'-'
                            } else {
                                panic::parameter_incorrect_format(meta_name, &correct_usage);
                            }
                        } else {
                            panic::parameter_incorrect_format(meta_name, &correct_usage);
                        }
                    } else {
                        panic::parameter_incorrect_format(meta_name, &correct_usage);
                    }
                }
                NestedMeta::Lit(lit) => {
                    match lit {
                        Lit::Char(c) => {
                            let c = c.value();

                            if c.is_ascii() {
                                c as u8
                            } else {
                                panic::parameter_incorrect_format(meta_name, &correct_usage);
                            }
                        }
                        Lit::Byte(b) => b.value(),
                        _ => {
                            panic::parameter_incorrect_format(meta_name, &correct_usage);
                        }
                    }
                }
            }
        } else {
            panic::parameter_incorrect_format(meta_name, &correct_usage);
        }
    } else {
        panic::parameter_incorrect_format(meta_name, &correct_usage);
    }
}

macro_rules! fetch_range {
    ($variant:ident, $meta_name:expr, $meta:expr, $correct_usage:expr) => {
        if let Meta::List(list) = $meta {
            let length = list.nested.len();

            if length >= 1 && length <= 2 {
                let mut min = None;
                let mut max = None;

                let mut min_is_set = false;
                let mut max_is_set = false;

                for p in list.nested.iter() {
                    match p {
                        NestedMeta::Meta(meta) => {
                            if let Meta::NameValue(name_value) = meta {
                                match &name_value.lit {
                                    Lit::Float(f) => {
                                        if let Some(ident) = meta.path().get_ident() {
                                            if ident == "min" {
                                                if min_is_set {
                                                    panic::reset_parameter("min");
                                                }

                                                min_is_set = true;

                                                min = Some(f.base10_digits().parse().unwrap());
                                            } else if ident == "max" {
                                                if max_is_set {
                                                    panic::reset_parameter("max");
                                                }

                                                max_is_set = true;

                                                max = Some(f.base10_digits().parse().unwrap());
                                            } else {
                                                panic::parameter_incorrect_format(
                                                    $meta_name,
                                                    &$correct_usage,
                                                );
                                            }
                                        } else {
                                            panic::parameter_incorrect_format(
                                                $meta_name,
                                                &$correct_usage,
                                            );
                                        }
                                    }
                                    Lit::Int(i) => {
                                        if let Some(ident) = meta.path().get_ident() {
                                            if ident == "min" {
                                                if min_is_set {
                                                    panic::reset_parameter("min");
                                                }

                                                min_is_set = true;

                                                min = Some(i.base10_digits().parse().unwrap());
                                            } else if ident == "max" {
                                                if max_is_set {
                                                    panic::reset_parameter("max");
                                                }

                                                max_is_set = true;

                                                max = Some(i.base10_digits().parse().unwrap());
                                            } else {
                                                panic::parameter_incorrect_format(
                                                    $meta_name,
                                                    &$correct_usage,
                                                );
                                            }
                                        } else {
                                            panic::parameter_incorrect_format(
                                                $meta_name,
                                                &$correct_usage,
                                            );
                                        }
                                    }
                                    _ => {
                                        panic::parameter_incorrect_format(
                                            $meta_name,
                                            &$correct_usage,
                                        );
                                    }
                                }
                            } else {
                                panic::parameter_incorrect_format($meta_name, &$correct_usage);
                            }
                        }
                        NestedMeta::Lit(_) => {
                            panic::parameter_incorrect_format($meta_name, &$correct_usage);
                        }
                    }
                }

                if let Some(min) = min {
                    if let Some(max) = max {
                        if min > max {
                            panic!("{} > {} (min > max)", min, max);
                        }
                    }
                }

                ValidatorRangeOption::$variant {
                    min,
                    max,
                }
            } else {
                panic::parameter_incorrect_format($meta_name, &$correct_usage);
            }
        } else {
            panic::parameter_incorrect_format($meta_name, &$correct_usage);
        }
    };
}

macro_rules! validator_range_option_impl {
    ($($ty:ident),* $(,)*) => {
        $(
            impl SynOption for ValidatorRangeOption<$ty> {
                fn from_meta(meta_name: &str, meta: &Meta, is_set: &mut bool, correct_usage: &[&str]) -> Self {
                    if let Meta::List(list) = meta {
                        if *is_set {
                            panic::reset_parameter(meta_name);
                        }

                        *is_set = true;

                        if list.nested.len() != 1 {
                            panic::parameter_incorrect_format(meta_name, &correct_usage);
                        }

                        let p = list.nested.iter().next().unwrap();

                        if let NestedMeta::Meta(meta) = p {
                            if let Some(ident) = meta.path().get_ident() {
                                if ident == "Inside" {
                                    fetch_range!(Inside, meta_name, meta, correct_usage)
                                } else if ident == "Outside" {
                                    fetch_range!(Outside, meta_name, meta, correct_usage)
                                } else if ident == "NotLimited" {
                                    ValidatorRangeOption::NotLimited
                                } else {
                                    panic::parameter_incorrect_format(meta_name, &correct_usage);
                                }
                            } else {
                                panic::parameter_incorrect_format(meta_name, &correct_usage);
                            }
                        } else {
                            panic::parameter_incorrect_format(meta_name, &correct_usage);
                        }
                    } else {
                        panic::parameter_incorrect_format(meta_name, &correct_usage);
                    }
                }

                #[inline]
                fn to_expr(&self) -> Expr {
                    match self {
                        ValidatorRangeOption::Inside { min, max } => {
                            match min {
                                Some(min) => {
                                    match max {
                                        Some(max) => {
                                            syn::parse2(quote! { validators_prelude::ValidatorRangeOption::<$ty>::Inside { min: Some(#min), max: Some(#max) } }).unwrap()
                                        }
                                        None => {
                                            syn::parse2(quote! { validators_prelude::ValidatorRangeOption::<$ty>::Inside { min: Some(#min), max: None } }).unwrap()
                                        }
                                    }
                                }
                                None => {
                                    match max {
                                        Some(max) => {
                                            syn::parse2(quote! { validators_prelude::ValidatorRangeOption::<$ty>::Inside { min: None, max: Some(#max) } }).unwrap()
                                        }
                                        None => {
                                            syn::parse2(quote! { validators_prelude::ValidatorRangeOption::<$ty>::Inside { min: None, max: None } }).unwrap()
                                        }
                                    }
                                }
                            }
                        }
                        ValidatorRangeOption::Outside { min, max } => {
                            match min {
                                Some(min) => {
                                    match max {
                                        Some(max) => {
                                            syn::parse2(quote! { validators_prelude::ValidatorRangeOption::<$ty>::Outside { min: Some(#min), max: Some(#max) } }).unwrap()
                                        }
                                        None => {
                                            syn::parse2(quote! { validators_prelude::ValidatorRangeOption::<$ty>::Outside { min: Some(#min), max: None } }).unwrap()
                                        }
                                    }
                                }
                                None => {
                                    match max {
                                        Some(max) => {
                                            syn::parse2(quote! { validators_prelude::ValidatorRangeOption::<$ty>::Outside { min: None, max: Some(#max) } }).unwrap()
                                        }
                                        None => {
                                            syn::parse2(quote! { validators_prelude::ValidatorRangeOption::<$ty>::Outside { min: None, max: None } }).unwrap()
                                        }
                                    }
                                }
                            }
                        }
                        ValidatorRangeOption::NotLimited => {
                            syn::parse2(quote! { validators_prelude::ValidatorRangeOption::<$ty>::NotLimited }).unwrap()
                        }
                    }
                }
            }
        )*
    }
}

validator_range_option_impl!(
    f32, f64, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
