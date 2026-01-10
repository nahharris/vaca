//! Vaca programming language tools.
//!
//! This crate is meant to be used both as:
//! - an SDK (to parse and eventually run/compile Vaca programs), and
//! - a binary (`vaca`) for developer tooling.
//!
//! Currently it contains Vaca's frontend reader: a strict EDN parser.

pub mod vedn;

pub use vedn::{parse, Error, ErrorKind, Keyword, Kind, Node, Parser, Span, Str, Symbol, Typed};
