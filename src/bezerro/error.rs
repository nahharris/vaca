use std::fmt;

#[derive(Debug, Clone)]
pub enum EvalError {
    UndefinedSymbol(String),
    TypeError {
        expected: &'static str,
        got: &'static str,
    },
    ArityError {
        expected: usize,
        got: usize,
    },
    DivisionByZero,
    IndexOutOfBounds {
        index: usize,
        len: usize,
    },
    NotCallable(&'static str),
    ParseError(String),
    Custom(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::UndefinedSymbol(s) => write!(f, "undefined symbol: {s}"),
            EvalError::TypeError { expected, got } => {
                write!(f, "type error: expected {expected}, got {got}")
            }
            EvalError::ArityError { expected, got } => {
                write!(f, "arity error: expected {expected}, got {got}")
            }
            EvalError::DivisionByZero => write!(f, "division by zero"),
            EvalError::IndexOutOfBounds { index, len } => {
                write!(f, "index out of bounds: {index} (len {len})")
            }
            EvalError::NotCallable(got) => write!(f, "value is not callable: {got}"),
            EvalError::ParseError(s) => write!(f, "parse error: {s}"),
            EvalError::Custom(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for EvalError {}

