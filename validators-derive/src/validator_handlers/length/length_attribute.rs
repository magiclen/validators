use syn::{punctuated::Punctuated, spanned::Spanned, Meta, Token};

use crate::{
    common::{number::meta_2_number, serde_options::SerdeOptions},
    panic,
};

pub(crate) struct LengthAttribute {
    pub(crate) min:           Option<usize>,
    pub(crate) max:           Option<usize>,
    #[cfg_attr(not(feature = "serde"), allow(dead_code))]
    pub(crate) serde_options: SerdeOptions,
}

impl LengthAttribute {
    pub(crate) fn build_from_meta(meta: &Meta) -> syn::Result<Self> {
        let correct_parameters = ["min", "max", "serde"];

        let mut min = None;
        let mut max = None;
        let mut serde_options = SerdeOptions::default();

        match meta {
            Meta::Path(_) => (),
            Meta::NameValue(_) => {
                return Err(panic::attribute_incorrect_format(meta.path().get_ident().unwrap()));
            },
            Meta::List(list) => {
                let result =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                let mut min_is_set = false;
                let mut max_is_set = false;
                let mut serde_options_is_set = false;

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
                            "serde" => {
                                let v = SerdeOptions::from_meta(meta)?;

                                if serde_options_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                serde_options_is_set = true;

                                serde_options = v;

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
                if min > max {
                    return Err(syn::Error::new(
                        meta.path().span(),
                        format!("{min} > {max} (min > max)"),
                    ));
                }
            }
        }

        Ok(Self {
            min,
            max,
            serde_options,
        })
    }
}
