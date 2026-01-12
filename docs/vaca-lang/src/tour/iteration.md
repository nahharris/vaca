# Iteration without stacks

Vaca encourages recursion and higher-order sequence operations, but it also provides a standard,
bounded way to express iteration without unbounded stack growth.

## Tail recursion (concept)

A function call is in **tail position** if it is the final action of a function body—i.e. the
calling function returns the callee’s result directly, without further work.

Many functional programs can be written with tail recursion by adding accumulator parameters.

## Loop/recur style

Vaca defines a conventional iteration pattern using `loop` and `recur`:

```clojure
(defn #int fac [#int n]
  (loop [n n acc 1]
    (if (< n 2)
      acc
      (recur (- n 1) (* acc n)))))
```

The precise rules of tail position and the semantics of `loop`/`recur` are specified later, in a
language-level way (independent of any single implementation strategy).
