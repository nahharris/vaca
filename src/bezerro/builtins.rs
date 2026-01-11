use std::cell::RefCell;
use std::f64::consts::PI;
use std::io::{self, BufRead, Write};
use std::rc::Rc;

use crate::bezerro::env::Env;
use crate::bezerro::error::EvalError;
use crate::bezerro::eval::apply;
use crate::bezerro::value::{BuiltinFn, Value};

pub fn register_builtins(env: &mut Env) {
    env.define("pi".into(), Value::Float(PI));

    // arithmetic
    env.define("+".into(), builtin("+", builtin_add));
    env.define("-".into(), builtin("-", builtin_sub));
    env.define("*".into(), builtin("*", builtin_mul));
    env.define("/".into(), builtin("/", builtin_div));
    env.define("//".into(), builtin("//", builtin_int_div));
    env.define("^".into(), builtin("^", builtin_pow));
    env.define("mod".into(), builtin("mod", builtin_mod));
    env.define("brt".into(), builtin("brt", builtin_brt));
    env.define("max".into(), builtin("max", builtin_max));
    env.define("min".into(), builtin("min", builtin_min));

    // comparison
    env.define(">".into(), builtin(">", builtin_gt));
    env.define("<".into(), builtin("<", builtin_lt));
    env.define(">=".into(), builtin(">=", builtin_gte));
    env.define("<=".into(), builtin("<=", builtin_lte));
    env.define("==".into(), builtin("==", builtin_eq));
    env.define("!=".into(), builtin("!=", builtin_neq));

    // logic
    env.define("&".into(), builtin("&", builtin_and));
    env.define("|".into(), builtin("|", builtin_or));

    // io
    env.define("readln".into(), builtin("readln", builtin_readln));
    env.define("format".into(), builtin("format", builtin_format));
    env.define("print".into(), builtin("print", builtin_print));
    env.define("println".into(), builtin("println", builtin_println));

    // parsing
    env.define("parse-int".into(), builtin("parse-int", builtin_parse_int));
    env.define("parse-float".into(), builtin("parse-float", builtin_parse_float));

    // collections
    env.define("concat".into(), builtin("concat", builtin_concat));
    env.define("append".into(), builtin("append", builtin_append));
    env.define("prepend".into(), builtin("prepend", builtin_prepend));
    env.define("nth".into(), builtin("nth", builtin_nth));
    env.define("map".into(), builtin("map", builtin_map));
    env.define("reduce".into(), builtin("reduce", builtin_reduce));
    env.define("scan".into(), builtin("scan", builtin_scan));

    // \"macro\" fns that we treat as builtins for now
    env.define("assert".into(), builtin("assert", builtin_assert));
}

fn builtin(name: &'static str, func: BuiltinFn) -> Value {
    Value::Builtin { name, func }
}

fn expect_arity(args: &[Value], n: usize) -> Result<(), EvalError> {
    if args.len() != n {
        return Err(EvalError::ArityError {
            expected: n,
            got: args.len(),
        });
    }
    Ok(())
}

fn promote(a: &Value, b: &Value) -> Result<(f64, f64, bool), EvalError> {
    // returns (af, bf, are_ints)
    match (a, b) {
        (Value::Int(ai), Value::Int(bi)) => Ok((*ai as f64, *bi as f64, true)),
        _ => {
            let af = match a {
                Value::Int(i) => *i as f64,
                Value::Float(f) => *f,
                _ => {
                    return Err(EvalError::TypeError {
                        expected: "number",
                        got: a.type_name(),
                    })
                }
            };
            let bf = match b {
                Value::Int(i) => *i as f64,
                Value::Float(f) => *f,
                _ => {
                    return Err(EvalError::TypeError {
                        expected: "number",
                        got: b.type_name(),
                    })
                }
            };
            Ok((af, bf, false))
        }
    }
}

fn builtin_add(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Ok(Value::Int(0));
    }

    let mut is_float = false;
    let mut acc_i: i64 = 0;
    let mut acc_f: f64 = 0.0;

    for a in args {
        match a {
            Value::Int(i) if !is_float => acc_i = acc_i.saturating_add(*i),
            Value::Int(i) => acc_f += *i as f64,
            Value::Float(f) => {
                if !is_float {
                    is_float = true;
                    acc_f = acc_i as f64;
                }
                acc_f += *f;
            }
            _ => {
                return Err(EvalError::TypeError {
                    expected: "number",
                    got: a.type_name(),
                })
            }
        }
    }

    Ok(if is_float {
        Value::Float(acc_f)
    } else {
        Value::Int(acc_i)
    })
}

fn builtin_sub(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Err(EvalError::ArityError {
            expected: 1,
            got: 0,
        });
    }
    if args.len() == 1 {
        return match &args[0] {
            Value::Int(i) => Ok(Value::Int(-i)),
            Value::Float(f) => Ok(Value::Float(-f)),
            other => Err(EvalError::TypeError {
                expected: "number",
                got: other.type_name(),
            }),
        };
    }

    let mut acc = args[0].clone();
    for a in &args[1..] {
        acc = match (&acc, a) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
            _ => {
                let (x, y, _are_ints) = promote(&acc, a)?;
                Value::Float(x - y)
            }
        };
    }
    Ok(acc)
}

fn builtin_mul(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Ok(Value::Int(1));
    }

    let mut is_float = false;
    let mut acc_i: i64 = 1;
    let mut acc_f: f64 = 1.0;

    for a in args {
        match a {
            Value::Int(i) if !is_float => acc_i = acc_i.saturating_mul(*i),
            Value::Int(i) => acc_f *= *i as f64,
            Value::Float(f) => {
                if !is_float {
                    is_float = true;
                    acc_f = acc_i as f64;
                }
                acc_f *= *f;
            }
            _ => {
                return Err(EvalError::TypeError {
                    expected: "number",
                    got: a.type_name(),
                })
            }
        }
    }

    Ok(if is_float {
        Value::Float(acc_f)
    } else {
        Value::Int(acc_i)
    })
}

fn builtin_div(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    let (a, b, _are_ints) = promote(&args[0], &args[1])?;
    if b == 0.0 {
        return Err(EvalError::DivisionByZero);
    }
    Ok(Value::Float(a / b))
}

fn builtin_int_div(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    let Value::Int(a) = args[0] else {
        return Err(EvalError::TypeError {
            expected: "int",
            got: args[0].type_name(),
        });
    };
    let Value::Int(b) = args[1] else {
        return Err(EvalError::TypeError {
            expected: "int",
            got: args[1].type_name(),
        });
    };
    if b == 0 {
        return Err(EvalError::DivisionByZero);
    }
    Ok(Value::Int(a / b))
}

fn builtin_pow(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => {
            if *b < 0 {
                return Ok(Value::Float((*a as f64).powf(*b as f64)));
            }
            Ok(Value::Int(a.saturating_pow(*b as u32)))
        }
        _ => {
            let (a, b, _) = promote(&args[0], &args[1])?;
            Ok(Value::Float(a.powf(b)))
        }
    }
}

fn builtin_mod(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    let Value::Int(a) = args[0] else {
        return Err(EvalError::TypeError {
            expected: "int",
            got: args[0].type_name(),
        });
    };
    let Value::Int(b) = args[1] else {
        return Err(EvalError::TypeError {
            expected: "int",
            got: args[1].type_name(),
        });
    };
    if b == 0 {
        return Err(EvalError::DivisionByZero);
    }
    Ok(Value::Int(a % b))
}

fn builtin_brt(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    let (a, b, _) = promote(&args[0], &args[1])?;
    if b == 0.0 {
        return Err(EvalError::DivisionByZero);
    }
    Ok(Value::Float(a.powf(1.0 / b)))
}

fn builtin_max(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int((*a).max(*b))),
        _ => {
            let (a, b, _) = promote(&args[0], &args[1])?;
            Ok(Value::Float(a.max(b)))
        }
    }
}

fn builtin_min(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int((*a).min(*b))),
        _ => {
            let (a, b, _) = promote(&args[0], &args[1])?;
            Ok(Value::Float(a.min(b)))
        }
    }
}

fn num_cmp<F>(args: &[Value], op: F) -> Result<Value, EvalError>
where
    F: Fn(f64, f64) -> bool,
{
    expect_arity(args, 2)?;
    let (a, b, _) = promote(&args[0], &args[1])?;
    Ok(Value::Bool(op(a, b)))
}

fn builtin_gt(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    num_cmp(args, |a, b| a > b)
}
fn builtin_lt(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    num_cmp(args, |a, b| a < b)
}
fn builtin_gte(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    num_cmp(args, |a, b| a >= b)
}
fn builtin_lte(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    num_cmp(args, |a, b| a <= b)
}

fn value_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Int(x), Value::Int(y)) => x == y,
        (Value::Float(x), Value::Float(y)) => x == y,
        (Value::Int(x), Value::Float(y)) => (*x as f64) == *y,
        (Value::Float(x), Value::Int(y)) => *x == (*y as f64),
        (Value::Char(x), Value::Char(y)) => x == y,
        (Value::String(x), Value::String(y)) => x == y,
        (Value::Keyword(x), Value::Keyword(y)) => x == y,
        (Value::Symbol(x), Value::Symbol(y)) => x == y,
        (Value::List(x), Value::List(y)) => x.len() == y.len() && x.iter().zip(y).all(|(a, b)| value_eq(a, b)),
        (Value::Vector(x), Value::Vector(y)) => x.len() == y.len() && x.iter().zip(y).all(|(a, b)| value_eq(a, b)),
        (Value::Set(x), Value::Set(y)) => x.len() == y.len() && x.iter().zip(y).all(|(a, b)| value_eq(a, b)),
        (Value::Map(x), Value::Map(y)) => {
            x.len() == y.len()
                && x.iter()
                    .zip(y)
                    .all(|((ka, va), (kb, vb))| value_eq(ka, kb) && value_eq(va, vb))
        }
        _ => false,
    }
}

fn builtin_eq(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    Ok(Value::Bool(value_eq(&args[0], &args[1])))
}

fn builtin_neq(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    Ok(Value::Bool(!value_eq(&args[0], &args[1])))
}

fn builtin_and(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    let Value::Bool(a) = args[0] else {
        return Err(EvalError::TypeError {
            expected: "bool",
            got: args[0].type_name(),
        });
    };
    let Value::Bool(b) = args[1] else {
        return Err(EvalError::TypeError {
            expected: "bool",
            got: args[1].type_name(),
        });
    };
    Ok(Value::Bool(a && b))
}

fn builtin_or(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    let Value::Bool(a) = args[0] else {
        return Err(EvalError::TypeError {
            expected: "bool",
            got: args[0].type_name(),
        });
    };
    let Value::Bool(b) = args[1] else {
        return Err(EvalError::TypeError {
            expected: "bool",
            got: args[1].type_name(),
        });
    };
    Ok(Value::Bool(a || b))
}

fn builtin_readln(_args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    // Read a single line from stdin (trim trailing newline).
    let mut input = String::new();
    let mut stdin = io::stdin().lock();
    stdin
        .read_line(&mut input)
        .map_err(|e| EvalError::Custom(format!("readln failed: {e}")))?;
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    Ok(Value::String(input))
}

fn builtin_format(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    if args.len() == 1 {
        if let Value::Vector(v) = &args[0] {
            let mut out = String::new();
            for item in v {
                out.push_str(&string_for_io(item));
            }
            return Ok(Value::String(out));
        }
    }
    let mut out = String::new();
    for a in args {
        out.push_str(&string_for_io(a));
    }
    Ok(Value::String(out))
}

fn builtin_print(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    let Value::String(s) = builtin_format(args, _env)? else {
        unreachable!("format returns string");
    };
    print!("{}", s);
    io::stdout()
        .flush()
        .map_err(|e| EvalError::Custom(format!("stdout flush failed: {e}")))?;
    Ok(Value::Nil)
}

fn builtin_println(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    let Value::String(s) = builtin_format(args, _env)? else {
        unreachable!("format returns string");
    };
    println!("{}", s);
    Ok(Value::Nil)
}

fn builtin_parse_int(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 1)?;
    let Value::String(s) = &args[0] else {
        return Err(EvalError::TypeError {
            expected: "string",
            got: args[0].type_name(),
        });
    };
    let n: i64 = s
        .parse()
        .map_err(|e| EvalError::Custom(format!("parse-int failed: {e}")))?;
    Ok(Value::Int(n))
}

fn builtin_parse_float(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 1)?;
    let Value::String(s) = &args[0] else {
        return Err(EvalError::TypeError {
            expected: "string",
            got: args[0].type_name(),
        });
    };
    let n: f64 = s
        .parse()
        .map_err(|e| EvalError::Custom(format!("parse-float failed: {e}")))?;
    Ok(Value::Float(n))
}

fn builtin_concat(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    let Value::Vector(a) = &args[0] else {
        return Err(EvalError::TypeError {
            expected: "vector",
            got: args[0].type_name(),
        });
    };
    let Value::Vector(b) = &args[1] else {
        return Err(EvalError::TypeError {
            expected: "vector",
            got: args[1].type_name(),
        });
    };
    let mut out = Vec::with_capacity(a.len() + b.len());
    out.extend_from_slice(a);
    out.extend_from_slice(b);
    Ok(Value::Vector(out))
}

fn builtin_append(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    let elem = args[0].clone();
    let Value::Vector(v) = &args[1] else {
        return Err(EvalError::TypeError {
            expected: "vector",
            got: args[1].type_name(),
        });
    };
    let mut out = Vec::with_capacity(v.len() + 1);
    out.push(elem);
    out.extend_from_slice(v);
    Ok(Value::Vector(out))
}

fn builtin_prepend(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    let elem = args[0].clone();
    let Value::Vector(v) = &args[1] else {
        return Err(EvalError::TypeError {
            expected: "vector",
            got: args[1].type_name(),
        });
    };
    let mut out = Vec::with_capacity(v.len() + 1);
    out.extend_from_slice(v);
    out.push(elem);
    Ok(Value::Vector(out))
}

fn builtin_nth(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    let Value::Int(n) = args[0] else {
        return Err(EvalError::TypeError {
            expected: "int",
            got: args[0].type_name(),
        });
    };
    let Value::Vector(v) = &args[1] else {
        return Err(EvalError::TypeError {
            expected: "vector",
            got: args[1].type_name(),
        });
    };
    if n < 0 {
        return Err(EvalError::IndexOutOfBounds {
            index: 0,
            len: v.len(),
        });
    }
    let idx = n as usize;
    v.get(idx)
        .cloned()
        .ok_or(EvalError::IndexOutOfBounds { index: idx, len: v.len() })
}

fn builtin_map(args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 2)?;
    let f = args[0].clone();
    let Value::Vector(v) = &args[1] else {
        return Err(EvalError::TypeError {
            expected: "vector",
            got: args[1].type_name(),
        });
    };
    let mut out = Vec::with_capacity(v.len());
    for item in v {
        out.push(apply(&f, &[item.clone()], env)?);
    }
    Ok(Value::Vector(out))
}

fn builtin_reduce(args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 3)?;
    let f = args[0].clone();
    let mut acc = args[1].clone();
    let Value::Vector(v) = &args[2] else {
        return Err(EvalError::TypeError {
            expected: "vector",
            got: args[2].type_name(),
        });
    };
    for item in v {
        acc = apply(&f, &[acc, item.clone()], env)?;
    }
    Ok(acc)
}

fn builtin_scan(args: &[Value], env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    expect_arity(args, 3)?;
    let f = args[0].clone();
    let mut acc = args[1].clone();
    let Value::Vector(v) = &args[2] else {
        return Err(EvalError::TypeError {
            expected: "vector",
            got: args[2].type_name(),
        });
    };
    let mut out = Vec::with_capacity(v.len());
    for item in v {
        acc = apply(&f, &[acc, item.clone()], env)?;
        out.push(acc.clone());
    }
    Ok(Value::Vector(out))
}

fn builtin_assert(args: &[Value], _env: &Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    for a in args {
        if !a.is_truthy() {
            return Err(EvalError::Custom("assertion failed".to_string()));
        }
    }
    Ok(Value::Nil)
}

fn string_for_io(v: &Value) -> String {
    match v {
        // I/O-oriented stringification: strings are raw (no quotes, no escaping).
        Value::String(s) => s.clone(),
        _ => v.to_string(),
    }
}

