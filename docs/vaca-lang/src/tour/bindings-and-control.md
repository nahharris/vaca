# Bindings and control

Vaca programs use bindings to name values and control forms to choose what to evaluate.

In a self-declaring Lisp, many of these constructs live in the standard prelude as macros, but the
spec defines their user-visible meaning precisely.

## Definitions

`def` introduces a top-level binding.

```clojure
(def #string language "vaca")
```

## Local bindings

`let` introduces local bindings for a body of forms:

```clojure
(let [x 10
      y 32]
  (+ x y))
```

## Conditionals

`if` chooses between two branches:

```clojure
(if (> n 0)
  "positive"
  "non-positive")
```

Truthiness is specified in the “Truthiness” chapter.

## Sequencing

`do` evaluates multiple forms in order and returns the last value:

```clojure
(do
  (println "side effect")
  42)
```
