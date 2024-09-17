use crate::any::{make_validate_any};
use crate::duration::{make_validate_duration};
use crate::registry::{FieldValidationFn, ValidationFn, REGISTRY};
use crate::timestamp::{make_validate_timestamp};
use crate::validate_proto::{FieldRules, MessageRules};
use crate::ValidatorExt;
use anyhow::{format_err};
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

pub(crate) fn make_validate_message(m: &mut HashMap<String, ValidationFn>, field: &FieldDescriptor, rules: &FieldRules) -> Vec<FieldValidationFn<Box<DynamicMessage>>> {
    let mut fns = Vec::new();
    let desc = match field.kind() {
        Kind::Message(desc) => desc,
        _ => return fns,
    };
    match desc.full_name() {
        "google.protobuf.Timestamp" => return make_validate_timestamp(field, rules),
        "google.protobuf.Duration" => return make_validate_duration(field, rules),
        "google.protobuf.Any" => return make_validate_any(field, rules),
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
    let rules = rules.message.unwrap_or_else(MessageRules::default);
    if rules.required() {
        let name = field.full_name().to_string();
        fns.push(Arc::new(move |val, _| {
            if val.is_some() {
                return Err(format_err!("{}: is required", name));
            }
            Ok(true)
        }))
    }
    if rules.skip() {
        fns.push(Arc::new(move |_, _| {
            Ok(false)
        }));
        return fns;
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
