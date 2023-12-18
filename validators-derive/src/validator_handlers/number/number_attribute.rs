use std::fmt::Display;

use syn::{punctuated::Punctuated, Meta, Token};

use crate::{
    common::{
        allow::Allow,
        range::RangedNumber,
        range_option::{RangeOption, RangeTokenStream},
        rocket_options::RocketOptions,
        serde_options::SerdeOptions,
        tri_allow::TriAllow,
    },
    panic,
};

pub(crate) struct NumberAttribute {
    pub(crate) range:          RangeTokenStream,
    pub(crate) nan:            TriAllow,
    pub(crate) conflict:       Allow,
    #[cfg_attr(not(feature = "serde"), allow(dead_code))]
    pub(crate) serde_options:  SerdeOptions,
    #[cfg_attr(not(feature = "rocket"), allow(dead_code))]
    pub(crate) rocket_options: RocketOptions,
}

impl NumberAttribute {
    pub(crate) fn build_from_meta<T: RangedNumber>(meta: &Meta) -> syn::Result<Self>
    where
        T::Err: Display, {
        let correct_parameters = ["range", "nan", "conflict", "serde", "rocket"];

        let mut range = RangeTokenStream::Unlimited;
        let mut nan = TriAllow::Allow;
        let mut conflict = Allow::Disallow;
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

                let mut range_is_set = false;
                let mut nan_is_set = false;
                let mut conflict_is_set = false;
                let mut serde_options_is_set = false;
                let mut rocket_options_is_set = false;

                let mut handler = |meta: &Meta| -> syn::Result<bool> {
                    if let Some(ident) = meta.path().get_ident() {
                        match ident.to_string().as_str() {
                            "range" => {
                                let v = RangeOption::<T>::from_meta(meta)?;

                                if range_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                range_is_set = true;

                                range = v.into();

                                return Ok(true);
                            },
                            "nan" => {
                                let v = TriAllow::from_meta(meta)?;

                                if nan_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                nan_is_set = true;

                                nan = v;

                                return Ok(true);
                            },
                            "conflict" => {
                                let v = Allow::from_meta(meta)?;

                                if conflict_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                conflict_is_set = true;

                                conflict = v;

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
            range,
            nan,
            conflict,
            serde_options,
            rocket_options,
        })
    }
}
