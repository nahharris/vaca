# `stl.edn`

`stl.edn` defines parsing and printing of EDN/VEDN data.

This module is useful for configuration, tooling, and metaprogramming.

## Relationship to the reader

The reader specified by this book parses VEDN into forms. `stl.edn` exposes this capability as an
API usable from programs.

Normative requirement:

- `stl.edn` MUST provide a way to parse a string into a sequence of forms (or a single form),
  preserving typed dispatch nodes and spans where possible.

## API (recommended)

- `(edn.read s)` → `(result (vec form) edn-error)`
- `(edn.read1 s)` → `(result form edn-error)` (read one form)
- `(edn.print x)` → `string`

