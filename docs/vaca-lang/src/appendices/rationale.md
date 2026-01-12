# Rationale

This appendix records non-normative motivations for some design choices.

## EDN-derived surface

EDN provides:

- a small, regular syntax
- excellent readability for nested data
- a natural “code is data” story for macros

VEDN keeps EDN’s core while reserving dispatch forms needed by Vaca.

## Typed dispatch uses `#`

Vaca uses `#T x` as a uniform, syntax-light way to:

- annotate values with types
- request conversions
- express parameterized type ascriptions (`#(vec int) ...`)

The reader preserves typed structure and later phases assign meaning.

## Truthiness includes emptiness and zero

Vaca’s truthiness model treats several “empty” and “zero” values as falsy.

This supports concise idioms such as:

- guarding on non-empty sequences
- numeric computations that treat `0` as “false”

The cost is that some Clojure-style assumptions (“everything but nil/false is truthy”) do not hold.

## Module resolution with `@` and `$`

The module resolution rules are designed to be:

- predictable
- toolchain-configurable (library roots)
- explicit about “parent directory” traversal

Using `@` avoids ambiguity between project-relative and library-provided modules. Using `$` makes
parent traversal visually obvious and avoids special keywords like `super`.
