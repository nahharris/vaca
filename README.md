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

## License

MIT
