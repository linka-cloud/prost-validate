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
        if self.skip {
            return quote! {};
        }
        let validate = self.skip.not().then(|| {
            let field = &ctx.name;
            if ctx.boxed {
                quote! {
                    ::prost_validate::Validator::validate(#name.as_ref()).map_err(|e| ::prost_validate::Error::new(#field, ::prost_validate::errors::message::Error::Message(Box::new(e))))?;
                }
            } else {
                quote! {
                    ::prost_validate::Validator::validate(#name).map_err(|e| ::prost_validate::Error::new(#field, ::prost_validate::errors::message::Error::Message(Box::new(e))))?;
                }
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
