# List processing

Vaca is a LISP, which means that list processing is a core part of the language. This section specifies 
semantics for list processing.

## List literal

As specified in [Collections](../syntax/collections.md#list), lists are delimited forms that contain zero 
or more nested forms. Processing a list is done by evaluating the forms inside the list one by one, from 
left to right, finally applying the arguments (from the second element onwards) to the first form (which 
should be a callable value).

```clojure
(println "hello") ;; println is a callable function which is invoked with the argument "hello"
```

### Argument passing

There are 4 ways to interpret the arguments of a list:

- Positional arguments
- Optional arguments
- Flags (boolean arguments)
- Remainder arguments

#### Positional arguments

Positional arguments are mandatory arguments that are in the beginning of the argument list whose order matters.
They are passed to the function as is.

They are specified in the function definition as the first argument vector right after the function name.

```clojure
(defn #int sum [#int a #int b]
  (+ a b))

(sum 1 2) ;; => 3
```

#### Optional arguments

Optional arguments come right after the positional arguments and are indicated using keyword-annotations. 

They are specified in the function definition as an optional argument map called `#options:`. In this map
are defined both the argument name (as a symbol) and the default value (to be used when it's omitted).

```clojure
(defn #float sqrt [#float x] 
    #options: {#float rad 2.0} ;; Optional argument map
  (stl.math/pow x rad))

(sqrt 4) ;; => 2.0
(sqrt 8 #rad: 3) ;; => 2.0
```

#### Flags

Flags work as an alternative to optional arguments of the boolean type. When a flag is present, it is
evaluated to `true`, otherwise it is evaluated to `false`. They are passed as keywords right after the positional 
arguments. 

They are specified in the function definition as an optional vector of symbols called `#flags:`. There is no need
to specify a default value or type annotations since they are always booleans that default to `false`.

```clojure
(defn #(vec int) sort [#(vec int) v] 
    #flags: [reversed] ;; Optional vector of flags
    ...)

(sort [1 3 2] :reversed) ;; => [3 2 1]
(sort [1 3 2]) ;; => [1 2 3]
```

#### Remainder arguments

Remainder arguments provide an alternative syntax to pass a variable amount of arguments to a function instead
of using a vector. They are specified in the function definition as an optional symbol called `#remainder:`.

Every non-positional argument after the first non-optional, non-flag argument is considered a remainder argument.
It means that once you pass an argument that is not optional or a flag, all the ones that follow will
be considered as a remainder arguments even if they could be interpreted as either optional or flag.

```clojure
(defn #string format [] 
    #remainder: parts ;; Optional symbol for remainder arguments
    ...)

(format "hello " 42 " world") ;; => "hello 42 world"
```
