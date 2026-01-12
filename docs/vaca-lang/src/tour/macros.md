# Macros

Macros are compile-time (or expand-time) functions that transform code (forms) into other code.

They are a major part of Vaca’s extensibility story: many “language constructs” can be defined as
macros in the standard prelude.

## Code is data

Vaca code is made of ordinary data structures (lists, vectors, symbols, …). Quotation prevents a
form from being evaluated and instead treats it as data:

```clojure
(quote (+ 1 2)) ;; a list value, not the number 3
```

## Defining a macro

At a high level:

- A macro receives **forms** (not evaluated values) as inputs.
- It returns a new form (its *expansion*).
- The expansion is then evaluated in place of the original macro call.

The exact expansion model is specified in the “Macros and metaprogramming” section.
