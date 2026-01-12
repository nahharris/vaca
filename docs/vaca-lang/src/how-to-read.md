# How to read this book

This book has two jobs at once:

- **Teaching**: explain ideas with examples, intuition, and recommended idioms.
- **Specifying**: define the language precisely (what programs mean, what errors exist, and what
  compliant implementations MUST do).

## Normative language

The keywords **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are to be interpreted
as described in RFC 2119.

When this book says an implementation “MUST” do something, that requirement applies to any Vaca
toolchain (interpreter, compiler, VM) claiming conformance with this specification.

## Core terms

- **Form**: a syntactic element of a Vaca program (numbers, symbols, lists, vectors, maps, …).
- **Value**: a runtime result of evaluating a form.
- **Evaluation**: the process that maps forms to values.
- **Prelude**: standard definitions written in Vaca that are automatically available (or imported
  by convention) in most programs.
- **Kernel**: the minimal host-provided primitives required to bootstrap the prelude and standard
  library.

## Examples

Examples use Vaca syntax. When an example shows results, it uses an
informal REPL style:

```clojure
vaca> (+ 1 2)
3
```

Result formatting is illustrative unless stated otherwise.
