//! Vaca Extensible Data Notation (VEDN) reader module, derived from [EDN](https://github.com/edn-format/edn/blob/master/README.md)
//!
//! This module implements a streaming parser based on the EDN specification.
//! The only difference is the presence of annotated elements.
//!
//! # Vaca extension: annotated forms
//! In Vaca, EDN's `#` dispatch (e.g. `#int 1`, `#inst "..."`) is used as a
//! general annotation mechanism. The frontend reader therefore parses annotated
//! forms into a generic node and leaves all semantic interpretation to later
//! stages (type checking, macro expansion, runtime, STL).
//!
//! # API
//! Use [`parse`] to parse an input string into a sequence of EDN nodes.
//!
//! The public AST types are in [`value`], and errors/spans are in [`error`].

pub mod cursor;
pub mod error;
pub mod parser;
pub mod value;

pub use error::{Error, ErrorKind, Span};
pub use parser::{parse, Parser};
pub use value::{Keyword, Kind, Node, Number, NumberSuffix, Str, Symbol};
