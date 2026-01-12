use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::bezerro::env::Env;
use crate::bezerro::error::EvalError;

pub type BuiltinFn = fn(&[Value], &Rc<RefCell<Env>>) -> Result<Value, EvalError>;

#[derive(Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
    Keyword(String),
    Symbol(String),
    List(Vec<Value>),
    Vector(Vec<Value>),
    Map(Rc<HashMap<Value, Value>>),
    Set(Rc<HashSet<Value>>),
    Recur(Vec<Value>),
    Builtin {
        name: &'static str,
        func: BuiltinFn,
    },
    Lambda {
        params: Vec<String>,
        body: Vec<Value>,
        env: Rc<RefCell<Env>>,
    },
    Macro {
        params: Vec<String>,
        body: Vec<Value>,
        env: Rc<RefCell<Env>>,
    },
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Bool(_) => "bool",
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Char(_) => "char",
            Value::String(_) => "string",
            Value::Keyword(_) => "keyword",
            Value::Symbol(_) => "symbol",
            Value::List(_) => "list",
            Value::Vector(_) => "vector",
            Value::Map(_) => "map",
            Value::Set(_) => "set",
            Value::Recur(_) => "recur",
            Value::Builtin { .. } => "builtin",
            Value::Lambda { .. } => "lambda",
            Value::Macro { .. } => "macro",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(false) => false,
            Value::Int(0) => false,
            Value::Float(f) if *f == 0.0 => false, // includes -0.0
            Value::Char('\0') => false,
            Value::String(s) if s.is_empty() => false,
            Value::List(v) if v.is_empty() => false,
            Value::Vector(v) if v.is_empty() => false,
            Value::Map(v) if v.is_empty() => false,
            Value::Set(v) if v.is_empty() => false,
            _ => true,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => float_eq(*a, *b),
            (Value::Int(a), Value::Float(b)) => float_eq(*a as f64, *b),
            (Value::Float(a), Value::Int(b)) => float_eq(*a, *b as f64),
            (Value::Char(a), Value::Char(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Keyword(a), Value::Keyword(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Vector(a), Value::Vector(b)) => a == b,
            (Value::Set(a), Value::Set(b)) => a.as_ref() == b.as_ref(),
            (Value::Map(a), Value::Map(b)) => a.as_ref() == b.as_ref(),
            (Value::Recur(a), Value::Recur(b)) => a == b,
            (Value::Builtin { name: a, func: af }, Value::Builtin { name: b, func: bf }) => {
                a == b && (*af as usize) == (*bf as usize)
            }
            (
                Value::Lambda {
                    params: ap,
                    body: ab,
                    env: ae,
                },
                Value::Lambda {
                    params: bp,
                    body: bb,
                    env: be,
                },
            ) => ap == bp && ab == bb && Rc::ptr_eq(ae, be),
            (
                Value::Macro {
                    params: ap,
                    body: ab,
                    env: ae,
                },
                Value::Macro {
                    params: bp,
                    body: bb,
                    env: be,
                },
            ) => ap == bp && ab == bb && Rc::ptr_eq(ae, be),
            _ => false,
        }
    }
}

impl Eq for Value {}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use std::collections::hash_map::DefaultHasher;

        // Hash the variant discriminant first so distinct variants don't collide easily.
        std::mem::discriminant(self).hash(state);

        match self {
            Value::Nil => {}
            Value::Bool(b) => b.hash(state),
            Value::Int(i) => i.hash(state),
            Value::Float(f) => float_hash(*f).hash(state),
            Value::Char(c) => c.hash(state),
            Value::String(s) => s.hash(state),
            Value::Keyword(k) => k.hash(state),
            Value::Symbol(s) => s.hash(state),
            Value::List(items) | Value::Vector(items) | Value::Recur(items) => {
                items.len().hash(state);
                for item in items {
                    item.hash(state);
                }
            }
            Value::Set(items) => {
                items.len().hash(state);

                // Order-independent hashing: combine element hashes commutatively.
                let mut acc: u64 = 0;
                for item in items.as_ref() {
                    let mut h = DefaultHasher::new();
                    item.hash(&mut h);
                    acc ^= h.finish();
                }
                acc.hash(state);
            }
            Value::Map(entries) => {
                entries.len().hash(state);

                // Order-independent hashing: combine entry hashes commutatively.
                let mut acc: u64 = 0;
                for (k, v) in entries.as_ref() {
                    let mut h = DefaultHasher::new();
                    k.hash(&mut h);
                    v.hash(&mut h);
                    acc ^= h.finish();
                }
                acc.hash(state);
            }
            Value::Builtin { name, func } => {
                name.hash(state);
                (*func as usize).hash(state);
            }
            Value::Lambda { params, body, env } => {
                params.hash(state);
                body.hash(state);
                Rc::as_ptr(env).hash(state);
            }
            Value::Macro { params, body, env } => {
                params.hash(state);
                body.hash(state);
                Rc::as_ptr(env).hash(state);
            }
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => {
                if n.is_nan() {
                    write!(f, "NaN")
                } else if n.is_infinite() && n.is_sign_positive() {
                    write!(f, "Infinity")
                } else if n.is_infinite() && n.is_sign_negative() {
                    write!(f, "-Infinity")
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Char(c) => write!(f, "\\{}", c),
            Value::String(s) => write!(f, "\"{}\"", escape_string(s)),
            Value::Keyword(k) => write!(f, ":{k}"),
            Value::Symbol(s) => write!(f, "{s}"),
            Value::List(items) => {
                write!(f, "(")?;
                write_joined(f, items)?;
                write!(f, ")")
            }
            Value::Vector(items) => {
                write!(f, "[")?;
                write_joined(f, items)?;
                write!(f, "]")
            }
            Value::Map(entries) => {
                write!(f, "{{")?;
                let mut items: Vec<_> = entries.iter().collect();
                items.sort_by(|(ka, _), (kb, _)| ka.to_string().cmp(&kb.to_string()));
                for (i, (k, v)) in items.into_iter().enumerate() {
                    if i != 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{} {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Set(items) => {
                write!(f, "#{{")?;
                let mut vec: Vec<_> = items.iter().collect();
                vec.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
                for (i, item) in vec.into_iter().enumerate() {
                    if i != 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "}}")
            }
            Value::Recur(_) => write!(f, "#<recur>"),
            Value::Builtin { name, .. } => write!(f, "#<builtin {name}>"),
            Value::Lambda { params, .. } => write!(f, "#<fn ({})>", params.join(" ")),
            Value::Macro { params, .. } => write!(f, "#<macro ({})>", params.join(" ")),
        }
    }
}

fn write_joined(f: &mut fmt::Formatter<'_>, items: &[Value]) -> fmt::Result {
    for (i, item) in items.iter().enumerate() {
        if i != 0 {
            write!(f, " ")?;
        }
        write!(f, "{}", item)?;
    }
    Ok(())
}

fn escape_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out
}

fn float_hash(f: f64) -> u64 {
    // Ensure hashing is consistent with equality:
    // - treat +0.0 and -0.0 as equal
    // - canonicalize NaNs (so NaN == NaN and hashes match)
    if f == 0.0 {
        return 0.0f64.to_bits();
    }
    if f.is_nan() {
        return f64::NAN.to_bits();
    }
    f.to_bits()
}

fn float_eq(a: f64, b: f64) -> bool {
    float_hash(a) == float_hash(b)
}
