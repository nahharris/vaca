# Atoms

An **atom** is a non-collection form.

VEDN defines the set of atom kinds; their lexical details SHOULD follow EDN unless explicitly
extended by this specification.

## `nil` and booleans

- **`nil`**: the null / “no value” literal.
- **Booleans**: `true` and `false`.

These tokens are recognized as atoms only when delimited (by whitespace, commas, comments, or
collection delimiters). For example, `truex` is a symbol, not a boolean.

## Numbers

VEDN supports EDN-like integer and floating-point literals.

- **Integers**:
  - **Digits**: `0`, `42`
  - **Sign**: `+1`, `-1`
  - **Big integer suffix**: `42N`
  - **No leading zeros** (except for zero itself): `0` is valid; `01` is invalid.
- **Floating point**:
  - **Fractional**: `1.5`
  - **Exponent**: `1e9`, `1E-9`, `1e+9`
  - **Big decimal suffix**: `2.0M`

Notes:

- **EDN float forms require an integer part**, so `.1` is not a valid number token in VEDN.
- A token that *looks* like a number is parsed as a number; otherwise it is parsed as a symbol.

## Strings

Strings have the shape `"..."` and support escape sequences.

- **Escapes**:
  - `\t`, `\r`, `\n`
  - `\\` (backslash), `\"` (double quote)
  - `\uNNNN` (4 hex digits)
- **Unescaped characters**: any UTF-8 character other than `"` and `\`.

Strings are delimited by the surrounding double quotes. Unterminated strings are a parse error.

## Characters

Character literals have the shape `\c`, `\uNNNN`, or a named character.

- **Single character**: `\c` where `c` is any non-whitespace character.
- **Named characters**:
  - `\newline`
  - `\return`
  - `\space`
  - `\tab`
- **Unicode escape**: `\uNNNN` (4 hex digits).

Character tokens end at the first delimiter/separator after the leading `\`.

## Keywords

Keywords are identifier-like atoms used primarily as map keys.

VEDN accepts both keyword spellings from the reader grammar:

- **Leading-colon keywords**: `:<symbol>`
- **Trailing-colon keywords**: `<symbol>:`

The `<symbol>` part may also use **backtick-quoted symbols**, which allows whitespace and other
delimiter characters inside the keyword name:

- **Backtick keyword (leading-colon)**: `` :`...` ``
- **Backtick keyword (trailing-colon)**: `` `...`: ``

Examples:

```clojure
:name
ns/name:
:`Complex Keyword`
`Complex Keyword`:
```

## Symbols

Symbols name variables, functions, macros, and namespaces, and can include operator-like symbols.

In VEDN there are **two symbol spellings**:

- **Bare symbols**: EDN-like symbol tokens that are delimited by separators or collection
  delimiters (see the reader grammar in `appendices/grammar-ebnf.md`).
  - Bare symbols may include letters, digits, and a set of punctuation characters (operator-like
    symbols are therefore valid): `+`, `<=`, `foo-bar`, `my.ns/foo`.
  - Bare symbols may include **at most one** `/` to separate an optional namespace from the name:
    `ns/name`. The special case `/` alone is also a valid symbol.
  - Bare symbols **must not start with a digit** (e.g. `1foo` is invalid).
- **Backtick-quoted symbols** *(Vaca extension)*: any UTF-8 string enclosed by backticks is a
  symbol, including whitespace and delimiter characters.
  - Form: `` `...` ``
  - The contents are taken verbatim; the only terminating character is the closing backtick.
  - This form is intended for “complex” names that would otherwise be split into multiple tokens.
  - Backticks may also be used on *either side* of the namespace separator:
    - `` Some/`sym bol` ``: namespace `Some`, name `sym bol`
    - `` `So me`/symbol ``: namespace `So me`, name `symbol`
    - `` `So me`/`sym bol` ``: namespace `So me`, name `sym bol`
    - `` `So me/sym bol` ``: no namespace (the `/` is part of the name)

Examples:

```clojure
name
ns/name
+
<=
`Complex Symbol`
```

