# Complete grammar (EBNF)

This appendix collects the normative reader-level grammar for VEDN.

The grammar is intentionally focused on structural forms. Lexical details of symbols, keywords,
numbers, and strings follow EDN conventions unless specified otherwise.

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
                  | Dispatch

Nil             ::= "nil"
Boolean         ::= "true" | "false"

List            ::= "(" Sep* (Form Sep*)* ")"
Vector          ::= "[" Sep* (Form Sep*)* "]"
Map             ::= "{" Sep* (Form Sep* Form Sep*)* "}"

Dispatch        ::= Set
                  | Discard
                  | Typed

Set             ::= "%{" Sep* (Form Sep*)* "}"
Discard         ::= "#_" Sep* Form

Typed           ::= "#" Type Sep* Form
Type            ::= Symbol | TypeList
TypeList        ::= "#(" Sep* (Form Sep*)* ")"
```

Conformance notes:

- A map MUST contain an even number of forms (key/value pairs); otherwise it is a read-time error.
- `Discard` removes the following form from the input stream as if it were not present.
- `Typed` forms MUST be preserved structurally; semantic meaning is assigned by later phases.
