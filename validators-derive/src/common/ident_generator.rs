use std::collections::HashSet;

use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::DeriveInput;

pub(crate) struct IdentGenerator {
    used: HashSet<String>,
}

impl IdentGenerator {
    pub(crate) fn new(input: &DeriveInput) -> Self {
        fn collect(tokens: TokenStream, used: &mut HashSet<String>) {
            for token in tokens {
                match token {
                    TokenTree::Ident(ident) => {
                        let name = ident.to_string();
                        used.insert(name.strip_prefix("r#").unwrap_or(&name).to_owned());
                    },
                    TokenTree::Group(group) => collect(group.stream(), used),
                    _ => (),
                }
            }
        }

        let mut used = HashSet::new();
        collect(input.to_token_stream(), &mut used);

        Self {
            used,
        }
    }

    pub(crate) fn next(&mut self, base: &str) -> Ident {
        let mut name = base.to_owned();
        let mut suffix = 0;

        while !self.used.insert(name.clone()) {
            suffix += 1;
            name = format!("{base}{suffix}");
        }

        Ident::new(&name, Span::call_site())
    }
}
