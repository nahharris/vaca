# Forms and grammar (VEDN)

This chapter defines the **Vaca Extensible Data Notation** (VEDN): the surface syntax used to
represent Vaca programs as a sequence of forms.

VEDN is derived from EDN. A Vaca source file is a sequence of forms (there is no required single
top-level delimiter).

## Forms

A **form** is one of:

- `nil`
- boolean
- number
- string
- character
- symbol
- keyword
- list
- vector
- map
- set
- annotation

## Collections

### List

Syntax: `(<form>*)`

Lists are used both as code and as data.

### Vector

Syntax: `[<form>*]`

Vectors are the primary ordered collection literal.

### Map

Syntax: `{(<form> <form>)*}`

Maps MUST contain an even number of forms. If a map contains an odd number of forms, it is a
read-time error.

### Set

Syntax: `%{<form>*}`

## EBNF (reader-level)

This EBNF specifies the reader-level structure. It intentionally treats atoms (numbers, symbols,
keywords) abstractly; their lexical details are constrained by EDN-like rules.

```text
Document        ::= Sep* (Form Sep*)*

Sep             ::= Whitespace | Comma | Comment
Whitespace      ::= " " | "\t" | "\n" | "\r"
Comma           ::= ","
Comment         ::= ";" (not_newline)* ("\n" | EOF)

Form            ::= Nil
                  | Boolean
                  | Number
                  | String
                  | Character
                  | Keyword
                  | Symbol
                  | List
                  | Vector
                  | Map
                  | Set
                  | Annotation
                  | Discard

Nil             ::= "nil"
Boolean         ::= "true" | "false"
Keyword         ::= ":" Symbol | Symbol ":"

List            ::= "(" Sep* (Form Sep*)* ")"
Vector          ::= "[" Sep* (Form Sep*)* "]"
Map             ::= "{" Sep* (Form Sep* Form Sep*)* "}"
Set             ::= "%{" Sep* (Form Sep*)* "}"

Annotation      ::= "#" Form Sep* Form

Discard         ::= "##" Sep* Form
```

Notes:

- A compliant reader MUST preserve the structural distinction of annotated forms for later stages
  (macro expansion, type checking). The reader MUST NOT assign semantic meaning to annotations.
