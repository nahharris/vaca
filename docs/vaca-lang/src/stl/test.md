# `stl.test`

`stl.test` defines testing helpers and conventions.

## Assertions

The module SHOULD provide rich assertions beyond `stl.core/assert`, such as:

- `(assert-eq a b)`
- `(assert-ne a b)`
- `(assert-true x)`
- `(assert-false x)`

## Test declaration (recommended)

If a test runner is provided, this module SHOULD define:

- `(deftest name body...)`
- `(run-tests ...)`

The exact runner integration is implementation-defined, but the semantics of the assertion helpers
are normative if provided.
