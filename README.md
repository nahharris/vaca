# The Vaca Programming Language

> Vaca is still alpha.
> Don't use it for serious projects.
> Lot of ground breaking changes may happen.
> No backwards compatibility guaranteed.

Vaca is a LISP language

"Vaca" comes from the portuguese word for "cow". It was born as a sandboxing for a future bigger project, but now Vaca has its own ambitions. Currently targeting a tree-walker interpreter, Vaca aims to be dynamic and expansible by nature.

It's syntax derives from [EDN](https://github.com/edn-format/edn).

## Example

```clojure
(use stl.io [println])

(defn #void main []
  (println "Hello World"))
```
## Typing

Vaca has a very expressive type system that enables the developer to have some level of guarantees, writting code that is reliable and maintainable.

Types are expressions just like any other value. The thing is that we can use `#` syntax to force types onto some forms.

```clojure
#uint 1 ; casts the integer 1 to an unsigned integer
(def #string name "Vaca Language") ; Sets the type of the constant `name` to string
```

Types can be defined with the `deftype` macro:

```clojure
(deftype text string) ; Creates a new type called `text` that is a subtype of `string`
(deftype number (union int float)) ; Creates a type that is a union of `int` and `float`
(deftype person (struct :name string :age uint))
```

### `any`

The largest type, every value is an instance of `any`.

### `void`

The smallest type, where no value is present.

## License

MIT
