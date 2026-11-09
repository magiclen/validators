use syn::{Meta, Token, punctuated::Punctuated};

use crate::{
    common::{case_option::CaseOption, rocket_options::RocketOptions, serde_options::SerdeOptions},
    panic,
};

pub(crate) struct HexDecodedAttribute {
    pub(crate) case:           CaseOption,
    #[cfg_attr(not(feature = "serde"), allow(dead_code))]
    pub(crate) serde_options:  SerdeOptions,
    #[cfg_attr(not(feature = "rocket"), allow(dead_code))]
    pub(crate) rocket_options: RocketOptions,
}

impl HexDecodedAttribute {
    pub(crate) fn build_from_meta(meta: &Meta) -> syn::Result<Self> {
        let mut case = None;
        let mut serde_options = None;
        let mut rocket_options = None;

        match meta {
            Meta::Path(_) => (),
            Meta::NameValue(_) => return Err(panic::attribute_incorrect_format(meta.path())),
            Meta::List(list) => {
                let parameters =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                for parameter in parameters {
                    let path = parameter.path();

                    match path.get_ident() {
                        Some(ident) if ident == "case" => {
                            if case.is_some() {
                                return Err(panic::parameter_reset(ident));
                            }

                            case = Some(CaseOption::from_meta(&parameter)?);
                        },
                        Some(ident) if ident == "serde" => {
                            if serde_options.is_some() {
                                return Err(panic::parameter_reset(ident));
                            }

                            serde_options = Some(SerdeOptions::from_meta(&parameter)?);
                        },
                        Some(ident) if ident == "rocket" => {
                            if rocket_options.is_some() {
                                return Err(panic::parameter_reset(ident));
                            }

                            rocket_options = Some(RocketOptions::from_meta(&parameter)?);
                        },
                        _ => {
                            return Err(panic::parameter_incorrect_format(path, &[
                                "case", "serde", "rocket",
                            ]));
                        },
                    }
                }
            },
        }

        Ok(Self {
            case:           case.unwrap_or(CaseOption::Any),
            serde_options:  serde_options.unwrap_or_default(),
            rocket_options: rocket_options.unwrap_or_default(),
        })
    }
}
