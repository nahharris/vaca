# Values and literals

This chapter introduces Vaca’s core data model as a programmer experiences it.

## Scalar literals

### Nil

`nil` denotes the absence of a value.

### Booleans

`true` and `false` are boolean literals.

### Numbers

Vaca has numeric literals for integers and floating-point numbers.

```clojure
42
-7
3.14
-0.5
```

Numeric tower details (sizes, overflow behavior, exactness) are specified in the standard library
modules and the type system chapters.

### Strings

Strings are written with double quotes:

```clojure
"hello"
"line1\nline2"
```

### Characters

Characters are written with a leading backslash:

```clojure
\a
\newline
```

### Keywords

Keywords begin with `:` and typically designate themselves:

```clojure
:name
:user/id
```

### Symbols

Symbols name bindings and refer to functions, values, and types:

```clojure
x
println
stl.str/trim
```

## Collection literals

### Lists

Lists are written with parentheses: `(...)`.

Lists are primarily used as code forms, but they are also ordinary data.

### Vectors

Vectors are written with brackets: `[...]`.

Vectors are the primary literal for ordered, indexed sequences.

### Maps

Maps are written with braces: `{k v ...}` and must contain an even number of forms.

### Sets

Sets are written with `%{...}`.

## Typed dispatch (preview)

Vaca uses EDN’s `#` dispatch syntax to express typing at the surface level:

```clojure
#uint 1
#string "hello"
#(vec int) [1 2 3]
```

Typed dispatch is specified precisely in the “Typed dispatch” and “Type system” parts of this
book.
