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
        let field = &ctx.name;
        let maybe_return = ctx.maybe_return();
        let min_items = self.min_items.map(|v| {
            let v = v as usize;
            quote! {
                if #name.len() < #v {
                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::list::Error::MinItems(#v)));
                }
            }
        });
        let max_items = self.max_items.map(|v| {
            let v = v as usize;
            quote! {
                if #name.len() > #v {
                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::list::Error::MaxItems(#v)));
                }
            }
        });
        let unique = self.unique.is_true_and(|| {
            quote! {
                if ::prost_validate::VecExt::unique(#name).len() != #name.len() {
                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::list::Error::Unique));
                }
            }
        });
        let map = quote! { |e| ::prost_validate::Error::new(format!("{}[{i}]", #field), ::prost_validate::errors::list::Error::Item(Box::new(e))) };
        let items = self.items.as_ref().map(|v| {
            let validation = v.to_validation_tokens(ctx, &format_ident!("item"));
            if ctx.multierrs {
                return quote! {
                    for (i, item) in #name.iter().enumerate() {
                        if let Err(es) = || -> ::core::result::Result<(), Vec<::prost_validate::Error>> {
                            let mut errs = vec![];
                            #validation
                            if errs.is_empty() { Ok(()) } else { Err(errs) }
                        }() {
                            errs.extend(es.into_iter().map(#map));
                        }
                    }
                };
            }
            quote! {
                for (i, item) in #name.iter().enumerate() {
                    || -> ::prost_validate::Result<_> {
                        #validation
                        Ok(())
                    }().map_err(#map)?;
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
            let (name_iter, item_ref) = if ctx.boxed {
                (quote! { #name.iter }, quote! { let item = item.as_ref(); })
            } else {
                (quote! { #name.iter() }, quote! {})
            };
            if ctx.multierrs {
                return quote! {
                    for (i, item) in #name_iter.enumerate() {
                        #item_ref
                        if let Err(es) = ::prost_validate::validate_all!(item) {
                            errs.extend(es.into_iter().map(#map));
                        }
                    }
                };
            }
            quote! {
                for (i, item) in #name_iter.enumerate() {
                    #item_ref
                    ::prost_validate::validate!(item).map_err(#map)?;
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
                #items
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
