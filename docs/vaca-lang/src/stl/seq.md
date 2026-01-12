# `stl.seq`

`stl.seq` defines sequence abstractions and higher-order operations.

Vaca favors a small set of general, composable operations.

## Sequence concept

A value is **seqable** if it can be viewed as a sequence of elements.

Seqable values include (at least):

- vectors
- lists
- strings (as a sequence of characters)
- map entry sequences (keys/values/entries)

The library SHOULD define a protocol or set of conventions to obtain a sequence view.

## Core functions

### Mapping and filtering

- `(map f xs)` returns a new sequence by applying `f` to each element.
- `(filter pred xs)` returns a new sequence of elements where `(pred x)` is truthy.
- `(keep f xs)` applies `f` and keeps only truthy (or non-`nil`) results (must be specified).

### Folding

- `(reduce f init xs)` folds left.
  Equivalent to `f(...f(f(init, x0), x1)..., xn)`.
- `(reduce f init xs)` folds left.
  Equivalent to `f(...f(f(init, x0), x1)..., xn)`.
- `(scan f init xs)` returns the intermediate accumulator values.

### Slicing

- `(take n xs)`, `(drop n xs)`
- `(take-while pred xs)`, `(drop-while pred xs)`

### Searching

- `(any? pred xs)` returns true if any element satisfies pred.
- `(all? pred xs)` returns true if all elements satisfy pred.
- `(find pred xs)` returns `(option T)` for the first match.

### Construction

- `(concat xs ys)` concatenates sequences.
- `(range start end step)` produces a numeric range.
- `(repeat x n)` repeats a value.

## Laziness vs strictness

This specification allows two models:

- **strict sequences**: operations eagerly produce realized vectors/lists.
- **lazy sequences**: operations produce lazy seqs.

If laziness is supported, the STL MUST specify:

- when evaluation occurs
- how side effects are handled
- resource safety (e.g. closing files)

Until otherwise specified, examples assume strict results for clarity.
