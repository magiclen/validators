use syn::{DeriveInput, Meta};

use super::{
    ValidatorHandler,
    base_xx::{Encoding, decoded_handler},
};

pub(crate) struct Base64DecodedHandler;

impl ValidatorHandler for Base64DecodedHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        decoded_handler(ast, meta, Encoding::Base64)
    }
}
