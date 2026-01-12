# Hygiene and conventions

This chapter specifies Vaca’s macro hygiene model and the conventions used to write robust macros.

## Hygiene model

Macros in Vaca are **not hygienic by default**.

Normative consequences:

- Symbols introduced by a macro expansion resolve according to the caller’s environment (unless the
  macro uses explicit qualification or gensym-based techniques).
- A macro can accidentally capture caller bindings, and caller code can accidentally capture macro
  introduced bindings, unless the macro is written defensively.

Implementations MUST preserve this resolution model.

## Gensym

To avoid name capture, the standard library MUST provide a `gensym` facility in `stl.macro` that
creates a fresh, unique symbol.

Normative properties of `gensym`:

- Each call returns a symbol that is not `==` to any symbol returned by any other `gensym` call in
  the same compilation run.
- The printed representation MAY include an opaque suffix; programs MUST NOT rely on that suffix’s
  format.

## Qualified names

Macros SHOULD introduce names using module-qualified symbols where possible, for example:

```clojure
; Prefer using @stl.seq/map rather than an unqualified `map`
```

The module system defines how qualified symbols are resolved.

## `macroexpand` tooling

The standard library SHOULD provide:

- `macroexpand-1` (expand one step)
- `macroexpand` (expand repeatedly)

These tools are essential for debugging and understanding macro behavior.
