# Hello, Vaca

Vaca source code is a sequence of forms written in an EDN-derived syntax.

## A first program

```clojure
(use @stl.io [println])

(defn #void main []
  (println "Hello World"))

(main)
```

## What is a form?

A *form* is one of:

- A literal (`1`, `3.14`, `"text"`, `true`, `nil`, `:kw`, `\c`)
- A symbol (`x`, `println`, `math/sin`)
- A collection form:
  - List: `(...)`
  - Vector: `[...]`
  - Map: `{k v ...}`
  - Set: `%{...}`

Most code is written using **list forms**, which look like function calls:

```clojure
(+ 1 2 3)
(println (format "n = " 42))
```

## Evaluation

Evaluation computes a value from a form.

- Literals evaluate to themselves.
- Symbols evaluate by looking up a binding in the current environment.
- Lists generally evaluate by evaluating the operator and operands and applying the operator.

The precise evaluation rules are specified later in this book; this chapter just gives the mental
model needed to get started.
