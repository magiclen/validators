use syn::{punctuated::Punctuated, Meta, Token};

use crate::{
    common::{length::Length, rocket_options::RocketOptions, serde_options::SerdeOptions},
    panic,
};

pub(crate) struct Utf8Attribute {
    pub(crate) length:         Option<(bool, Length)>,
    #[cfg_attr(not(feature = "serde"), allow(dead_code))]
    pub(crate) serde_options:  SerdeOptions,
    #[cfg_attr(not(feature = "rocket"), allow(dead_code))]
    pub(crate) rocket_options: RocketOptions,
}

impl Utf8Attribute {
    pub(crate) fn build_from_meta(meta: &Meta) -> syn::Result<Self> {
        let correct_parameters = ["char_length", "byte_length", "serde", "rocket"];

        let mut length = None;
        let mut serde_options = SerdeOptions::default();
        let mut rocket_options = RocketOptions::default();

        match meta {
            Meta::Path(_) => (),
            Meta::NameValue(_) => {
                return Err(panic::attribute_incorrect_format(meta.path().get_ident().unwrap()));
            },
            Meta::List(list) => {
                let result =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                let mut length_is_set = false;
                let mut serde_options_is_set = false;
                let mut rocket_options_is_set = false;

                let mut handler = |meta: &Meta| -> syn::Result<bool> {
                    if let Some(ident) = meta.path().get_ident() {
                        match ident.to_string().as_str() {
                            "char_length" => {
                                let v = Length::from_meta(meta)?;

                                if length_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                length_is_set = true;

                                length = Some((false, v));

                                return Ok(true);
                            },
                            "byte_length" => {
                                let v = Length::from_meta(meta)?;

                                if length_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                length_is_set = true;

                                length = Some((true, v));

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
                            "rocket" => {
                                let v = RocketOptions::from_meta(meta)?;

                                if rocket_options_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                rocket_options_is_set = true;

                                rocket_options = v;

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

        Ok(Self {
            length,
            serde_options,
            rocket_options,
        })
    }
}
