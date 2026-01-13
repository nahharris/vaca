use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::bezerro::env::Env;
use crate::bezerro::error::EvalError;
use crate::bezerro::value::Value;
use crate::vedn::{Kind, Node, Number};

use super::special_forms::{
    special_def, special_defmacro, special_defn, special_do, special_fn, special_if, special_let,
    special_loop, special_pipe, special_quote, special_recur,
};
use super::use_form::special_use;

pub(super) const MAX_STACK_DEPTH: usize = 10_000;

pub(super) const SPECIAL_FORM_HEADS: &[&str] = &[
    "def", "defn", "fn", "if", "do", "let", "quote", "defmacro", "deftype", "use", "|>", "recur",
    "loop",
];

pub(super) fn recur_tail_position_error() -> EvalError {
    EvalError::Custom("recur must be in tail position".to_string())
}

pub fn eval(node: &Node<'_>, env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    let form = node_to_form(node);
    let out = eval_value_impl(&form, env, 0)?;
    if matches!(out, Value::Recur(_)) {
        return Err(EvalError::Custom(
            "recur must be inside a function body or loop".to_string(),
        ));
    }
    Ok(out)
}

pub fn eval_value(form: &Value, env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    let out = eval_value_impl(form, env, 0)?;
    if matches!(out, Value::Recur(_)) {
        return Err(EvalError::Custom(
            "recur must be inside a function body or loop".to_string(),
        ));
    }
    Ok(out)
}

pub(super) fn eval_value_impl(
    form: &Value,
    env: &Rc<RefCell<Env>>,
    depth: usize,
) -> Result<Value, EvalError> {
    if depth > MAX_STACK_DEPTH {
        return Err(EvalError::StackOverflow {
            limit: MAX_STACK_DEPTH,
        });
    }

    match form {
        Value::Nil
        | Value::Bool(_)
        | Value::Int(_)
        | Value::Float(_)
        | Value::Char(_)
        | Value::String(_)
        | Value::Keyword(_)
        | Value::Builtin { .. }
        | Value::Lambda { .. }
        | Value::Macro { .. }
        | Value::Recur(_) => Ok(form.clone()),

        Value::Symbol(name) => env
            .borrow()
            .get(name)
            .ok_or_else(|| EvalError::UndefinedSymbol(name.clone())),

        Value::Vector(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                let v = eval_value_impl(item, env, depth + 1)?;
                if matches!(v, Value::Recur(_)) {
                    return Err(recur_tail_position_error());
                }
                out.push(v);
            }
            Ok(Value::Vector(out))
        }
        Value::Set(items) => {
            let mut out = HashSet::with_capacity(items.len());
            for item in items.iter() {
                let v = eval_value_impl(item, env, depth + 1)?;
                if matches!(v, Value::Recur(_)) {
                    return Err(recur_tail_position_error());
                }
                out.insert(v);
            }
            Ok(Value::Set(Rc::new(out)))
        }
        Value::Map(entries) => {
            let mut out: HashMap<Value, Value> = HashMap::with_capacity(entries.len());
            for (k, v) in entries.iter() {
                let kk = eval_value_impl(k, env, depth + 1)?;
                if matches!(kk, Value::Recur(_)) {
                    return Err(recur_tail_position_error());
                }
                let vv = eval_value_impl(v, env, depth + 1)?;
                if matches!(vv, Value::Recur(_)) {
                    return Err(recur_tail_position_error());
                }
                out.insert(kk, vv);
            }
            Ok(Value::Map(Rc::new(out)))
        }

        Value::List(items) => eval_list_impl(items, env, depth + 1),
    }
}

fn eval_list_impl(
    items: &[Value],
    env: &Rc<RefCell<Env>>,
    depth: usize,
) -> Result<Value, EvalError> {
    if items.is_empty() {
        return Ok(Value::List(vec![]));
    }

    // Special forms dispatch on the first element if it's a symbol.
    if let Value::Symbol(head) = &items[0] {
        match head.as_str() {
            "def" => return special_def(&items[1..], env, depth),
            "defn" => return special_defn(&items[1..], env, depth),
            "fn" => return special_fn(&items[1..], env, false),
            "if" => return special_if(&items[1..], env, depth),
            "do" => return special_do(&items[1..], env, depth),
            "let" => return special_let(&items[1..], env, depth),
            "quote" => return special_quote(&items[1..]),
            "defmacro" => return special_defmacro(&items[1..], env),
            "deftype" => return Ok(Value::Nil),
            "use" => return special_use(&items[1..], env, depth),
            "|>" => return special_pipe(&items[1..], env, depth),
            "recur" => return special_recur(&items[1..], env, depth),
            "loop" => return special_loop(&items[1..], env, depth),
            _ => {}
        }
    }

    // Macro / function call:
    // - Evaluate callee in the current env.
    // - If it's a macro, apply to raw args (forms), then eval expansion.
    // - Otherwise evaluate args, then apply.
    let callee = eval_value_impl(&items[0], env, depth + 1)?;
    if matches!(callee, Value::Macro { .. }) {
        let expanded = apply_macro(&callee, &items[1..], depth + 1)?;
        return eval_value_impl(&expanded, env, depth + 1);
    }

    let mut args = Vec::with_capacity(items.len().saturating_sub(1));
    for arg in &items[1..] {
        let v = eval_value_impl(arg, env, depth + 1)?;
        if matches!(v, Value::Recur(_)) {
            return Err(recur_tail_position_error());
        }
        args.push(v);
    }
    apply_impl(&callee, &args, env, depth + 1)
}

pub fn apply(func: &Value, args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    apply_impl(func, args, env, 0)
}

fn apply_impl(
    func: &Value,
    args: &[Value],
    env: &Rc<RefCell<Env>>,
    depth: usize,
) -> Result<Value, EvalError> {
    if depth > MAX_STACK_DEPTH {
        return Err(EvalError::StackOverflow {
            limit: MAX_STACK_DEPTH,
        });
    }

    match func {
        Value::Builtin { func, .. } => func(args, env),
        Value::Lambda {
            params,
            body,
            env: captured,
        } => {
            if args.len() != params.len() {
                return Err(EvalError::ArityError {
                    expected: params.len(),
                    got: args.len(),
                });
            }

            let mut current_args: Vec<Value> = args.to_vec();
            loop {
                let new_env = Rc::new(RefCell::new(Env::with_parent(captured.clone())));
                for (p, a) in params.iter().zip(current_args.iter()) {
                    new_env.borrow_mut().define(p.clone(), a.clone());
                }

                let result = eval_do_forms_impl(body, &new_env, depth + 1)?;
                match result {
                    Value::Recur(new_args) => {
                        if new_args.len() != params.len() {
                            return Err(EvalError::ArityError {
                                expected: params.len(),
                                got: new_args.len(),
                            });
                        }
                        current_args = new_args;
                    }
                    other => return Ok(other),
                }
            }
        }
        other => Err(EvalError::NotCallable(other.type_name())),
    }
}

fn apply_macro(func: &Value, raw_args: &[Value], depth: usize) -> Result<Value, EvalError> {
    let Value::Macro {
        params,
        body,
        env: captured,
    } = func
    else {
        return Err(EvalError::NotCallable(func.type_name()));
    };

    if raw_args.len() != params.len() {
        return Err(EvalError::ArityError {
            expected: params.len(),
            got: raw_args.len(),
        });
    }

    let macro_env = Rc::new(RefCell::new(Env::with_parent(captured.clone())));
    for (p, a) in params.iter().zip(raw_args.iter()) {
        macro_env.borrow_mut().define(p.clone(), a.clone());
    }

    let expansion = eval_do_forms_impl(body, &macro_env, depth + 1)?;
    // Expansion is a form; evaluate it back in the call site env.
    Ok(expansion)
}

pub(super) fn eval_do_forms_impl(
    forms: &[Value],
    env: &Rc<RefCell<Env>>,
    depth: usize,
) -> Result<Value, EvalError> {
    let mut last = Value::Nil;
    for (i, form) in forms.iter().enumerate() {
        last = eval_value_impl(form, env, depth + 1)?;
        if i + 1 != forms.len() && matches!(last, Value::Recur(_)) {
            return Err(recur_tail_position_error());
        }
    }
    Ok(last)
}

pub fn node_to_form(node: &Node<'_>) -> Value {
    match &node.kind {
        Kind::Nil => Value::Nil,
        Kind::Bool(b) => Value::Bool(*b),
        Kind::Char(c) => Value::Char(*c),
        Kind::String(s) => Value::String(s.as_str().to_string()),
        Kind::Keyword(k) => Value::Keyword(crate::bezerro::value::Keyword {
            namespace: k.namespace.map(str::to_string),
            name: k.name.to_string(),
        }),
        Kind::Symbol(s) => Value::Symbol(s.raw.to_string()),
        Kind::Number(n) => number_to_value(n),
        Kind::List(items) => Value::List(items.iter().map(node_to_form).collect()),
        Kind::Vector(items) => Value::Vector(items.iter().map(node_to_form).collect()),
        Kind::Set(items) => Value::Set(Rc::new(
            items.iter().map(node_to_form).collect::<HashSet<_>>(),
        )),
        Kind::Map(entries) => Value::Map(Rc::new(
            entries
                .iter()
                .map(|(k, v)| (node_to_form(k), node_to_form(v)))
                .collect::<HashMap<_, _>>(),
        )),
    }
}

pub(super) fn number_to_value(n: &Number<'_>) -> Value {
    match n {
        Number::Int { lexeme, .. } => lexeme
            .parse::<i64>()
            .map(Value::Int)
            .unwrap_or_else(|_| Value::Int(0)),
        Number::Float { lexeme, .. } => lexeme
            .parse::<f64>()
            .map(Value::Float)
            .unwrap_or_else(|_| Value::Float(0.0)),
    }
}
