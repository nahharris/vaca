# `stl.core`

`stl.core` defines the fundamental functions, predicates, macros, and types that underpin most
Vaca programs and are available in the prelude (i.e. are automatically imported).

## `def`

`def` introduces a top-level binding. Its signature is:

```clojure
(defmacro def [symbol value]
               #options: [doc]
               #flags: [private])
```

## `defn`

`defn` defines a function. Its signature is:

```clojure
(defmacro defn [symbol params]
            #options: {doc "" options {} flags [] remainder nil}
            #flags: [private]
            #remainder: body)
```

## `fn`

`fn` defines a function. Its signature is:

```clojure
(defmacro fn [params]
            #options: {options {} flags [] remainder nil}
            #flags: [private]
            #remainder: body)
```

## `let`

`let` defines a local binding. Its signature is:

```clojure
(defmacro let [bindings]
            #remainder: body)
```

## `if`

`if` defines a conditional. Its signature is:
```clojure
(defmacro if [condition then else])
```

## `do`

`do` evaluates multiple forms in order and returns the last value. Its signature is:
```clojure
(defmacro do [] #remainder: body)
```

## `quote`

`quote` quotes a form. Its signature is:
```clojure
(defmacro quote [form])
```

## `|>`

`|>` is a macro that pipes the result of the first form to the second form, and so on. Its signature is:
```clojure
(defmacro |> [] #remainder: body)
```

## `loop`

`loop` defines a loop. Its signature is:
```clojure
(defmacro loop [bindings] #remainder: body)
(defmacro recur [args])
```

## `defmacro`

`defmacro` defines a macro. Its signature is:
```clojure
(defmacro defmacro [symbol params]
            #options: {options {} flags [] remainder nil}
            #remainder: body)
```


### `(& a b ...)` (short-circuit and)

`&` is a macro that evaluates its arguments left-to-right and returns:

- the first falsy value, if any
- otherwise the last argument value

If no arguments are provided, `&` returns `true`.

### `(| a b ...)` (short-circuit or)

`|` is a macro that evaluates its arguments left-to-right and returns:

- the first truthy value, if any
- otherwise the last argument value

If no arguments are provided, `|` returns `false`.

### `(assert x1 x2 ... xN)`

`assert` evaluates each argument left-to-right and fails with an AssertionError if any argument is
falsy.

The error MUST report the failing span and SHOULD include the original asserted form.

## Equality

### `(== a b)`

Returns `true` if `a` and `b` are equal under Vaca equality (see “Equality and hashing”).

### `(!= a b)`

Logical negation of `==`.

## Numeric operations

This section specifies the canonical arithmetic functions. Numeric tower details live in
`stl.math`, but these are always available.

- `(+ x1 ... xN)` addition (identity: `0`)
- `(* x1 ... xN)` multiplication (identity: `1`)
- `(- x)` negation
- `(- x y)` subtraction
- `(/ x y)` division (errors on division by zero unless otherwise specified)
- `(// x y)` integer division
- `(^ x y)` exponentiation
- `(mod x y)` remainder
- `(max x y)` maximum
- `(min x y)` minimum
- `(brt x y)` y-th root of x

All numeric functions MUST specify their behavior for mixed numeric types and overflow in
`stl.math`.

## Comparisons

Comparisons return booleans:

- `(> a b)`, `(< a b)`, `(>= a b)`, `(<= a b)`

The comparison domain for numbers MUST be total and consistent with numeric equality, including
handling of NaN as specified in “Equality and hashing”.

## Predicates

The standard library MUST provide at least:

- `(nil? x)`
- `(bool? x)`, `(int? x)`, `(uint? x)`, `(float? x)`
- `(string? x)`, `(char? x)`
- `(keyword? x)`, `(symbol? x)`
- `(list? x)`, `(vec? x)`, `(map? x)`, `(set? x)`
- `(empty? x)` for sequences/collections

## Casting

### `(cast T x)`

Per the type system, `cast` is the canonical operation used by typed dispatch `#T x`.

`cast` MUST:

- return a value of type `T` on success
- fail with TypeError on failure

The available conversions are specified in `stl.math`, `stl.str`, and related modules.

## Error model

The core library MUST define standard error values/types, including (at minimum):

- `TypeError`
- `ArityError`
- `UndefinedSymbolError`
- `IndexOutOfBoundsError`
- `DivisionByZeroError`
- `AssertionError`

How errors are represented (exception vs `result`) MUST be consistent across the standard library
and documented in `stl.result` and this book’s “Errors” chapter.
