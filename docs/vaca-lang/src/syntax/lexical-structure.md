# Lexical structure

This chapter specifies how a Vaca source file is broken into readable forms.

Vaca’s surface syntax is derived from EDN, with a small number of Vaca-specific conventions.

## Character set

Vaca source files are Unicode text. Implementations MUST accept UTF-8 source files.

## Whitespace and separators

Whitespace separates forms. The following characters are whitespace:

- Space (`U+0020`)
- Tab (`U+0009`)
- Newline (`U+000A`)
- Carriage return (`U+000D`)

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

## Reader discard

The discard form `##` will discard the next readable form.

- `## <form>` MUST be treated as if `<form>` did not appear in the source.
- Discard applies during reading, before evaluation and macro expansion.

Examples:

```clojure
[1 ## 2 3]         ; => [1 3]
## (+ 1 2) (+ 3 4) ; => (+ 3 4)
```

## Annotations

Annotations have the shape:

```text
#<form> <form>
```

This reduces to one single form during reading where the second form is annotated with the first form (the `#` annotation). 
The reader MUST NOT give any semantic meaning to the annotation.

```clojure
#uint 2 ; type-annotates the form as a uint
#color: {r: 0, g: 0, b: 0} ; kw-annotates a map as a color
```

## Delimiters

The following delimiters form collections:

- Lists: `(` … `)`
- Vectors: `[` … `]`
- Maps: `{` … `}`
- Sets: `%{` … `}`

An unterminated collection is a read-time error.

## Tokens

Vaca has the following token-like atoms (their internal structure is specified in subsequent
chapters):

- `nil`
- booleans: `true`, `false`
- numbers: integer and float literals
- strings: `"..."` with escape sequences
- characters: `\c`, `\newline`, …
- keywords: `:name`, `ns/name:`
- symbols: `name`, `ns/name`, and operator symbols like `+` and `<=`

The exact definitions for number, symbol, and keyword character classes SHOULD follow EDN unless
explicitly extended by this specification.
