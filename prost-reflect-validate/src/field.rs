use crate::bool::make_validate_bool;
use crate::bytes::make_validate_bytes;
use crate::message::make_validate_message;
use crate::number::{
    make_validate_double, make_validate_fixed32, make_validate_fixed64, make_validate_float,
    make_validate_i32, make_validate_i64, make_validate_sfixed32, make_validate_sfixed64,
    make_validate_sint32, make_validate_sint64, make_validate_u32, make_validate_u64,
};
use crate::r#enum::make_validate_enum;
use crate::registry::ValidationFn;
use crate::string::make_validate_string;
use prost_reflect::{FieldDescriptor, Kind, Value};
use prost_validate::Result;
use prost_validate_types::FieldRules;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

type ValueValidationFn = Arc<
    dyn Fn(Cow<Value>, &FieldRules, &HashMap<String, ValidationFn>) -> Result<bool> + Send + Sync,
>;

macro_rules! as_validation_func {
    ($fns:expr,$typ:ident,$conv:ident) => {{
        let fns = $fns;
        Arc::new(
            move |val: Cow<Value>,
                  rules: &FieldRules,
                  _: &HashMap<String, ValidationFn>|
                  -> Result<bool> {
                for f in &fns {
                    if !f(val.$conv(), &rules)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            },
        )
    }};
}

pub(crate) fn make_validate_field(
    m: &mut HashMap<String, ValidationFn>,
    field: &FieldDescriptor,
    rules: &FieldRules,
) -> ValueValidationFn {
    match field.kind() {
        Kind::Uint64 => as_validation_func!(make_validate_u64(field, rules), Uint64, as_u64),
        Kind::Uint32 => as_validation_func!(make_validate_u32(field, rules), Uint32, as_u32),
        Kind::Int64 => as_validation_func!(make_validate_i64(field, rules), Int64, as_i64),
        Kind::Int32 => as_validation_func!(make_validate_i32(field, rules), Int32, as_i32),
        Kind::Double => as_validation_func!(make_validate_double(field, rules), Double, as_f64),
        Kind::Float => as_validation_func!(make_validate_float(field, rules), Float, as_f32),
        Kind::Sint32 => as_validation_func!(make_validate_sint32(field, rules), Int32, as_i32),
        Kind::Sint64 => as_validation_func!(make_validate_sint64(field, rules), Int64, as_i64),
        Kind::Fixed32 => as_validation_func!(make_validate_fixed32(field, rules), Uint32, as_u32),
        Kind::Fixed64 => as_validation_func!(make_validate_fixed64(field, rules), Uint64, as_u64),
        Kind::Sfixed32 => as_validation_func!(make_validate_sfixed32(field, rules), Int32, as_i32),
        Kind::Sfixed64 => as_validation_func!(make_validate_sfixed64(field, rules), Int64, as_i64),
        Kind::Bool => as_validation_func!(make_validate_bool(field, rules), Bool, as_bool),
        Kind::String => as_validation_func!(make_validate_string(field, rules), String, as_string),
        Kind::Enum(_) => as_validation_func!(make_validate_enum(field, rules), i32, as_enum_number),
        Kind::Bytes => {
            let fns = make_validate_bytes(field, rules);
            Arc::new(
                move |val: Cow<Value>, rules: &FieldRules, _| -> prost_validate::Result<bool> {
                    let bytes = val.as_bytes().map(|v| Arc::new(v.clone()));
                    for f in &fns {
                        let bytes = bytes.clone();
                        if !f(bytes, rules)? {
                            return Ok(false);
                        }
                    }
                    Ok(true)
                },
            )
        }
        Kind::Message(_) => {
            let fns = make_validate_message(m, field, rules);
            Arc::new(
                move |val: Cow<Value>, rules: &FieldRules, m| -> prost_validate::Result<bool> {
                    // When the value is not set the Value is a Cow::Owned(desc.default_value())
                    let msg = match val {
                        Cow::Borrowed(_) => val.as_message().map(|v| Box::new(v.clone())),
                        Cow::Owned(_) => None,
                    };
                    for f in &fns {
                        let msg = msg.clone();
                        if !f(msg, rules, m)? {
                            break;
                        }
                    }
                    Ok(true)
                },
            )
        }
    }
}

trait AsOptionStringExt {
    fn as_string(&self) -> Option<String>;
}

impl AsOptionStringExt for Value {
    fn as_string(&self) -> Option<String> {
        self.as_str().map(|s| s.to_string())
    }
}
