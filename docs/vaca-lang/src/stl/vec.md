# `stl.vec`

`stl.vec` defines operations on vectors (ordered, indexed collections).

## Core operations

- `(len v)` returns the number of elements.
- `(empty? v)` tests emptiness.
- `(nth i v)` returns the element at index `i` or fails with IndexOutOfBounds.
  The first element is index `0`.
- `(get i v)` returns `(option T)` rather than failing (recommended).

## Construction and update

Vectors are immutable; update operations return new vectors.

- `(conj v x)` returns a new vector with `x` appended.
  (If `conj` is generic over collections, its behavior per type MUST be specified.)
- `(conj v x)` returns a new vector with `x` appended.
  (If `conj` is generic over collections, its behavior per type MUST be specified.)
- `(concat v1 v2)` concatenates vectors.
- `(slice v start end)` returns a subvector.

## Higher-order operations

`stl.vec` MAY re-export sequence operations specialized to vectors:

- `(map f v)`
- `(filter pred v)`
- `(reduce f init v)`
