use std::fmt;

/// A byte span inside the source input.
///
/// `Span` uses byte offsets into the original input string.
///
/// For a `&str` input, offsets are guaranteed to land on UTF-8 boundaries
/// because the parser only slices at ASCII delimiter positions.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
pub struct Span {
    /// Inclusive start byte offset.
    pub start: usize,
    /// Exclusive end byte offset.
    pub end: usize,
}

impl Span {
    /// Creates a new [`Span`].
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

/// Parser error kinds.
///
/// The goal of these variants is to be specific enough to help tooling and to
/// produce friendly diagnostics, while remaining stable and easy to extend.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ErrorKind {
    /// Input ended unexpectedly.
    UnexpectedEof,
    /// The parser encountered an unexpected character.
    UnexpectedChar {
        /// The character that was found.
        found: char,
        /// A short description of what was expected.
        expected: &'static str,
    },
    /// An unknown or malformed `#` dispatch sequence.
    InvalidDispatch,
    /// A symbol failed EDN strict validation.
    InvalidSymbol,
    /// A backtick-quoted symbol/keyword wasn't terminated by a closing `` ` ``.
    UnterminatedSymbol,
    /// A keyword failed EDN strict validation.
    InvalidKeyword,
    /// A number token failed EDN strict validation.
    InvalidNumber,
    /// A string literal wasn't terminated by a closing `"`.
    UnterminatedString,
    /// A collection wasn't terminated by its matching delimiter.
    UnterminatedCollection {
        /// The delimiter that was expected to close the collection.
        expected: char,
    },
    /// Maps must contain an even number of forms (key/value pairs).
    MapOddNumberOfForms,
    /// A character literal did not match EDN's character literal rules.
    InvalidCharacterLiteral,
    /// A `\uNNNN` escape was malformed or out of range.
    InvalidUnicodeEscape,
}

/// A parsing error with source location.
///
/// `line` and `column` are 1-based.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    /// The error category.
    pub kind: ErrorKind,
    /// The span best associated with this error.
    pub span: Span,
    /// The line number at the point the error was detected (1-based).
    pub line: u32,
    /// The column number at the point the error was detected (1-based).
    pub column: u32,
}

impl Error {
    /// Creates a new parsing error.
    pub fn new(kind: ErrorKind, span: Span, line: u32, column: u32) -> Self {
        Error {
            kind,
            span,
            line,
            column,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} at {}:{} ({}..{})",
            self.kind, self.line, self.column, self.span.start, self.span.end
        )
    }
}

impl std::error::Error for Error {}
