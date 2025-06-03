use crate::field::{with_ignore_empty, Context, FieldValidationInner, ToValidationTokens};
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
            let field = &ctx.name;
            quote! {
                if #name.len() < #v {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::list::Error::MinItems(#v)));
                }
            }
        });
        let max_items = self.max_items.map(|v| {
            let v = v as usize;
            let field = &ctx.name;
            quote! {
                if #name.len() > #v {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::list::Error::MaxItems(#v)));
                }
            }
        });
        let unique = self.unique.is_true_and(|| {
            let field = &ctx.name;
            quote! {
                if ::prost_validate::VecExt::unique(#name).len() != #name.len() {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::list::Error::Unique));
                }
            }
        });
        let item = self
            .items
            .as_ref()
            .map(|v| {
                let field = &ctx.name;
                let item = format_ident!("item");
                let validation = v.to_validation_tokens(ctx, &item);
                quote! {
                    for (i, item) in #name.iter().enumerate() {
                        || -> ::prost_validate::Result<_> {
                            #validation
                            Ok(())
                        }().map_err(|e| ::prost_validate::Error::new(format!("{}[{}]", #field, i), ::prost_validate::errors::list::Error::Item(Box::new(e))))?;
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
                let field = &ctx.name;
                if ctx.boxed {
                    quote! {
                        for (i, item) in #name.iter.enumerate() {
                            let item = item.as_ref();
                            ::prost_validate::validate!(item).map_err(|e| ::prost_validate::Error::new(format!("{}[{}]", #field, i), ::prost_validate::errors::list::Error::Item(Box::new(e))))?;
                        }
                    }
                } else {
                    quote! {
                        for (i, item) in #name.iter().enumerate() {
                            ::prost_validate::validate!(item).map_err(|e| ::prost_validate::Error::new(format!("{}[{}]", #field, i), ::prost_validate::errors::list::Error::Item(Box::new(e))))?;
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
