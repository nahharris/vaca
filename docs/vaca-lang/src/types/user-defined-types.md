# User-defined types

Vaca supports user-defined types that can be used in typed dispatch and type checking.

This chapter specifies the core type-definition forms that the standard prelude MUST provide.

## `deftype`

### Syntax

```clojure
(deftype Name Super)
```

### Semantics

`deftype` introduces a new **nominal** type named `Name` that is a subtype of `Super`.

Normative requirements:

- `Name` MUST be a symbol.
- `Super` MUST be a type expression.
- Values of type `Name` are values of type `Super`.
- The prelude MUST define a way to construct values of the new type or to tag existing values as
  the new type (e.g. via `#Name x` cast rules), and MUST specify how equality and printing behave.

Example:

```clojure
(deftype text string)
```

## `union`

### Syntax

```clojure
(union T1 T2 ... Tn)
```

### Semantics

`(union T1 ... Tn)` denotes the type containing all values that are in at least one `Ti`.

## `struct`

### Syntax

```clojure
(struct :field1 T1 :field2 T2 ... :fieldN TN)
```

### Semantics

`struct` denotes a record type with keyword-named fields.

Normative requirements:

- Field keys MUST be keywords.
- Field keys MUST be unique.
- A value is an instance of the struct type if it provides all fields with values of the specified
  field types.

The standard library MUST define:

- constructors
- field access
- update operations

## `enum`

### Syntax (keyword variants)

```clojure
(enum :tag1 :tag2 ... :tagN)
```

### Semantics

`enum` denotes a sum type whose values are one of the listed tags.

Normative requirements:

- Each tag MUST be a keyword.
- Tags MUST be unique.

The standard library MUST define:

- constructors for each tag (or a uniform constructor)
- a way to test the active tag
- pattern matching facilities (recommended; see `stl.core/match` in the stdlib section)

## Derived algebraic types

The standard types `option` and `result` are specified separately. They MAY be defined in terms of
`enum`/`struct`/`union` by a particular toolchain, but their user-visible behavior is normative.
