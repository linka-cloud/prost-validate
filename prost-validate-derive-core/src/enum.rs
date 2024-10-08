use crate::field::{Context, ToValidationTokens};
use crate::utils::IsTrueAnd;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::ops::Not;
use syn::LitInt;

#[derive(Debug, Default, FromMeta, Clone)]
pub struct EnumRules {
    #[darling(default)]
    pub r#const: Option<i32>,
    pub defined_only: Option<bool>,
    pub r#in: Option<Vec<LitInt>>,
    pub not_in: Option<Vec<LitInt>>,
}

impl ToValidationTokens for EnumRules {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        let rules = prost_validate_types::EnumRules::from(self.to_owned());
        let r#const = rules.r#const.map(|v| {
            let field = &ctx.name;
            let err = format!("must be equal to {}", v);
            quote! {
                if (*#name as i32) != #v {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let defined_only = rules.defined_only.is_true_and(|| {
            let enum_type: syn::Path = syn::parse_str(ctx.to_owned().enumeration.expect("missing enum type").as_str())
                .expect("Invalid enum path");
            let field = &ctx.name;
            let err = format!("{} must be a defined enum value", name);
            quote! {
                if !#enum_type::is_valid(*#name) {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let r#in = rules.r#in.is_empty().not().then(|| {
            let v = rules.r#in.to_owned();
            let field = &ctx.name;
            let err = format!("must be one of {:?}", rules.r#in);
            quote! {
                if ![#(#v),*].contains(&#name) {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let not_in = rules.not_in.is_empty().not().then(|| {
            let v = rules.not_in.to_owned();
            let field = &ctx.name;
            let err = format!("is in {:?}", v);
            quote! {
                if [#(#v),*].contains(#name) {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        quote! {
            #r#const
            #defined_only
            #r#in
            #not_in
        }
    }
}

impl From<EnumRules> for prost_validate_types::EnumRules {
    fn from(value: EnumRules) -> Self {
        prost_validate_types::EnumRules {
            r#const: value.r#const,
            defined_only: value.defined_only,
            r#in: value
                .r#in
                .unwrap_or_default()
                .iter()
                .map(|v| v.base10_parse().unwrap())
                .collect(),
            not_in: value
                .not_in
                .unwrap_or_default()
                .iter()
                .map(|v| v.base10_parse().unwrap())
                .collect(),
        }
    }
}
