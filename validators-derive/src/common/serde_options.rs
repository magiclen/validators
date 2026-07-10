use syn::Meta;

use crate::common::flag_options;

#[derive(Debug)]
pub(crate) struct SerdeOptions {
    pub(crate) serialize:   bool,
    pub(crate) deserialize: bool,
}

impl Default for SerdeOptions {
    #[inline]
    fn default() -> Self {
        Self {
            serialize: true, deserialize: true
        }
    }
}

impl SerdeOptions {
    #[inline]
    pub(crate) fn from_meta(meta: &Meta) -> syn::Result<Self> {
        debug_assert!(meta.path().is_ident("serde"));

        let (serialize, deserialize) = flag_options::meta_2_flags(
            meta,
            "serde",
            cfg!(feature = "serde"),
            "Serialize",
            "Deserialize",
        )?;

        Ok(Self {
            serialize,
            deserialize,
        })
    }
}
