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
    fn to_validation_tokens(&self, _: &Context, name: &Ident) -> TokenStream {
        let rules = prost_validate_types::AnyRules::from(self.to_owned());
        let r#in = rules.r#in.is_empty().not().then(|| {
            let v = rules.r#in;
            let err = format!("{name}: must be in {:?}", v);
            quote! {
                if ![#(#v),*].contains(&#name.type_url.as_str()) {
                    return Err(anyhow::anyhow!(#err));
                }
            }
        });
        let not_in = rules.not_in.is_empty().not().then(|| {
            let v = rules.not_in;
            let err = format!("{name}: must not be in {:?}", v);
            quote! {
                if [#(#v),*].contains(&#name.type_url.as_str()) {
                    return Err(anyhow::anyhow!(#err));
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
