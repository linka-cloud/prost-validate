use crate::field::{with_ignore_empty, Context, FieldValidationInner, ToValidationTokens};
use crate::message::MessageRules;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::ops::Not;

#[derive(Debug, FromMeta, Clone, Default)]
pub struct MapRules {
    pub min_pairs: Option<u64>,
    pub max_pairs: Option<u64>,
    pub no_sparse: Option<bool>,
    pub keys: Option<Box<FieldValidationInner>>,
    pub values: Option<Box<FieldValidationInner>>,
    #[darling(default)]
    pub ignore_empty: bool,
}

impl ToValidationTokens for MapRules {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        let field = &ctx.name;
        let rules = prost_validate_types::MapRules::from(self.to_owned());
        let maybe_return = ctx.maybe_return();
        let min_pairs = rules.min_pairs.map(|v| {
            let v = v as usize;
            quote! {
                if #name.len() < #v {
                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::map::Error::MinPairs(#v)));
                }
            }
        });
        let max_pairs = rules.max_pairs.map(|v| {
            let v = v as usize;
            quote! {
                if #name.len() > #v {
                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::map::Error::MaxPairs(#v)));
                }
            }
        });
        let keys = self.keys.as_ref().map(|rules| {
            let key = format_ident!("key");
            let validate = rules.to_validation_tokens(ctx, &key);
            let map = quote! { |e| ::prost_validate::Error::new(format!("{}[{}]", #field, #key), ::prost_validate::errors::map::Error::Keys(Box::new(e))) };
            validate.is_empty().not().then(|| {
                if ctx.multierrs {
                    return quote! {
                        for #key in #name.keys() {
                            if let Err(es) = || -> ::core::result::Result<(), Vec<::prost_validate::Error>> {
                                let mut errs = vec![];
                                #validate
                                if errs.is_empty() { Ok(()) } else { Err(errs) }
                            }() {
                                errs.extend(es.into_iter().map(#map));
                            }
                        }
                    };
                }
                quote! {
                    for #key in #name.keys() {
                        || -> ::prost_validate::Result<_> {
                            #validate
                            Ok(())
                        }().map_err(#map)?;
                    }
                }
            })
        });
        let value = format_ident!("value");
        let quote_values = |validation: TokenStream| {
            let map = quote! { |e| ::prost_validate::Error::new(format!("{}[{k}]", #field), ::prost_validate::errors::map::Error::Values(Box::new(e))) };
            if ctx.multierrs {
                return quote! {
                    for (k, #value) in #name.iter() {
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
                for (k, #value) in #name.iter() {
                    || -> ::prost_validate::Result<_> {
                        #validation
                        Ok(())
                    }().map_err(#map)?;
                }
            }
        };
        let msg = (ctx.message
            && !ctx.wkt
            && !self
                .values
                .as_ref()
                .map(|v| v.message.map(|v| v.skip).unwrap_or_default())
                .unwrap_or_default())
        .then(|| {
            let validation = MessageRules::default().to_validation_tokens(ctx, &value);
            quote_values(validation)
        });
        let values = self.values.as_ref().map(|rules| {
            let validate = rules.to_validation_tokens(ctx, &value);
            validate.is_empty().not().then(|| quote_values(validate))
        });
        with_ignore_empty(
            name,
            self.ignore_empty,
            quote! {
                #min_pairs
                #max_pairs
                #keys
                #values
                #msg
            },
        )
    }
}

impl From<MapRules> for prost_validate_types::MapRules {
    fn from(value: MapRules) -> Self {
        prost_validate_types::MapRules {
            min_pairs: value.min_pairs,
            max_pairs: value.max_pairs,
            no_sparse: value.no_sparse,
            keys: value.keys.map(|v| (*v).into()).map(Box::new),
            values: value.values.map(|v| (*v).into()).map(Box::new),
            ignore_empty: Some(value.ignore_empty),
        }
    }
}
