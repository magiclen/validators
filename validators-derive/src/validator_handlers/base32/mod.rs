use syn::{DeriveInput, Meta};

use super::{
    ValidatorHandler,
    base_xx::{Encoding, encoded_handler},
};

pub(crate) struct Base32Handler;

impl ValidatorHandler for Base32Handler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        encoded_handler(ast, meta, Encoding::Base32)
    }
}
