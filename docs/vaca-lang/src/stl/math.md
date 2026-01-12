# `stl.math`

`stl.math` specifies the numeric tower and numeric operations beyond `stl.core`.

## Numeric tower

Conforming implementations MUST provide at least:

- `int`: signed integer
- `uint`: unsigned integer
- `float`: IEEE 754 floating point (at least 64-bit)

Implementations MAY provide:

- big integers (`bigint`)
- exact decimals (`decimal`)

If additional numeric types exist, they MUST integrate with:

- `cast` rules
- numeric equality
- comparison ordering

## Overflow and rounding

This specification requires that each implementation choose and document one of:

- **checked arithmetic**: overflow yields an error
- **wrapping arithmetic**: overflow wraps modulo word size
- **saturating arithmetic**: overflow clamps to min/max

The chosen behavior MUST be consistent across the STL.

For floating-point operations, rounding follows IEEE 754 semantics.

## Functions

The library MUST provide common functions such as:

- `abs`, `signum`
- `floor`, `ceil`, `round`, `trunc`
- `sqrt`, `cbrt`, `pow`, `log`, `exp`
- `sin`, `cos`, `tan`, `asin`, `acos`, `atan`
- constants: `pi`, `tau`, `e`

## Randomness (optional but recommended)

If `stl.math` provides randomness, it MUST define:

- a deterministic PRNG interface
- seeded vs system-entropy APIs
- reproducibility guarantees
