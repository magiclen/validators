use quote::{quote, ToTokens};

use crate::common::{
    allow::Allow, case_option::CaseOption, range::Range, range_option::RangeTokenStream,
    separator_option::SeparatorOption, tri_allow::TriAllow,
};

pub(crate) struct OptionToken<T: ToTokens>(pub(crate) Option<T>);

impl<T: ToTokens> ToTokens for OptionToken<T> {
    #[inline]
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        match &self.0 {
            Some(t) => token_stream.extend(quote! { Some(#t) }),
            None => token_stream.extend(quote! { None }),
        }
    }
}

impl ToTokens for TriAllow {
    #[inline]
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        match self {
            Self::Must => {
                token_stream.extend(quote! { validators_prelude::TriAllow::Must });
            },
            Self::Allow => {
                token_stream.extend(quote! { validators_prelude::TriAllow::Allow });
            },
            Self::Disallow => {
                token_stream.extend(quote! { validators_prelude::TriAllow::Disallow });
            },
        }
    }
}

impl ToTokens for Allow {
    #[inline]
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        // we don't have `Allow` in the `validators` crate, use `TriAllow` instead
        match self {
            Self::Allow => {
                token_stream.extend(quote! { validators_prelude::TriAllow::Allow });
            },
            Self::Disallow => {
                token_stream.extend(quote! { validators_prelude::TriAllow::Disallow });
            },
        }
    }
}

impl ToTokens for CaseOption {
    #[inline]
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        match self {
            Self::Any => {
                token_stream.extend(quote! { validators_prelude::CaseOption::Any });
            },
            Self::Lower => {
                token_stream.extend(quote! { validators_prelude::CaseOption::Lower });
            },
            Self::Upper => {
                token_stream.extend(quote! { validators_prelude::CaseOption::Upper });
            },
        }
    }
}

impl ToTokens for SeparatorOption {
    #[inline]
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        match self {
            Self::Must(c) => {
                token_stream.extend(quote! { validators_prelude::SeparatorOption::Must(#c) });
            },
            Self::Allow(c) => {
                token_stream.extend(quote! { validators_prelude::SeparatorOption::Allow(#c) });
            },
            Self::Disallow => {
                token_stream.extend(quote! { validators_prelude::SeparatorOption::Disallow });
            },
        }
    }
}

impl ToTokens for Range<u128> {
    #[inline]
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        let min = OptionToken(self.min.as_ref());
        let max = OptionToken(self.max.as_ref());
        let inclusive = self.inclusive;

        token_stream.extend(quote! {
            validators_prelude::RangeOption::Inside {
                min: #min,
                max: #max,
                inclusive: #inclusive,
            }
        });
    }
}

impl ToTokens for RangeTokenStream {
    #[inline]
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        match self {
            Self::Inside {
                min,
                max,
                inclusive,
                equal: _,
            } => {
                let min = OptionToken(min.as_ref());
                let max = OptionToken(max.as_ref());

                token_stream.extend(quote! {
                    validators_prelude::RangeOption::Inside {
                        min: #min,
                        max: #max,
                        inclusive: #inclusive,
                    }
                });
            },
            Self::Outside {
                min,
                max,
                inclusive,
                equal: _,
            } => {
                let min = OptionToken(min.as_ref());
                let max = OptionToken(max.as_ref());

                token_stream.extend(quote! {
                    validators_prelude::RangeOption::Outside {
                        min: #min,
                        max: #max,
                        inclusive: #inclusive,
                    }
                });
            },
            Self::Unlimited => {
                token_stream.extend(quote! { validators_prelude::RangeOption::Unlimited });
            },
        }
    }
}
