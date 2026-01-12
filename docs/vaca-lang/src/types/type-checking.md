# Type checking

This chapter specifies what it means for a Vaca program to be well-typed and what guarantees a
conforming toolchain MUST provide.

## Gradual typing model

Vaca supports a gradual typing style:

- Programs MAY include type ascriptions (`#T`) and type definitions.
- Programs MAY omit types; omitted types are treated as `any` for static purposes.

Toolchains MAY provide:

- static type checking (compile-time), and/or
- dynamic type checking (runtime),

but they MUST enforce the same **type rules** and produce equivalent errors for invalid programs.

## Core obligations of a conforming implementation

At minimum, a conforming implementation MUST enforce:

- **typed dispatch casts** (`#T x`) according to the cast rules
- **typed binding positions** (typed parameters, typed returns) according to the cast rules
- **type constructor well-formedness** (e.g. `(struct ...)` has keyword field names, even arity)

## Inference

This specification does not require global type inference.

Implementations MAY perform local inference, but it MUST NOT change program meaning.

## Soundness boundary

Because types can be omitted (defaulting to `any`), type checking provides sound guarantees only
where types are present.

Normative guarantee:

- If a program successfully type-checks (statically) under a toolchain’s rules, and if the program
  is executed under a conforming runtime, then no TypeError due to `#T` casts or typed bindings may
  occur at runtime.

## Error reporting

Type errors MUST be reported with:

- a message identifying the expected and actual types
- source location (span) covering the violating form

