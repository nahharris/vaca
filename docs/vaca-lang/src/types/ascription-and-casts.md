# Type ascription and casts

Vaca uses the `#` dispatch syntax (`#T x`) for type-directed behavior.

This chapter specifies the normative semantics of typed dispatch in evaluation and in binding
positions.

## `#T x` in expression position

### Meaning

In expression position, `#T x` denotes a **type-directed cast**:

1. Evaluate `x` to a value `v`.
2. Convert and/or check `v` against type `T` according to the cast rules.
3. Produce the resulting value.

### Cast rules

The standard library defines a canonical cast operation `stl.core/cast` used by typed dispatch.

Normative requirements:

- If `v` is already an instance of `T`, the cast MUST succeed and MUST return a value equal to `v`.
- If `v` can be converted to `T` by a standard conversion, the cast MUST succeed and return the
  converted value.
- Otherwise, the cast MUST fail with a TypeError.

Standard conversions include at least:

- numeric widening/narrowing where defined (e.g. `int` ↔ `uint`, `int` ↔ `float`)
- identity conversions

Conversions that can lose information MUST be specified precisely (overflow behavior, rounding).

### Examples

```clojure
#uint 1         ; converts int 1 into uint 1
#string 42      ; error unless a numeric-to-string conversion is specified
#(vec int) [1 2 3]
```

## Typed bindings

Typed dispatch MAY appear in binding positions by applying `#T` to a symbol.

### Typed function parameters

```clojure
(defn f [#int x] ...)
```

Normative behavior:

- When `f` is called, the argument value is cast to `int` (per `#T x` cast rules) and the result is
  bound to `x` within `f`.

### Typed return position

```clojure
(defn #int f [x] ...)
```

Normative behavior:

- The function result is cast to `int` before it is returned to the caller.

### Typed top-level definitions

```clojure
(def #string name "Vaca")
```

Normative behavior:

- The initializer is evaluated and cast to `string`, then bound.

## Type forms are not evaluated as runtime values

Type expressions (`T` in `#T x`) are interpreted in the type system domain. A conforming
implementation MUST NOT treat `T` as an ordinary runtime value expression.

This permits compilers to type-check without requiring runtime “type objects” for all types.

## Errors

If a cast fails, it MUST raise a TypeError with source span pointing to the typed form.
