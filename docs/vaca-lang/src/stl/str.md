# `stl.str`

`stl.str` defines operations on strings and characters.

Strings are immutable Unicode text.

## Core operations

- `(len s)` length in characters (or code units—MUST be specified)
- `(empty? s)`
- `(concat s1 s2 ...)` string concatenation
- `(split s sep)` split into vector of strings
- `(join sep xs)` join a sequence of strings
- `(trim s)`, `(trim-start s)`, `(trim-end s)`
- `(to-upper s)`, `(to-lower s)`

## Parsing and formatting

- `(parse-int s)` parse integer (returns `result int ParseError` preferred)
- `(parse-float s)` parse float
- `(format x1 x2 ... xN)` convert values to string and concatenate (or format with templates—MUST be specified)

## Unicode and indexing

If the string model uses Unicode code points, indexing and slicing MUST specify:

- whether indices are code point indices or byte indices
- error behavior on invalid boundaries
