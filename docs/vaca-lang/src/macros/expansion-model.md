# Macro expansion model

Macros transform forms into forms.

This chapter specifies the normative expansion model that all conforming implementations MUST
preserve, regardless of whether they are interpreters, compilers, or VMs.

## Macro identity

A **macro** is a callable entity `M` with the following properties:

- When applied, it receives **argument forms** (unevaluated).
- It executes in an expansion-time environment.
- It returns a **form** (the expansion).

## Expansion algorithm (normative)

Given an input form `F`, macro expansion proceeds as follows:

1. If `F` is not a list form, it expands to itself.
2. If `F` is an empty list `()`, it expands to itself.
3. If `F` is a non-empty list `(op a1 ... an)`:
   1. Resolve `op` in the current expansion environment.
   2. If the resolved value is a macro `M`:
      - Apply `M` to the raw forms `[a1 ... an]`.
      - Let the returned form be `E`.
      - Replace `F` by `E` and repeat expansion (i.e. macros expand recursively).
   3. Otherwise, `F` expands to itself.

Macro expansion MAY be defined as a fixed point (repeat until no macro call remains at the head).

## Expansion environment

Macro expansion uses lexical environments, similar to runtime evaluation.

Normative constraints:

- A macro MUST have access to the bindings of its definition environment (closure capture).
- A macro expansion MUST be evaluated/compiled in the caller’s environment after expansion.

## Expansion errors

If applying a macro fails (arity error, type error, explicit error), the program is invalid.
Implementations MUST report macro expansion errors with source location information for the macro
call site.

## Introspection utilities (recommended)

The standard library SHOULD provide tools such as:

- `macroexpand-1` and `macroexpand`
- `gensym`
- predicates like `macro?`, `form?`

These utilities belong in `stl.macro`.
