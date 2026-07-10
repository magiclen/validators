use syn::Meta;

use crate::common::flag_options;

#[derive(Debug)]
pub(crate) struct RocketOptions {
    pub(crate) from_form_field: bool,
    pub(crate) from_param:      bool,
}

impl Default for RocketOptions {
    #[inline]
    fn default() -> Self {
        Self {
            from_form_field: true, from_param: true
        }
    }
}

impl RocketOptions {
    #[inline]
    pub(crate) fn from_meta(meta: &Meta) -> syn::Result<Self> {
        debug_assert!(meta.path().is_ident("rocket"));

        let (from_form_field, from_param) = flag_options::meta_2_flags(
            meta,
            "rocket",
            cfg!(feature = "rocket"),
            "FromFormField",
            "FromParam",
        )?;

        Ok(Self {
            from_form_field,
            from_param,
        })
    }
}
