use syn::{DeriveInput, Meta};

use super::{
    ValidatorHandler,
    base_xx::{Encoding, encoded_handler},
};

pub(crate) struct Base64UrlHandler;

impl ValidatorHandler for Base64UrlHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        encoded_handler(ast, meta, Encoding::Base64Url)
    }
}
