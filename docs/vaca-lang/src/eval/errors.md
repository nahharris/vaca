# Errors

This chapter specifies the required classes of errors and the minimum diagnostic information a
conforming implementation MUST produce.

## Error phases

Errors can arise in multiple phases:

- **Read-time errors**: invalid syntax or malformed forms (e.g. unterminated string, map with odd
  number of forms).
- **Expansion-time errors**: errors encountered while expanding macros.
- **Compile-time errors** (for compilers): type errors, unresolved names, invalid module graphs.
- **Runtime errors**: undefined symbol, type mismatch, arity mismatch, out-of-bounds, etc.

An implementation MAY merge phases (e.g. an interpreter may type-check at runtime), but the error
categories and messages SHOULD remain consistent.

## Minimum diagnostic information

When an error is associated with a specific source location, the implementation MUST provide:

- a human-readable message
- the file/module identity if available
- a span (start/end) in the source text
- enough context to identify the failing form

When an error is not associated with a single location (e.g. module cycle), the implementation
MUST provide:

- a human-readable message
- relevant entities (e.g. the cycle path or involved modules)

## Required runtime error classes

The standard library error model MUST include at least:

- **UndefinedSymbol**: a referenced symbol has no binding in scope.
- **NotCallable**: a value was used in operator position but is not callable.
- **ArityError**: a callable was invoked with the wrong number of arguments.
- **TypeError**: a value does not satisfy a required type (including failed `#T` casts/ascriptions).
- **IndexOutOfBounds**: indexing a sequence/vector outside its bounds.
- **DivisionByZero**: division by zero where not otherwise defined.

Implementations MAY add additional error classes.

## Error values

Vaca provides an idiomatic error-handling story through standard types like `result` and `option`.
Those are specified in the standard library and type system chapters.

Some operations MAY also raise exceptions/panics as a control mechanism; if so, the semantics MUST
be specified in `stl.core` (and a catch mechanism MUST be specified if exceptions are user-visible).
