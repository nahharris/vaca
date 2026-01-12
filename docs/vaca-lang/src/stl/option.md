# `stl.option`

`stl.option` provides the `option` type and its combinators.

## Constructors and predicates

- `(option.none)`
- `(option.some x)`
- `(some? o)`
- `(none? o)`

## Extractors

- `(unwrap o)` returns the contained value or fails with an error
- `(unwrap-or o default)`
- `(unwrap-or-else o f)`

## Combinators

- `(map f o)`
- `(and-then o f)` (also known as flat-map)
- `(or-else o f)`

All functions MUST be specified in terms of `option` semantics from the type system section.
