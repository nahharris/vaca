# Separators

Separators define how a source file is split into readable forms.

## Whitespace

Whitespace separates forms. The following characters are whitespace:

- Space (`U+0020`)
- Tab (`U+0009`)
- Newline (`U+000A`)
- Carriage return (`U+000D`)

## Commas

Additionally, **comma** (`,`, `U+002C`) is treated as whitespace (a separator) and has no semantic
meaning. This allows formatting like:

```clojure
{x: 1, y: 2, z: 3}
```

## Comments

`;` starts a line comment. A line comment continues until the next newline or the end of file.

Comments are treated as whitespace and do not produce forms.

```clojure
; this is a comment
(+ 1 2) ; also a comment
```

