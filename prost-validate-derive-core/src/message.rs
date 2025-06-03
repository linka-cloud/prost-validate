use crate::field::{Context, ToValidationTokens};
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::ops::Not;

#[derive(Debug, FromMeta, Clone, Default, Copy)]
pub struct MessageRules {
    #[darling(default)]
    pub skip: bool,
    #[darling(default)]
    pub required: bool,
}

impl ToValidationTokens for MessageRules {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        let field = &ctx.name;
        if self.skip {
            return quote! {};
        }
        let validate = self.skip.not().then(|| {
            let map = quote! { |e| ::prost_validate::Error::new(#field, ::prost_validate::errors::message::Error::Message(Box::new(e))) };
            let name_ref = ctx.boxed.then(|| quote! { let #name = #name.as_ref(); });
            if ctx.multierrs {
                return quote! {
                    #name_ref
                    if let Err(es) = ::prost_validate::validate_all!(#name) {
                        errs.extend(es.into_iter().map(#map));
                    }
                };
            }
            quote! {
                #name_ref
                ::prost_validate::validate!(#name).map_err(#map)?;
            }
        });
        validate.unwrap_or_default()
    }
}

impl From<MessageRules> for prost_validate_types::MessageRules {
    fn from(value: MessageRules) -> Self {
        Self {
            skip: Some(value.skip),
            required: Some(value.required),
        }
    }
}
