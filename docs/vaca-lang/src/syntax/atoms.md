# Atoms

An **atom** is a non-collection form.

VEDN defines the set of atom kinds; their lexical details SHOULD follow EDN unless explicitly
extended by this specification.

## `nil` and booleans

- `nil`
- `true`, `false`

## Numbers

VEDN supports integer and float literals (EDN-like).

## Strings

Strings have the shape `"..."` and support escape sequences (EDN-like).

## Characters

Character literals have the shape `\c` or a named character like `\newline` (EDN-like).

## Keywords

Keywords are identifier-like atoms used primarily as map keys.

Examples:

```clojure
:name
ns/name:
```

## Symbols

Symbols name variables, functions, macros, and namespaces, and can include operator-like symbols.

Examples:

```clojure
name
ns/name
+
<=
```

