use syn::{punctuated::Punctuated, spanned::Spanned, Meta, Token};

use crate::{common::number::meta_2_number, panic};

#[derive(Debug, Clone)]
pub(crate) struct Length {
    pub(crate) min:         Option<usize>,
    pub(crate) trimmed_min: Option<usize>,
    pub(crate) max:         Option<usize>,
}

impl Length {
    #[inline]
    pub(crate) fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let correct_parameters = ["min", "trimmed_min", "max"];

        let mut min = None;
        let mut trimmed_min = None;
        let mut max = None;

        match meta {
            Meta::Path(_) | Meta::NameValue(_) => {
                return Err(panic::attribute_incorrect_format(meta.path().get_ident().unwrap()));
            },
            Meta::List(list) => {
                let result =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                let mut min_is_set = false;
                let mut trimmed_min_is_set = false;
                let mut max_is_set = false;

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
                            "trimmed_min" => {
                                let v = meta_2_number(meta)?;

                                if trimmed_min_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                trimmed_min_is_set = true;

                                trimmed_min = Some(v);

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
            if let Some(trimmed_min) = trimmed_min {
                if trimmed_min > min {
                    return Err(syn::Error::new(
                        meta.path().span(),
                        format!("{trimmed_min} > {min} (trimmed_min > min)"),
                    ));
                }
            }

            if let Some(max) = max {
                if min > max {
                    return Err(syn::Error::new(
                        meta.path().span(),
                        format!("{min} > {max} (min > max)"),
                    ));
                }
            }
        }

        if let Some(trimmed_min) = trimmed_min {
            if let Some(max) = max {
                if trimmed_min > max {
                    return Err(syn::Error::new(
                        meta.path().span(),
                        format!("{trimmed_min} > {max} (trimmed_min > max)"),
                    ));
                }
            }
        }

        Ok(Self {
            min,
            trimmed_min,
            max,
        })
    }
}
