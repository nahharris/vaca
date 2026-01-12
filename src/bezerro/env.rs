use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;

use crate::bezerro::error::EvalError;
use crate::bezerro::value::Value;

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub exports: HashSet<String>,
    pub mangle_map: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Env {
    bindings: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Env>>>,
    source_dir: Option<PathBuf>,
    module_cache: Rc<RefCell<HashMap<PathBuf, ModuleInfo>>>,
    module_loading: Rc<RefCell<HashSet<PathBuf>>>,
}

impl Env {
    pub fn new() -> Self {
        let module_cache = Rc::new(RefCell::new(HashMap::new()));
        let module_loading = Rc::new(RefCell::new(HashSet::new()));
        Env {
            bindings: HashMap::new(),
            parent: None,
            source_dir: None,
            module_cache,
            module_loading,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Env>>) -> Self {
        let source_dir = parent.borrow().source_dir.clone();
        let module_cache = parent.borrow().module_cache.clone();
        let module_loading = parent.borrow().module_loading.clone();
        Env {
            bindings: HashMap::new(),
            parent: Some(parent),
            source_dir,
            module_cache,
            module_loading,
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

    pub fn contains_local(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
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

    pub fn parent(&self) -> Option<Rc<RefCell<Env>>> {
        self.parent.clone()
    }

    pub fn source_dir(&self) -> Option<PathBuf> {
        self.source_dir.clone()
    }

    pub fn set_source_dir(&mut self, dir: PathBuf) {
        self.source_dir = Some(dir);
    }

    pub fn set_source_dir_opt(&mut self, dir: Option<PathBuf>) {
        self.source_dir = dir;
    }

    pub fn module_cache(&self) -> Rc<RefCell<HashMap<PathBuf, ModuleInfo>>> {
        self.module_cache.clone()
    }

    pub fn module_loading(&self) -> Rc<RefCell<HashSet<PathBuf>>> {
        self.module_loading.clone()
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

pub fn root_env(env: &Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
    let mut cur = env.clone();
    loop {
        let parent = { cur.borrow().parent.clone() };
        match parent {
            Some(p) => cur = p,
            None => return cur,
        }
    }
}
