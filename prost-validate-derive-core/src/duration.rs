use crate::field::{Context, ToValidationTokens};
use crate::sec_and_nanos::{SecAndNanosVec, SecsAndNanos};
use crate::utils::AsDuration;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use prost_types::Duration as PbDuration;
use quote::quote;
use std::ops::Not;
use time::Duration;

#[derive(Debug, FromMeta, Clone)]
pub struct DurationRules {
    #[darling(default)]
    pub required: bool,
    pub r#const: Option<SecsAndNanos>,
    pub lt: Option<SecsAndNanos>,
    pub lte: Option<SecsAndNanos>,
    pub gt: Option<SecsAndNanos>,
    pub gte: Option<SecsAndNanos>,
    pub r#in: Option<SecAndNanosVec>,
    pub not_in: Option<SecAndNanosVec>,
}

pub fn duration_to_tokens(name: &Ident, want: &Duration) -> (TokenStream, TokenStream) {
    let s = want.whole_seconds();
    let n = want.subsec_nanoseconds();
    (
        quote!(::prost_validate::utils::duration(#name.seconds, #name.nanos)),
        quote!(::prost_validate::utils::duration(#s, #n)),
    )
}

impl ToValidationTokens for DurationRules {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        let field = &ctx.name;
        let rules = prost_validate_types::DurationRules::from(self.clone());
        let r#const = rules.r#const.map(|v| v.as_duration()).map(|v| {
            let (got, want) = duration_to_tokens(name, &v);
            quote! {
                if #got != #want {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::Const(#want)));
                }
            }
        });
        // reference implementation: https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/templates/goshared/timestamp.go
        let gte_lte = if let Some(lt) = rules.lt.map(|v| v.as_duration()) {
            if let Some(gt) = rules.gt.map(|v| v.as_duration()) {
                if lt > gt {
                    let (val, lt) = duration_to_tokens(name, &lt);
                    let (_, gt) = duration_to_tokens(name, &gt);
                    quote! {
                        if #val <= #gt || #val >= #lt {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::in_range(false, #gt, #lt, false)));
                        }
                    }
                } else {
                    let (val, lt) = duration_to_tokens(name, &lt);
                    let (_, gt) = duration_to_tokens(name, &gt);
                    quote! {
                        if #val >= #lt && #val <= #gt {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::not_in_range(true, #lt, #gt, true)));
                        }
                    }
                }
            } else if let Some(gte) = rules.gte.map(|v| v.as_duration()) {
                if lt > gte {
                    let (val, lt) = duration_to_tokens(name, &lt);
                    let (_, gte) = duration_to_tokens(name, &gte);
                    quote! {
                        if #val < #gte || #val >= #lt {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::in_range(true, #gte, #lt, false)));
                        }
                    }
                } else {
                    let (val, lt) = duration_to_tokens(name, &lt);
                    let (_, gte) = duration_to_tokens(name, &gte);
                    quote! {
                        if #val >= #lt && #val < #gte {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::not_in_range(true, #lt, #gte, false)));
                        }
                    }
                }
            } else {
                let (val, lt) = duration_to_tokens(name, &lt);
                quote! {
                    if #val >= #lt {
                        return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::Lt(#lt)));
                    }
                }
            }
        } else if let Some(lte) = rules.lte.map(|v| v.as_duration()) {
            if let Some(gt) = rules.gt.map(|v| v.as_duration()) {
                if lte > gt {
                    let (val, lte) = duration_to_tokens(name, &lte);
                    let (_, gt) = duration_to_tokens(name, &gt);
                    quote! {
                        if #val <= #gt || #val > #lte {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::in_range(false, #gt, #lte, true)));
                        }
                    }
                } else {
                    let (val, lte) = duration_to_tokens(name, &lte);
                    let (_, gt) = duration_to_tokens(name, &gt);
                    quote! {
                        if #val >= #lte && #val < #gt {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::not_in_range(false, #lte, #gt, true)));
                        }
                    }
                }
            } else if let Some(gte) = rules.gte.map(|v| v.as_duration()) {
                if lte > gte {
                    let (val, lte) = duration_to_tokens(name, &lte);
                    let (_, gte) = duration_to_tokens(name, &gte);
                    quote! {
                        if #val < #gte || #val > #lte {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::in_range(true, #gte, #lte, true)));
                        }
                    }
                } else {
                    let (val, lte) = duration_to_tokens(name, &lte);
                    let (_, gte) = duration_to_tokens(name, &gte);
                    quote! {
                        if #val > #lte && #val < #gte {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::not_in_range(false, #lte, #gte, false)));
                        }
                    }
                }
            } else {
                let (val, lte) = duration_to_tokens(name, &lte);
                quote! {
                    if #val > #lte {
                        return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::Lte(#lte)));
                    }
                }
            }
        } else if let Some(gt) = rules.gt.map(|v| v.as_duration()) {
            let (val, gt) = duration_to_tokens(name, &gt);
            quote! {
                if #val <= #gt {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::Gt(#gt)));
                }
            }
        } else if let Some(gte) = rules.gte.map(|v| v.as_duration()) {
            let (val, gte) = duration_to_tokens(name, &gte);
            quote! {
                if #val < #gte {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::Gte(#gte)));
                }
            }
        } else {
            quote! {}
        };
        let r#in = rules.r#in.is_empty().not().then(|| {
            let vals = rules
                .r#in
                .iter()
                .map(|v| v.as_duration())
                .collect::<Vec<Duration>>();
            let (val, _) = duration_to_tokens(name, &vals[0]);
            let vals = rules
                .r#in
                .iter()
                .map(|PbDuration { seconds, nanos }| quote! { ::time::Duration::new(#seconds, #nanos)})
                .collect::<Vec<_>>();
            quote! {
                let values = [#(#vals),*];
                if !values.contains(&#val) {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::In(values.to_vec())));
                }
            }
        });
        let not_in = rules.not_in.is_empty().not().then(|| {
            let vals = rules
                .not_in
                .iter()
                .map(|v| v.as_duration())
                .collect::<Vec<Duration>>();
            let (val, _) = duration_to_tokens(name, &vals[0]);
            let vals = rules
                .not_in
                .iter()
                .map(|PbDuration { seconds, nanos }| quote! { ::time::Duration::new(#seconds, #nanos)})
                .collect::<Vec<_>>();
            quote! {
                let values = [#(#vals),*];
                if values.contains(&#val) {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::duration::Error::NotIn(values.to_vec())));
                }
            }
        });
        quote! {
            #r#const
            #gte_lte
            #r#in
            #not_in
        }
    }
}

impl From<DurationRules> for prost_validate_types::DurationRules {
    fn from(value: DurationRules) -> Self {
        prost_validate_types::DurationRules {
            required: Some(value.required),
            r#const: value.r#const.map(From::from),
            lt: value.lt.map(From::from),
            lte: value.lte.map(From::from),
            gt: value.gt.map(From::from),
            gte: value.gte.map(From::from),
            r#in: value
                .r#in
                .unwrap_or_default()
                .iter()
                .map(|v| v.to_owned())
                .map(From::from)
                .collect(),
            not_in: value
                .not_in
                .unwrap_or_default()
                .iter()
                .map(|v| v.to_owned())
                .map(From::from)
                .collect(),
        }
    }
}
