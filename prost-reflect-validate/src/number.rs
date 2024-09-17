use crate::registry::FieldValidationFn;
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::FieldRules;
use anyhow::{format_err, Result};
use prost_reflect::FieldDescriptor;
use std::sync::Arc;

macro_rules! validate_number {
    ($name:ident,$enum_value:ident,$typ:ident,$default:expr) => {
        pub(crate) fn $name(val: Option<$typ>, field: &FieldDescriptor, rules: &FieldRules) -> Result<()> {
            if !matches!(rules.r#type, Some(Type::$enum_value(_))) {
                return Ok(());
            }

            let val = val.unwrap_or($default);
            let rules = match &rules.r#type {
                Some(Type::$enum_value(rules)) => rules,
                _ => return Ok(()),
            };
            if rules.ignore_empty() && val == $default {
                return Ok(());
            }
            if let Some(v) = rules.r#const {
                if val != v {
                    return Err(format_err!("{}: must be {}", field.full_name(), v));
                }
            }
            if let Some(v) = rules.lt {
                if val >= v {
                    return Err(format_err!("{}: must be lt {}", field.full_name(), v));
                }
            }
            if let Some(v) = rules.lte {
                if val > v {
                    return Err(format_err!("{}: must be lte {}", field.full_name(), v));
                }
            }
            if let Some(v) = rules.gt {
                if val <= v {
                    return Err(format_err!("{}: must be gt {}", field.full_name(), v));
                }
            }
            if let Some(v) = rules.gte {
                if val < v {
                    return Err(format_err!("{}: must be gte {}", field.full_name(), v));
                }
            }
            if !rules.r#in.is_empty() && !rules.r#in.contains(&val) {
                return Err(format_err!("{}: must be in {:?}", field.full_name(), rules.r#in));
            }
            if !rules.not_in.is_empty() && rules.not_in.contains(&val) {
                return Err(format_err!("{}: must not be in {:?}", field.full_name(), rules.r#in));
            }
            Ok(())
        }
    };
}

validate_number!(validate_u64, Uint64, u64, 0);
validate_number!(validate_u32, Uint32, u32, 0);
validate_number!(validate_i64, Int64, i64, 0);
validate_number!(validate_i32, Int32, i32, 0);
validate_number!(validate_double, Double, f64, 0.0);
validate_number!(validate_float, Float, f32, 0.0);

macro_rules! number_rules {
    ($rules:ident,$enum_value:ident) => {
        match &$rules.r#type {
            Some(Type::$enum_value(rules)) => rules,
            _ => return Err(format_err!("unexpected $enum_value rules")),
        }
    };
}

macro_rules! make_validate_number {
    ($name:ident,$enum_value:ident,$typ:ident,$default:expr) => {
        pub(crate) fn $name(field: &FieldDescriptor, rules: &FieldRules) -> Vec<FieldValidationFn<$typ>> {
            let mut fns: Vec<FieldValidationFn<$typ>> = Vec::new();
            if !matches!(rules.r#type, Some(Type::$enum_value(_))) {
                return fns;
            }

            let rules = match &rules.r#type {
                Some(Type::$enum_value(rules)) => rules,
                _ => return fns,
            };
            if rules.ignore_empty() {
                fns.push(Arc::new(|val, _| Ok(val.is_some() && val.unwrap() != $default)))
            }
            let name = Arc::new(field.full_name().to_string());
            if let Some(_) = rules.r#const {
                let name = name.clone();
                fns.push(Arc::new(move |val, rules| {
                    let val = val.unwrap_or($default);
                    let v = number_rules!(rules, $enum_value).r#const.unwrap();
                    if val != v {
                        return Err(format_err!("{}: must be {}", name, v));
                    }
                    Ok(true)
                }));
            }
            if rules.lt.is_some() {
                let name = name.clone();
                fns.push(Arc::new(move |val, rules| {
                    let val = val.unwrap_or($default);
                    let v = number_rules!(rules, $enum_value).lt.unwrap();
                    if val >= v {
                        return Err(format_err!("{}: must be lt {}", name, v));
                    }
                    Ok(true)
                }));
            }
            if rules.lte.is_some() {
                let name = name.clone();
                fns.push(Arc::new(move |val, rules| {
                    let val = val.unwrap_or($default);
                    let v = number_rules!(rules, $enum_value).lte.unwrap();
                    if val > v {
                        return Err(format_err!("{}: must be lte {}", name, v));
                    }
                    Ok(true)
                }));
            }
            if rules.gt.is_some() {
                let name = name.clone();
                fns.push(Arc::new(move |val, rules| {
                    let val = val.unwrap_or($default);
                    let v = number_rules!(rules, $enum_value).gt.unwrap();
                    if val <= v {
                        return Err(format_err!("{}: must be gt {}", name, v));
                    }
                    Ok(true)
                }));
            }
            if rules.gte.is_some() {
                let name = name.clone();
                fns.push(Arc::new(move |val, rules| {
                    let val = val.unwrap_or($default);
                    let v = number_rules!(rules, $enum_value).gte.unwrap();
                    if val < v {
                        return Err(format_err!("{}: must be gte {}", name, v));
                    }
                    Ok(true)
                }));
            }
            if !rules.r#in.is_empty() {
                let name = name.clone();
                fns.push(Arc::new(move |val, rules| {
                    let val = val.unwrap_or($default);
                    let rules = number_rules!(rules, $enum_value);
                    if !rules.r#in.contains(&val) {
                        return Err(format_err!("{}: must be in {:?}", name, rules.r#in));
                    }
                    Ok(true)
                }));
            }
            if !rules.not_in.is_empty() {
                let name = name.clone();
                fns.push(Arc::new(move |val, rules| {
                    let val = val.unwrap_or($default);
                    let rules = number_rules!(rules, $enum_value);
                    if rules.not_in.contains(&val) {
                        return Err(format_err!("{}: must not be in {:?}", name, rules.r#in));
                    }
                    Ok(true)
                }));
            }
            fns
        }
    };
}

make_validate_number!(make_validate_u64, Uint64, u64, 0);
make_validate_number!(make_validate_u32, Uint32, u32, 0);
make_validate_number!(make_validate_i64, Int64, i64, 0);
make_validate_number!(make_validate_i32, Int32, i32, 0);
make_validate_number!(make_validate_double, Double, f64, 0.0);
make_validate_number!(make_validate_float, Float, f32, 0.0);
