# Syntax overview

Vaca programs are written as **a sequence of forms**. A source file does not require a single
top-level delimiter.

This chapter set specifies the **reader**: the stage that turns Unicode text into forms. Vaca’s
surface syntax is called **VEDN** (Vaca Extensible Data Notation) and is derived from EDN, with a
small number of Vaca-specific conventions.

## Character set

Vaca source files are Unicode text. Implementations MUST accept UTF-8 source files.

## What this section covers

- [Separators](./separators.md) (whitespace, commas, comments).
- [Atoms](./atoms.md) (numbers, strings, symbols, keywords, …).
- [Collections](./collections.md) (lists, vectors, maps, sets).
- [Reader directives](./annotations.md) (annotations and discard).

## Grammar

The grammar is available in full inthe [complete grammar (EBNF)](../appendices/grammar-ebnf.md) appendix.

