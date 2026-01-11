use std::cell::RefCell;
use std::fmt;
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
    Map(Vec<(Value, Value)>),
    Set(Vec<Value>),
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
                for (i, (k, v)) in entries.iter().enumerate() {
                    if i != 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{} {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Set(items) => {
                write!(f, "#{{")?;
                write_joined(f, items)?;
                write!(f, "}}")
            }
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
