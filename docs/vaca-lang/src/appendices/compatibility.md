# Compatibility notes

This appendix summarizes differences between VEDN/Vaca and related ecosystems.

## EDN vs VEDN

VEDN is based on EDN with these notable constraints/extensions:

- **Typed dispatch is reserved**: `#T x` is used for typing, not arbitrary host tags.
  (A toolchain MAY support extra dispatches as extensions, but the standard meaning is typing.)
- **Typed dispatch is reserved**: `#T x` is used for typing, not arbitrary host tags.
  (A toolchain MAY support extra dispatches as extensions, but the standard meaning is typing.)
- `#_` discard is supported as in EDN.
- Sets use `%{...}` (Vaca-specific; EDN uses `#{...}`).

## Clojure vs Vaca

Vaca is Lisp-family, but it is not Clojure. Key differences include:

- **Truthiness**: Vaca treats `0`, `0.0`, empty strings, and empty collections as falsy.
- **Types**: Vaca has a first-class type system and type-directed `#T` syntax.
- **Module resolution**: Vaca uses library roots with `@` and relative module paths with `$` parent
  segments.

## Rust/Go/Python influence

The standard library aims for a breadth comparable to mainstream languages, while keeping APIs
functional and composable.

`result` and `option` are standardized in the type system and STL for explicit error handling,
similar in spirit to Rust.
