use crate::registry::FieldValidationFn;
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::FieldRules;
use anyhow::format_err;
use prost_reflect::FieldDescriptor;
use std::sync::Arc;

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
        pub(crate) fn $name(
            field: &FieldDescriptor,
            rules: &FieldRules,
        ) -> Vec<FieldValidationFn<$typ>> {
            let mut fns: Vec<FieldValidationFn<$typ>> = Vec::new();
            if !matches!(rules.r#type, Some(Type::$enum_value(_))) {
                return fns;
            }

            let rules = match &rules.r#type {
                Some(Type::$enum_value(rules)) => rules,
                _ => return fns,
            };
            if rules.ignore_empty() {
                fns.push(Box::new(|val, _| {
                    Ok(val.is_some() && val.unwrap() != $default)
                }))
            }
            let name = Arc::new(field.full_name().to_string());
            if let Some(_) = rules.r#const {
                let name = name.clone();
                fns.push(Box::new(move |val, rules| {
                    let val = val.unwrap_or($default);
                    let v = number_rules!(rules, $enum_value).r#const.unwrap();
                    if val != v {
                        return Err(format_err!("{}: must be {}", name, v));
                    }
                    Ok(true)
                }));
            }
            // reference implementation: https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/templates/goshared/ltgt.go
            if let Some(lt) = rules.lt {
                let name = name.clone();
                if let Some(gt) = rules.gt {
                    if lt > gt {
                        fns.push(Box::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val <= gt || val >= lt {
                                return Err(format_err!(
                                    "{}: must be inside range ({}, {})",
                                    name,
                                    gt,
                                    lt
                                ));
                            }
                            Ok(true)
                        }));
                    } else {
                        fns.push(Box::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val >= lt && val <= gt {
                                return Err(format_err!(
                                    "{}: must be outside range [{}, {}]",
                                    name,
                                    lt,
                                    gt
                                ));
                            }
                            Ok(true)
                        }));
                    }
                } else if let Some(gte) = rules.gte {
                    if lt > gte {
                        fns.push(Box::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val < gte || val >= lt {
                                return Err(format_err!(
                                    "{}: must be inside range [{}, {})",
                                    name,
                                    gte,
                                    lt
                                ));
                            }
                            Ok(true)
                        }));
                    } else {
                        fns.push(Box::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val >= lt && val < gte {
                                return Err(format_err!(
                                    "{}: must be outside range [{}, {})",
                                    name,
                                    gte,
                                    lt
                                ));
                            }
                            Ok(true)
                        }));
                    }
                } else {
                    fns.push(Box::new(move |val, _| {
                        let val = val.unwrap_or($default);
                        if val >= lt {
                            return Err(format_err!("{}: must be less than {}", name, lt));
                        }
                        Ok(true)
                    }));
                }
            } else if let Some(lte) = rules.lte {
                let name = name.clone();
                if let Some(gt) = rules.gt {
                    if lte > gt {
                        fns.push(Box::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val <= gt || val > lte {
                                return Err(format_err!(
                                    "{}: must be inside range ({}, {}]",
                                    name,
                                    gt,
                                    lte
                                ));
                            }
                            Ok(true)
                        }));
                    } else {
                        fns.push(Box::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val > lte && val <= gt {
                                return Err(format_err!(
                                    "{}: must be outside range ({}, {}]",
                                    name,
                                    lte,
                                    gt
                                ));
                            }
                            Ok(true)
                        }));
                    }
                } else if let Some(gte) = rules.gte {
                    if lte > gte {
                        fns.push(Box::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val < gte || val > lte {
                                return Err(format_err!(
                                    "{}: must be inside range [{}, {}]",
                                    name,
                                    gte,
                                    lte
                                ));
                            }
                            Ok(true)
                        }));
                    } else {
                        fns.push(Box::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val > lte && val < gte {
                                return Err(format_err!(
                                    "{}: must be outside range ({}, {})",
                                    name,
                                    lte,
                                    gte
                                ));
                            }
                            Ok(true)
                        }));
                    }
                } else {
                    fns.push(Box::new(move |val, _| {
                        let val = val.unwrap_or($default);
                        if val > lte {
                            return Err(format_err!("{}: must be less or equal to {}", name, lte));
                        }
                        Ok(true)
                    }));
                }
            } else if let Some(gt) = rules.gt {
                let name = name.clone();
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or($default);
                    if val <= gt {
                        return Err(format_err!("{}: must be greater than {}", name, gt));
                    }
                    Ok(true)
                }));
            } else if let Some(gte) = rules.gte {
                let name = name.clone();
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or($default);
                    if val < gte {
                        return Err(format_err!("{}: must be greater or equal to {}", name, gte));
                    }
                    Ok(true)
                }));
            }

            if !rules.r#in.is_empty() {
                let name = name.clone();
                fns.push(Box::new(move |val, rules| {
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
                fns.push(Box::new(move |val, rules| {
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
make_validate_number!(make_validate_sint32, Sint32, i32, 0);
make_validate_number!(make_validate_sint64, Sint64, i64, 0);
make_validate_number!(make_validate_fixed32, Fixed32, u32, 0);
make_validate_number!(make_validate_fixed64, Fixed64, u64, 0);
make_validate_number!(make_validate_sfixed32, Sfixed32, i32, 0);
make_validate_number!(make_validate_sfixed64, Sfixed64, i64, 0);
