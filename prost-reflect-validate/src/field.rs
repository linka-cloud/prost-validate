use crate::bool::{make_validate_bool, validate_bool};
use crate::bytes::{make_validate_bytes, validate_bytes};
use crate::message::{make_validate_message, validate_message};
use crate::number::{make_validate_double, make_validate_float, make_validate_i32, make_validate_i64, make_validate_u32, make_validate_u64, validate_double, validate_float, validate_i32, validate_i64, validate_u32, validate_u64};
use crate::r#enum::{make_validate_enum, validate_enum};
use crate::registry::ValidationFn;
use crate::string::{make_validate_string, validate_string};
use crate::validate_proto::FieldRules;
use anyhow::Result;
use prost_reflect::{FieldDescriptor, Kind, Value};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

type ValueValidationFn = Arc<dyn Fn(Cow<Value>, &FieldRules) -> Result<bool> + Send + Sync>;

pub(crate) fn validate_field(val: Cow<Value>, field: &FieldDescriptor, rules: &FieldRules) -> anyhow::Result<()> {
    match field.kind() {
        Kind::Double => validate_double(val.as_f64(), &field, rules),
        Kind::Float => validate_float(val.as_f32(), &field, rules),
        Kind::Int32 => validate_i32(val.as_i32(), &field, rules),
        Kind::Int64 => validate_i64(val.as_i64(), &field, rules),
        Kind::Uint32 => validate_u32(val.as_u32(), &field, rules),
        Kind::Uint64 => validate_u64(val.as_u64(), &field, rules),
        Kind::Sint32 => validate_i32(val.as_i32(), &field, rules),
        Kind::Sint64 => validate_i64(val.as_i64(), &field, rules),
        Kind::Fixed32 => validate_u32(val.as_u32(), &field, rules),
        Kind::Fixed64 => validate_u64(val.as_u64(), &field, rules),
        Kind::Sfixed32 => validate_i32(val.as_i32(), &field, rules),
        Kind::Sfixed64 => validate_i64(val.as_i64(), &field, rules),
        Kind::Bool => validate_bool(val.as_bool(), &field, rules),
        Kind::String => validate_string(val.as_str(), &field, rules),
        Kind::Bytes => validate_bytes(val.as_bytes(), &field, rules),
        Kind::Message(_) => validate_message(val.as_message(), &field, rules),
        Kind::Enum(_) => validate_enum(val.as_enum_number(), &field, rules),
    }
}

macro_rules! as_validation_func {
    ($fns:expr,$typ:ident,$conv:ident) => {
        {
            let fns = $fns;
            Arc::new(move |val: Cow<Value>, rules: &FieldRules| -> Result<bool> {
                for f in &fns {
                    if !f(val.$conv(), &rules)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            })
        }
    }
}

pub(crate) fn make_validate_field(m: &mut HashMap<String, ValidationFn>, field: &FieldDescriptor, rules: &FieldRules) -> ValueValidationFn {
    match field.kind() {
        Kind::Double => as_validation_func!(make_validate_double(&field, &rules), Double, as_f64),
        Kind::Float => as_validation_func!(make_validate_float(&field, &rules), Float, as_f32),
        Kind::Int32 => as_validation_func!(make_validate_i32(&field, &rules), Int32, as_i32),
        Kind::Int64 => as_validation_func!(make_validate_i64(&field, &rules), Int64, as_i64),
        Kind::Uint32 => as_validation_func!(make_validate_u32(&field, &rules), Uint32, as_u32),
        Kind::Uint64 => as_validation_func!(make_validate_u64(&field, &rules), Uint64, as_u64),
        Kind::Sint32 => as_validation_func!(make_validate_i32(&field, &rules), Int32, as_i32),
        Kind::Sint64 => as_validation_func!(make_validate_i64(&field, &rules), Int64, as_i64),
        Kind::Fixed32 => as_validation_func!(make_validate_u32(&field, &rules), Uint32, as_u32),
        Kind::Fixed64 => as_validation_func!(make_validate_u64(&field, &rules), Uint64, as_u64),
        Kind::Sfixed32 => as_validation_func!(make_validate_i32(&field, &rules), Int32, as_i32),
        Kind::Sfixed64 => as_validation_func!(make_validate_i64(&field, &rules), Int64, as_i64),
        Kind::Bool => as_validation_func!(make_validate_bool(&field, &rules), Bool, as_bool),
        Kind::String => as_validation_func!(make_validate_string(&field, &rules), String, as_string),
        Kind::Enum(_) => as_validation_func!(make_validate_enum(&field, &rules), i32, as_enum_number),
        Kind::Bytes => {
            let fns = make_validate_bytes(&field, &rules);
            Arc::new(move |val: Cow<Value>, rules: &FieldRules| -> anyhow::Result<bool> {
                let bytes = val.as_bytes().map(|v| Arc::new(v.clone()));
                for f in &fns {
                    let bytes = bytes.clone();
                    if !f(bytes, &rules)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            })
        }
        Kind::Message(_) => {
            let fns = make_validate_message(m, &field, &rules);
            Arc::new(move |val: Cow<Value>, rules: &FieldRules| -> anyhow::Result<bool> {
                let msg = val.as_message().map(|v| Box::new(v.clone()));
                for f in &fns {
                    let msg = msg.clone();
                    if !f(msg, &rules)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            })
        }
    }
}

trait AsOptionStringExt {
    fn as_string(&self) -> Option<String>;
}

impl AsOptionStringExt for Value {
    fn as_string(&self) -> Option<String> {
        match self.as_str() {
            Some(s) => Some(s.to_string()),
            None => None,
        }
    }
}
