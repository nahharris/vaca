# `stl.map`

`stl.map` defines operations on maps (key/value dictionaries).

Maps are immutable; update operations return new maps.

## Core operations

- `(empty? m)`
- `(len m)`
- `(contains? m k)` returns boolean
- `(get m k)` returns `(option V)`
- `(get-or m k default)` returns V

## Update operations

- `(assoc m k v)` returns a new map where `k` maps to `v`
- `(dissoc m k)` returns a new map without `k`
- `(merge m1 m2 ...)` merges maps (rightmost wins)

## Views

- `(keys m)` returns a sequence of keys
- `(vals m)` returns a sequence of values
- `(entries m)` returns a sequence of `[k v]` pairs
