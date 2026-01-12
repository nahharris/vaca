# Quotation

Quotation is the mechanism that allows Vaca programs to treat code as data.

## `quote`

### Syntax

```clojure
(quote form)
```

### Semantics

`quote` returns `form` **without evaluating it**.

The returned value is the ordinary data representation of the form (lists, vectors, symbols, …),
preserving structure and literals.

### Examples

```clojure
(quote (+ 1 2))     ; => a list value, not 3
(quote [a b c])     ; => a vector of symbols
(quote {:x 1 :y 2}) ; => a map
```

## Quasiquote

The standard prelude SHOULD provide quasiquotation to make macro writing ergonomic.

### Syntax (recommended)

- `(quasiquote form)` or a reader sugar such as `` `form `` (if the reader provides it)
- `(unquote form)` (recommended sugar: `~form`)
- `(unquote-splicing form)` (recommended sugar: `~@form`)

If reader sugars are provided, they MUST expand to the corresponding long forms.

### Semantics

Quasiquote recursively walks `form` and produces a new form, with escape hatches:

- `(unquote x)` evaluates `x` and inserts its result in place.
- `(unquote-splicing xs)` evaluates `xs` and splices its elements into the surrounding list/vector
  context.

Quasiquote is defined over Vaca’s ordinary data structures and is primarily used to construct
macro expansions.

### Examples

```clojure
(quasiquote (a (unquote x) c))
; => (a <value-of-x> c)

(quasiquote (a (unquote-splicing xs) c))
; if xs evaluates to [1 2], => (a 1 2 c)
```

The exact splicing contexts and error behavior (e.g. splicing a non-sequence) are specified by the
standard prelude and `stl.macro` utilities.
