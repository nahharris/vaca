# Glossary

- **Binding**: association from a name (symbol) to a value.
- **Callable**: a value that can be applied to arguments (function or macro).
- **Closure**: a function value that captures its lexical environment.
- **Document**: a sequence of VEDN forms (a source file).
- **Environment**: mapping from names to bindings, with optional parent link.
- **Expansion**: the result of applying a macro to forms; a form that replaces the macro call.
- **Form**: a syntactic element (literal, symbol, list, vector, map, set, typed form).
- **Kernel**: minimal host-provided capabilities required to bootstrap the prelude.
- **Module**: a compilation/loading unit associated with a source file.
- **Prelude**: canonical definitions (in Vaca) that provide the everyday surface language.
- **Reader**: component that parses source text into forms (VEDN).
- **Scope**: region of program text where a binding is visible.
- **Seqable**: a value that can be viewed as a sequence for `stl.seq` operations.
- **Tail position**: a program position where the result of a form is returned directly.
- **Typed form**: a `#T x` pair produced by the reader, used for type-directed checking/conversion.
