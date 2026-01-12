# Functions

Functions are first-class values in Vaca.

## Calling a function

A call is written as a list form:

```clojure
(f 1 2 3)
```

The first element evaluates to a function value; the remaining elements are arguments.

## Anonymous functions

Anonymous functions are created with `fn`:

```clojure
(def add2 (fn [x] (+ x 2)))
(add2 40) ;; => 42
```

## Named functions

Named functions are defined with `defn`:

```clojure
(defn #int sum [#int a #int b]
  (+ a b))
```

This example uses typed dispatch (`#int`) to ascribe types to the function result and parameters.

## Arity

Functions have an arity: the number of arguments they accept.

This specification defines the observable behavior of calling functions with the wrong arity in
the “Errors” chapter.
