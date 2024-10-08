use crate::field::{with_ignore_empty, Context, ToValidationTokens};
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use prost_validate_types::string_rules;
use quote::quote;
use std::ops::Not;

#[derive(Debug, FromMeta, Clone)]
pub struct StringRules {
    pub r#const: Option<String>,
    pub len: Option<u64>,
    pub min_len: Option<u64>,
    pub max_len: Option<u64>,
    pub len_bytes: Option<u64>,
    pub min_bytes: Option<u64>,
    pub max_bytes: Option<u64>,
    pub pattern: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub contains: Option<String>,
    pub not_contains: Option<String>,
    pub r#in: Option<Vec<syn::LitStr>>,
    pub not_in: Option<Vec<syn::LitStr>>,
    pub strict: Option<bool>,
    #[darling(default)]
    pub ignore_empty: bool,
    pub well_known: Option<WellKnown>,
}

impl ToValidationTokens for StringRules {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        let rules = prost_validate_types::StringRules::from(self.to_owned());
        let r#const = rules.r#const.map(|v| {
            let field = &ctx.name;
            quote! {
                if #name != #v {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Const(#v.to_string())));
                }
            }
        });
        let len = rules.len.map(|v| {
            let v = v as usize;
            let field = &ctx.name;
            quote! {
                if #name.chars().count() != #v {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Len(#v)));
                }
            }
        });
        let min_len = rules.min_len.map(|v| {
            let v = v as usize;
            let field = &ctx.name;
            quote! {
                if #name.chars().count() < #v {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::MinLen(#v)));
                }
            }
        });
        let max_len = rules.max_len.map(|v| {
            let v = v as usize;
            let field = &ctx.name;
            quote! {
                if #name.chars().count() > #v {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::MaxLen(#v)));
                }
            }
        });
        let len_bytes = rules.len_bytes.map(|v| {
            let v = v as usize;
            let field = &ctx.name;
            quote! {
                if #name.len() != #v {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::LenBytes(#v)));
                }
            }
        });
        let min_bytes = rules.min_bytes.map(|v| {
            let v = v as usize;
            let field = &ctx.name;
            quote! {
                if #name.len() < #v {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::MinLenBytes(#v)));
                }
            }
        });
        let max_bytes = rules.max_bytes.map(|v| {
            let v = v as usize;
            let field = &ctx.name;
            quote! {
                if #name.len() > #v {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::MaxLenBytes(#v)));
                }
            }
        });
        let pattern = rules.pattern.map(|v| {
            let field = &ctx.name;
            if let Err(err) = regex::Regex::new(&v) {
                panic!("{field}: Invalid regex pattern: {}", err);
            }
            quote! {
                let regex = ::regex::Regex::new(#v).map_err(|e| {
                    ::prost_validate::Error::new(#field, format!("invalid regex pattern: {}", e))
                })?;
                if !regex.is_match(#name.as_str()) {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Pattern(#v.to_string())));
                }
            }
        });
        let prefix = rules.prefix.map(|v| {
            let field = &ctx.name;
            quote! {
                if !#name.starts_with(#v) {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Prefix(#v.to_string())));
                }
            }
        });
        let suffix = rules.suffix.map(|v| {
            let field = &ctx.name;
            quote! {
                if !#name.ends_with(#v) {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Suffix(#v.to_string())));
                }
            }
        });
        let contains = rules.contains.map(|v| {
            let field = &ctx.name;
            quote! {
                if !#name.contains(#v) {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Contains(#v.to_string())));
                }
            }
        });
        let not_contains = rules.not_contains.map(|v| {
            let field = &ctx.name;
            quote! {
                if #name.contains(#v) {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::NotContains(#v.to_string())));
                }
            }
        });
        let r#in = rules.r#in.is_empty().not().then(|| {
            let v = rules.r#in;
            let field = &ctx.name;
            quote! {
                let values = [#(#v),*];
                if !values.contains(&#name.as_str()) {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::In(values.iter().map(|v| v.to_string()).collect())));
                }
            }
        });
        let not_in = rules.not_in.is_empty().not().then(|| {
            let v = rules.not_in;
            let field = &ctx.name;
            quote! {
                let values = [#(#v),*];
                if values.contains(&#name.as_str()) {
                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::NotIn(values.iter().map(|v| v.to_string()).collect())));
                }
            }
        });
        let well_known = rules.well_known.map(|v| {
            match v {
                string_rules::WellKnown::Email(true) => {
                    let field = &ctx.name;
                    quote! {
                        if let Err(_) = ::prost_validate::ValidateStringExt::validate_email(&#name) {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Email));
                        }
                    }
                }
                string_rules::WellKnown::Hostname(true) => {
                    let field = &ctx.name;
                    quote! {
                        if let Err(_) = ::prost_validate::ValidateStringExt::validate_hostname(&#name) {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Hostname));
                        }
                    }
                }
                string_rules::WellKnown::Ip(true) => {
                    let field = &ctx.name;
                    quote! {
                        if let Err(_) = ::prost_validate::ValidateStringExt::validate_ip(&#name) {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Ip));
                        }
                    }
                }
                string_rules::WellKnown::Ipv4(true) => {
                    let field = &ctx.name;
                    quote! {
                        if let Err(_) = ::prost_validate::ValidateStringExt::validate_ipv4(&#name) {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Ipv4));
                        }
                    }
                }
                string_rules::WellKnown::Ipv6(true) => {
                    let field = &ctx.name;
                    quote! {
                        if let Err(_) = ::prost_validate::ValidateStringExt::validate_ipv6(&#name) {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Ipv6));
                        }
                    }
                }
                string_rules::WellKnown::Uri(true) => {
                    let field = &ctx.name;
                    quote! {
                        if let Err(_) = ::prost_validate::ValidateStringExt::validate_uri(&#name) {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Uri));
                        }
                    }
                }
                string_rules::WellKnown::UriRef(true) => {
                    let field = &ctx.name;
                    quote! {
                        if let Err(_) = ::prost_validate::ValidateStringExt::validate_uri_ref(&#name) {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::UriRef));
                        }
                    }
                }
                string_rules::WellKnown::Address(true) => {
                    let field = &ctx.name;
                    quote! {
                        if let Err(_) = ::prost_validate::ValidateStringExt::validate_address(&#name) {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Address));
                        }
                    }
                }
                string_rules::WellKnown::Uuid(true) => {
                    let field = &ctx.name;
                    quote! {
                        if let Err(_) = ::prost_validate::ValidateStringExt::validate_uuid(&#name) {
                            return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::Uuid));
                        }
                    }
                }
                string_rules::WellKnown::WellKnownRegex(wk) => {
                    let strict = rules.strict.unwrap_or(true);
                    match prost_validate_types::KnownRegex::try_from(wk) {
                        Ok(prost_validate_types::KnownRegex::HttpHeaderName) => {
                            let field = &ctx.name;
                            quote! {
                                if let Err(_) = ::prost_validate::ValidateStringExt::validate_header_name(&#name, #strict) {
                                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::HttpHeaderName));
                                }
                            }
                        }
                        Ok(prost_validate_types::KnownRegex::HttpHeaderValue) => {
                            let field = &ctx.name;
                            quote! {
                                if let Err(_) = ::prost_validate::ValidateStringExt::validate_header_value(&#name, #strict) {
                                    return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::string::Error::HttpHeaderValue));
                                }
                            }
                        }
                        _ => quote! {}
                    }
                }
                _ => quote! {}
            }
        });
        with_ignore_empty(
            name,
            self.ignore_empty,
            quote! {
                #r#const
                #len
                #min_len
                #max_len
                #len_bytes
                #min_bytes
                #max_bytes
                #pattern
                #prefix
                #suffix
                #contains
                #not_contains
                #r#in
                #not_in
                #well_known
            },
        )
    }
}

impl From<StringRules> for prost_validate_types::StringRules {
    fn from(value: StringRules) -> Self {
        prost_validate_types::StringRules {
            r#const: value.r#const,
            len: value.len,
            min_len: value.min_len,
            max_len: value.max_len,
            len_bytes: value.len_bytes,
            min_bytes: value.min_bytes,
            max_bytes: value.max_bytes,
            pattern: value.pattern,
            prefix: value.prefix,
            suffix: value.suffix,
            contains: value.contains,
            not_contains: value.not_contains,
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
            strict: value.strict,
            ignore_empty: Some(value.ignore_empty),
            well_known: value.well_known.map(|v| v.into()),
        }
    }
}

#[derive(Debug, FromMeta, Clone)]
pub enum WellKnown {
    Email(bool),
    Hostname(bool),
    Ip(bool),
    Ipv4(bool),
    Ipv6(bool),
    Uri(bool),
    UriRef(bool),
    Address(bool),
    Uuid(bool),
    #[allow(clippy::enum_variant_names)]
    WellKnownRegex(i32),
}

impl From<WellKnown> for prost_validate_types::string_rules::WellKnown {
    fn from(value: WellKnown) -> Self {
        match value {
            WellKnown::Email(value) => prost_validate_types::string_rules::WellKnown::Email(value),
            WellKnown::Hostname(value) => {
                prost_validate_types::string_rules::WellKnown::Hostname(value)
            }
            WellKnown::Ip(value) => prost_validate_types::string_rules::WellKnown::Ip(value),
            WellKnown::Ipv4(value) => prost_validate_types::string_rules::WellKnown::Ipv4(value),
            WellKnown::Ipv6(value) => prost_validate_types::string_rules::WellKnown::Ipv6(value),
            WellKnown::Uri(value) => prost_validate_types::string_rules::WellKnown::Uri(value),
            WellKnown::UriRef(value) => {
                prost_validate_types::string_rules::WellKnown::UriRef(value)
            }
            WellKnown::Address(value) => {
                prost_validate_types::string_rules::WellKnown::Address(value)
            }
            WellKnown::Uuid(value) => prost_validate_types::string_rules::WellKnown::Uuid(value),
            WellKnown::WellKnownRegex(value) => {
                prost_validate_types::string_rules::WellKnown::WellKnownRegex(value)
            }
        }
    }
}
