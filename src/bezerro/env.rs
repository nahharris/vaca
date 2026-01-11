use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::bezerro::error::EvalError;
use crate::bezerro::value::Value;

#[derive(Debug, Clone)]
pub struct Env {
    bindings: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Env>>) -> Self {
        Env {
            bindings: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.bindings.get(name) {
            return Some(v.clone());
        }
        self.parent.as_ref().and_then(|p| p.borrow().get(name))
    }

    pub fn set(&mut self, name: &str, value: Value) -> Result<(), EvalError> {
        if self.bindings.contains_key(name) {
            self.bindings.insert(name.to_string(), value);
            return Ok(());
        }
        if let Some(parent) = self.parent.as_ref() {
            return parent.borrow_mut().set(name, value);
        }
        Err(EvalError::UndefinedSymbol(name.to_string()))
    }
}

pub fn define_global(env: &Rc<RefCell<Env>>, name: String, value: Value) {
    let parent = env.borrow().parent.clone();
    if let Some(p) = parent {
        define_global(&p, name, value);
        return;
    }
    env.borrow_mut().define(name, value);
}

