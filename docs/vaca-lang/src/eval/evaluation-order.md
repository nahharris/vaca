# Evaluation order

This chapter specifies how forms are evaluated, in what order, and how macro expansion fits into
the process.

## Phases (conceptual)

Vaca evaluation is defined in terms of two conceptual phases:

1. **Macro expansion**: repeatedly rewrite macro calls into other forms.
2. **Evaluation**: compute values from the resulting forms.

Implementations MAY interleave these phases (e.g. expand-on-demand), but the observable behavior
MUST match the semantics described here.

## Atomic forms

The following forms evaluate to themselves:

- `nil`
- booleans
- numbers
- strings
- characters
- keywords

## Symbols

A symbol evaluates by resolving it in the current environment. If no binding exists, evaluation
MUST fail with an undefined symbol error.

## Collection literals

### Vectors

Vector evaluation is strict:

- A vector literal evaluates to a vector value whose elements are the results of evaluating each
  element form from left to right.

### Maps

Map evaluation is strict:

- A map literal evaluates to a map value by evaluating key/value pairs from left to right.
- A map literal with an odd number of forms is a read-time error (see reader grammar).

### Sets

Set evaluation is strict:

- A set literal evaluates to a set value by evaluating each element form from left to right.

### Lists

Lists are the primary *code* form. A list literal evaluates as an application form unless quoted.

## Application

Given a non-empty list form:

```clojure
(op arg1 arg2 ... argN)
```

Evaluation proceeds as follows:

1. Determine whether the list is a macro call:
   - Evaluate `op` in the current environment to an operator value.
   - If the operator is a macro, perform macro expansion (see below).
2. Otherwise, treat it as a function call:
   - Evaluate `op` to a function value.
   - Evaluate each `argi` from left to right to a value.
   - Apply the function to the argument values.

If `op` does not evaluate to a callable value, evaluation MUST fail with a “not callable” error.

## Macro calls and expansion

A macro is a callable entity that receives **forms**, not evaluated values.

For a macro call:

```clojure
(m a b c)
```

Macro expansion is defined as:

- Resolve `m` to a macro value.
- Apply the macro to the **raw argument forms** `[a b c]` (not evaluated).
- The macro returns a form `E` (the expansion).
- The program meaning is the meaning of evaluating `E` in place of the macro call.

Expansion MAY repeat (if `E` is itself a macro call).

Errors during macro expansion are errors of the program and MUST be reported with source context.

## Typed dispatch in evaluation

Typed dispatch (`#T x`) has evaluation behavior defined by the type system and conversion rules.

At minimum:

- `#T x` MUST evaluate `x` and produce a value.
- The result MUST be checked and/or converted according to the semantics of `#T` specified in the
  “Type ascription and casts” chapter.
