use crate::syn::{Expr, Lit, Meta, NestedMeta};

use crate::panic;
use crate::validators_options::*;

pub trait SynOption: Sized {
    fn from_meta(
        meta_name: &str,
        meta: &Meta,
        is_set: &mut bool,
        correct_usage: &[&str],
    ) -> Option<Self>;
    fn to_expr(&self) -> Expr;
}

impl SynOption for ValidatorOption {
    fn from_meta(
        meta_name: &str,
        meta: &Meta,
        is_set: &mut bool,
        correct_usage: &[&str],
    ) -> Option<Self> {
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
                            return Some(ValidatorOption::Allow);
                        } else if ident == "Must" {
                            return Some(ValidatorOption::Must);
                        } else if ident == "NotAllow" {
                            return Some(ValidatorOption::NotAllow);
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

            None
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
    fn from_meta(
        meta_name: &str,
        meta: &Meta,
        is_set: &mut bool,
        correct_usage: &[&str],
    ) -> Option<Self> {
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
                            return Some(ValidatorCaseOption::Any);
                        } else if ident == "Upper" {
                            return Some(ValidatorCaseOption::Upper);
                        } else if ident == "Lower" {
                            return Some(ValidatorCaseOption::Lower);
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

            None
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
    fn from_meta(
        meta_name: &str,
        meta: &Meta,
        is_set: &mut bool,
        correct_usage: &[&str],
    ) -> Option<Self> {
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
                        return Some(ValidatorSeparatorOption::Allow(fetch_separator(
                            meta_name,
                            meta,
                            correct_usage,
                        )));
                    } else if ident == "Must" {
                        return Some(ValidatorSeparatorOption::Must(fetch_separator(
                            meta_name,
                            meta,
                            correct_usage,
                        )));
                    } else if ident == "NotAllow" {
                        return Some(ValidatorSeparatorOption::NotAllow);
                    }
                } else {
                    panic::parameter_incorrect_format(meta_name, &correct_usage);
                }
            } else {
                panic::parameter_incorrect_format(meta_name, &correct_usage);
            }

            None
        } else {
            panic::parameter_incorrect_format(meta_name, &correct_usage);
        }
    }

    #[inline]
    fn to_expr(&self) -> Expr {
        match self {
            ValidatorSeparatorOption::Must(c) => {
                let c = *c;

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
                        Lit::Byte(b) => return b.value(),
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
