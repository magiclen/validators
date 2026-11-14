use syn::{DeriveInput, Meta};

use super::{
    ValidatorHandler,
    http_xx_url::{UrlKind, url_handler},
};

pub(crate) struct HttpUrlHandler;

impl ValidatorHandler for HttpUrlHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        url_handler(ast, meta, UrlKind::Http)
    }
}
