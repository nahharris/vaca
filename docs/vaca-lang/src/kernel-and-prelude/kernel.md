# Kernel requirements

Vaca is designed to be **self-declaring**: most user-visible constructs live in the standard
prelude and standard library, implemented in Vaca itself.

To make that possible, every conforming implementation MUST provide a minimal **kernel**. This
kernel is not a “standard library”; it is the smallest set of primitives required to bootstrap the
prelude.

This chapter specifies the kernel in terms of **required capabilities**, not a particular VM or
compiler design.

## 1. Reader and form representation

The implementation MUST provide a reader for VEDN (see “Syntax and the reader”) that produces an
in-memory representation of forms that preserves:

- list/vector/map/set structure
- symbols, keywords, and literals
- typed dispatch nodes (`#T x`) as a structured pair `(ty, value)`
- source spans sufficient for diagnostics

The reader MUST NOT assign semantic meaning to types.

## 2. Environments and symbol resolution

The implementation MUST provide:

- lexical environments (or an equivalent mechanism) as specified in “Environments and scopes”
- symbol resolution with shadowing and undefined-symbol errors

## 3. Callables: functions and macros

The implementation MUST support:

- first-class functions (closures) that capture lexical environments
- application of functions to argument values
- first-class macros that:
  - receive **raw argument forms**
  - return an expansion form
  - can be applied during macro expansion

The macro expansion model is specified in “Macros and metaprogramming”.

## 4. Evaluation of core data forms

The implementation MUST support evaluation of:

- atomic forms (literals)
- symbols
- vectors, maps, and sets as strict-evaluation literals (see “Evaluation order”)
- lists as application forms (subject to macro expansion)

## 5. Module loading hook

The implementation MUST provide a module loading facility described by the module system
specification (see “Module identifiers and resolution”):

- resolve module identifiers using configured library roots, `@` prefix, and `$` parent segments
- read and expand module sources
- make module exports available for import

The implementation MAY cache modules, but caching MUST NOT change observable behavior.

## 6. Errors and diagnostics

The implementation MUST provide:

- the error classes specified in “Errors”
- source-aware diagnostics (file/module identity + span where applicable)

## 7. Guaranteed constant-space iteration (tail call facility)

Because Vaca targets functional programming, the language provides a standard way to express
iteration without unbounded stack growth.

The user-visible interface (`loop` and `recur`) is part of the standard prelude. However, to make
their semantics portable and efficient, the kernel MUST provide a **tail-call facility** that can
be invoked by prelude definitions.

Normative requirement:

- The kernel MUST provide a mechanism by which a computation in tail position can request a jump
  to a designated re-entry point (current function or loop) with a new set of arguments/bindings,
  without consuming additional call stack space.

This specification intentionally does not mandate how the mechanism is represented (bytecode
instruction, trampoline marker, CPS transform, etc.).

The surface semantics and tail-position rules are defined in the prelude chapter.
