# Functions

Functions are first-class values in Vaca.

## Calling a function

A call is written as a list form:

```clojure
(f 1 2 3)
```

The first element evaluates to a function value; the remaining elements are the arguments.

## Anonymous functions

Anonymous functions are created with the `fn` macro:

```clojure
((fn [x] (+ x 2)) 40) ;; => 42
```

## Named functions

Named functions are defined with `defn`:

```clojure
(defn #int sum [#int a #int b]
  (+ a b))

(sum 40 2) ;; => 42
```
> Notice how we use `#int` to type-annotate the function parameters `a` and `b` but also to 
> annotate the function result.

> The number of arguments a function accepts is called its **arity**. In the above `sum`example, 
> the arity is 2.
