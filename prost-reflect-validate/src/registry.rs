use crate::field::make_validate_field;
use crate::list::make_validate_list;
use crate::map::make_validate_map;
use crate::validate::{IsTrue, VALIDATION_DISABLED, VALIDATION_FIELD_RULES, VALIDATION_IGNORED, VALIDATION_ONE_OF_RULES};
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::FieldRules;
use anyhow::{format_err, Result};
use once_cell::sync::Lazy;
use prost_reflect::{DynamicMessage, MessageDescriptor, ReflectMessage};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub(crate) type ValidationFn = Arc<dyn Fn(&DynamicMessage) -> Result<bool> + Send + Sync>;
pub(crate) type FieldValidationFn<T> = Arc<dyn Fn(Option<T>, &FieldRules) -> Result<bool> + Send + Sync>;

pub(crate) static REGISTRY: Lazy<Registry> = Lazy::new(|| Registry::default());

#[derive(Default, Clone)]
pub(crate) struct Registry {
    m: Arc<RwLock<HashMap<String, ValidationFn>>>,
}

impl Registry {
    pub(crate) fn register(&self, m: &mut HashMap<String, ValidationFn>, desc: &MessageDescriptor) -> Result<()> {
        if m.get(desc.full_name()).is_some() {
            println!("validation already exists for {}", desc.full_name());
            return Ok(());
        }
        // insert a dummy validation to prevent recursion
        let _ = m.insert(desc.full_name().to_string(), Arc::new(|_| Ok(true)));

        let desc = desc.clone();
        let opts = desc.options();
        if opts.get_extension(&VALIDATION_DISABLED).is_true() || opts.get_extension(&VALIDATION_IGNORED).is_true() {
            let _ = m.insert(desc.full_name().to_string(), Arc::new(|_| Ok(true)));
            return Ok(());
        }
        let mut fns: Vec<ValidationFn> = Vec::new();
        for field in desc.fields() {
            let opts = field.options();
            if field.containing_oneof().is_some_and(|v| v.options().get_extension(&VALIDATION_ONE_OF_RULES).is_true()) {
                let field = field.clone();
                fns.push(Arc::new(move |msg| {
                    let mut has = false;
                    for field in field.containing_oneof().unwrap().fields() {
                        let desc = msg.descriptor().get_field(field.number()).ok_or(format_err!("oneof field {} not found", field.name()))?;
                        let is_default = msg.get_field(&field).is_default(&desc.kind());
                        if has && !is_default {
                            return Err(format_err!("oneof {} contains multiple values", field.containing_oneof().unwrap().name()));
                        }
                        has = !is_default;
                    }
                    if !has {
                        return Err(format_err!("oneof {} does not contains any value", field.containing_oneof().unwrap().name()));
                    }
                    Ok(true)
                }))
            }
            let rules = opts.get_extension(&VALIDATION_FIELD_RULES);
            let rules = match rules.as_message() {
                Some(r) => r,
                None => continue,
            };
            let rules = Arc::new(rules.transcode_to::<FieldRules>()?);
            if rules.message.is_none() && rules.r#type.is_none() {
                continue;
            }
            // println!("{}: {:#?})", field.full_name(), rules);
            if field.is_list() {
                if let Some(Type::Repeated(r)) = &rules.r#type {
                    let validate_list = make_validate_list(m, field.clone(), r.clone());
                    fns.push(Arc::new(move |v| {
                        let v = v.get_field(&field).as_list().map(|v| Box::new(v.to_owned()));
                        for f in &validate_list {
                            let v = v.clone();
                            if !f(v, &rules)? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }));
                }
                continue;
            }
            if field.is_map() {
                if let Some(Type::Map(r)) = &rules.r#type {
                    let validate_map = make_validate_map(m, field.clone(), r.clone());
                    fns.push(Arc::new(move |v| {
                        let v = v.get_field(&field).as_map().map(|v| Box::new(v.to_owned()));
                        for f in &validate_map {
                            let v = v.clone();
                            if !f(v, &rules)? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }))
                }
                continue;
            }
            let validate_field = make_validate_field(m, &field, &rules);
            let field = field.clone();
            fns.push(Arc::new(move |v| {
                let v = v.get_field(&field);
                return validate_field(v, &rules);
            }));
        }
        let _ = m.insert(desc.full_name().to_string(), Arc::new(move |v| {
            for f in &fns {
                if !f(v)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }));
        Ok(())
    }

    pub(crate) fn validate(&self, msg: &DynamicMessage) -> Result<()> {
        if let Some(f) = self.m.read().unwrap().get(msg.descriptor().full_name()) {
            let _ = f(msg)?;
            return Ok(());
        }
        {
            let mut m = self.m.write().unwrap();
            let desc = msg.descriptor();
            self.register(&mut m, &desc)?;
        }
        self.validate(msg)
    }
}
