use prost_reflect::{DynamicMessage, FieldDescriptor, Kind, ReflectMessage, Value};
use prost_validate_derive_core::sanitize_identifier;
use prost_validate_types::FieldRules;
use std::collections::HashMap;

pub(crate) trait IntoFieldAttribute {
    fn into_field_attribute(self) -> Option<String>;
}

impl IntoFieldAttribute for FieldRules {
    fn into_field_attribute(self) -> Option<String> {
        let msg = self.transcode_to_dynamic();
        if msg == FieldRules::default().transcode_to_dynamic() {
            return None;
        }
        Some(message_to_annotation(msg))
    }
}

fn message_to_annotation(msg: DynamicMessage) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut oneofs: HashMap<String, String> = HashMap::new();
    for (desc, val) in msg.fields() {
        let kind = desc.kind();
        if !val.is_valid_for_field(&desc) {
            continue;
        }
        let name = sanitize_identifier(desc.name());
        if desc.is_list() {
            let mut list: Vec<String> = Vec::new();
            for val in val.as_list().unwrap() {
                if matches!(desc.kind(), Kind::Message(_)) {
                    list.push(format!("({})", field_to_string(&desc, val)));
                } else {
                    list.push(field_to_string(&desc, val));
                }
            }
            let s = list.join(", ");
            parts.push(format!("{} = [{}]", name, s));
            continue;
        }
        if !val.is_valid(&kind) && !val.is_default(&kind) {
            continue;
        }
        if let Some(oneof) = desc.containing_oneof() {
            for field in oneof.fields() {
                oneofs.insert(field.full_name().to_string(), oneof.name().to_string());
            }
        }
        let s = field_to_string(&desc, val);
        if let Some(oneof) = oneofs.get(desc.full_name()) {
            let oneof = sanitize_identifier(oneof);
            if let Kind::Message(_) = kind {
                parts.push(format!("{}({}({}))", oneof, name, s));
            } else {
                parts.push(format!("{}({} = {})", oneof, name, s));
            }
        } else if let Kind::Message(_) = kind {
            parts.push(format!("{}({})", name, s));
        } else {
            parts.push(format!("{} = {}", name, s));
        }
    }
    parts.join(", ")
}

fn field_to_string(desc: &FieldDescriptor, val: &Value) -> String {
    match desc.kind() {
        Kind::Double => format!("{:?}", val.as_f64().unwrap()),
        Kind::Float => format!("{:?}", val.as_f32().unwrap()),
        Kind::Int32 => format!("{:?}", val.as_i32().unwrap()),
        Kind::Int64 => format!("{:?}", val.as_i64().unwrap()),
        Kind::Uint32 => format!("{:?}", val.as_u32().unwrap()),
        Kind::Uint64 => format!("{:?}", val.as_u64().unwrap()),
        Kind::Sint32 => format!("{:?}", val.as_i32().unwrap()),
        Kind::Sint64 => format!("{:?}", val.as_i64().unwrap()),
        Kind::Fixed32 => format!("{:?}", val.as_u32().unwrap()),
        Kind::Fixed64 => format!("{:?}", val.as_u64().unwrap()),
        Kind::Sfixed32 => format!("{:?}", val.as_i32().unwrap()),
        Kind::Sfixed64 => format!("{:?}", val.as_i64().unwrap()),
        Kind::Bool => format!("{:?}", val.as_bool().unwrap()),
        Kind::String => format!("{:?}", val.as_str().unwrap()),
        Kind::Bytes => format!("{:?}", val.as_bytes().unwrap()),
        Kind::Message(_) => message_to_annotation(val.as_message().unwrap().clone()).to_string(),
        Kind::Enum(_) => format!("{:?}", val.as_enum_number().unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost_validate_types::field_rules::Type;
    use prost_validate_types::AnyRules;

    #[test]
    fn test() {
        let rules = FieldRules {
            message: None,
            r#type: Some(Type::Any(AnyRules {
                r#in: vec!["google.protobuf.Any".to_string()],
                ..AnyRules::default()
            })),
        };
        println!("{}", rules.into_field_attribute().unwrap());
    }
}
