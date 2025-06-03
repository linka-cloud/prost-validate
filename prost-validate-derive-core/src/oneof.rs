use crate::field::{Context, ToValidationTokens};
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

#[derive(Debug, FromMeta, Clone, Default)]
pub struct OneOfRules {
    #[darling(default)]
    pub required: bool,
}

impl ToValidationTokens for OneOfRules {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        if ctx.multierrs {
            quote! {
                errs.extend(::prost_validate::validate_all!(#name).into_iter());
            }
        } else {
            quote! {
                ::prost_validate::validate!(#name)?;
            }
        }
    }
}
