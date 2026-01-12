# Introduction

Vaca is a functional, Lisp-family programming language with a clean EDN-derived surface syntax.
It is designed to be small at the core, extensible by nature, and pleasant for both scripting and
large programs.

This book is the **normative specification** of the Vaca language.

## Design goals

- **Simple syntax, powerful abstraction**: a small set of orthogonal concepts with a uniform
  representation (forms) and strong composability.
- **Functional-first**: immutable data by default, first-class functions, and a standard library
  centered on higher-order operations.
- **Self-declaring**: almost all language constructs are definable in Vaca itself as part of the
  standard prelude and standard library; the host toolchain provides only a minimal kernel needed
  for bootstrapping.
- **Tooling-friendly**: readable syntax, stable semantics, and precise error reporting.

## Non-goals

- **Accidental complexity**: the language avoids multiple competing syntaxes for the same concept.
- **Spec tied to any single implementation**: this specification is independent of any specific 
 implementation.

## A minimal taste

```clojure
(use @stl.io [println])

(defn #void main []
  (println "Hello, Vaca"))

(main)
```

Vaca programs are made of **forms**: lists, vectors, maps, symbols, numbers, and so on. Most code
is written as list forms, where the first element denotes an operation and the remaining elements
are its operands.
