use crate::registry::FieldValidationFn;
use prost_reflect::FieldDescriptor;
use prost_validate::format_err;
use prost_validate::Error;
use prost_validate_types::field_rules::Type;
use prost_validate_types::FieldRules;
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
    ($name:ident,$enum_value:ident,$typ:ident,$default:expr,$module:ident) => {
        pub(crate) fn $name(
            field: &FieldDescriptor,
            rules: &FieldRules,
        ) -> Vec<FieldValidationFn<$typ>> {
            use prost_validate::errors::$module;

            let mut fns: Vec<FieldValidationFn<$typ>> = Vec::new();
            if !matches!(rules.r#type, Some(Type::$enum_value(_))) {
                return fns;
            }

            let rules = match &rules.r#type {
                Some(Type::$enum_value(rules)) => rules,
                _ => return fns,
            };
            if rules.ignore_empty() {
                fns.push(Arc::new(|val, _| {
                    Ok(val.is_some() && val.unwrap() != $default)
                }))
            }
            let name = Arc::new(field.full_name().to_string());
            if let Some(_) = rules.r#const {
                let name = name.clone();
                fns.push(Arc::new(move |val, rules| {
                    let val = val.unwrap_or($default);
                    let v = number_rules!(rules, $enum_value).r#const.unwrap();
                    if val != v {
                        return Err(Error::new(name.to_string(), $module::Error::Const(v)));
                    }
                    Ok(true)
                }));
            }
            // reference implementation: https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/templates/goshared/ltgt.go
            if let Some(lt) = rules.lt {
                let name = name.clone();
                if let Some(gt) = rules.gt {
                    if lt > gt {
                        fns.push(Arc::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val <= gt || val >= lt {
                                return Err(Error::new(
                                    name.to_string(),
                                    $module::Error::in_range(false, gt, lt, false),
                                ));
                            }
                            Ok(true)
                        }));
                    } else {
                        fns.push(Arc::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val >= lt && val <= gt {
                                return Err(Error::new(
                                    name.to_string(),
                                    $module::Error::not_in_range(true, lt, gt, true),
                                ));
                            }
                            Ok(true)
                        }));
                    }
                } else if let Some(gte) = rules.gte {
                    if lt > gte {
                        fns.push(Arc::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val < gte || val >= lt {
                                return Err(Error::new(
                                    name.to_string(),
                                    $module::Error::in_range(true, gte, lt, false),
                                ));
                            }
                            Ok(true)
                        }));
                    } else {
                        fns.push(Arc::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val >= lt && val < gte {
                                return Err(Error::new(
                                    name.to_string(),
                                    $module::Error::not_in_range(true, lt, gte, false),
                                ));
                            }
                            Ok(true)
                        }));
                    }
                } else {
                    fns.push(Arc::new(move |val, _| {
                        let val = val.unwrap_or($default);
                        if val >= lt {
                            return Err(Error::new(name.to_string(), $module::Error::Lt(lt)));
                        }
                        Ok(true)
                    }));
                }
            } else if let Some(lte) = rules.lte {
                let name = name.clone();
                if let Some(gt) = rules.gt {
                    if lte > gt {
                        fns.push(Arc::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val <= gt || val > lte {
                                return Err(Error::new(
                                    name.to_string(),
                                    $module::Error::in_range(false, gt, lte, true),
                                ));
                            }
                            Ok(true)
                        }));
                    } else {
                        fns.push(Arc::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val > lte && val <= gt {
                                return Err(Error::new(
                                    name.to_string(),
                                    $module::Error::not_in_range(false, lte, gt, true),
                                ));
                            }
                            Ok(true)
                        }));
                    }
                } else if let Some(gte) = rules.gte {
                    if lte > gte {
                        fns.push(Arc::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val < gte || val > lte {
                                return Err(Error::new(
                                    name.to_string(),
                                    $module::Error::in_range(true, gte, lte, true),
                                ));
                            }
                            Ok(true)
                        }));
                    } else {
                        fns.push(Arc::new(move |val, _| {
                            let val = val.unwrap_or($default);
                            if val > lte && val < gte {
                                return Err(Error::new(
                                    name.to_string(),
                                    $module::Error::not_in_range(false, lte, gte, false),
                                ));
                            }
                            Ok(true)
                        }));
                    }
                } else {
                    fns.push(Arc::new(move |val, _| {
                        let val = val.unwrap_or($default);
                        if val > lte {
                            return Err(Error::new(name.to_string(), $module::Error::Lte(lte)));
                        }
                        Ok(true)
                    }));
                }
            } else if let Some(gt) = rules.gt {
                let name = name.clone();
                fns.push(Arc::new(move |val, _| {
                    let val = val.unwrap_or($default);
                    if val <= gt {
                        return Err(Error::new(name.to_string(), $module::Error::Gt(gt)));
                    }
                    Ok(true)
                }));
            } else if let Some(gte) = rules.gte {
                let name = name.clone();
                fns.push(Arc::new(move |val, _| {
                    let val = val.unwrap_or($default);
                    if val < gte {
                        return Err(Error::new(name.to_string(), $module::Error::Gte(gte)));
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
                        return Err(Error::new(
                            name.to_string(),
                            $module::Error::In(rules.r#in.clone()),
                        ));
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
                        return Err(Error::new(
                            name.to_string(),
                            $module::Error::NotIn(rules.not_in.clone()),
                        ));
                    }
                    Ok(true)
                }));
            }
            fns
        }
    };
}

make_validate_number!(make_validate_u64, Uint64, u64, 0, uint64);
make_validate_number!(make_validate_u32, Uint32, u32, 0, uint32);
make_validate_number!(make_validate_i64, Int64, i64, 0, int64);
make_validate_number!(make_validate_i32, Int32, i32, 0, int32);
make_validate_number!(make_validate_double, Double, f64, 0.0, double);
make_validate_number!(make_validate_float, Float, f32, 0.0, float);
make_validate_number!(make_validate_sint32, Sint32, i32, 0, sint32);
make_validate_number!(make_validate_sint64, Sint64, i64, 0, sint64);
make_validate_number!(make_validate_fixed32, Fixed32, u32, 0, fixed32);
make_validate_number!(make_validate_fixed64, Fixed64, u64, 0, fixed64);
make_validate_number!(make_validate_sfixed32, Sfixed32, i32, 0, sfixed32);
make_validate_number!(make_validate_sfixed64, Sfixed64, i64, 0, sfixed64);
