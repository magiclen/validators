use quote::quote;
use syn::{Meta, Path, Token, punctuated::Punctuated};

#[cfg(feature = "hex_color")]
use crate::common::tri_allow::TriAllow;
use crate::{
    common::{case_option::CaseOption, rocket_options::RocketOptions, serde_options::SerdeOptions},
    panic,
};

pub(crate) struct HexAttribute {
    pub(crate) case:           CaseOption,
    #[cfg(feature = "hex_color")]
    pub(crate) alpha:          TriAllow,
    #[cfg_attr(not(feature = "serde"), allow(dead_code))]
    pub(crate) serde_options:  SerdeOptions,
    #[cfg_attr(not(feature = "rocket"), allow(dead_code))]
    pub(crate) rocket_options: RocketOptions,
}

impl HexAttribute {
    pub(crate) fn digit_decoder(&self, error_path: &Path) -> proc_macro2::TokenStream {
        let lower = self.case.lower().then(|| {
            quote! {
                b'a'..=b'f' => ::core::result::Result::Ok(digit - (b'a' - 10)),
            }
        });
        let upper = self.case.upper().then(|| {
            quote! {
                b'A'..=b'F' => ::core::result::Result::Ok(digit - (b'A' - 10)),
            }
        });

        quote! {
            #[inline]
            fn v_decode_hex_digit(digit: u8) -> ::core::result::Result<u8, #error_path> {
                match digit {
                    b'0'..=b'9' => ::core::result::Result::Ok(digit - b'0'),
                    #lower
                    #upper
                    _ => ::core::result::Result::Err(#error_path),
                }
            }
        }
    }

    pub(crate) fn build_from_meta(meta: &Meta) -> syn::Result<Self> {
        let correct_parameters: &[&str] = if meta.path().is_ident("hex_color") {
            &["case", "alpha", "serde", "rocket"]
        } else {
            &["case", "serde", "rocket"]
        };
        let mut case = None;
        #[cfg(feature = "hex_color")]
        let mut alpha = None;
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
                        #[cfg(feature = "hex_color")]
                        Some(ident) if ident == "alpha" && meta.path().is_ident("hex_color") => {
                            if alpha.is_some() {
                                return Err(panic::parameter_reset(ident));
                            }

                            alpha = Some(TriAllow::from_meta(&parameter)?);
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
                            return Err(panic::parameter_incorrect_format(
                                path,
                                correct_parameters,
                            ));
                        },
                    }
                }
            },
        }

        Ok(Self {
            case:                                case.unwrap_or(CaseOption::Any),
            #[cfg(feature = "hex_color")]
            alpha:                               alpha.unwrap_or(TriAllow::Allow),
            serde_options:                       serde_options.unwrap_or_default(),
            rocket_options:                      rocket_options.unwrap_or_default(),
        })
    }
}
