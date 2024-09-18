use crate::any::make_validate_any;
use crate::duration::make_validate_duration;
use crate::registry::{FieldValidationFn, ValidationFn, REGISTRY};
use crate::timestamp::make_validate_timestamp;
use crate::validate_proto::{FieldRules, MessageRules};
use crate::ValidatorExt;
use anyhow::format_err;
use prost_reflect::{DynamicMessage, FieldDescriptor, Kind};
use std::collections::HashMap;
use std::sync::Arc;
// macro_rules! make_validate_wrapper {
//     ($fns:expr,$typ:ident,$conv:ident) => {
//         {
//             let fns = $fns;
//             Arc::new(move |val: Cow<Value>, rules: &FieldRules| -> Result<bool> {
//                 for f in &fns {
//                     if !f(val.$conv(), &rules)? {
//                         return Ok(false);
//                     }
//                 }
//                 Ok(true)
//             })
//         }
//     }
// }

macro_rules! append {
    ($fns:ident, $other:expr) => {
        {
            $fns.extend($other);
            $fns
        }
    };
}

pub(crate) fn make_validate_message(m: &mut HashMap<String, ValidationFn>, field: &FieldDescriptor, field_rules: &FieldRules) -> Vec<FieldValidationFn<Box<DynamicMessage>>> {
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
        fns.push(Arc::new(move |val, _| {
            if val.is_none() {
                return Err(format_err!("{}: is required", name));
            }
            Ok(true)
        }))
    } else {
        fns.push(Arc::new(move |val, rules| {
            Ok(val.is_some() || rules.r#type.is_some())
        }))
    }
    if rules.skip() {
        fns.push(Arc::new(move |_, _| {
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
            // let fns = make_validate_string(field, rules);
            // return vec![Arc::new(move | val, rules | {
            //     for f in fns {
            //         if !f(val, rules)? {
            //             return Ok(false);
            //         }
            //     }
            //     Ok(true)
            // })]
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
    fns.push(Arc::new(move |val, _| {
        match val.map(|v| v.validate()) {
            Some(Err(err)) => Err(err),
            Some(Ok(())) => Ok(true),
            None => Ok(true),
        }
    }));
    fns
}
