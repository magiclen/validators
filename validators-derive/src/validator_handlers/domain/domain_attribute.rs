use syn::{punctuated::Punctuated, Meta, Token};

use crate::{
    common::{
        allow::Allow, rocket_options::RocketOptions, serde_options::SerdeOptions,
        tri_allow::TriAllow,
    },
    panic,
};

pub(crate) struct DomainAttribute {
    pub(crate) ipv4:                TriAllow,
    pub(crate) local:               TriAllow,
    pub(crate) port:                TriAllow,
    pub(crate) at_least_two_labels: TriAllow,
    pub(crate) conflict:            Allow,
    #[cfg_attr(not(feature = "serde"), allow(dead_code))]
    pub(crate) serde_options:       SerdeOptions,
    #[cfg_attr(not(feature = "rocket"), allow(dead_code))]
    pub(crate) rocket_options:      RocketOptions,
}

impl DomainAttribute {
    pub(crate) fn build_from_meta(meta: &Meta) -> syn::Result<Self> {
        let correct_parameters =
            ["ipv4", "local", "port", "at_least_two_labels", "conflict", "serde", "rocket"];

        let mut ipv4 = TriAllow::Allow;
        let mut local = TriAllow::Allow;
        let mut port = TriAllow::Allow;
        let mut at_least_two_labels = TriAllow::Allow;
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

                let mut ipv4_is_set = false;
                let mut local_is_set = false;
                let mut port_is_set = false;
                let mut at_least_two_labels_is_set = false;
                let mut conflict_is_set = false;
                let mut serde_options_is_set = false;
                let mut rocket_options_is_set = false;

                let mut handler = |meta: &Meta| -> syn::Result<bool> {
                    if let Some(ident) = meta.path().get_ident() {
                        match ident.to_string().as_str() {
                            "ipv4" => {
                                let v = TriAllow::from_meta(meta)?;

                                if ipv4_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                ipv4_is_set = true;

                                ipv4 = v;

                                return Ok(true);
                            },
                            "local" => {
                                let v = TriAllow::from_meta(meta)?;

                                if local_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                local_is_set = true;

                                local = v;

                                return Ok(true);
                            },
                            "port" => {
                                let v = TriAllow::from_meta(meta)?;

                                if port_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                port_is_set = true;

                                port = v;

                                return Ok(true);
                            },
                            "at_least_two_labels" => {
                                let v = TriAllow::from_meta(meta)?;

                                if at_least_two_labels_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                at_least_two_labels_is_set = true;

                                at_least_two_labels = v;

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
            ipv4,
            local,
            port,
            at_least_two_labels,
            conflict,
            serde_options,
            rocket_options,
        })
    }
}
