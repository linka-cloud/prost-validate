use crate::field::make_validate_field;
use crate::list::make_validate_list;
use crate::map::make_validate_map;
use crate::validate::{IsTrue, VALIDATION_DISABLED, VALIDATION_FIELD_RULES, VALIDATION_IGNORED, VALIDATION_ONE_OF_RULES};
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::FieldRules;
use anyhow::{format_err, Result};
use once_cell::sync::Lazy;
use prost_reflect::{DynamicMessage, FieldDescriptor, MessageDescriptor, OneofDescriptor, ReflectMessage};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc};
use no_deadlocks::RwLock;
use crate::utils::is_set;

pub(crate) type ValidationFn = Arc<dyn Fn(&DynamicMessage) -> Result<()> + Send + Sync>;
pub(crate) type FieldValidationFn<T> = Arc<dyn Fn(Option<T>, &FieldRules) -> Result<bool> + Send + Sync>;

pub(crate) static REGISTRY: Lazy<Registry> = Lazy::new(|| Registry::default());

#[derive(Default, Clone)]
pub(crate) struct Registry {
    m: Arc<RwLock<HashMap<String, ValidationFn>>>,
}

impl Registry {
    pub(crate) fn register(&self, m: &mut HashMap<String, ValidationFn>, desc: &MessageDescriptor) -> Result<()> {
        if m.get(desc.full_name()).is_some() {
            return Ok(());
        }
        // insert a dummy validation to prevent recursion
        let _ = m.insert(desc.full_name().to_string(), Arc::new(|_| Ok(())));

        let desc = desc.clone();
        let opts = desc.options();
        if opts.get_extension(&VALIDATION_DISABLED).is_true() || opts.get_extension(&VALIDATION_IGNORED).is_true() {
            let _ = m.insert(desc.full_name().to_string(), Arc::new(|_| Ok(())));
            return Ok(());
        }
        let mut fns: Vec<ValidationFn> = Vec::new();
        let mut oneofs: HashMap<String, Rc<OneofDescriptor>> = HashMap::new();
        for field in desc.fields() {
            let rules = match field_rules(&field)? {
                Some(r) => r,
                None => continue,
            };
            if oneofs.contains_key(field.full_name()) {
                continue;
            }
            if let Some(ref desc) = field.containing_oneof() {
                let desc = Rc::new(desc.clone());
                for field in desc.fields() {
                    let field = field.clone();
                    oneofs.insert(field.full_name().to_string(), desc.clone());
                    let rules = match field_rules(&field)? {
                        Some(r) => r,
                        None => continue,
                    };
                    let validate_field = make_validate_field(m, &field, &rules);
                    fns.push(Arc::new(move |msg| {
                        let val = msg.get_field(&field);
                        if !is_set(&val) {
                            return Ok(());
                        }
                        validate_field(val, &rules)?;
                        Ok(())
                    }));
                }
                let field = field.clone();
                if desc.options().get_extension(&VALIDATION_ONE_OF_RULES).is_true() {
                    fns.push(Arc::new(move |msg| {
                        let mut has = false;
                        for field in field.containing_oneof().unwrap().fields() {
                            let ok = is_set(&msg.get_field(&field));
                            if ok {
                                if has {
                                    return Err(format_err!("oneof {} contains multiple values", field.containing_oneof().unwrap().name()));
                                }
                                has = true;
                            }
                        }
                        if !has {
                            return Err(format_err!("oneof {} does not contains any value", field.containing_oneof().unwrap().name()));
                        }
                        Ok(())
                    }))
                }
                continue;
            }
            if field.is_list() {
                if let Some(Type::Repeated(r)) = &rules.r#type {
                    let validate_list = make_validate_list(m, field.clone(), r.clone());
                    fns.push(Arc::new(move |v| {
                        let v = v.get_field(&field).as_list().map(|v| Box::new(v.to_owned()));
                        for f in &validate_list {
                            let v = v.clone();
                            if !f(v, &rules)? {
                                break;
                            }
                        }
                        Ok(())
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
                                break;
                            }
                        }
                        Ok(())
                    }))
                }
                continue;
            }
            let validate_field = make_validate_field(m, &field, &rules);
            let field = field.clone();
            fns.push(Arc::new(move |v| {
                let v = v.get_field(&field);
                validate_field(v, &rules)?;
                Ok(())
            }));
        }
        let _ = m.insert(desc.full_name().to_string(), Arc::new(move |v| {
            for f in &fns {
                f(v)?;
            }
            Ok(())
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

fn field_rules(field: &FieldDescriptor) -> Result<Option<Arc<FieldRules>>> {
    let opts = field.options();
    let rules = opts.get_extension(&VALIDATION_FIELD_RULES);
    let rules = match rules.as_message() {
        Some(r) => r,
        None => return Ok(None),
    };
    Ok(Some(Arc::new(rules.transcode_to::<FieldRules>()?)))
}
