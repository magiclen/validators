use syn::{DeriveInput, Meta};

use super::{
    ValidatorHandler,
    http_xx_url::{UrlKind, url_handler},
};

pub(crate) struct HttpFtpUrlHandler;

impl ValidatorHandler for HttpFtpUrlHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        url_handler(ast, meta, UrlKind::HttpFtp)
    }
}
