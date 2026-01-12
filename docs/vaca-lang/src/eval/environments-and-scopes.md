# Environments and scopes

This chapter defines how names are bound and how symbol references are resolved during evaluation.

## Bindings

A **binding** associates a symbol name with a value (or, during compilation, with a typed entity
that will produce a value at runtime).

Bindings arise from:

- top-level definitions in a module
- local bindings (e.g. `let`-like constructs)
- function parameters
- macro parameters (during expansion)

## Environments

An **environment** is a mapping from symbol names to bindings, plus a reference to an optional
parent environment.

Implementations MUST support lexical environments with parent links (or an equivalent mechanism)
so that:

- inner scopes can shadow outer scopes
- a reference resolves to the nearest (innermost) binding with the same name

## Lexical scoping

Vaca is lexically scoped:

- The meaning of a symbol reference is determined by the structure of the program, not by the call
  stack at runtime.
- Functions are **closures**: they capture the lexical environment where they are created.

## Symbol resolution

When evaluating a symbol `s`:

- If `s` is bound in the current environment, its associated value is produced.
- Otherwise, if a parent environment exists, resolution continues in the parent.
- Otherwise, evaluation fails with an **undefined symbol** error.

Module-qualified symbols (e.g. `stl.str/trim`) are resolved according to the module system
specification (see “Module system”).

## Shadowing

If a new binding is introduced with a name that already exists in an outer scope, it shadows the
outer binding within the new scope.

Shadowing MUST be lexical and MUST NOT mutate the outer binding.

## Mutation

Vaca is functional-first. This specification treats mutation of bindings (reassignment) as either:

- not supported, or
- supported only via explicit, separately specified facilities (e.g. references/atoms)

The standard prelude and standard library SHOULD be designed assuming immutable values and
persistent data structures.
