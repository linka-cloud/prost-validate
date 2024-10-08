use crate::field::make_validate_field;
use crate::list::make_validate_list;
use crate::map::make_validate_map;
use crate::utils::{get_field_rules, is_set};
use no_deadlocks::RwLock;
use once_cell::sync::Lazy;
use prost_reflect::{DynamicMessage, MessageDescriptor, OneofDescriptor, ReflectMessage};
use prost_validate_types::FieldRules;
use prost_validate_types::{MessageRulesExt, OneofRulesExt};
use prost_validate::{Result, format_err};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

pub(crate) struct Args<'a> {
    pub(crate) m: &'a HashMap<String, ValidationFn>,
    pub(crate) msg: &'a DynamicMessage,
}

pub(crate) type ValidationFn = Arc<dyn Fn(&Args) -> Result<()> + Send + Sync>;
pub(crate) type FieldValidationFn<T> =
Arc<dyn Fn(Option<T>, &FieldRules) -> Result<bool> + Send + Sync>;
pub(crate) type NestedValidationFn<T> = Arc<
    dyn Fn(Option<T>, &FieldRules, &HashMap<String, ValidationFn>) -> Result<bool> + Send + Sync,
>;

pub(crate) static REGISTRY: Lazy<Registry> = Lazy::new(Registry::default);

#[derive(Default, Clone)]
pub(crate) struct Registry {
    m: Arc<RwLock<HashMap<String, ValidationFn>>>,
}

impl Registry {
    #[allow(clippy::unwrap_used)]
    pub(crate) fn register(
        &self,
        m: &mut HashMap<String, ValidationFn>,
        desc: &MessageDescriptor,
    ) -> Result<()> {
        if m.get(desc.full_name()).is_some() {
            return Ok(());
        }
        // insert a dummy validation to prevent recursion
        let _ = m.insert(desc.full_name().to_string(), Arc::new(|_| Ok(())));

        if desc.validation_disabled() || desc.validation_ignored() {
            let _ = m.insert(desc.full_name().to_string(), Arc::new(|_| Ok(())));
            return Ok(());
        }
        let mut fns: Vec<ValidationFn> = Vec::new();
        let mut oneofs: HashMap<String, Rc<OneofDescriptor>> = HashMap::new();
        for field in desc.fields() {
            let rules = match get_field_rules(&field).map_err(|e| format_err!(field.full_name(), "{}", e))? {
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
                    let rules = match get_field_rules(&field).map_err(|e| format_err!(field.full_name(), "{}", e))? {
                        Some(r) => r,
                        None => continue,
                    };
                    let validate_field = make_validate_field(m, &field, &rules);
                    fns.push(Arc::new(move |Args { msg, m }| {
                        let val = msg.get_field(&field);
                        if !is_set(&val) {
                            return Ok(());
                        }
                        validate_field(val, &rules, m)?;
                        Ok(())
                    }));
                }
                let field = field.clone();
                if desc.required() {
                    fns.push(Arc::new(move |Args { msg, .. }| {
                        let mut has = false;
                        for field in field.containing_oneof().unwrap().fields() {
                            let ok = is_set(&msg.get_field(&field));
                            if ok {
                                if has {
                                    return Err(format_err!(
                                        field.containing_oneof().unwrap().name(),
                                        "oneof contains multiple values",
                                    ));
                                }
                                has = true;
                            }
                        }
                        if !has {
                            return Err(format_err!(
                                field.containing_oneof().unwrap().name(),
                                "oneof does not contains any value",
                            ));
                        }
                        Ok(())
                    }))
                }
                continue;
            }
            if field.is_list() {
                let validate_list = make_validate_list(m, &field, &rules);
                fns.push(Arc::new(move |Args { msg, m }| {
                    let v = msg.get_field(&field).as_list().map(|v| v.to_vec());
                    for f in &validate_list {
                        let v = v.clone();
                        if !f(v, &rules, m)? {
                            break;
                        }
                    }
                    Ok(())
                }));
                continue;
            }
            if field.is_map() {
                let validate_map = make_validate_map(m, &field, &rules);
                fns.push(Arc::new(move |Args { msg, m }| {
                    let v = msg.get_field(&field).as_map().map(|v| v.to_owned());
                    for f in &validate_map {
                        let v = v.clone();
                        if !f(v, &rules, m)? {
                            break;
                        }
                    }
                    Ok(())
                }));
                continue;
            }
            let validate_field = make_validate_field(m, &field, &rules);
            let field = field.clone();
            fns.push(Arc::new(move |Args { msg, m }| {
                let v = msg.get_field(&field);
                validate_field(v, &rules, m)?;
                Ok(())
            }));
        }
        let _ = m.insert(
            desc.full_name().to_string(),
            Arc::new(move |v| {
                for f in &fns {
                    f(v)?;
                }
                Ok(())
            }),
        );
        Ok(())
    }

    pub(crate) fn validate(&self, msg: &DynamicMessage) -> Result<()> {
        {
            #[allow(clippy::unwrap_used)]
            let m = self.m.read().unwrap();
            if let Some(f) = m.get(msg.descriptor().full_name()) {
                f(&Args { msg, m: &m })?;
                return Ok(());
            }
        }
        {
            #[allow(clippy::unwrap_used)]
            let mut m = self.m.write().unwrap();
            let desc = msg.descriptor();
            self.register(&mut m, &desc)?;
        }
        self.validate(msg)
    }

    pub(crate) fn do_validate(
        &self,
        msg: &DynamicMessage,
        m: &HashMap<String, ValidationFn>,
    ) -> Result<()> {
        if let Some(f) = m.get(msg.descriptor().full_name()) {
            f(&Args { msg, m })?;
            Ok(())
        } else {
            Err(format_err!(
                msg.descriptor().full_name(),
                "no validator",
            ))
        }
    }
}
