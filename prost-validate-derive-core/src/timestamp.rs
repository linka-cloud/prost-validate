use crate::duration::duration_to_tokens;
use crate::field::{Context, ToValidationTokens};
use crate::sec_and_nanos::SecsAndNanos;
use crate::utils::{AsDateTime, AsDuration};
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use time::OffsetDateTime;

#[derive(Debug, FromMeta, Clone)]
pub struct TimestampRules {
    #[darling(default)]
    pub required: bool,
    pub r#const: Option<SecsAndNanos>,
    pub lt: Option<SecsAndNanos>,
    pub lte: Option<SecsAndNanos>,
    pub gt: Option<SecsAndNanos>,
    pub gte: Option<SecsAndNanos>,
    pub lt_now: Option<bool>,
    pub gt_now: Option<bool>,
    pub within: Option<SecsAndNanos>,
}

pub fn datetime_to_tokens(name: &Ident, want: &OffsetDateTime) -> (TokenStream, TokenStream) {
    let s = want.unix_timestamp();
    let n = want.nanosecond() as i32;
    (
        quote!(::prost_validate::utils::datetime(#name.seconds, #name.nanos)),
        quote!(::prost_validate::utils::datetime(#s, #n)),
    )
}

impl From<TimestampRules> for prost_validate_types::TimestampRules {
    fn from(value: TimestampRules) -> Self {
        prost_validate_types::TimestampRules {
            required: Some(value.required),
            r#const: value.r#const.map(From::from),
            lt: value.lt.map(From::from),
            lte: value.lte.map(From::from),
            gt: value.gt.map(From::from),
            gte: value.gte.map(From::from),
            lt_now: value.lt_now,
            gt_now: value.gt_now,
            within: value.within.map(From::from),
        }
    }
}

impl ToValidationTokens for TimestampRules {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        let field = &ctx.name;
        let rules = prost_validate_types::TimestampRules::from(self.clone());
        let r#const = rules.r#const.map(|v| v.as_datetime()).map(|v| {
            let (got, want) = datetime_to_tokens(name, &v);
            quote! {
                if #got != #want {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::Const(#want)));
                }
            }
        });
        // reference implementation: https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/templates/goshared/timestamp.go
        let gte_lte = if let Some(lt) = rules.lt.map(|v| v.as_datetime()) {
            if let Some(gt) = rules.gt.map(|v| v.as_datetime()) {
                if lt > gt {
                    let (val, lt) = datetime_to_tokens(name, &lt);
                    let (_, gt) = datetime_to_tokens(name, &gt);
                    quote! {
                        if #val <= #gt || #val >= #lt {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::in_range(false, #gt, #lt, false)));
                        }
                    }
                } else {
                    let (val, lt) = datetime_to_tokens(name, &lt);
                    let (_, gt) = datetime_to_tokens(name, &gt);
                    quote! {
                        if #val >= #lt && #val <= #gt {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::not_in_range(true, #lt, #gt, true)));
                        }
                    }
                }
            } else if let Some(gte) = rules.gte.map(|v| v.as_datetime()) {
                if lt > gte {
                    let (val, lt) = datetime_to_tokens(name, &lt);
                    let (_, gte) = datetime_to_tokens(name, &gte);
                    quote! {
                        if #val < #gte || #val >= #lt {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::in_range(true, #gte, #lt, false)));
                        }
                    }
                } else {
                    let (val, lt) = datetime_to_tokens(name, &lt);
                    let (_, gte) = datetime_to_tokens(name, &gte);
                    quote! {
                        if #val >= #lt && #val < #gte {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::not_in_range(true, #lt, #gte, false)));
                        }
                    }
                }
            } else {
                let (val, lt) = datetime_to_tokens(name, &lt);
                quote! {
                    if #val >= #lt {
                        return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::Lt(#lt)));
                    }
                }
            }
        } else if let Some(lte) = rules.lte.map(|v| v.as_datetime()) {
            if let Some(gt) = rules.gt.map(|v| v.as_datetime()) {
                if lte > gt {
                    let (val, lte) = datetime_to_tokens(name, &lte);
                    let (_, gt) = datetime_to_tokens(name, &gt);
                    quote! {
                        if #val <= #gt || #val > #lte {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::in_range(false, #gt, #lte, true)));
                        }
                    }
                } else {
                    let (val, lte) = datetime_to_tokens(name, &lte);
                    let (_, gt) = datetime_to_tokens(name, &gt);
                    quote! {
                        if #val >= #lte && #val < #gt {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::not_in_range(false, #lte, #gt, true)));
                        }
                    }
                }
            } else if let Some(gte) = rules.gte.map(|v| v.as_datetime()) {
                if lte > gte {
                    let (val, lte) = datetime_to_tokens(name, &lte);
                    let (_, gte) = datetime_to_tokens(name, &gte);
                    quote! {
                        if #val < #gte || #val > #lte {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::in_range(true, #gte, #lte, true)));
                        }
                    }
                } else {
                    let (val, lte) = datetime_to_tokens(name, &lte);
                    let (_, gte) = datetime_to_tokens(name, &gte);
                    quote! {
                        if #val > #lte && #val < #gte {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::not_in_range(false, #lte, #gte, false)));
                        }
                    }
                }
            } else {
                let (val, lte) = datetime_to_tokens(name, &lte);
                quote! {
                    if #val > #lte {
                        return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::Lte(#lte)));
                    }
                }
            }
        } else if let Some(gt) = rules.gt.map(|v| v.as_datetime()) {
            let (val, gt) = datetime_to_tokens(name, &gt);
            quote! {
                if #val <= #gt {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::Gt(#gt)));
                }
            }
        } else if let Some(gte) = rules.gte.map(|v| v.as_datetime()) {
            let (val, gte) = datetime_to_tokens(name, &gte);
            quote! {
                if #val < #gte {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::Gte(#gte)));
                }
            }
        } else if let Some(true) = rules.lt_now {
            if let Some(ref within) = rules.within.map(|v| v.as_duration()) {
                let (val, _) = datetime_to_tokens(name, &OffsetDateTime::now_utc());
                let (_, d) = duration_to_tokens(name, within);
                quote! {
                    let now = ::time::OffsetDateTime::now_utc();
                    let d = #d;
                    if #val >= now || #val < now - d {
                        return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::LtNowWithin(d)));
                    }
                }
            } else {
                let (val, _) = datetime_to_tokens(name, &OffsetDateTime::now_utc());
                quote! {
                    let now = ::time::OffsetDateTime::now_utc();
                    if #val >= now {
                        return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::LtNow));
                    }
                }
            }
        } else if let Some(true) = rules.gt_now {
            if let Some(ref within) = rules.within.map(|v| v.as_duration()) {
                let (val, _) = datetime_to_tokens(name, &OffsetDateTime::now_utc());
                let (_, d) = duration_to_tokens(name, within);
                quote! {
                     let now = ::time::OffsetDateTime::now_utc();
                     let d = #d;
                     if #val <= now || #val > now + d {
                         return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::GtNowWithin(d)));
                     }
                }
            } else {
                let (val, _) = datetime_to_tokens(name, &OffsetDateTime::now_utc());
                quote! {
                    let now = ::time::OffsetDateTime::now_utc();
                    if #val <= now {
                        return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::GtNow));
                    }
                }
            }
        } else if let Some(ref within) = rules.within.map(|v| v.as_duration()) {
            let (val, _) = datetime_to_tokens(name, &OffsetDateTime::now_utc());
            let (_, d) = duration_to_tokens(name, within);
            quote! {
                let now = ::time::OffsetDateTime::now_utc();
                let d = #d;
                if #val < now - d || #val > now + d {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::timestamp::Error::Within(d)));
                }
            }
        } else {
            quote! {}
        };
        quote! {
            #r#const
            #gte_lte
        }
    }
}
