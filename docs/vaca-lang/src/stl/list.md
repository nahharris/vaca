# `stl.list`

`stl.list` defines operations on lists.

Lists are commonly used for code-as-data and for simple recursive processing.

## Core operations

- `(empty? xs)`
- `(len xs)` (may be O(n) unless lists are sized)
- `(head xs)` returns first element as `(option T)` (recommended)
- `(tail xs)` returns remaining list as `(option (list T))` (recommended)
- `(cons x xs)` constructs a new list by prepending `x` to `xs`

## Iteration

List processing is often expressed with recursion or `reduce`/`fold`.

If the implementation provides lazy seqs, lists MAY participate as seqs.
