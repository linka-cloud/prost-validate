use crate::field::{Context, ToValidationTokens};
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::ops::Not;
use syn::LitStr;

#[derive(Debug, FromMeta, Clone)]
pub struct AnyRules {
    #[darling(default)]
    pub required: bool,
    pub r#in: Option<Vec<LitStr>>,
    pub not_in: Option<Vec<LitStr>>,
}

impl ToValidationTokens for AnyRules {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        let field = &ctx.name;
        let rules = prost_validate_types::AnyRules::from(self.to_owned());
        let maybe_return = ctx.maybe_return();
        let r#in = rules.r#in.is_empty().not().then(|| {
            let v = rules.r#in;
            quote! {
                let values = vec![#(#v),*];
                if !values.contains(&#name.type_url.as_str()) {
                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::any::Error::In(values.iter().map(|v|v.to_string()).collect())));
                }
            }
        });
        let not_in = rules.not_in.is_empty().not().then(|| {
            let v = rules.not_in;
            quote! {
                let values = vec![#(#v),*];
                if values.contains(&#name.type_url.as_str()) {
                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::any::Error::NotIn(values.iter().map(|v|v.to_string()).collect())));
                }
            }
        });

        quote! {
            #r#in
            #not_in
        }
    }
}

impl From<AnyRules> for prost_validate_types::AnyRules {
    fn from(value: AnyRules) -> Self {
        prost_validate_types::AnyRules {
            required: Some(value.required),
            r#in: value
                .r#in
                .unwrap_or_default()
                .iter()
                .map(|v| v.value())
                .collect(),
            not_in: value
                .not_in
                .unwrap_or_default()
                .iter()
                .map(|v| v.value())
                .collect(),
        }
    }
}
