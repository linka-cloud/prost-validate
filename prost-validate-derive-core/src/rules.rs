use crate::any::AnyRules;
use crate::bool::BoolRules;
use crate::bytes::BytesRules;
use crate::duration::DurationRules;
use crate::field::{Context, ToValidationTokens};
use crate::list::RepeatedRules;
use crate::map::MapRules;
use crate::message::MessageRules;
use crate::number::{
    DoubleRules, Fixed32Rules, Fixed64Rules, FloatRules, Int32Rules, Int64Rules, SFixed32Rules,
    SFixed64Rules, SInt32Rules, SInt64Rules, UInt32Rules, UInt64Rules,
};
use crate::oneof::OneOfRules;
use crate::r#enum::EnumRules;
use crate::string::StringRules;
use crate::timestamp::TimestampRules;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};

#[derive(Debug, Clone, FromMeta)]
pub enum FieldRules {
    None,
    Any(AnyRules),
    Bool(BoolRules),
    Bytes(BytesRules),
    Duration(DurationRules),
    Message(MessageRules),
    #[darling(rename = "r#enum")]
    Enum(EnumRules),
    Repeated(Box<RepeatedRules>),
    Map(MapRules),
    OneOf(OneOfRules),
    String(StringRules),
    Timestamp(TimestampRules),
    Int32(Int32Rules),
    Int64(Int64Rules),
    Uint32(UInt32Rules),
    Uint64(UInt64Rules),
    Sint32(SInt32Rules),
    Sint64(SInt64Rules),
    Fixed32(Fixed32Rules),
    Fixed64(Fixed64Rules),
    Sfixed32(SFixed32Rules),
    Sfixed64(SFixed64Rules),
    Float(FloatRules),
    Double(DoubleRules),
}

impl FieldRules {
    pub fn is_required(&self) -> bool {
        match self {
            FieldRules::Any(v) => v.required,
            FieldRules::Duration(v) => v.required,
            FieldRules::Message(v) => v.required,
            FieldRules::OneOf(v) => v.required,
            FieldRules::Timestamp(v) => v.required,
            _ => false,
        }
    }
}

impl ToValidationTokens for FieldRules {
    fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
        match self {
            FieldRules::None => TokenStream::new(),
            FieldRules::Any(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Bool(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Bytes(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Duration(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Message(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Enum(v) => v.to_validation_tokens(ctx, name),
            FieldRules::OneOf(v) => v.to_validation_tokens(ctx, name),
            FieldRules::String(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Timestamp(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Int32(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Int64(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Uint32(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Uint64(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Sint32(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Sint64(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Fixed32(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Fixed64(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Sfixed32(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Sfixed64(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Float(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Double(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Repeated(v) => v.to_validation_tokens(ctx, name),
            FieldRules::Map(v) => v.to_validation_tokens(ctx, name),
        }
    }
}

impl From<FieldRules> for prost_validate_types::field_rules::Type {
    fn from(value: FieldRules) -> Self {
        match value {
            FieldRules::Any(v) => prost_validate_types::field_rules::Type::Any(v.into()),
            FieldRules::Bool(v) => prost_validate_types::field_rules::Type::Bool(v.into()),
            FieldRules::Bytes(v) => prost_validate_types::field_rules::Type::Bytes(v.into()),
            FieldRules::Duration(v) => prost_validate_types::field_rules::Type::Duration(v.into()),
            FieldRules::Enum(v) => prost_validate_types::field_rules::Type::Enum(v.into()),
            FieldRules::Repeated(v) => prost_validate_types::field_rules::Type::Repeated(v.into()),
            FieldRules::Map(v) => prost_validate_types::field_rules::Type::Map(Box::new(v.into())),
            FieldRules::String(v) => prost_validate_types::field_rules::Type::String(v.into()),
            FieldRules::Timestamp(v) => {
                prost_validate_types::field_rules::Type::Timestamp(v.into())
            }
            FieldRules::Int32(v) => prost_validate_types::field_rules::Type::Int32(v.into()),
            FieldRules::Int64(v) => prost_validate_types::field_rules::Type::Int64(v.into()),
            FieldRules::Uint32(v) => prost_validate_types::field_rules::Type::Uint32(v.into()),
            FieldRules::Uint64(v) => prost_validate_types::field_rules::Type::Uint64(v.into()),
            FieldRules::Sint32(v) => prost_validate_types::field_rules::Type::Sint32(v.into()),
            FieldRules::Sint64(v) => prost_validate_types::field_rules::Type::Sint64(v.into()),
            FieldRules::Fixed32(v) => prost_validate_types::field_rules::Type::Fixed32(v.into()),
            FieldRules::Fixed64(v) => prost_validate_types::field_rules::Type::Fixed64(v.into()),
            FieldRules::Sfixed32(v) => prost_validate_types::field_rules::Type::Sfixed32(v.into()),
            FieldRules::Sfixed64(v) => prost_validate_types::field_rules::Type::Sfixed64(v.into()),
            FieldRules::Float(v) => prost_validate_types::field_rules::Type::Float(v.into()),
            FieldRules::Double(v) => prost_validate_types::field_rules::Type::Double(v.into()),

            FieldRules::None => panic!("FieldRules::None is not supported"),
            FieldRules::Message(_) => panic!("FieldRules::Message is not supported"),
            FieldRules::OneOf(_) => panic!("FieldRules::OneOf is not supported"),
        }
    }
}
