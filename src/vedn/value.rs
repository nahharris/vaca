use std::borrow::Cow;

use super::Span;

/// A parsed EDN element with an associated source span.
///
/// EDN has no explicit "root" delimiter, so parsing a document yields a
/// sequence of [`Node`]s.
///
/// The `span` field is byte offsets into the original input buffer and can be
/// used by later compilation stages (macro expansion, type checking, runtime)
/// to produce high-quality diagnostics.
#[derive(Debug, Clone, PartialEq)]
pub struct Node<'a> {
    /// Byte span of the syntactic element within the input.
    pub span: Span,
    /// The element kind.
    pub kind: Kind<'a>,
}

impl<'a> Node<'a> {
    /// Constructs a new node.
    pub fn new(span: Span, kind: Kind<'a>) -> Self {
        Node { span, kind }
    }
}

/// EDN value kinds.
///
/// This enum is intentionally focused on syntactic structure.
///
/// The `#` dispatch is used by Vaca as typing syntax; typed elements
/// (`#<type> <form>`) are represented as [`Kind::Typed`].
#[derive(Debug, Clone, PartialEq)]
pub enum Kind<'a> {
    /// The `nil` value.
    Nil,
    /// A boolean literal: `true` or `false`.
    Bool(bool),
    /// A character literal, e.g. `\c` or `\newline`.
    Char(char),
    /// A string literal, e.g. `"hello"`.
    String(Str<'a>),
    /// A symbol (identifier), e.g. `foo` or `my.ns/foo`.
    Symbol(Symbol<'a>),
    /// A keyword, e.g. `:foo` or `:my.ns/foo`.
    Keyword(Keyword<'a>),
    /// An integer or floating point number, optionally suffixed.
    Number(Number<'a>),
    /// A list: `(<value>...)`.
    List(Vec<Node<'a>>),
    /// A vector: `[<value>...]`.
    Vector(Vec<Node<'a>>),
    /// A map: `{<key> <value> ...}`.
    Map(Vec<(Node<'a>, Node<'a>)>),
    /// A set: `#{<value>...}`.
    Set(Vec<Node<'a>>),
    /// A typed element: `#<type> <value>`.
    Typed(Typed<'a>),
}

/// A parsed EDN string literal.
///
/// To keep parsing fast and allocation-light, strings are represented in one of
/// two ways:
///
/// - [`Str::Borrowed`]: the string contained no escapes and therefore can be
///   returned as a direct slice of the input.
/// - [`Str::Owned`]: the string contained escapes (e.g. `\n`, `\uNNNN`) and has
///   been unescaped into a newly allocated buffer.
#[derive(Debug, Clone, PartialEq)]
pub enum Str<'a> {
    /// A direct slice into the input buffer.
    Borrowed(&'a str),
    /// An unescaped string buffer.
    Owned(String),
}

impl<'a> Str<'a> {
    /// Returns the string contents.
    pub fn as_str(&self) -> &str {
        match self {
            Str::Borrowed(s) => s,
            Str::Owned(s) => s,
        }
    }
}

/// An EDN symbol.
///
/// Symbols are used to represent identifiers. They can optionally include a
/// namespace/prefix separated by `/`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Symbol<'a> {
    /// Full symbol text as it appeared in the input.
    pub raw: &'a str,
    /// Optional namespace/prefix component, e.g. `my.ns` in `my.ns/foo`.
    pub namespace: Option<&'a str>,
    /// Name component, e.g. `foo` in `my.ns/foo`.
    pub name: &'a str,
}

/// An EDN keyword.
///
/// Keywords are identifiers which typically designate themselves, and always
/// begin with `:`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Keyword<'a> {
    /// Full keyword text as it appeared in the input (including the leading `:`).
    pub raw: &'a str,
    /// Optional namespace/prefix component.
    pub namespace: Option<&'a str>,
    /// Name component.
    pub name: &'a str,
}

/// A typed EDN element: `#<type> <value>`.
///
/// EDN's `#` dispatch is originally meant for tagged elements.
///
/// Vaca uses this syntax specifically for typing:
///
/// - `#int 1` means "the value `1` of type `int`".
/// - `#inst "..."` means "the string literal typed as `inst`".
///
/// The reader does not attach meaning to types. It only preserves structure.
#[derive(Debug, Clone, PartialEq)]
pub struct Typed<'a> {
    /// The type form.
    ///
    /// Historically this was just a symbol (`#int 1`). Vaca also allows a
    /// parameterized type using a list whose first element is a symbol
    /// (`#(vec int) value`).
    pub ty: Box<Node<'a>>,
    /// The typed value.
    pub value: Box<Node<'a>>,
}

/// Numeric suffix.
///
/// EDN supports suffixes to express desired precision.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum NumberSuffix {
    /// No suffix.
    None,
    /// `N` suffix: arbitrary precision integer.
    BigInt,
    /// `M` suffix: exact precision floating point / decimal.
    BigDecimal,
}

/// EDN number.
///
/// Numbers are stored as borrowed lexemes to keep parsing fast and preserve the
/// original representation. Later compilation stages can decide how to interpret
/// numeric values and whether to eagerly parse them.
#[derive(Debug, Clone, PartialEq)]
pub enum Number<'a> {
    /// An integer literal.
    Int {
        /// Full numeric text as it appeared in the input (including sign/suffix).
        lexeme: &'a str,
        /// Precision suffix.
        suffix: NumberSuffix,
    },
    /// A floating point literal.
    Float {
        /// Full numeric text as it appeared in the input (including sign/suffix).
        lexeme: &'a str,
        /// Precision suffix.
        suffix: NumberSuffix,
    },
}

impl<'a> Number<'a> {
    /// Returns the original numeric text.
    pub fn lexeme(&self) -> &'a str {
        match self {
            Number::Int { lexeme, .. } => lexeme,
            Number::Float { lexeme, .. } => lexeme,
        }
    }

    /// Returns the precision suffix.
    pub fn suffix(&self) -> NumberSuffix {
        match self {
            Number::Int { suffix, .. } => *suffix,
            Number::Float { suffix, .. } => *suffix,
        }
    }

    /// Returns the lexeme as a borrowed [`Cow<str>`].
    pub fn as_cow_str(&self) -> Cow<'a, str> {
        Cow::Borrowed(self.lexeme())
    }
}
