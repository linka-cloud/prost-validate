use crate::field::{
    with_ignore_empty, Context, FieldValidationInner,
    ToValidationTokens,
};
use crate::utils::IsTrueAnd;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

#[derive(Debug, FromMeta, Clone, Default)]
pub struct RepeatedRules {
    pub min_items: Option<u64>,
    pub max_items: Option<u64>,
    pub unique: Option<bool>,
    pub items: Option<Box<FieldValidationInner>>,
    #[darling(default)]
    pub ignore_empty: bool,
}

impl ToValidationTokens for RepeatedRules {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        let min_items = self.min_items.map(|v| {
            let v = v as usize;
            let err = format!("{name} length is less than {v}");
            quote! {
                if #name.len() < #v {
                    return Err(anyhow::anyhow!(#err));
                }
            }
        });
        let max_items = self.max_items.map(|v| {
            let v = v as usize;
            let err = format!("{name} length is greater than {v}");
            quote! {
                if #name.len() > #v {
                    return Err(anyhow::anyhow!(#err));
                }
            }
        });
        let unique = self.unique.is_true_and(|| {
            let err = format!("{name} has duplicate items");
            quote! {
                if ::prost_validate::VecExt::unique(#name).len() != #name.len() {
                    return Err(anyhow::anyhow!(#err));
                }
            }
        });
        let item = self
            .items
            .as_ref()
            .map(|v| {
                let item = format_ident!("item");
                let validation = v.to_validation_tokens(ctx, &item);
                quote! {
                    for item in #name.iter() {
                        #validation
                    }
                }
            });
        let msg = (ctx.message
            && !ctx.wkt
            && !self
            .items
            .as_ref()
            .map(|v| v.message.map(|v| v.skip).unwrap_or_default())
            .unwrap_or_default())
            .then(|| {
                if ctx.boxed {
                    quote! {
                    for item in #name.iter() {
                        ::prost_validate::Validator::validate(item.as_ref())?;
                    }
                }
                } else {
                    quote! {
                    for item in #name.iter() {
                        ::prost_validate::Validator::validate(item)?;
                    }
                }
                }
            });
        with_ignore_empty(
            name,
            self.ignore_empty,
            quote! {
                #min_items
                #max_items
                #unique
                #item
                #msg
            },
        )
    }
}

impl From<Box<RepeatedRules>> for Box<prost_validate_types::RepeatedRules> {
    fn from(value: Box<RepeatedRules>) -> Self {
        Box::new(prost_validate_types::RepeatedRules {
            min_items: value.min_items,
            max_items: value.max_items,
            unique: value.unique,
            items: value.items.map(|v| (*v).into()).map(Box::new),
            ignore_empty: Some(value.ignore_empty),
        })
    }
}
