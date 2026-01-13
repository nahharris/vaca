# `stl.macro`

`stl.macro` is a *documentation namespace* used by the standard library reference to describe
macro signatures and their expected syntactic forms.

It names the kinds of forms a macro accepts at expansion time (before evaluation), e.g. “this
parameter must be a symbol” or “this parameter is a vector of forms”.

## Common form types

- `stl.macro/Form`: any form
- `stl.macro/Symbol`: a symbol form
- `stl.macro/Keyword`: a keyword form
- `stl.macro/List`: a list form
- `stl.macro/Vector`: a vector form
- `stl.macro/Map`: a map form
- `stl.macro/StringLiteral`: a string literal form

## Typed wrapper

Some macro parameters are annotated as “typed” to express a specific subtype of a broader form:

```clojure
#(stl.macro/Typed stl.macro/Symbol)
```

This indicates “a form that must be a symbol”, while still being a syntactic annotation (not a
runtime cast).

