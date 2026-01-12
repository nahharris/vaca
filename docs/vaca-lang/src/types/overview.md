# Overview

Vaca has an expressive type system designed to support reliable programs without sacrificing the
flexibility and simplicity of a Lisp.

Types are **first-class**: they can be named, composed, and (where specified) manipulated by
programs and tooling.

## Type expressions

A **type expression** is a form that denotes a type. Common type expression shapes include:

- a type name symbol (e.g. `int`, `string`, `any`)
- a parameterized type constructor (e.g. `(vec int)`, `(map keyword any)`)
- a structural type (e.g. `(struct :x int :y int)`)
- an algebraic type (e.g. `(union int float)`, `(enum :a :b :c)`)

The canonical library of type constructors is specified in this part and in `stl.core`.

## Top and bottom

### `any`

`any` is the top type: every value is an instance of `any`.

### `void`

`void` is the bottom type: it has no values.

`void` is commonly used to annotate functions that return no meaningful value.

## Primitive types (core)

The standard prelude and standard library define the following primitive types:

- `nil`
- `bool`
- `int`
- `uint`
- `float`
- `char`
- `string`
- `keyword`
- `symbol`

Collection and callable types are also part of the core vocabulary:

- `list`
- `vec`
- `map`
- `set`
- function types `(fn [T1 T2 ...] Tr)`

Precise operational semantics for these types and conversions live in the corresponding standard
library modules.
