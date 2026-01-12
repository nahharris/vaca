# Standard library overview

The Vaca Standard Library (STL) is a normative set of modules available to all conforming
implementations.

The STL is designed to feel familiar to users of languages like Clojure, Rust, Go, and Python,
while remaining consistent with Vaca’s small-core, functional-first philosophy.

## Organization

STL modules are typically imported as library modules:

```clojure
(use @stl.io [println])
(use @stl.seq [map filter reduce])
```

This book documents the STL by module.

## Conventions

- **Pure by default**: most functions are pure and return new values rather than mutating inputs.
- **Total where practical**: APIs prefer returning `option`/`result` over throwing errors, except
  for programmer errors (e.g. index out of bounds) where the behavior is explicitly specified.
- **Truthiness-aware control**: boolean-like combinators use Vaca truthiness (see “Truthiness”).

## Naming

Functions use `kebab-case` by convention:

- `parse-int`
- `readln`
- `unwrap-or`

Predicates typically end with `?`:

- `empty?`
- `some?`

## Common types

The following types appear throughout the STL:

- `option`, `result`
- `vec`, `list`, `map`, `set`
- `string`, `keyword`, `symbol`
