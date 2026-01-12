# `stl.io`

`stl.io` defines console I/O and basic stream operations.

## Console I/O

- `(readln)` reads a line from standard input and returns a string (without trailing newline).
  On EOF it SHOULD return `nil` or an `option`/`result` (MUST be specified by the API).
- `(print x1 x2 ... xN)` writes values to standard output without an implied newline.
- `(println x1 x2 ... xN)` writes values followed by a newline.

The conversion from values to text uses `stl.str/format` rules.

## Streams (recommended)

For larger programs, `stl.io` SHOULD provide stream interfaces:

- `Reader`, `Writer`
- buffered variants
- reading bytes and text with explicit encoding

These are specified if and when the corresponding module is included in the standard distribution.
