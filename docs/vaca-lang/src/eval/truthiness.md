# Truthiness

Truthiness determines which values count as “false” in conditional contexts (e.g. `if`) and in
assertions.

## Truthy and falsy values

Vaca defines the following values as **falsy**:

- `nil`
- `false`
- the integer `0`
- the floating-point value `0.0` (including `-0.0`)
- the null character `\\0`
- the empty string `""`
- empty collections:
  - empty list `()`
  - empty vector `[]`
  - empty map `{}`
  - empty set `%{}`

All other values are **truthy**.

## Conditional context

In a conditional context, a value `v` is treated as:

- false if `v` is falsy (as defined above)
- true otherwise

## Rationale (non-normative)

This truthiness model supports idioms like:

- treating empty collections as falsey in guards
- treating numeric zero as falsey in guards

See the “Rationale” appendix for additional discussion.
