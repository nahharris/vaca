# `stl.result`

`stl.result` provides the `result` type and its combinators.

## Constructors and predicates

- `(result.ok x)`
- `(result.err e)`
- `(ok? r)`
- `(err? r)`

## Extractors

- `(unwrap r)` returns the ok value or fails with an error
- `(unwrap-or r default)`
- `(unwrap-or-else r f)`

## Combinators

- `(map f r)`
- `(map-err f r)`
- `(and-then r f)` (flat-map)
- `(or-else r f)`

## Interop with exceptions (optional)

If the implementation has exceptions, `stl.result` MAY provide:

- `(try f)` converting thrown errors into `(result ... ...)`

Such behavior MUST be specified precisely if present.
