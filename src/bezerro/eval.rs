use std::cell::RefCell;
use std::rc::Rc;

use crate::bezerro::env::{define_global, Env};
use crate::bezerro::error::EvalError;
use crate::bezerro::value::Value;
use crate::vedn::{Kind, Node, Number};

pub fn eval(node: &Node<'_>, env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    let form = node_to_form(node);
    eval_value(&form, env)
}

pub fn eval_value(form: &Value, env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
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
        | Value::Macro { .. } => Ok(form.clone()),

        Value::Symbol(name) => env
            .borrow()
            .get(name)
            .ok_or_else(|| EvalError::UndefinedSymbol(name.clone())),

        Value::Vector(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(eval_value(item, env)?);
            }
            Ok(Value::Vector(out))
        }
        Value::Set(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(eval_value(item, env)?);
            }
            Ok(Value::Set(out))
        }
        Value::Map(entries) => {
            let mut out = Vec::with_capacity(entries.len());
            for (k, v) in entries {
                out.push((eval_value(k, env)?, eval_value(v, env)?));
            }
            Ok(Value::Map(out))
        }

        Value::List(items) => eval_list(items, env),
    }
}

fn eval_list(items: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    if items.is_empty() {
        return Ok(Value::List(vec![]));
    }

    // Special forms dispatch on the first element if it's a symbol.
    if let Value::Symbol(head) = &items[0] {
        match head.as_str() {
            "def" => return special_def(&items[1..], env),
            "defn" => return special_defn(&items[1..], env),
            "fn" => return special_fn(&items[1..], env, false),
            "if" => return special_if(&items[1..], env),
            "do" => return special_do(&items[1..], env),
            "let" => return special_let(&items[1..], env),
            "quote" => return special_quote(&items[1..]),
            "defmacro" => return special_defmacro(&items[1..], env),
            "deftype" => return Ok(Value::Nil),
            "use" => return Ok(Value::Nil),
            "|>" => return special_pipe(&items[1..], env),
            _ => {}
        }
    }

    // Macro / function call:
    // - Evaluate callee in the current env.
    // - If it's a macro, apply to raw args (forms), then eval expansion.
    // - Otherwise evaluate args, then apply.
    let callee = eval_value(&items[0], env)?;
    if matches!(callee, Value::Macro { .. }) {
        let expanded = apply_macro(&callee, &items[1..])?;
        return eval_value(&expanded, env);
    }

    let mut args = Vec::with_capacity(items.len().saturating_sub(1));
    for arg in &items[1..] {
        args.push(eval_value(arg, env)?);
    }
    apply(&callee, &args, env)
}

pub fn apply(func: &Value, args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    match func {
        Value::Builtin { func, .. } => func(args, env),
        Value::Lambda { params, body, env: captured } => {
            if args.len() != params.len() {
                return Err(EvalError::ArityError {
                    expected: params.len(),
                    got: args.len(),
                });
            }

            let new_env = Rc::new(RefCell::new(Env::with_parent(captured.clone())));
            for (p, a) in params.iter().zip(args.iter()) {
                new_env.borrow_mut().define(p.clone(), a.clone());
            }
            eval_do_forms(body, &new_env)
        }
        other => Err(EvalError::NotCallable(other.type_name())),
    }
}

fn apply_macro(
    func: &Value,
    raw_args: &[Value],
) -> Result<Value, EvalError> {
    let Value::Macro { params, body, env: captured } = func else {
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

    let expansion = eval_do_forms(body, &macro_env)?;
    // Expansion is a form; evaluate it back in the call site env.
    Ok(expansion)
}

fn special_def(args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError {
            expected: 2,
            got: args.len(),
        });
    }
    let Value::Symbol(name) = &args[0] else {
        return Err(EvalError::TypeError {
            expected: "symbol",
            got: args[0].type_name(),
        });
    };
    let value = eval_value(&args[1], env)?;
    define_global(env, name.clone(), value.clone());
    Ok(value)
}

fn special_defn(args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    if args.len() < 3 {
        return Err(EvalError::Custom(
            "defn expects: (defn name [params] body...)".to_string(),
        ));
    }
    let Value::Symbol(name) = &args[0] else {
        return Err(EvalError::TypeError {
            expected: "symbol",
            got: args[0].type_name(),
        });
    };
    let lambda = special_fn(&args[1..], env, true)?;
    define_global(env, name.clone(), lambda.clone());
    Ok(lambda)
}

fn special_fn(args: &[Value], env: &Rc<RefCell<Env>>, _named: bool) -> Result<Value, EvalError> {
    if args.len() < 2 {
        return Err(EvalError::Custom(
            "fn expects: (fn [params] body...)".to_string(),
        ));
    }
    let params = parse_params(&args[0])?;
    let body = args[1..].to_vec();
    Ok(Value::Lambda {
        params,
        body,
        env: env.clone(),
    })
}

fn special_defmacro(args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    if args.len() < 3 {
        return Err(EvalError::Custom(
            "defmacro expects: (defmacro name [params] body...)".to_string(),
        ));
    }
    let Value::Symbol(name) = &args[0] else {
        return Err(EvalError::TypeError {
            expected: "symbol",
            got: args[0].type_name(),
        });
    };
    let params = parse_params(&args[1])?;
    let body = args[2..].to_vec();
    let mac = Value::Macro {
        params,
        body,
        env: env.clone(),
    };
    define_global(env, name.clone(), mac.clone());
    Ok(mac)
}

fn parse_params(form: &Value) -> Result<Vec<String>, EvalError> {
    let Value::Vector(items) = form else {
        return Err(EvalError::TypeError {
            expected: "vector",
            got: form.type_name(),
        });
    };
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        let Value::Symbol(name) = item else {
            return Err(EvalError::TypeError {
                expected: "symbol",
                got: item.type_name(),
            });
        };
        out.push(name.clone());
    }
    Ok(out)
}

fn special_if(args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    if args.len() != 3 {
        return Err(EvalError::ArityError {
            expected: 3,
            got: args.len(),
        });
    }
    let cond = eval_value(&args[0], env)?;
    if cond.is_truthy() {
        eval_value(&args[1], env)
    } else {
        eval_value(&args[2], env)
    }
}

fn special_do(args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    eval_do_forms(args, env)
}

fn eval_do_forms(forms: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    let mut last = Value::Nil;
    for form in forms {
        last = eval_value(form, env)?;
    }
    Ok(last)
}

fn special_let(args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    if args.len() < 2 {
        return Err(EvalError::Custom(
            "let expects: (let [name value ...] body...)".to_string(),
        ));
    }
    let Value::Vector(bindings) = &args[0] else {
        return Err(EvalError::TypeError {
            expected: "vector",
            got: args[0].type_name(),
        });
    };
    if bindings.len() % 2 != 0 {
        return Err(EvalError::Custom(
            "let bindings must have even number of forms".to_string(),
        ));
    }

    let new_env = Rc::new(RefCell::new(Env::with_parent(env.clone())));
    for pair in bindings.chunks(2) {
        let Value::Symbol(name) = &pair[0] else {
            return Err(EvalError::TypeError {
                expected: "symbol",
                got: pair[0].type_name(),
            });
        };
        let value = eval_value(&pair[1], &new_env)?;
        new_env.borrow_mut().define(name.clone(), value);
    }
    eval_do_forms(&args[1..], &new_env)
}

fn special_quote(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError {
            expected: 1,
            got: args.len(),
        });
    }
    Ok(args[0].clone())
}

fn special_pipe(args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Ok(Value::Nil);
    }
    let mut acc = eval_value(&args[0], env)?;
    for step in &args[1..] {
        let next_form = match step {
            Value::List(list) if !list.is_empty() => {
                let mut new_list = Vec::with_capacity(list.len() + 1);
                new_list.push(list[0].clone());
                new_list.push(acc);
                new_list.extend_from_slice(&list[1..]);
                Value::List(new_list)
            }
            other => Value::List(vec![other.clone(), acc]),
        };
        acc = eval_value(&next_form, env)?;
    }
    Ok(acc)
}

pub fn node_to_form(node: &Node<'_>) -> Value {
    match &node.kind {
        Kind::Nil => Value::Nil,
        Kind::Bool(b) => Value::Bool(*b),
        Kind::Char(c) => Value::Char(*c),
        Kind::String(s) => Value::String(s.as_str().to_string()),
        Kind::Keyword(k) => Value::Keyword(k.raw.trim_start_matches(':').to_string()),
        Kind::Symbol(s) => Value::Symbol(s.raw.to_string()),
        Kind::Number(n) => number_to_value(n),
        Kind::List(items) => Value::List(items.iter().map(node_to_form).collect()),
        Kind::Vector(items) => Value::Vector(items.iter().map(node_to_form).collect()),
        Kind::Set(items) => Value::Set(items.iter().map(node_to_form).collect()),
        Kind::Map(entries) => Value::Map(
            entries
                .iter()
                .map(|(k, v)| (node_to_form(k), node_to_form(v)))
                .collect(),
        ),
        Kind::Typed(t) => node_to_form(&t.value),
    }
}

fn number_to_value(n: &Number<'_>) -> Value {
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

