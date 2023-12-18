use quote::quote;
use syn::{Ident, Path};

#[inline]
pub(crate) fn impl_from_form_field(token_stream: &mut proc_macro2::TokenStream, name: &Ident) {
    token_stream.extend(quote! {
        impl<'r> validators_prelude::rocket::form::FromFormField<'r> for #name {
            #[inline]
            fn from_value(v: validators_prelude::rocket::form::ValueField<'r>) -> validators_prelude::rocket::form::Result<'r, Self> {
                Ok(<Self as ValidateString>::parse_str(v.value).map_err(validators_prelude::rocket::form::Error::custom)?)
            }
        }
    });
}

#[inline]
pub(crate) fn impl_from_param(
    token_stream: &mut proc_macro2::TokenStream,
    name: &Ident,
    error_path: &Path,
) {
    token_stream.extend(quote! {
        impl<'r> validators_prelude::rocket::request::FromParam<'r> for #name {
            type Error = #error_path;

            #[inline]
            fn from_param(v: &'r str) -> Result<Self, Self::Error> {
                <Self as ValidateString>::parse_str(v)
            }
        }
    });
}
