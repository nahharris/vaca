# Hello, Vaca

Vaca source code is a sequence of forms written in VEDN syntax. This first chapter gives a taste of the
language, the more in-depth details of the language are covered in the following chapters.

## A first program

```clojure
(use @stl.io [println]) ; Imports the `println` function from the `stl.io` module

(defn #void main [] ; Defines a `main` function
  (println "Hello World")) ; Prints "Hello World" to the console

(main) ; Calls the `main` function
```

## What is a form?

A *form* is one of:

- A literal (`1`, `3.14`, `"text"`, `true`, `nil`, `:keyword`, `\c`)
- A symbol (`x`, `println`, `stl.math/sin`)
- A collection form:
  - List: `(...)`
  - Vector: `[...]`
  - Map: `{k v ...}`
  - Set: `%{...}`

Most code is written using **list forms**, which look like function calls:

```clojure
(+ 1 2 3) ; Sums the numbers 1, 2, and 3
(println (format "n = " 42)) ; Creates a string "n = 42" and prints it to the console
```

## Evaluation

Evaluation computes a value from a form.

- Literals and collections (except lists) evaluate to themselves.
- Symbols evaluate by looking up a binding in the current environment.
- Lists evaluate by evaluating the operator (first element) and operands and applying the operator.

The precise evaluation rules are specified later in this book; this chapter just gives the mental
model needed to get started.
