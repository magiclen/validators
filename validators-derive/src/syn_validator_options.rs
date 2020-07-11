use crate::syn::{Meta, NestedMeta, Path};

use crate::panic;
use crate::validators_options::*;

pub trait SynOption: Sized {
    fn from_meta(
        meta_name: &str,
        meta: &Meta,
        is_set: &mut bool,
        correct_usage: &[&str],
    ) -> Option<Self>;
    fn to_path(&self) -> Path;
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

            for p in list.nested.iter() {
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
            }

            None
        } else {
            panic::parameter_incorrect_format(meta_name, &correct_usage);
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
