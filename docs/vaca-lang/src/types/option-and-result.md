# Option and Result

Vaca standardizes two algebraic types for absence and error handling:

- `option`
- `result`

These types are part of the normative standard library and are also referenced by the type system.

## `option`

### Type

`(option T)` is the type of optional values of type `T`.

### Variants

`option` has two variants:

- `none`: no value present
- `some`: a value present

### Constructors (standard)

The standard library MUST provide:

- `(option.none)` producing a `none`
- `(option.some x)` producing a `some` containing `x`

### Semantics

- `(option.some x)` contains a value of type `T` (after applying cast rules if a typed signature is
  used).
- `(option.none)` contains no value.

`option` is intended to replace sentinel values in APIs.

## `result`

### Type

`(result T E)` is the type of computations that either:

- succeed with a value of type `T`, or
- fail with an error value of type `E`.

### Variants

`result` has two variants:

- `ok`: success
- `err`: failure

### Constructors (standard)

The standard library MUST provide:

- `(result.ok x)` producing an `ok` containing `x`
- `(result.err e)` producing an `err` containing `e`

### Semantics

`result` is the standard way to represent recoverable failures without exceptions.

## Pattern matching and combinators

The standard library MUST provide combinators in `stl.option` and `stl.result` such as:

- `map`, `and-then`/`flat-map`
- `unwrap`, `unwrap-or`, `unwrap-or-else`
- `ok?`, `err?`, `some?`, `none?`
