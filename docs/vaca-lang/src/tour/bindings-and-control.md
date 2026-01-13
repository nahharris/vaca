# Bindings and control

Vaca programs use bindings to name values and control forms to choose what to evaluate.

In a self-declaring Lisp, many of these constructs live in the standard prelude as macros, but the
spec defines their user-visible meaning precisely.

## Definitions

`def` introduces a top-level binding.

```clojure
(def #string language "vaca")
```

> Top-level bindings can only be accessed from other modules when they are imported with the `use` macro.

## Local bindings

`let` introduces local bindings for a body of forms:

```clojure
(let {x 10
      y 32}
  (+ x y)) ;; => 42
(println x) ;; => undefined symbol: x
```

The symbols `x` and `y` are local to the `let` form and are not accessible outside of it.

## Conditionals

`if` chooses between two branches and only evaluating one of them:

```clojure
(if (> n 0)
  "positive"
  "non-positive")
```

## Sequencing

`do` is a special form that evaluates multiple forms in order and returns the last value:

```clojure
(do
  (println "some operation")
  42) ;; => 42
```
