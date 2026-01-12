# `stl.core`

`stl.core` defines the fundamental functions, predicates, macros, and types that underpin most
Vaca programs.

## Fundamental macros

The following macros are part of the prelude surface and are specified in “The Standard Prelude”:

- `def`, `defn`, `fn`
- `let`, `if`, `do`
- `quote`
- `|>`
- `loop`, `recur`
- `defmacro`

This module additionally defines core convenience macros.

### `(& a b ...)` (short-circuit and)

`&` is a macro that evaluates its arguments left-to-right and returns:

- the first falsy value, if any
- otherwise the last argument value

If no arguments are provided, `&` returns `true`.

### `(| a b ...)` (short-circuit or)

`|` is a macro that evaluates its arguments left-to-right and returns:

- the first truthy value, if any
- otherwise the last argument value

If no arguments are provided, `|` returns `false`.

### `(assert x1 x2 ... xN)`

`assert` evaluates each argument left-to-right and fails with an AssertionError if any argument is
falsy.

The error MUST report the failing span and SHOULD include the original asserted form.

## Equality

### `(== a b)`

Returns `true` if `a` and `b` are equal under Vaca equality (see “Equality and hashing”).

### `(!= a b)`

Logical negation of `==`.

## Numeric operations

This section specifies the canonical arithmetic functions. Numeric tower details live in
`stl.math`, but these are always available.

- `(+ x1 ... xN)` addition (identity: `0`)
- `(* x1 ... xN)` multiplication (identity: `1`)
- `(- x)` negation
- `(- x y)` subtraction
- `(/ x y)` division (errors on division by zero unless otherwise specified)
- `(// x y)` integer division
- `(^ x y)` exponentiation
- `(mod x y)` remainder
- `(max x y)` maximum
- `(min x y)` minimum
- `(brt x y)` y-th root of x

All numeric functions MUST specify their behavior for mixed numeric types and overflow in
`stl.math`.

## Comparisons

Comparisons return booleans:

- `(> a b)`, `(< a b)`, `(>= a b)`, `(<= a b)`

The comparison domain for numbers MUST be total and consistent with numeric equality, including
handling of NaN as specified in “Equality and hashing”.

## Predicates

The standard library MUST provide at least:

- `(nil? x)`
- `(bool? x)`, `(int? x)`, `(uint? x)`, `(float? x)`
- `(string? x)`, `(char? x)`
- `(keyword? x)`, `(symbol? x)`
- `(list? x)`, `(vec? x)`, `(map? x)`, `(set? x)`
- `(empty? x)` for sequences/collections

## Casting

### `(cast T x)`

Per the type system, `cast` is the canonical operation used by typed dispatch `#T x`.

`cast` MUST:

- return a value of type `T` on success
- fail with TypeError on failure

The available conversions are specified in `stl.math`, `stl.str`, and related modules.

## Error model

The core library MUST define standard error values/types, including (at minimum):

- `TypeError`
- `ArityError`
- `UndefinedSymbolError`
- `IndexOutOfBoundsError`
- `DivisionByZeroError`
- `AssertionError`

How errors are represented (exception vs `result`) MUST be consistent across the standard library
and documented in `stl.result` and this book’s “Errors” chapter.
