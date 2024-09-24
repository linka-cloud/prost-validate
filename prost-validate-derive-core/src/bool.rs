use crate::field::{Context, ToValidationTokens};
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

#[derive(Debug, FromMeta, Clone)]
pub struct BoolRules {
    pub r#const: Option<bool>,
}

impl ToValidationTokens for BoolRules {
    fn to_validation_tokens(&self, _: &Context, name: &Ident) -> TokenStream {
        let r#const = self.r#const.map(|v| {
            let err = format!("{} must be equal to {}", name, v);
            quote! {
                if *#name != #v {
                    return Err(anyhow::anyhow!(#err));
                }
            }
        });
        quote! {
            #r#const
        }
    }
}

impl From<BoolRules> for prost_validate_types::BoolRules {
    fn from(value: BoolRules) -> Self {
        Self {
            r#const: value.r#const,
        }
    }
}
