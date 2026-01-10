//! Vaca programming language tools.
//!
//! This crate is meant to be used both as:
//! - an SDK (to parse and eventually run/compile Vaca programs), and
//! - a binary (`vaca`) for developer tooling.
//!
//! Currently it contains Vaca's frontend reader: a strict EDN parser.

pub mod edn;

pub use edn::{parse, Error, ErrorKind, Keyword, Kind, Node, Parser, Span, Str, Symbol, Tagged};
