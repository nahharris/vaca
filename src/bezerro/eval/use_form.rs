use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::bezerro::env::{define_global, root_env, Env, ModuleInfo};
use crate::bezerro::error::{EvalError, UseError};
use crate::bezerro::value::Value;

use super::core::{eval_value_impl, node_to_form, recur_tail_position_error, SPECIAL_FORM_HEADS};

pub(super) fn special_use(args: &[Value], env: &Rc<RefCell<Env>>, depth: usize) -> Result<Value, EvalError> {
    if args.is_empty() || args.len() > 2 {
        return Err(EvalError::Use(UseError::BadArity { got: args.len() }));
    }

    let Value::Symbol(module_spec) = &args[0] else {
        return Err(EvalError::Use(UseError::ExpectedModuleSymbol {
            got: args[0].type_name(),
        }));
    };

    let root = root_env(env);
    let module_path = resolve_module_path(module_spec, &root)?;
    let module_info = ensure_module_loaded(&module_path, &root, depth + 1)?;

    let requested = if args.len() == 1 {
        // Import all exports with their original names.
        module_info
            .exports
            .iter()
            .cloned()
            .map(|orig| (orig.clone(), orig))
            .collect::<Vec<_>>()
    } else {
        parse_use_import_list(&args[1])?
    };

    // Define visible aliases in the root env.
    for (orig, visible) in requested {
        if !module_info.exports.contains(&orig) {
            return Err(EvalError::Use(UseError::MissingExport {
                module: module_spec.clone(),
                symbol: orig,
            }));
        }

        if root.borrow().contains_local(&visible) {
            return Err(EvalError::Use(UseError::NameCollision { name: visible }));
        }

        let mangled = module_info
            .mangle_map
            .get(&orig)
            .ok_or_else(|| {
                EvalError::Use(UseError::Internal {
                    message: format!("missing mangle for `{orig}`"),
                })
            })?;

        let value = root
            .borrow()
            .get(mangled)
            .ok_or_else(|| {
                EvalError::Use(UseError::Internal {
                    message: format!("missing value for `{orig}`"),
                })
            })?;

        define_global(&root, visible, value);
    }

    Ok(Value::Nil)
}

fn parse_use_import_list(form: &Value) -> Result<Vec<(String, String)>, EvalError> {
    let Value::Vector(items) = form else {
        return Err(EvalError::Use(UseError::ExpectedImportVector {
            got: form.type_name(),
        }));
    };

    let mut out = Vec::new();
    let mut i = 0;
    while i < items.len() {
        let Value::Symbol(orig) = &items[i] else {
            return Err(EvalError::Use(UseError::ExpectedImportSymbol {
                got: items[i].type_name(),
            }));
        };
        let mut visible = orig.clone();

        if i + 2 < items.len() {
            if let Value::Keyword(k) = &items[i + 1] {
                if k == "as" {
                    let Value::Symbol(alias) = &items[i + 2] else {
                        return Err(EvalError::Use(UseError::ExpectedAliasSymbol {
                            got: items[i + 2].type_name(),
                        }));
                    };
                    visible = alias.clone();
                    i += 3;
                    out.push((orig.clone(), visible));
                    continue;
                }
            }
        }

        i += 1;
        out.push((orig.clone(), visible));
    }
    Ok(out)
}

fn resolve_module_path(module_spec: &str, root: &Rc<RefCell<Env>>) -> Result<PathBuf, EvalError> {
    let base_dir = root
        .borrow()
        .source_dir()
        .or_else(|| std::env::current_dir().ok())
        .ok_or_else(|| EvalError::Use(UseError::FailedToDetermineBaseDir))?;

    let parts: Vec<&str> = module_spec.split('.').filter(|p| !p.is_empty()).collect();
    if parts.is_empty() {
        return Err(EvalError::Use(UseError::EmptyModulePath));
    }

    // Flat file mapping: `a.b.c` -> `<base>/a/b/c.vaca`
    let mut dir = base_dir;
    for seg in &parts[..parts.len() - 1] {
        if *seg == "super" {
            dir = dir.parent().map(|p| p.to_path_buf()).ok_or_else(|| {
                EvalError::Use(UseError::SuperBeyondRoot {
                    module: module_spec.to_string(),
                })
            })?;
        } else {
            dir.push(seg);
        }
    }

    let file = parts[parts.len() - 1];
    if file == "super" {
        return Err(EvalError::Use(UseError::LastSegmentCannotBeSuper));
    }
    dir.push(format!("{file}.vaca"));
    Ok(dir)
}

fn ensure_module_loaded(
    module_path: &Path,
    root: &Rc<RefCell<Env>>,
    depth: usize,
) -> Result<ModuleInfo, EvalError> {
    let module_path = fs::canonicalize(module_path).map_err(|e| {
        EvalError::Use(UseError::ResolveFailed {
            path: module_path.display().to_string(),
            error: e.to_string(),
        })
    })?;

    let cache = root.borrow().module_cache();
    if let Some(info) = cache.borrow().get(&module_path).cloned() {
        return Ok(info);
    }

    let loading = root.borrow().module_loading();
    {
        let mut loading = loading.borrow_mut();
        if loading.contains(&module_path) {
            return Err(EvalError::Use(UseError::CyclicUse {
                path: module_path.display().to_string(),
            }));
        }
        loading.insert(module_path.clone());
    }

    let result = (|| {
        let src = fs::read_to_string(&module_path).map_err(|e| {
            EvalError::Use(UseError::ReadFailed {
                path: module_path.display().to_string(),
                error: e.to_string(),
            })
        })?;

        let nodes = crate::parse(&src).map_err(|e| EvalError::ParseError(e.to_string()))?;
        let forms: Vec<Value> = nodes.iter().map(node_to_form).collect();

        let exports = collect_module_exports(&forms)?;
        let module_key = module_key_hash(&module_path);
        let mangle_map = exports
            .iter()
            .map(|orig| (orig.clone(), format!("__use__{module_key}__{orig}")))
            .collect::<HashMap<_, _>>();

        // Rewrite module forms so the module defines / refers to mangled names.
        let rewritten = forms
            .iter()
            .map(|f| rewrite_module_form(f, &mangle_map, false))
            .collect::<Vec<_>>();

        // Evaluate module in the importer's *global* env, but with source_dir temporarily set to
        // the module's directory so nested `(use ...)` resolve correctly.
        let prev_source_dir = root.borrow().source_dir();
        if let Some(dir) = module_path.parent() {
            root.borrow_mut().set_source_dir(dir.to_path_buf());
        }

        for f in &rewritten {
            let v = eval_value_impl(f, root, depth + 1)?;
            if matches!(v, Value::Recur(_)) {
                return Err(recur_tail_position_error());
            }
        }

        // Restore previous source dir
        root.borrow_mut().set_source_dir_opt(prev_source_dir);

        Ok(ModuleInfo { exports, mangle_map })
    })();

    // Ensure we always clear loading marker.
    loading.borrow_mut().remove(&module_path);

    if let Ok(info) = &result {
        cache.borrow_mut().insert(module_path, info.clone());
    }
    result
}

fn module_key_hash(path: &Path) -> String {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    path.to_string_lossy().hash(&mut h);
    format!("{:x}", h.finish())
}

fn collect_module_exports(forms: &[Value]) -> Result<HashSet<String>, EvalError> {
    let mut out = HashSet::new();
    for form in forms {
        let Value::List(items) = form else { continue };
        if items.len() < 2 {
            continue;
        }
        let Value::Symbol(head) = &items[0] else { continue };
        if !matches!(head.as_str(), "def" | "defn" | "defmacro") {
            continue;
        }
        let Value::Symbol(name) = &items[1] else {
            return Err(EvalError::Use(UseError::InvalidExportForm {
                head: head.to_string(),
            }));
        };
        out.insert(name.clone());
    }
    Ok(out)
}

fn rewrite_module_form(form: &Value, mangle: &HashMap<String, String>, in_defmacro: bool) -> Value {
    rewrite_form_impl(form, mangle, &HashSet::new(), in_defmacro)
}

fn rewrite_form_impl(
    form: &Value,
    mangle: &HashMap<String, String>,
    shadowed: &HashSet<String>,
    rewrite_in_quote: bool,
) -> Value {
    match form {
        Value::Symbol(name) => {
            if shadowed.contains(name) {
                return form.clone();
            }
            if let Some(mapped) = mangle.get(name) {
                return Value::Symbol(mapped.clone());
            }
            form.clone()
        }
        Value::List(items) => rewrite_list_impl(items, mangle, shadowed, rewrite_in_quote),
        Value::Vector(items) => Value::Vector(
            items
                .iter()
                .map(|v| rewrite_form_impl(v, mangle, shadowed, rewrite_in_quote))
                .collect(),
        ),
        Value::Map(entries) => {
            let mut out: HashMap<Value, Value> = HashMap::with_capacity(entries.len());
            for (k, v) in entries.iter() {
                out.insert(
                    rewrite_form_impl(k, mangle, shadowed, rewrite_in_quote),
                    rewrite_form_impl(v, mangle, shadowed, rewrite_in_quote),
                );
            }
            Value::Map(Rc::new(out))
        }
        Value::Set(items) => {
            let mut out = HashSet::with_capacity(items.len());
            for item in items.iter() {
                out.insert(rewrite_form_impl(item, mangle, shadowed, rewrite_in_quote));
            }
            Value::Set(Rc::new(out))
        }
        _ => form.clone(),
    }
}

fn rewrite_list_impl(
    items: &[Value],
    mangle: &HashMap<String, String>,
    shadowed: &HashSet<String>,
    rewrite_in_quote: bool,
) -> Value {
    if items.is_empty() {
        return Value::List(vec![]);
    }

    let head_sym = match &items[0] {
        Value::Symbol(s) => Some(s.as_str()),
        _ => None,
    };

    // Handle quote. By default, we do NOT rewrite inside quote, but inside defmacro bodies we do.
    if head_sym == Some("quote") && items.len() == 2 && !rewrite_in_quote {
        return Value::List(vec![items[0].clone(), items[1].clone()]);
    }

    match head_sym {
        Some("def") => {
            if items.len() != 3 {
                return Value::List(items.to_vec());
            }
            let name = match &items[1] {
                Value::Symbol(s) => s,
                _ => return Value::List(items.to_vec()),
            };
            let new_name = mangle
                .get(name)
                .cloned()
                .map(Value::Symbol)
                .unwrap_or_else(|| items[1].clone());
            Value::List(vec![
                items[0].clone(),
                new_name,
                rewrite_form_impl(&items[2], mangle, shadowed, rewrite_in_quote),
            ])
        }
        Some("defn") => {
            if items.len() < 4 {
                return Value::List(items.to_vec());
            }
            let name = match &items[1] {
                Value::Symbol(s) => s,
                _ => return Value::List(items.to_vec()),
            };
            let new_name = mangle
                .get(name)
                .cloned()
                .map(Value::Symbol)
                .unwrap_or_else(|| items[1].clone());

            let Value::Vector(params) = &items[2] else {
                return Value::List(items.to_vec());
            };
            let mut new_shadowed = shadowed.clone();
            for p in params {
                if let Value::Symbol(s) = p {
                    new_shadowed.insert(s.clone());
                }
            }

            let mut out = Vec::with_capacity(items.len());
            out.push(items[0].clone()); // defn
            out.push(new_name);
            out.push(items[2].clone()); // params untouched
            for b in &items[3..] {
                out.push(rewrite_form_impl(b, mangle, &new_shadowed, rewrite_in_quote));
            }
            Value::List(out)
        }
        Some("defmacro") => {
            if items.len() < 4 {
                return Value::List(items.to_vec());
            }
            let name = match &items[1] {
                Value::Symbol(s) => s,
                _ => return Value::List(items.to_vec()),
            };
            let new_name = mangle
                .get(name)
                .cloned()
                .map(Value::Symbol)
                .unwrap_or_else(|| items[1].clone());

            let Value::Vector(params) = &items[2] else {
                return Value::List(items.to_vec());
            };
            let mut new_shadowed = shadowed.clone();
            for p in params {
                if let Value::Symbol(s) = p {
                    new_shadowed.insert(s.clone());
                }
            }

            let mut out = Vec::with_capacity(items.len());
            out.push(items[0].clone()); // defmacro
            out.push(new_name);
            out.push(items[2].clone()); // params untouched
            for b in &items[3..] {
                out.push(rewrite_form_impl(b, mangle, &new_shadowed, true));
            }
            Value::List(out)
        }
        Some("let") => {
            if items.len() < 3 {
                return Value::List(items.to_vec());
            }
            let Value::Vector(bindings) = &items[1] else {
                return Value::List(items.to_vec());
            };
            if bindings.len() % 2 != 0 {
                return Value::List(items.to_vec());
            }

            let mut new_bindings = Vec::with_capacity(bindings.len());
            let mut scoped = shadowed.clone();
            for pair in bindings.chunks(2) {
                let name = &pair[0];
                let value = &pair[1];
                new_bindings.push(name.clone()); // binder symbol untouched
                new_bindings.push(rewrite_form_impl(value, mangle, &scoped, rewrite_in_quote));
                if let Value::Symbol(s) = name {
                    scoped.insert(s.clone());
                }
            }

            let mut out = Vec::with_capacity(items.len());
            out.push(items[0].clone());
            out.push(Value::Vector(new_bindings));
            for b in &items[2..] {
                out.push(rewrite_form_impl(b, mangle, &scoped, rewrite_in_quote));
            }
            Value::List(out)
        }
        Some("fn") => {
            if items.len() < 3 {
                return Value::List(items.to_vec());
            }
            let Value::Vector(params) = &items[1] else {
                return Value::List(items.to_vec());
            };
            let mut scoped = shadowed.clone();
            for p in params {
                if let Value::Symbol(s) = p {
                    scoped.insert(s.clone());
                }
            }

            let mut out = Vec::with_capacity(items.len());
            out.push(items[0].clone());
            out.push(items[1].clone()); // params untouched
            for b in &items[2..] {
                out.push(rewrite_form_impl(b, mangle, &scoped, rewrite_in_quote));
            }
            Value::List(out)
        }
        Some("loop") => {
            if items.len() < 3 {
                return Value::List(items.to_vec());
            }
            let Value::Vector(bindings) = &items[1] else {
                return Value::List(items.to_vec());
            };
            if bindings.len() % 2 != 0 {
                return Value::List(items.to_vec());
            }

            let mut new_bindings = Vec::with_capacity(bindings.len());
            let mut scoped = shadowed.clone();
            for pair in bindings.chunks(2) {
                let name = &pair[0];
                let value = &pair[1];
                new_bindings.push(name.clone());
                new_bindings.push(rewrite_form_impl(value, mangle, &scoped, rewrite_in_quote));
                if let Value::Symbol(s) = name {
                    scoped.insert(s.clone());
                }
            }

            let mut out = Vec::with_capacity(items.len());
            out.push(items[0].clone());
            out.push(Value::Vector(new_bindings));
            for b in &items[2..] {
                out.push(rewrite_form_impl(b, mangle, &scoped, rewrite_in_quote));
            }
            Value::List(out)
        }
        Some("quote") => {
            // rewrite_in_quote == true case
            let mut out = Vec::with_capacity(items.len());
            out.push(items[0].clone());
            for a in &items[1..] {
                out.push(rewrite_form_impl(a, mangle, shadowed, rewrite_in_quote));
            }
            Value::List(out)
        }
        _ => {
            let mut out = Vec::with_capacity(items.len());
            // Head element: don't rewrite if it is a special form name.
            if let Value::Symbol(s) = &items[0] {
                if SPECIAL_FORM_HEADS.contains(&s.as_str()) {
                    out.push(items[0].clone());
                } else {
                    out.push(rewrite_form_impl(&items[0], mangle, shadowed, rewrite_in_quote));
                }
            } else {
                out.push(rewrite_form_impl(&items[0], mangle, shadowed, rewrite_in_quote));
            }
            for v in &items[1..] {
                out.push(rewrite_form_impl(v, mangle, shadowed, rewrite_in_quote));
            }
            Value::List(out)
        }
    }
}

