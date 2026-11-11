use syn::{DeriveInput, Meta};

use super::{
    ValidatorHandler,
    base_xx::{Encoding, encoded_handler},
};

pub(crate) struct Base64Handler;

impl ValidatorHandler for Base64Handler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        encoded_handler(ast, meta, Encoding::Base64)
    }
}
