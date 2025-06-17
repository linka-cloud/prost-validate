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
        let field = &ctx.name;
        let rules = prost_validate_types::EnumRules::from(self.to_owned());
        let r#const = rules.r#const.map(|v| {
            quote! {
                if (*#name as i32) != #v {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::r#enum::Error::Const(#v)));
                }
            }
        });
        let defined_only = rules.defined_only.is_true_and(|| {
            let enumeration = ctx.enumeration.to_owned().expect("missing enum type");
            let enumeration = if let Some(ref m) = ctx.module {
                if enumeration.starts_with("super::") {
                    let enumeration = enumeration.strip_prefix("super::").unwrap();
                    let parts: Vec<_> = m.split("::").collect();
                    if parts.len() > 1 {
                        format!("{}::{}", parts[..parts.len() - 1].join("::"), enumeration)
                    } else {
                        enumeration.to_string()
                    }
                } else {
                    format!("{}::{}", m, enumeration)
                }
            } else {
                enumeration
            };
            let enum_type: syn::Path = syn::parse_str(enumeration.as_str())
                .expect("Invalid enum path");
            quote! {
                if !#enum_type::is_valid(*#name) {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::r#enum::Error::DefinedOnly));
                }
            }
        });
        let r#in = rules.r#in.is_empty().not().then(|| {
            let v = rules.r#in.to_owned();
            quote! {
                let values = [#(#v),*];
                if !values.contains(&#name) {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::r#enum::Error::In(values.to_vec())));
                }
            }
        });
        let not_in = rules.not_in.is_empty().not().then(|| {
            let v = rules.not_in.to_owned();
            quote! {
                let values = [#(#v),*];
                if values.contains(#name) {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::r#enum::Error::NotIn(values.to_vec())));
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
