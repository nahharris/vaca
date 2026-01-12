# Equality and hashing

This chapter specifies value equality and the requirements on hashing for hash-based collections.

## Equality predicate

The standard library defines a value equality predicate `==` (and its negation `!=`).

Semantically, `==` compares values for structural equality:

- Scalars compare by their scalar meaning.
- Collections compare by their elements/entries recursively.

Implementations MUST ensure `==` is an equivalence relation (reflexive, symmetric, transitive).

## Numeric equality

Vaca defines **numeric equality** across numeric types:

- An integer and a floating-point value compare equal if they denote the same mathematical number.
  Example: `1 == 1.0` is true.
- `0.0` and `-0.0` MUST compare equal.
- NaN handling: Vaca defines a canonical NaN equality where **NaN compares equal to NaN** for the
  purpose of `==` and hashing (see below).

This definition exists to allow numeric values (including floats) to participate consistently as
keys in hash maps and elements of hash sets.

## Collection equality

### Lists and vectors

Lists and vectors compare equal if and only if:

- they have the same length, and
- their elements are pairwise equal in order.

### Maps

Two maps compare equal if and only if:

- they contain the same set of keys (by `==`), and
- for each key, the associated values are equal.

### Sets

Two sets compare equal if and only if they contain the same elements (by `==`), irrespective of
order.

## Hashing requirements

If the language provides hash maps and hash sets, it MUST provide a hash function `hash` used by
those collections.

The hash function MUST satisfy:

- If `a == b`, then `hash(a) == hash(b)`.

In particular, the numeric equality rules above imply:

- `hash(0.0) == hash(-0.0)`
- `hash(NaN)` is a canonical value shared by all NaN payloads

Implementations MAY choose any hashing algorithm, as long as these constraints hold.
