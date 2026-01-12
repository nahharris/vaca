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

### Symbols

Symbols are names for bindings that refer to functions, values, or types:

```clojure
x
println
stl.str/trim
```

### Keywords

Keywords begin or end with `:` and typically designate themselves:

```clojure
:name
user/id:

(== :kw kw:) ;; => true
```

> If a keyword both begins and ends with `:` it won't be the same the keyword that starts with `:` but not 
> ends with `:` (and vice versa).
> So `:kw:` is not the same as `:kw` or `kw:` or `kw` (the symbol).

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

## Annotations

Vaca uses `#` to annotate a form with another form as sort of adding metadata to the form. 
If you annotate a form with a form that evaluates to a type, it will be type-annotating the form.
This has 2 main uses:

1. Annotating a new symbol in its definition 
2. Casting a value

```clojure
(def #string name "Vaca") ; type-annotates the `name` binding as a string
(def age #uint 2026) ; casts the integer 2026 to an unsigned integer
```