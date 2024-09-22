use crate::any::make_validate_any;
use crate::bool::make_validate_bool;
use crate::bytes::make_validate_bytes;
use crate::duration::make_validate_duration;
use crate::number::{
    make_validate_double, make_validate_float, make_validate_i32, make_validate_i64,
    make_validate_u32, make_validate_u64,
};
use crate::registry::{Args, NestedValidationFn, ValidationFn, REGISTRY};
use crate::string::make_validate_string;
use crate::timestamp::make_validate_timestamp;
use crate::validate_proto::FieldRules;
use anyhow::format_err;
use prost_reflect::{DynamicMessage, FieldDescriptor, Kind};
use std::collections::HashMap;

macro_rules! append {
    ($fns:ident, $other:expr) => {{
        $fns.extend($other);
        $fns
    }};
}

#[allow(clippy::unwrap_used)]
pub(crate) fn make_validate_message(
    m: &mut HashMap<String, ValidationFn>,
    field: &FieldDescriptor,
    field_rules: &FieldRules,
) -> Vec<NestedValidationFn<Box<DynamicMessage>>> {
    let mut fns = Vec::new();
    let desc = match field.kind() {
        Kind::Message(desc) => desc,
        _ => return fns,
    };
    let rules = field_rules.message.unwrap_or_default();
    // there is no way currently to check for "synthetic" oneof
    let optional = field
        .containing_oneof()
        .map(|d| d.fields().len() == 1 && d.fields().any(|f| &f == field))
        .unwrap_or(false);
    if rules.required() && !optional {
        let name = field.full_name().to_string();
        fns.push(Box::new(move |val, _, _| {
            if val.is_none() {
                return Err(format_err!("{}: is required", name));
            }
            Ok(true)
        }))
    } else {
        fns.push(Box::new(move |val, rules, _| {
            Ok(val.is_some() || rules.r#type.is_some())
        }))
    }
    if rules.skip() {
        fns.push(Box::new(move |_, _, _| Ok(false)));
        return fns;
    }

    macro_rules! make_validate_wrapper {
        ($make:ident,$conv:ident) => {{
            let wkt_field = desc.get_field(1).unwrap();
            let wkt_fns = $make(field, field_rules);
            let f: NestedValidationFn<Box<DynamicMessage>> = Box::new(move |val, rules, _| {
                let val: Box<DynamicMessage> = match val {
                    Some(v) => v,
                    None => return Ok(true),
                };
                let val = val.get_field(&wkt_field).$conv();
                for f in &wkt_fns {
                    let val = val.clone();
                    if !f(val, rules)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            });
            append!(fns, vec![f])
        }};
    }
    match desc.full_name() {
        "google.protobuf.StringValue" => {
            let wkt_field = desc.get_field(1).unwrap();
            let wkt_fns = make_validate_string(field, field_rules);
            let f: NestedValidationFn<Box<DynamicMessage>> = Box::new(move |val, rules, _| {
                let val: Box<DynamicMessage> = match val {
                    Some(v) => v,
                    None => return Ok(true),
                };
                let val = val.get_field(&wkt_field).as_str().map(|v| v.to_string());
                for f in &wkt_fns {
                    let val = val.clone();
                    if !f(val, rules)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            });
            return append!(fns, vec![f]);
        }
        "google.protobuf.BytesValue" => {
            let wkt_field = desc.get_field(1).unwrap();
            let wkt_fns = make_validate_bytes(field, field_rules);
            let f: NestedValidationFn<Box<DynamicMessage>> = Box::new(move |val, rules, _| {
                let val: Box<DynamicMessage> = match val {
                    Some(v) => v,
                    None => return Ok(true),
                };
                let val = val
                    .get_field(&wkt_field)
                    .as_bytes()
                    .map(|v| Box::new(v.clone()));
                for f in &wkt_fns {
                    let val = val.clone();
                    if !f(val, rules)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            });
            return append!(fns, vec![f]);
        }
        "google.protobuf.BoolValue" => return make_validate_wrapper!(make_validate_bool, as_bool),
        "google.protobuf.UInt64Value" => return make_validate_wrapper!(make_validate_u64, as_u64),
        "google.protobuf.UInt32Value" => return make_validate_wrapper!(make_validate_u32, as_u32),
        "google.protobuf.Int64Value" => return make_validate_wrapper!(make_validate_i64, as_i64),
        "google.protobuf.Int32Value" => return make_validate_wrapper!(make_validate_i32, as_i32),
        "google.protobuf.DoubleValue" => {
            return make_validate_wrapper!(make_validate_double, as_f64)
        }
        "google.protobuf.FloatValue" => return make_validate_wrapper!(make_validate_float, as_f32),
        "google.protobuf.Timestamp" => {
            return append!(fns, make_validate_timestamp(field, field_rules))
        }
        "google.protobuf.Duration" => {
            return append!(fns, make_validate_duration(field, field_rules))
        }
        "google.protobuf.Any" => return append!(fns, make_validate_any(field, field_rules)),
        _ => {}
    }
    if REGISTRY.register(m, &desc).is_err() {
        return fns;
    }
    fns.push(Box::new(move |val, _, m| {
        let validate = m
            .get(&desc.full_name().to_string())
            .ok_or(format_err!("no validator for {}", desc.full_name()))?;
        match val.map(|v| validate(&Args { msg: &v, m })) {
            Some(Err(err)) => Err(err),
            Some(Ok(())) => Ok(true),
            None => Ok(true),
        }
    }));
    fns
}
