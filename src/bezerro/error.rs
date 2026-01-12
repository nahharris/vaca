use std::fmt;

#[derive(Debug, Clone)]
pub enum UseError {
    BadArity {
        got: usize,
    },
    ExpectedModuleSymbol {
        got: &'static str,
    },
    ExpectedImportVector {
        got: &'static str,
    },
    ExpectedImportSymbol {
        got: &'static str,
    },
    ExpectedAliasSymbol {
        got: &'static str,
    },
    MissingExport {
        module: String,
        symbol: String,
    },
    NameCollision {
        name: String,
    },
    EmptyModulePath,
    LastSegmentCannotBeSuper,
    SuperBeyondRoot {
        module: String,
    },
    FailedToDetermineBaseDir,
    ResolveFailed {
        path: String,
        error: String,
    },
    ReadFailed {
        path: String,
        error: String,
    },
    CyclicUse {
        path: String,
    },
    InvalidExportForm {
        head: String,
    },
    Internal {
        message: String,
    },
}

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
    StackOverflow {
        limit: usize,
    },
    DivisionByZero,
    IndexOutOfBounds {
        index: usize,
        len: usize,
    },
    NotCallable(&'static str),
    ParseError(String),
    Use(UseError),
    Custom(String),
}

impl fmt::Display for UseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UseError::BadArity { got } => write!(
                f,
                "use expects: (use path.to.file) or (use path.to.file [symbols...]) (got {got} args)"
            ),
            UseError::ExpectedModuleSymbol { got } => {
                write!(f, "use: expected module path symbol, got {got}")
            }
            UseError::ExpectedImportVector { got } => {
                write!(f, "use: expected import list vector, got {got}")
            }
            UseError::ExpectedImportSymbol { got } => {
                write!(f, "use: expected symbol in import list, got {got}")
            }
            UseError::ExpectedAliasSymbol { got } => {
                write!(f, "use: expected alias symbol after :as, got {got}")
            }
            UseError::MissingExport { module, symbol } => {
                write!(f, "use: symbol `{symbol}` is not exported by `{module}`")
            }
            UseError::NameCollision { name } => {
                write!(f, "use: name collision: `{name}` already defined")
            }
            UseError::EmptyModulePath => write!(f, "use: empty module path"),
            UseError::LastSegmentCannotBeSuper => {
                write!(f, "use: last path segment cannot be `super`")
            }
            UseError::SuperBeyondRoot { module } => {
                write!(f, "use: `super` goes beyond filesystem root in `{module}`")
            }
            UseError::FailedToDetermineBaseDir => {
                write!(f, "use: failed to determine base directory")
            }
            UseError::ResolveFailed { path, error } => {
                write!(f, "use: failed to resolve module path `{path}`: {error}")
            }
            UseError::ReadFailed { path, error } => {
                write!(f, "use: failed to read module `{path}`: {error}")
            }
            UseError::CyclicUse { path } => write!(f, "use: cyclic use detected while loading `{path}`"),
            UseError::InvalidExportForm { head } => {
                write!(f, "use: expected symbol name in ({head} name ...)")
            }
            UseError::Internal { message } => write!(f, "use: internal error: {message}"),
        }
    }
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
            EvalError::StackOverflow { limit } => {
                write!(f, "stack overflow: depth exceeded {limit}")
            }
            EvalError::DivisionByZero => write!(f, "division by zero"),
            EvalError::IndexOutOfBounds { index, len } => {
                write!(f, "index out of bounds: {index} (len {len})")
            }
            EvalError::NotCallable(got) => write!(f, "value is not callable: {got}"),
            EvalError::ParseError(s) => write!(f, "parse error: {s}"),
            EvalError::Use(e) => write!(f, "{e}"),
            EvalError::Custom(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for EvalError {}

