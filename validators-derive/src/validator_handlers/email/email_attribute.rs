use syn::{punctuated::Punctuated, Meta, Token};

use crate::{
    common::{
        allow::Allow, rocket_options::RocketOptions, serde_options::SerdeOptions,
        tri_allow::TriAllow,
    },
    panic,
};

pub(crate) struct EmailAttribute {
    pub(crate) comment:             Allow,
    pub(crate) ip:                  TriAllow,
    pub(crate) local:               TriAllow,
    pub(crate) at_least_two_labels: TriAllow,
    pub(crate) non_ascii:           Allow,
    pub(crate) conflict:            Allow,
    #[cfg_attr(not(feature = "serde"), allow(dead_code))]
    pub(crate) serde_options:       SerdeOptions,
    #[cfg_attr(not(feature = "rocket"), allow(dead_code))]
    pub(crate) rocket_options:      RocketOptions,
}

impl EmailAttribute {
    pub(crate) fn build_from_meta(meta: &Meta) -> syn::Result<Self> {
        let correct_parameters = [
            "comment",
            "ip",
            "local",
            "at_least_two_labels",
            "non_ascii",
            "conflict",
            "serde",
            "rocket",
        ];

        let mut comment = Allow::Allow;
        let mut ip = TriAllow::Allow;
        let mut local = TriAllow::Allow;
        let mut at_least_two_labels = TriAllow::Allow;
        let mut non_ascii = Allow::Allow;
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

                let mut comment_is_set = false;
                let mut ip_is_set = false;
                let mut local_is_set = false;
                let mut at_least_two_labels_is_set = false;
                let mut non_ascii_is_set = false;
                let mut conflict_is_set = false;
                let mut serde_options_is_set = false;
                let mut rocket_options_is_set = false;

                let mut handler = |meta: &Meta| -> syn::Result<bool> {
                    if let Some(ident) = meta.path().get_ident() {
                        match ident.to_string().as_str() {
                            "comment" => {
                                let v = Allow::from_meta(meta)?;

                                if comment_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                comment_is_set = true;

                                comment = v;

                                return Ok(true);
                            },
                            "ip" => {
                                let v = TriAllow::from_meta(meta)?;

                                if ip_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                ip_is_set = true;

                                ip = v;

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
                            "at_least_two_labels" => {
                                let v = TriAllow::from_meta(meta)?;

                                if at_least_two_labels_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                at_least_two_labels_is_set = true;

                                at_least_two_labels = v;

                                return Ok(true);
                            },
                            "non_ascii" => {
                                let v = Allow::from_meta(meta)?;

                                if non_ascii_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                non_ascii_is_set = true;

                                non_ascii = v;

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
            comment,
            ip,
            local,
            at_least_two_labels,
            non_ascii,
            conflict,
            serde_options,
            rocket_options,
        })
    }
}
