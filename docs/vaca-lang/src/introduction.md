# Introduction

Vaca (from portuguese "cow") is a functional, Lisp-family programming language with a clean EDN-derived 
syntax called VEDN. It is designed to be small at the core, extensible by nature, and pleasant for both 
scrips and larger programs.

This book is the **normative specification** of the Vaca language.

## Design goals

- **Simple syntax, powerful abstraction**: a small set of orthogonal concepts with a uniform
  representation (forms) and strong composability.
- **Functional-first**: immutable data by default, first-class functions, and a standard library
  centered on higher-order operations.
- **Self-declaring**: almost all language constructs can be defined in Vaca itself built on top
  of a minimal kernel.
- **Tooling-friendly**: readable syntax, stable semantics, and precise error reporting.

## A minimal taste

```clojure
(use @stl.io [println]) ; Imports the `println` function from the `stl.io` module

(defn #void main [] ; Defines a `main` function
  (println "Hello, Vaca")) ; Prints "Hello, Vaca" to the console

(main) ; Calls the `main` function
```
