use crate::any::make_validate_any;
use crate::duration::make_validate_duration;
use crate::registry::{Args, NestedValidationFn, ValidationFn, REGISTRY};
use crate::timestamp::make_validate_timestamp;
use crate::validate_proto::{FieldRules, MessageRules};
use anyhow::format_err;
use prost_reflect::{DynamicMessage, FieldDescriptor, Kind};
use std::collections::HashMap;
use std::sync::Arc;

macro_rules! append {
    ($fns:ident, $other:expr) => {
        {
            $fns.extend($other);
            $fns
        }
    };
}

pub(crate) fn make_validate_message(m: &mut HashMap<String, ValidationFn>, field: &FieldDescriptor, field_rules: &FieldRules) -> Vec<NestedValidationFn<Box<DynamicMessage>>> {
    let mut fns = Vec::new();
    let desc = match field.kind() {
        Kind::Message(desc) => desc,
        _ => return fns,
    };
    let rules = field_rules.message.unwrap_or_else(MessageRules::default);
    // there is no way currently to check for "synthetic" oneof
    let optional = field.containing_oneof()
        .map(|d| d.fields().len() == 1 && d.fields().find(|f| f == field).is_some())
        .unwrap_or(false);
    if rules.required() && !optional {
        let name = field.full_name().to_string();
        fns.push(Arc::new(move |val, _, _| {
            if val.is_none() {
                return Err(format_err!("{}: is required", name));
            }
            Ok(true)
        }))
    } else {
        fns.push(Arc::new(move |val, rules, _| {
            Ok(val.is_some() || rules.r#type.is_some())
        }))
    }
    if rules.skip() {
        fns.push(Arc::new(move |_, _, _| {
            Ok(false)
        }));
        return fns;
    }
    match desc.full_name() {
        "google.protobuf.Timestamp" => return append!(fns, make_validate_timestamp(field, field_rules)),
        "google.protobuf.Duration" => return append!(fns, make_validate_duration(field, field_rules)),
        "google.protobuf.Any" => return append!(fns, make_validate_any(field, field_rules)),
        // TODO(adphi): well-known types
        "google.protobuf.StringValue" => {
            // let wkt_field = desc.get_field(1).unwrap();
            // let wkt_rules = match get_field_rules(&wkt_field) {
            //     Ok(Some(r)) => r,
            //     _ => return fns,
            // };
            // let wkt_fns = make_validate_string(&field, &wkt_rules);
            // let field = field.clone();
            // return append!(fns, vec![Arc::new(move | val, rules | {
            //     let val: Box<DynamicMessage> = match val {
            //         Some(v) => v,
            //         None => return Ok(true),
            //     };
            //     let binding = val.get_field(&field);
            //     let val = match binding.as_message() {
            //         Some(m) => m,
            //         None => return Ok(true),
            //     };
            //     let val = val.get_field(&wkt_field).as_str().map(|v| v.to_string());
            //     for f in &wkt_fns {
            //         let val = val.clone();
            //         if !f(val, rules)? {
            //             return Ok(false);
            //         }
            //     }
            //     Ok(true)
            // })])
            {}
        }
        "google.protobuf.BoolValue" => {}
        "google.protobuf.BytesValue" => {}
        "google.protobuf.FloatValue" => {}
        "google.protobuf.DoubleValue" => {}
        "google.protobuf.Int32Value" => {}
        "google.protobuf.Int64Value" => {}
        "google.protobuf.UInt32Value" => {}
        "google.protobuf.UInt64Value" => {}
        _ => {}
    }
    if REGISTRY.register(m, &desc).is_err() {
        return fns;
    }
    fns.push(Arc::new(move |val, _, m| {
        let validate = m.get(&desc.full_name().to_string()).ok_or(format_err!("no validator for {}", desc.full_name()))?;
        match val.map(|v| validate(&Args { msg: &v, m })) {
            Some(Err(err)) => Err(err),
            Some(Ok(())) => Ok(true),
            None => Ok(true),
        }
    }));
    fns
}
