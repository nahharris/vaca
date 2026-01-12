use std::cell::RefCell;
use std::rc::Rc;

use crate::bezerro::env::define_global;
use crate::bezerro::env::Env;
use crate::bezerro::error::EvalError;
use crate::bezerro::value::Value;

use super::core::MAX_STACK_DEPTH;
use super::core::{eval_do_forms_impl, eval_value_impl, recur_tail_position_error};

pub(super) fn special_def(args: &[Value], env: &Rc<RefCell<Env>>, depth: usize) -> Result<Value, EvalError> {
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
    let value = eval_value_impl(&args[1], env, depth + 1)?;
    if matches!(value, Value::Recur(_)) {
        return Err(recur_tail_position_error());
    }
    define_global(env, name.clone(), value.clone());
    Ok(value)
}

pub(super) fn special_defn(args: &[Value], env: &Rc<RefCell<Env>>, _depth: usize) -> Result<Value, EvalError> {
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

pub(super) fn special_fn(args: &[Value], env: &Rc<RefCell<Env>>, _named: bool) -> Result<Value, EvalError> {
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

pub(super) fn special_defmacro(args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
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

pub(super) fn special_if(args: &[Value], env: &Rc<RefCell<Env>>, depth: usize) -> Result<Value, EvalError> {
    if args.len() != 3 {
        return Err(EvalError::ArityError {
            expected: 3,
            got: args.len(),
        });
    }
    let cond = eval_value_impl(&args[0], env, depth + 1)?;
    if matches!(cond, Value::Recur(_)) {
        return Err(recur_tail_position_error());
    }
    if cond.is_truthy() {
        eval_value_impl(&args[1], env, depth + 1)
    } else {
        eval_value_impl(&args[2], env, depth + 1)
    }
}

pub(super) fn special_do(args: &[Value], env: &Rc<RefCell<Env>>, depth: usize) -> Result<Value, EvalError> {
    eval_do_forms_impl(args, env, depth + 1)
}

pub(super) fn special_let(args: &[Value], env: &Rc<RefCell<Env>>, depth: usize) -> Result<Value, EvalError> {
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
        let value = eval_value_impl(&pair[1], &new_env, depth + 1)?;
        if matches!(value, Value::Recur(_)) {
            return Err(recur_tail_position_error());
        }
        new_env.borrow_mut().define(name.clone(), value);
    }
    eval_do_forms_impl(&args[1..], &new_env, depth + 1)
}

pub(super) fn special_quote(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError {
            expected: 1,
            got: args.len(),
        });
    }
    Ok(args[0].clone())
}

pub(super) fn special_pipe(args: &[Value], env: &Rc<RefCell<Env>>, depth: usize) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Ok(Value::Nil);
    }
    let mut acc = eval_value_impl(&args[0], env, depth + 1)?;
    if matches!(acc, Value::Recur(_)) {
        return Err(recur_tail_position_error());
    }
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
        acc = eval_value_impl(&next_form, env, depth + 1)?;
        if matches!(acc, Value::Recur(_)) {
            return Err(recur_tail_position_error());
        }
    }
    Ok(acc)
}

pub(super) fn special_recur(args: &[Value], env: &Rc<RefCell<Env>>, depth: usize) -> Result<Value, EvalError> {
    let mut out = Vec::with_capacity(args.len());
    for arg in args {
        let v = eval_value_impl(arg, env, depth + 1)?;
        if matches!(v, Value::Recur(_)) {
            return Err(recur_tail_position_error());
        }
        out.push(v);
    }
    Ok(Value::Recur(out))
}

pub(super) fn special_loop(args: &[Value], env: &Rc<RefCell<Env>>, depth: usize) -> Result<Value, EvalError> {
    if args.len() < 2 {
        return Err(EvalError::Custom(
            "loop expects: (loop [name value ...] body...)".to_string(),
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
            "loop bindings must have even number of forms".to_string(),
        ));
    }

    let loop_env = Rc::new(RefCell::new(Env::with_parent(env.clone())));
    let mut names: Vec<String> = Vec::with_capacity(bindings.len() / 2);

    for pair in bindings.chunks(2) {
        let Value::Symbol(name) = &pair[0] else {
            return Err(EvalError::TypeError {
                expected: "symbol",
                got: pair[0].type_name(),
            });
        };
        let value = eval_value_impl(&pair[1], &loop_env, depth + 1)?;
        if matches!(value, Value::Recur(_)) {
            return Err(recur_tail_position_error());
        }
        loop_env.borrow_mut().define(name.clone(), value);
        names.push(name.clone());
    }

    loop {
        if depth > MAX_STACK_DEPTH {
            return Err(EvalError::StackOverflow {
                limit: MAX_STACK_DEPTH,
            });
        }

        let result = eval_do_forms_impl(&args[1..], &loop_env, depth + 1)?;
        match result {
            Value::Recur(new_vals) => {
                if new_vals.len() != names.len() {
                    return Err(EvalError::ArityError {
                        expected: names.len(),
                        got: new_vals.len(),
                    });
                }
                for (name, value) in names.iter().zip(new_vals.into_iter()) {
                    loop_env.borrow_mut().define(name.clone(), value);
                }
            }
            other => return Ok(other),
        }
    }
}

