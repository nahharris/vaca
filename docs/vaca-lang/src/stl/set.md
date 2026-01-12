# `stl.set`

`stl.set` defines operations on sets (unordered collections of unique elements).

Sets are immutable; update operations return new sets.

## Core operations

- `(empty? s)`
- `(len s)`
- `(contains? s x)`
- `(conj s x)` returns a new set with `x` included
- `(disj s x)` returns a new set without `x`

## Set algebra

- `(union a b ...)`
- `(intersection a b ...)`
- `(difference a b)` (elements in `a` not in `b`)

## Relationship to hashing

Sets rely on `hash` and `==` consistency as specified in “Equality and hashing”.
