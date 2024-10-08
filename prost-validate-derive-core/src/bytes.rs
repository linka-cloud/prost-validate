use crate::field::{with_ignore_empty, Context, ToValidationTokens};
use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream};
use prost_validate_types::bytes_rules;
use quote::quote;
use std::ops::Not;
use syn::LitByteStr;

#[derive(Debug, FromMeta, Clone)]
pub struct BytesRules {
    pub r#const: Option<LitByteStr>,
    pub len: Option<u64>,
    pub min_len: Option<u64>,
    pub max_len: Option<u64>,
    pub pattern: Option<String>,
    pub prefix: Option<LitByteStr>,
    pub suffix: Option<LitByteStr>,
    pub contains: Option<LitByteStr>,
    pub r#in: Option<Vec<LitByteStr>>,
    pub not_in: Option<Vec<LitByteStr>>,
    #[darling(default)]
    pub ignore_empty: bool,
    pub well_known: Option<WellKnown>,
}

impl ToValidationTokens for BytesRules {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        let rules = prost_validate_types::BytesRules::from(self.to_owned());
        let r#const = rules.r#const.map(|v| {
            let v = LitByteStr::new(v.as_slice(), Span::call_site());
            let field = &ctx.name;
            let err = format!("is not equal to \"{:?}\"", v.value());
            quote! {
                if !#name.iter().eq(#v.iter()) {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let len = rules.len.map(|v| {
            let v = v as usize;
            let field = &ctx.name;
            let err = format!("length is not equal to {v}");
            quote! {
                if #name.len() != #v {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let min_len = rules.min_len.map(|v| {
            let v = v as usize;
            let field = &ctx.name;
            let err = format!("length is less than {v}");
            quote! {
                if #name.len() < #v {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let max_len = rules.max_len.map(|v| {
            let v = v as usize;
            let field = &ctx.name;
            let err = format!("length is greater than {v}");
            quote! {
                if #name.len() > #v {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let pattern = rules.pattern.map(|v| {
            let field = &ctx.name;
            let err = format!("does not match pattern \"{v}\"");
            if let Err(err ) = regex::bytes::Regex::new(&v) {
                panic!("{field}: Invalid regex pattern: {}", err);
            }
            quote! {
                let regex = ::regex::bytes::Regex::new(#v).map_err(|err| {
                    ::prost_validate::Error::new(#field, format!("Invalid regex pattern: {}", err))
                })?;
                if !regex.is_match(#name.iter().as_slice()) {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let prefix = rules.prefix.map(|v| {
            let v = LitByteStr::new(v.as_slice(), Span::call_site());
            let field = &ctx.name;
            let err = format!("does not start with \"{:?}\"", v.value());
            quote! {
                if !#name.starts_with(#v) {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let suffix = rules.suffix.map(|v| {
            let v = LitByteStr::new(v.as_slice(), Span::call_site());
            let field = &ctx.name;
            let err = format!("does not end with \"{:?}\"", v.value());
            quote! {
                if !#name.ends_with(#v) {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let contains = rules.contains.map(|v| {
            let v = LitByteStr::new(v.as_slice(), Span::call_site());
            let field = &ctx.name;
            let err = format!("does not contain \"{:?}\"", v.value());
            quote! {
                if !::prost_validate::ValidateBytesExt::contains(&#name, #v.as_slice()) {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let r#in = rules.r#in.is_empty().not().then(|| {
            let v = rules
                .r#in
                .iter()
                .map(|v| LitByteStr::new(v.as_slice(), Span::call_site()))
                .collect::<Vec<_>>();
            let field = &ctx.name;
            let err = format!(
                "is not in {:?}",
                v.iter().map(|v| v.value()).collect::<Vec<_>>()
            );
            quote! {
                if ![#(#v.to_vec()),*].contains(&#name) {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let not_in = rules.not_in.is_empty().not().then(|| {
            let v = rules
                .not_in
                .iter()
                .map(|v| LitByteStr::new(v.as_slice(), Span::call_site()))
                .collect::<Vec<_>>();
            let field = &ctx.name;
            let err = format!(
                "is in {:?}",
                v.iter().map(|v| v.value()).collect::<Vec<_>>()
            );
            quote! {
                if [#(#v.to_vec()),*].contains(&#name) {
                    return Err(::prost_validate::Error::new(#field, #err));
                }
            }
        });
        let well_known = rules.well_known.map(|v| match v {
            bytes_rules::WellKnown::Ip(true) => {
                let field = &ctx.name;
                let err = "is not a valid ip";
                quote! {
                    if #name.len() != 4 && #name.len() != 16 {
                        return Err(::prost_validate::Error::new(#field, #err));
                    }
                }
            }
            bytes_rules::WellKnown::Ipv4(true) => {
                let field = &ctx.name;
                let err = "is not a valid ipv4";
                quote! {
                    if #name.len() != 4 {
                        return Err(::prost_validate::Error::new(#field, #err));
                    }
                }
            }
            bytes_rules::WellKnown::Ipv6(true) => {
                let field = &ctx.name;
                let err = "is not a valid ipv6";
                quote! {
                    if #name.len() != 16 {
                        return Err(::prost_validate::Error::new(#field, #err));
                    }
                }
            }
            _ => quote! {},
        });
        with_ignore_empty(
            name,
            self.ignore_empty,
            quote! {
                #r#const
                #len
                #min_len
                #max_len
                #pattern
                #prefix
                #suffix
                #contains
                #r#in
                #not_in
                #well_known
            },
        )
    }
}

impl From<BytesRules> for prost_validate_types::BytesRules {
    fn from(value: BytesRules) -> Self {
        prost_validate_types::BytesRules {
            r#const: value.r#const.map(|v| v.value()),
            len: value.len,
            min_len: value.min_len,
            max_len: value.max_len,
            pattern: value.pattern,
            prefix: value.prefix.map(|v| v.value()),
            suffix: value.suffix.map(|v| v.value()),
            contains: value.contains.map(|v| v.value()),
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
            ignore_empty: Some(value.ignore_empty),
            well_known: value.well_known.map(|v| v.into()),
        }
    }
}

#[derive(Debug, Clone, FromMeta)]
pub enum WellKnown {
    Ip(bool),
    Ipv4(bool),
    Ipv6(bool),
}

impl From<WellKnown> for prost_validate_types::bytes_rules::WellKnown {
    fn from(value: WellKnown) -> Self {
        match value {
            WellKnown::Ip(v) => prost_validate_types::bytes_rules::WellKnown::Ip(v),
            WellKnown::Ipv4(v) => prost_validate_types::bytes_rules::WellKnown::Ipv4(v),
            WellKnown::Ipv6(v) => prost_validate_types::bytes_rules::WellKnown::Ipv6(v),
        }
    }
}
