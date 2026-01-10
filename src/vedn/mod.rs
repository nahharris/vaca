//! Strict EDN reader used by Vaca's frontend.
//!
//! This module implements a streaming parser for the full EDN specification.
//! It intentionally does **not** attach semantics to tagged elements.
//!
//! # Vaca extension: tags as types
//! In Vaca, EDN tagged elements (e.g. `#int 1`, `#inst "..."`) are used as a
//! syntax to express typing. The frontend reader therefore parses tags into a
//! generic tagged node and leaves all semantic interpretation to later stages
//! (type checking, macro expansion, runtime, STL).
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
pub use value::{Keyword, Kind, Node, Number, NumberSuffix, Str, Symbol, Tagged};
