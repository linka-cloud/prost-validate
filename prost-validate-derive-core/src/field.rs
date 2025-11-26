use crate::map::MapRules;
use crate::message::MessageRules;
use crate::oneof::OneOfRules;
use crate::rules::FieldRules;
use crate::to_snake;
use crate::utils::{IsTrueAnd, StringOrBool};
use crate::wkt::{WKT, WKT_WRAPPERS};
use anyhow::format_err;
use darling::{FromField, FromMeta, FromVariant};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Type, Variant};

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub name: &'a String,
    #[allow(unused)]
    pub ty: &'a Option<syn::Type>,
    pub optional: bool,
    pub optional_keyword: bool,
    pub required: bool,
    pub boxed: bool,
    pub repeated: bool,
    pub enumeration: Option<String>,
    pub wkt: bool,
    pub message: bool,
    pub map: bool,
    pub oneof: bool,
    pub prost_types: bool,
    pub wrapper: bool,
    pub module: Option<String>,
}

pub trait ToValidationTokens {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream;
}

#[derive(Debug, Clone)]
pub struct Field {
    pub ident: Option<Ident>,
    pub ty: Option<Type>,
    pub validation: FieldValidation,
    pub prost: ProstField,
    pub oneof: bool,
    pub map: bool,
    pub module: Option<String>,
}

impl Field {
    fn new(
        ident: Option<Ident>,
        ty: Option<Type>,
        prost: ProstField,
        validation: FieldValidation,
        map: Option<(String, String)>,
        oneof: bool,
    ) -> Self {
        let typ = ty.to_token_stream().to_string().replace(' ', "");
        let mut validation = validation;
        if validation.r#type.is_none() && validation.message.is_none() {
            if map.is_some() {
                validation.r#type = Some(FieldRules::Map(MapRules::default()));
            } else if prost.message.is_some() && !prost.repeated && !WKT.contains(&typ.as_str()) {
                validation.r#type = Some(FieldRules::Message(MessageRules::default()));
            } else if prost.message.is_some() && !WKT.contains(&typ.as_str()) {
                validation.r#type = Some(FieldRules::Repeated(Box::default()));
            } else if prost.oneof.is_some() {
                validation.r#type = Some(FieldRules::OneOf(OneOfRules::default()));
            }
        }
        Self {
            ident,
            ty,
            prost,
            validation,
            oneof,
            map: map.is_some(),
            module: None,
        }
    }
    pub fn is_wkt(&self) -> bool {
        self.prost
            .message
            .is_true_and(|| {
                let typ = self.ty.to_token_stream().to_string().replace(' ', "");
                WKT.contains(&typ.as_str())
            })
            .unwrap_or_default()
    }

    pub fn is_wrapper(&self) -> bool {
        self.prost
            .message
            .is_true_and(|| {
                let typ = self.ty.to_token_stream().to_string().replace(' ', "");
                WKT_WRAPPERS.contains(&typ.as_str())
            })
            .unwrap_or_default()
    }

    pub fn is_prost_types(&self) -> bool {
        if self.is_wrapper() {
            {
                let typ = self.ty.to_token_stream().to_string().replace(' ', "");
                !typ.contains("::pbjson_types::") && !typ.contains("::google::protobuf::")
            }
        } else {
            Default::default()
        }
    }

    pub fn validate(&self) -> darling::Result<()> {
        let name = self.ident.to_token_stream().to_string();
        if self.prost.repeated
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Repeated(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for repeated field",
                name
            )));
        }
        if self.prost.repeated && self.validation.r#type.is_some() {
            if let FieldRules::Repeated(rules) = self.validation.r#type.as_ref().unwrap() {
                if let Some(ref rules) = rules.items {
                    return Field {
                        prost: ProstField {
                            repeated: false,
                            ..self.prost.clone()
                        },
                        validation: FieldValidation {
                            message: rules.message,
                            r#type: rules.r#type.clone(),
                            ..FieldValidation::default()
                        },
                        ..self.clone()
                    }
                    .validate();
                }
                return Ok(());
            }
        }
        // TODO(adphi): implement
        if self.map {
            return Ok(());
        }
        if self.prost.message.is_none() && self.validation.message.is_some() {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected message rules",
                name
            )));
        }
        if self.prost.message.is_some() {
            let typ = self.ty.to_token_stream().to_string().replace(' ', "");
            match typ.as_str() {
                "::core::option::Option<::prost_types::Timestamp>" => {
                    if self.validation.r#type.is_some()
                        && !matches!(
                            self.validation.r#type.as_ref().unwrap(),
                            FieldRules::Timestamp(_)
                        )
                    {
                        return Err(darling::Error::custom(format_err!(
                            "{}: unexpected rules for timestamp field",
                            name
                        )));
                    }
                }
                "::core::option::Option<::prost_types::Duration>" => {
                    if self.validation.r#type.is_some()
                        && !matches!(
                            self.validation.r#type.as_ref().unwrap(),
                            FieldRules::Duration(_)
                        )
                    {
                        return Err(darling::Error::custom(format_err!(
                            "{}: unexpected rules for duration field",
                            name
                        )));
                    }
                }
                "::core::option::Option<::prost_types::Any>" => {
                    if self.validation.r#type.is_some()
                        && !matches!(self.validation.r#type.as_ref().unwrap(), FieldRules::Any(_))
                    {
                        return Err(darling::Error::custom(format_err!(
                            "{}: unexpected rules for any field",
                            name
                        )));
                    }
                }
                "::core::option::Option<::prost::alloc::string::String>" => {
                    if self.validation.r#type.is_some()
                        && !matches!(
                            self.validation.r#type.as_ref().unwrap(),
                            FieldRules::String(_)
                        )
                    {
                        return Err(darling::Error::custom(format_err!(
                            "{}: unexpected rules for string field",
                            name
                        )));
                    }
                }
                "::core::option::Option<::prost::alloc::vec::Vec<u8>>" => {
                    if self.validation.r#type.is_some()
                        && !matches!(
                            self.validation.r#type.as_ref().unwrap(),
                            FieldRules::Bytes(_)
                        )
                    {
                        return Err(darling::Error::custom(format_err!(
                            "{}: unexpected rules for bytes field",
                            name
                        )));
                    }
                }
                "::core::option::Option<bool>" => {
                    if self.validation.r#type.is_some()
                        && !matches!(
                            self.validation.r#type.as_ref().unwrap(),
                            FieldRules::Bool(_)
                        )
                    {
                        return Err(darling::Error::custom(format_err!(
                            "{}: unexpected rules for bool field",
                            name
                        )));
                    }
                }
                "::core::option::Option<u64>" => {
                    if self.validation.r#type.is_some()
                        && !matches!(
                            self.validation.r#type.as_ref().unwrap(),
                            FieldRules::Uint64(_)
                        )
                    {
                        return Err(darling::Error::custom(format_err!(
                            "{}: unexpected rules for uint64 field",
                            name
                        )));
                    }
                }
                "::core::option::Option<u32>" => {
                    if self.validation.r#type.is_some()
                        && !matches!(
                            self.validation.r#type.as_ref().unwrap(),
                            FieldRules::Uint32(_)
                        )
                    {
                        return Err(darling::Error::custom(format_err!(
                            "{}: unexpected rules for uint32 field",
                            name
                        )));
                    }
                }
                "::core::option::Option<i64>" => {
                    if self.validation.r#type.is_some()
                        && !matches!(
                            self.validation.r#type.as_ref().unwrap(),
                            FieldRules::Int64(_)
                        )
                    {
                        return Err(darling::Error::custom(format_err!(
                            "{}: unexpected rules for int64 field",
                            name
                        )));
                    }
                }
                "::core::option::Option<i32>" => {
                    if self.validation.r#type.is_some()
                        && !matches!(
                            self.validation.r#type.as_ref().unwrap(),
                            FieldRules::Int32(_)
                        )
                    {
                        return Err(darling::Error::custom(format_err!(
                            "{}: unexpected rules for int32 field",
                            name
                        )));
                    }
                }
                "::core::option::Option<f64>" => {
                    if self.validation.r#type.is_some()
                        && !matches!(
                            self.validation.r#type.as_ref().unwrap(),
                            FieldRules::Double(_)
                        )
                    {
                        return Err(darling::Error::custom(format_err!(
                            "{}: unexpected rules for double field",
                            name
                        )));
                    }
                }
                "::core::option::Option<f32>" => {
                    if self.validation.r#type.is_some()
                        && !matches!(
                            self.validation.r#type.as_ref().unwrap(),
                            FieldRules::Float(_)
                        )
                    {
                        return Err(darling::Error::custom(format_err!(
                            "{}: unexpected rules for float field",
                            name
                        )));
                    }
                }
                _ => {}
            }
        }
        if self.prost.enumeration.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Enum(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for enum field",
                name
            )));
        }
        if self.prost.bool.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Bool(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for bool field",
                name
            )));
        }
        if self.prost.string.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::String(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for string field",
                name
            )));
        }
        if self.prost.bytes.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Bytes(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for bytes field",
                name
            )));
        }
        if self.prost.int32.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Int32(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for int32 field",
                name
            )));
        }
        if self.prost.int64.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Int64(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for int64 field",
                name
            )));
        }
        if self.prost.uint32.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Uint32(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for uint32 field",
                name
            )));
        }
        if self.prost.uint64.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Uint64(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for uint64 field",
                name
            )));
        }
        if self.prost.sint32.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Sint32(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for sint32 field",
                name
            )));
        }
        if self.prost.sint64.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Sint64(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for sint64 field",
                name
            )));
        }
        if self.prost.fixed32.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Fixed32(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for fixed32 field",
                name
            )));
        }
        if self.prost.fixed64.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Fixed64(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for fixed64 field",
                name
            )));
        }
        if self.prost.sfixed32.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Sfixed32(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for sfixed32 field",
                name
            )));
        }
        if self.prost.sfixed64.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Sfixed64(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for sfixed64 field",
                name
            )));
        }
        if self.prost.float.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Float(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for float field",
                name
            )));
        }
        if self.prost.double.is_some()
            && self.validation.r#type.is_some()
            && !matches!(
                self.validation.r#type.as_ref().unwrap(),
                FieldRules::Double(_)
            )
        {
            return Err(darling::Error::custom(format_err!(
                "{}: unexpected rules for double field",
                name
            )));
        }
        Ok(())
    }
}

impl FromField for Field {
    fn from_field(field: &syn::Field) -> darling::Result<Self> {
        let mut prost = ProstField::from_field(field)?;
        if prost.oneof.is_some() {
            prost.optional = true;
        }
        let map = prost.parse_map();
        Ok(Self::new(
            field.clone().ident,
            Some(field.clone().ty),
            prost,
            FieldValidation::from_field(field)?,
            map,
            false,
        ))
    }
}

impl FromVariant for Field {
    fn from_variant(variant: &Variant) -> darling::Result<Self> {
        let mut prost = ProstField::from_variant(variant)?;
        let map = prost.parse_map();
        Ok(Self::new(
            Some(variant.clone().ident),
            None,
            prost,
            FieldValidation::from_variant(variant)?,
            map,
            true,
        ))
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Err(err) = self.validate() {
            panic!("{}", err);
        }
        if let Some(ident) = &self.ident {
            let ctx = Context {
                name: &self.validation.name,
                ty: &self.ty,
                optional: self.prost.optional,
                optional_keyword: self.validation.optional.unwrap_or_default(),
                required: self.validation.required(),
                boxed: self.prost.boxed.unwrap_or_default(),
                repeated: self.prost.repeated,
                enumeration: self.prost.enumeration.to_owned(),
                wkt: self.is_wkt(),
                wrapper: self.is_wrapper(),
                message: self.prost.message.unwrap_or_default(),
                map: self.map,
                oneof: self.oneof,
                prost_types: self.is_prost_types(),
                module: self.module.clone(),
            };
            let name = if self.oneof {
                let name = to_snake(ident.to_string());
                if let Some(name) = name.strip_prefix("r#") {
                    Ident::new_raw(name, ident.span())
                } else {
                    Ident::new(&name, ident.span())
                }
            } else {
                ident.to_owned()
            };
            let name = &name;
            let body = self.validation.to_validation_tokens(&ctx, name);
            let required = (ctx.required && !ctx.optional_keyword).then(|| {
                let field = &ctx.name;
                quote! {
                    if self.#name.is_none() {
                        return Err(::prost_validate::Error::new(#field, ::prost_validate::errors::message::Error::Required));
                    }
                }
            });
            let stream = if body.is_empty() {
                quote! {
                    #required
                }
            } else if ctx.oneof {
                if ctx.optional {
                    quote! {
                        if let Some(Self::#ident(ref #name)) = self {
                            #body
                        }
                    }
                } else {
                    quote! {
                        if let Self::#ident(ref #name) = self {
                            #body
                        }
                    }
                }
            } else if ctx.optional {
                if ctx.wrapper && !ctx.prost_types {
                    quote! {
                        #required
                        if let Some(ref #name) = self.#name {
                            let #name = &#name.value;
                            #body
                        }
                    }
                } else {
                    quote! {
                        #required
                        if let Some(ref #name) = self.#name {
                            #body
                        }
                    }
                }
            } else {
                quote! {
                    #required
                    let #name = &self.#name;
                    #body
                }
            };
            tokens.extend(stream);
        }
    }
}

#[derive(Debug, Clone, Default, FromField, FromVariant)]
#[darling(attributes(validate))]
pub struct FieldValidation {
    #[darling(default)]
    pub name: String,
    #[darling(default)]
    pub required: bool,
    pub optional: Option<bool>,
    pub repeated: Option<bool>,
    pub message: Option<MessageRules>,
    pub r#type: Option<FieldRules>,
}

impl FieldValidation {
    pub fn required(&self) -> bool {
        self.required
            || self
                .r#type
                .as_ref()
                .map(|v| v.is_required())
                .unwrap_or_default()
            || self
                .message
                .as_ref()
                .map(|v| v.required)
                .unwrap_or_default()
    }
}

impl ToValidationTokens for FieldValidation {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        FieldValidationInner {
            message: self.message.to_owned(),
            r#type: self.r#type.to_owned(),
        }
        .to_validation_tokens(ctx, name)
    }
}

#[derive(Debug, Clone, FromMeta)]
pub struct FieldValidationInner {
    pub message: Option<MessageRules>,
    pub r#type: Option<FieldRules>,
}

impl ToValidationTokens for FieldValidationInner {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        let stream = match self.to_owned().r#type {
            Some(r) => Some(r.to_validation_tokens(ctx, name)),
            None => self
                .to_owned()
                .message
                .map(|m| m.to_validation_tokens(ctx, name)),
        };
        if stream.is_none() && ctx.message && !ctx.wkt && !ctx.repeated && !ctx.map {
            MessageRules::default().to_validation_tokens(ctx, name)
        } else {
            stream.unwrap_or_default()
        }
    }
}

impl From<FieldValidation> for prost_validate_types::FieldRules {
    fn from(value: FieldValidation) -> Self {
        Self {
            message: value.message.map(|v| v.into()),
            r#type: value.r#type.map(|v| v.into()),
        }
    }
}

impl From<FieldValidationInner> for prost_validate_types::FieldRules {
    fn from(value: FieldValidationInner) -> Self {
        Self {
            message: value.message.map(|v| v.into()),
            r#type: value.r#type.map(|v| v.into()),
        }
    }
}

#[derive(Debug, FromField, FromVariant, Clone)]
#[darling(attributes(prost))]
#[allow(dead_code)]
pub struct ProstField {
    #[allow(unused)]
    tag: Option<u32>,
    #[darling(default)]
    pub optional: bool,
    #[allow(unused)]
    required: Option<bool>,
    #[darling(default)]
    pub repeated: bool,
    map: Option<String>,

    oneof: Option<String>,
    #[allow(unused)]
    tags: Option<String>,

    message: Option<bool>,
    pub enumeration: Option<String>,
    bool: Option<bool>,
    string: Option<bool>,
    bytes: Option<StringOrBool>,
    float: Option<bool>,
    double: Option<bool>,
    int32: Option<bool>,
    int64: Option<bool>,
    uint32: Option<bool>,
    uint64: Option<bool>,
    sint32: Option<bool>,
    sint64: Option<bool>,
    fixed32: Option<bool>,
    fixed64: Option<bool>,
    sfixed32: Option<bool>,
    sfixed64: Option<bool>,

    // others
    #[allow(unused)]
    packed: Option<String>,
    boxed: Option<bool>,
    #[allow(unused)]
    default: Option<String>,
}

impl ProstField {
    fn parse_map(&mut self) -> Option<(String, String)> {
        if self.map.is_none() || self.map.as_ref().is_some_and(|v| v.is_empty()) {
            return None;
        }
        self.map.as_ref().map(|v| {
            let mut iter = v.splitn(2, ',');
            let key = iter.next().unwrap().trim().to_owned();
            let value = iter.next().unwrap().trim().to_owned();
            if value.starts_with("enumeration(") {
                self.enumeration = Some(value[12..value.len() - 1].to_owned());
            }
            if value == "message" {
                self.message = Some(true);
            }
            (key.to_string(), value.to_string())
        })
    }
}

pub fn with_ignore_empty(name: &syn::Ident, ignore_empty: bool, body: TokenStream) -> TokenStream {
    if body.is_empty() {
        return quote! {};
    }
    if ignore_empty {
        quote! {
            if #name.is_empty() {
                return Ok(());
            }
            #body
        }
    } else {
        quote! {
            #body
        }
    }
}
