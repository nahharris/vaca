# `stl.time`

`stl.time` defines time and date abstractions.

## Types

At minimum, the STL SHOULD provide:

- `instant`: a point on the UTC timeline
- `duration`: a non-negative or signed time span (MUST specify)

If calendar types exist (date, datetime, timezone), their semantics MUST be specified explicitly
(including leap seconds handling).

## Core operations

- `(now)` → `instant`
- `(duration ms)` → `duration` (constructor; units MUST be explicit or encoded in the name)
- `(add inst dur)` → `instant`
- `(sub inst dur)` → `instant`
- `(diff a b)` → `duration`

## Parsing and formatting

- `(parse-instant s)` → `(result instant ParseError)`
- `(format-instant inst)` → `string`
