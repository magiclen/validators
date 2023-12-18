use std::{collections::HashSet, str::FromStr};

use phonenumber::country::Id;
use proc_macro2::Ident;
use syn::{punctuated::Punctuated, spanned::Spanned, Meta, Token};

use crate::{
    common::{rocket_options::RocketOptions, serde_options::SerdeOptions},
    panic,
};

pub(crate) struct PhoneAttribute {
    pub(crate) countries:      HashSet<Id>,
    #[cfg_attr(not(feature = "serde"), allow(dead_code))]
    pub(crate) serde_options:  SerdeOptions,
    #[cfg_attr(not(feature = "rocket"), allow(dead_code))]
    pub(crate) rocket_options: RocketOptions,
}

impl PhoneAttribute {
    pub(crate) fn build_from_meta(meta: &Meta) -> syn::Result<Self> {
        let correct_parameters = ["countries", "serde", "rocket"];

        let mut countries = HashSet::new();
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

                let mut countries_is_set = false;
                let mut serde_options_is_set = false;
                let mut rocket_options_is_set = false;

                let mut handler = |meta: &Meta| -> syn::Result<bool> {
                    if let Some(ident) = meta.path().get_ident() {
                        match ident.to_string().as_str() {
                            "countries" => {
                                if countries_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                countries_is_set = true;

                                meta_2_countries(&mut countries, meta)?;

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
            countries,
            serde_options,
            rocket_options,
        })
    }
}

fn meta_2_countries(countries: &mut HashSet<Id>, meta: &Meta) -> syn::Result<()> {
    if let Meta::List(list) = meta {
        let result =
            list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_separated_nonempty)?;

        for ident in result {
            let country = ident.to_string();

            let id = Id::from_str(country.as_str())
                .map_err(|error| syn::Error::new(ident.span(), error))?;

            if !countries.insert(id) {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("the country `{country}` of the phone validator is repeated"),
                ));
            }
        }

        return Ok(());
    }

    let path = meta.path();

    Err(syn::Error::new(path.span(), "expected `countries(TW, US, ...)`"))
}
