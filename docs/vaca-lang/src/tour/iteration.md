# Iteration

Vaca has many different ways to iterate (i.e. do something multiple times). The most basic ones are:

## `map`

Map is used when you have a collection of items and you want to apply a function to each item producing a new collection.

```clojure
(map (fn [x] (* x 2)) [1 2 3]) ;; => [2 4 6]
```

## `filter`

Filter is used when you have a collection of items and you want to keep only the items that satisfy a condition.

```clojure
(filter (fn [x] (> x 5)) [1 2 3 4 5 6 7 8 9 10]) ;; => [6 7 8 9 10]
```

## `loop` and `recur`

Loop and recur are used when you want to iterate an indefinite number of times, you then use `recur` to indicate 
when to iterate again, and you return some value when its time to stop.

```clojure
(loop [n 0] ; Loop with one parameter `n` initialized to 0
  (if (> n 10)
    n ; If `n` is greater than 10, return `n`
    (recur (+ n 1)))) ; Otherwise increment `n` and iterate again
```