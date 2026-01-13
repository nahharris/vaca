# Annotations

This chapter specifies reader directives that affect how forms are produced during reading.

## Annotations (`#`)

Annotations have the shape:

```text
#<form> <form>
```

This reduces to a single form during reading where the second form is annotated with the first
form (the `#` annotation).

The reader MUST NOT give any semantic meaning to the annotation; later stages MAY interpret it
(macro expansion, type checking, tooling).

Examples:

```clojure
#uint 2
#color: {r: 0, g: 0, b: 0}
```

## Reader discard (`##`)

The discard form `##` discards the next readable form.

- `## <form>` MUST be treated as if `<form>` did not appear in the source.
- Discard applies during reading, before evaluation and macro expansion.

Examples:

```clojure
[1 ## 2 3]         ; => [1 3]
## (+ 1 2) (+ 3 4) ; => (+ 3 4)
```

